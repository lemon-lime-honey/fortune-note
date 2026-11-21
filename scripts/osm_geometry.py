"""Pure-Python point-in-polygon containment for OSM relation geometries."""

from dataclasses import dataclass


@dataclass(frozen=True)
class PolygonBoundary:
    osm_id: int
    level: str
    full_name: str
    province: str
    center_lat: float
    center_lon: float
    min_lat: float
    max_lat: float
    min_lon: float
    max_lon: float
    segments: tuple[tuple[float, float, float, float], ...]

    def contains(self, lat: float, lon: float) -> bool:
        if not (self.min_lat <= lat <= self.max_lat and self.min_lon <= lon <= self.max_lon):
            return False
        inside = False
        for y1, x1, y2, x2 in self.segments:
            if (y1 > lat) != (y2 > lat):
                x_hit = x1 + (lat - y1) * (x2 - x1) / (y2 - y1)
                if lon < x_hit:
                    inside = not inside
        return inside


def extract_relation_segments(element: dict) -> list[tuple[float, float, float, float]]:
    segments: list[tuple[float, float, float, float]] = []
    for member in element.get("members", []):
        if member.get("type") != "way":
            continue
        role = member.get("role", "")
        if role not in ("outer", "inner", ""):
            continue
        geom = member.get("geometry")
        if not isinstance(geom, list) or len(geom) < 2:
            continue
        for idx in range(len(geom) - 1):
            p1, p2 = geom[idx], geom[idx + 1]
            segments.append((float(p1["lat"]), float(p1["lon"]), float(p2["lat"]), float(p2["lon"])))
    return segments


def build_polygon(
    element: dict,
    level: str,
    full_name: str,
    province: str,
    center_lat: float,
    center_lon: float,
) -> PolygonBoundary | None:
    segments = extract_relation_segments(element)
    if not segments:
        return None
    lats = [s[0] for s in segments] + [s[2] for s in segments]
    lons = [s[1] for s in segments] + [s[3] for s in segments]
    return PolygonBoundary(
        osm_id=int(element["id"]),
        level=level,
        full_name=full_name,
        province=province,
        center_lat=center_lat,
        center_lon=center_lon,
        min_lat=min(lats),
        max_lat=max(lats),
        min_lon=min(lons),
        max_lon=max(lons),
        segments=tuple(segments),
    )


def find_containing_polygon(
    lat: float,
    lon: float,
    level7_polys: list[PolygonBoundary],
    level6_polys: list[PolygonBoundary],
) -> PolygonBoundary | None:
    for poly in level7_polys:
        if poly.contains(lat, lon):
            return poly
    for poly in level6_polys:
        if poly.contains(lat, lon):
            return poly
    # Fallback only for edge points slightly outside coastal polygons (e.g., piers/islands)
    pool = level7_polys + level6_polys
    if not pool:
        return None
    return min(pool, key=lambda p: (p.center_lat - lat) ** 2 + ((p.center_lon - lon) * 0.81) ** 2)
