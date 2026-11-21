#!/usr/bin/env python3
"""
Generate bundled offline location seed CSV for Fortune Note Android app.

Sources (100% official OpenStreetMap Overpass API, ODbL, Python stdlib only):
  1. South Korea administrative boundaries (L4 시·도, L6 시·군·자치구, L7 일반구) with exact polygon geometry
     and official MOIS administrative codes (`ref:KR:mois:admin`).
  2. South Korea 행정동(L8), 법정동(`boundary=legal`), and railway/subway/GTX stations (`railway=station|halt`),
     matched to their exact containing 시·군·구 via Jordan-curve ray-casting point-in-polygon.
  3. Curated verified world major cities (Korean & English names).

Output:
  android/app/src/main/assets/locations_seed.csv
  Columns: query,displayName,latitude,longitude
"""

import csv
import json
import sys
import time
import urllib.parse
import urllib.request
from pathlib import Path

from osm_geometry import PolygonBoundary, build_polygon, find_containing_polygon
from world_cities_data import WORLD_MAJOR_CITIES

ROOT_DIR = Path(__file__).resolve().parent.parent
OUTPUT_CSV = ROOT_DIR / "android" / "app" / "src" / "main" / "assets" / "locations_seed.csv"

OVERPASS_ENDPOINTS = [
    "https://overpass-api.de/api/interpreter",
    "https://overpass.private.coffee/api/interpreter",
]

USER_AGENT = "FortuneNoteSeedBuilder/0.1.0 (offline-ephemeris-dataset)"

BOUNDARIES_QUERY = """
[out:json][timeout:120];
area["ISO3166-1"="KR"][admin_level=2]->.kr;
rel["boundary"="administrative"]["admin_level"="4"](area.kr);
out center tags;
rel["boundary"="administrative"]["admin_level"~"^(6|7)$"](area.kr);
out geom;
"""

LEAF_QUERY = """
[out:json][timeout:120];
area["ISO3166-1"="KR"][admin_level=2]->.kr;
(
  rel["boundary"="administrative"]["admin_level"="8"](area.kr);
  rel["boundary"="legal"](area.kr);
  node["railway"~"^(station|halt)$"](area.kr);
  way["railway"~"^(station|halt)$"](area.kr);
  rel["railway"~"^(station|halt)$"](area.kr);
);
out center tags;
"""

KR_PROVINCE_SHORT = {
    "서울특별시": "서울",
    "부산광역시": "부산",
    "대구광역시": "대구",
    "인천광역시": "인천",
    "광주광역시": "광주",
    "대전광역시": "대전",
    "울산광역시": "울산",
    "세종특별자치시": "세종",
    "경기도": "경기",
    "강원특별자치도": "강원",
    "강원도": "강원",
    "충청북도": "충북",
    "충청남도": "충남",
    "전북특별자치도": "전북",
    "전라북도": "전북",
    "전라남도": "전남",
    "경상북도": "경북",
    "경상남도": "경남",
    "제주특별자치도": "제주",
}

MOIS_PROVINCE_PREFIX = {
    "11": "서울특별시",
    "26": "부산광역시",
    "27": "대구광역시",
    "28": "인천광역시",
    "29": "광주광역시",
    "30": "대전광역시",
    "31": "울산광역시",
    "36": "세종특별자치시",
    "41": "경기도",
    "42": "강원특별자치도",
    "43": "충청북도",
    "44": "충청남도",
    "45": "전북특별자치도",
    "46": "전라남도",
    "47": "경상북도",
    "48": "경상남도",
    "50": "제주특별자치도",
    "51": "강원특별자치도",
    "52": "전북특별자치도",
}

ADMIN_SUFFIXES = ("특별시", "광역시", "특별자치시", "시", "군", "구")


def sanitize(text: str) -> str:
    return " ".join(text.replace(",", " ").replace("\t", " ").replace("\n", " ").replace("\r", " ").split()).strip()


def has_hangul(text: str) -> bool:
    return any("\uac00" <= ch <= "\ud7a3" for ch in text)


def is_in_korea_bbox(lat: float, lon: float) -> bool:
    return 32.5 <= lat <= 39.0 and 124.0 <= lon <= 132.5


def extract_coords(el: dict) -> tuple[float | None, float | None]:
    if "lat" in el and "lon" in el:
        return float(el["lat"]), float(el["lon"])
    center = el.get("center")
    if isinstance(center, dict) and "lat" in center and "lon" in center:
        return float(center["lat"]), float(center["lon"])
    bounds = el.get("bounds")
    if isinstance(bounds, dict) and "minlat" in bounds and "maxlat" in bounds:
        return (
            (float(bounds["minlat"]) + float(bounds["maxlat"])) / 2.0,
            (float(bounds["minlon"]) + float(bounds["maxlon"])) / 2.0,
        )
    return None, None


def strip_admin_suffix(name: str) -> str:
    for suffix in ADMIN_SUFFIXES:
        if len(name) > len(suffix) + 1 and name.endswith(suffix):
            return name[: -len(suffix)]
    return name


def normalize_station_name(raw_name: str) -> str:
    name = sanitize(raw_name)
    if not name:
        return ""
    if "(" in name and ")" in name:
        base = name.split("(")[0].strip()
        if base and has_hangul(base):
            name = base
    if has_hangul(name) and not name.endswith("역"):
        return f"{name}역"
    return name


def fetch_overpass_json(query: str, label: str) -> list[dict]:
    payload = urllib.parse.urlencode({"data": query}).encode("utf-8")
    last_err = None
    for endpoint in OVERPASS_ENDPOINTS:
        for attempt in range(2):
            try:
                req = urllib.request.Request(
                    endpoint,
                    data=payload,
                    headers={"User-Agent": USER_AGENT},
                    method="POST",
                )
                print(f"[{label}] Fetching from {endpoint} (attempt {attempt + 1})...", flush=True)
                with urllib.request.urlopen(req, timeout=180) as resp:
                    data = json.loads(resp.read().decode("utf-8"))
                    elements = data.get("elements", [])
                    if not elements:
                        raise RuntimeError(f"Empty elements returned (remark={data.get('remark')})")
                    return elements
            except Exception as exc:
                last_err = exc
                print(f"[{label}] Warning: {endpoint} failed: {exc}", file=sys.stderr, flush=True)
                time.sleep(3)
    raise RuntimeError(f"Overpass fetch failed for {label}: {last_err}")


def get_mois_code(tags: dict) -> str:
    for key in ("ref:KR:mois:admin", "ref:KR:mois:legal", "ref"):
        val = str(tags.get(key, "")).strip()
        if len(val) >= 5 and val[:5].isdigit():
            return val
    return ""


def partition_boundary_elements(
    elements: list[dict],
) -> tuple[list[tuple[str, str, float, float]], dict[str, tuple[str, str]], list[tuple], list[tuple]]:
    prov_rows: list[tuple[str, str, float, float]] = []
    city_by_prefix4: dict[str, tuple[str, str]] = {}
    level6_items: list[tuple] = []
    level7_items: list[tuple] = []

    for el in elements:
        tags = el.get("tags", {})
        lvl = tags.get("admin_level", "")
        name_ko = sanitize(tags.get("name:ko") or tags.get("name") or "")
        lat, lon = extract_coords(el)
        if not name_ko or lat is None or lon is None or not is_in_korea_bbox(lat, lon):
            continue
        name_en = sanitize(tags.get("name:en") or "")
        code = get_mois_code(tags)

        if lvl == "4":
            short_p = KR_PROVINCE_SHORT.get(name_ko, name_ko)
            prov_rows.append((sanitize(f"{short_p} {name_ko} {name_en}"), name_ko, lat, lon))
        elif lvl == "6":
            prov = MOIS_PROVINCE_PREFIX.get(code[:2], "")
            if len(code) >= 4:
                city_by_prefix4[code[:4]] = (prov, name_ko)
            level6_items.append((el, name_ko, name_en, prov, lat, lon))
        elif lvl == "7":
            level7_items.append((el, name_ko, name_en, code, lat, lon))

    return prov_rows, city_by_prefix4, level6_items, level7_items


def build_level6_polys(
    level6_items: list[tuple],
) -> tuple[list[tuple[str, str, float, float]], list[PolygonBoundary]]:
    rows: list[tuple[str, str, float, float]] = []
    polys: list[PolygonBoundary] = []
    for el, name_ko, name_en, prov, lat, lon in level6_items:
        full_name = f"{prov} {name_ko}".strip() if prov and prov != name_ko else name_ko
        short_p = KR_PROVINCE_SHORT.get(prov, "")
        rows.append((sanitize(f"{strip_admin_suffix(name_ko)} {name_ko} {short_p} {name_en}"), full_name, lat, lon))
        poly = build_polygon(el, "6", full_name, prov, lat, lon)
        if poly is not None:
            polys.append(poly)
    return rows, polys


def build_level7_polys(
    level7_items: list[tuple],
    city_by_prefix4: dict[str, tuple[str, str]],
    level6_polys: list[PolygonBoundary],
) -> tuple[list[tuple[str, str, float, float]], list[PolygonBoundary]]:
    rows: list[tuple[str, str, float, float]] = []
    polys: list[PolygonBoundary] = []
    for el, name_ko, name_en, code, lat, lon in level7_items:
        prov, city_name = city_by_prefix4.get(code[:4], ("", ""))
        if not prov:
            parent = find_containing_polygon(lat, lon, [], level6_polys)
            if parent is not None:
                prov = parent.province
                city_name = parent.full_name.removeprefix(prov).strip()
        full_name = " ".join(dict.fromkeys(p for p in (prov, city_name, name_ko) if p))
        short_p = KR_PROVINCE_SHORT.get(prov, "")
        kw = sanitize(f"{strip_admin_suffix(name_ko)} {name_ko} {city_name} {short_p} {name_en}")
        rows.append((kw, full_name, lat, lon))
        poly = build_polygon(el, "7", full_name, prov, lat, lon)
        if poly is not None:
            polys.append(poly)
    return rows, polys


def merge_station_into(deduped: list[dict], st: dict) -> None:
    for existing in deduped:
        same_place = (
            existing["name"] == st["name"]
            and abs(existing["lat"] - st["lat"]) < 0.004
            and abs(existing["lon"] - st["lon"]) < 0.004
        )
        if not same_place:
            continue
        if st["name_en"] and st["name_en"] not in existing["name_en"]:
            existing["name_en"] = sanitize(f"{existing['name_en']} {st['name_en']}")
        if st["network"] and st["network"] not in existing["network"]:
            existing["network"] = sanitize(f"{existing['network']} {st['network']}")
        return
    deduped.append(st)


def format_station_rows(dedup_stations: list[dict]) -> list[tuple[str, str, float, float]]:
    rows: list[tuple[str, str, float, float]] = []
    for st in dedup_stations:
        display_name = f"{st['name']} ({st['region']})" if st["region"] else st["name"]
        base = st["name"][:-1] if st["name"].endswith("역") and len(st["name"]) > 2 else st["name"]
        kw = sanitize(f"{st['name']} {base} {st['name_en']} {st['network']}")
        rows.append((kw, display_name, st["lat"], st["lon"]))
    return rows


def process_leaf_element(
    el: dict,
    level7_polys: list[PolygonBoundary],
    level6_polys: list[PolygonBoundary],
    dong_rows: list[tuple[str, str, float, float]],
    dedup_stations: list[dict],
) -> None:
    tags = el.get("tags", {})
    name_ko = sanitize(tags.get("name:ko") or tags.get("name") or "")
    lat, lon = extract_coords(el)
    if not name_ko or lat is None or lon is None or not is_in_korea_bbox(lat, lon):
        return

    parent = find_containing_polygon(lat, lon, level7_polys, level6_polys)
    region = parent.full_name if parent else ""
    short_p = KR_PROVINCE_SHORT.get(parent.province, "") if parent else ""
    name_en = sanitize(tags.get("name:en") or "")

    if tags.get("railway") in ("station", "halt"):
        st_name = normalize_station_name(name_ko) if has_hangul(name_ko) else ""
        if st_name:
            network = sanitize(tags.get("network") or tags.get("operator") or "")
            merge_station_into(
                dedup_stations,
                {"name": st_name, "name_en": name_en, "network": network, "region": region, "lat": lat, "lon": lon},
            )
        return

    boundary = tags.get("boundary", "")
    if (boundary == "administrative" and tags.get("admin_level") == "8") or boundary == "legal":
        full_name = f"{region} {name_ko}".strip()
        dong_rows.append((sanitize(f"{name_ko} {short_p} {name_en}"), full_name, lat, lon))


def deduplicate_all_rows(rows: list[tuple[str, str, float, float]]) -> list[tuple[str, str, str, str]]:
    seen: set[tuple[str, float, float]] = set()
    final_rows: list[tuple[str, str, str, str]] = []
    for kw, display_name, lat, lon in rows:
        if not display_name or not (-90.0 <= lat <= 90.0) or not (-180.0 <= lon <= 180.0):
            continue
        key = (display_name, round(lat, 3), round(lon, 3))
        if key in seen:
            continue
        seen.add(key)
        final_rows.append((sanitize(kw), sanitize(display_name), f"{lat:.4f}", f"{lon:.4f}"))
    return final_rows


def write_and_verify_csv(final_rows: list[tuple[str, str, str, str]]) -> None:
    OUTPUT_CSV.parent.mkdir(parents=True, exist_ok=True)
    with OUTPUT_CSV.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(["query", "displayName", "latitude", "longitude"])
        writer.writerows(final_rows)

    size_kb = OUTPUT_CSV.stat().st_size / 1024.0
    print(f"[SUCCESS] Wrote {len(final_rows)} locations to {OUTPUT_CSV} ({size_kb:.1f} KB)", flush=True)
    for keyword in ("킨텍스역", "강남역", "대화동", "역삼동", "판교역", "해운대구", "뉴욕"):
        matches = [r for r in final_rows if keyword in r[0] or keyword in r[1]]
        print(f"  Check '{keyword}': {len(matches)} matches -> {matches[:2]}", flush=True)


def main() -> None:
    t0 = time.time()
    boundary_elements = fetch_overpass_json(BOUNDARIES_QUERY, "KR_BOUNDARIES")
    prov_rows, city_by_prefix4, level6_items, level7_items = partition_boundary_elements(boundary_elements)
    l6_rows, level6_polys = build_level6_polys(level6_items)
    l7_rows, level7_polys = build_level7_polys(level7_items, city_by_prefix4, level6_polys)
    admin_rows = prov_rows + l6_rows + l7_rows
    print(
        f"[KR_BOUNDARIES] admin_rows={len(admin_rows)}, L7_polys={len(level7_polys)}, L6_polys={len(level6_polys)}",
        flush=True,
    )

    time.sleep(2)
    leaf_elements = fetch_overpass_json(LEAF_QUERY, "KR_LEAF")
    dong_rows: list[tuple[str, str, float, float]] = []
    dedup_stations: list[dict] = []
    for el in leaf_elements:
        process_leaf_element(el, level7_polys, level6_polys, dong_rows, dedup_stations)
    station_rows = format_station_rows(dedup_stations)
    print(f"[KR_LEAF] dongs={len(dong_rows)}, stations={len(station_rows)} (elapsed {time.time() - t0:.1f}s)", flush=True)

    final_rows = deduplicate_all_rows(admin_rows + dong_rows + station_rows + WORLD_MAJOR_CITIES)
    write_and_verify_csv(final_rows)


if __name__ == "__main__":
    main()
