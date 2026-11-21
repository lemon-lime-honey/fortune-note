use serde::{Deserialize, Serialize};
use vedaksha::ephem::bodies::Body;
use vedaksha::ephem::julian::calendar_to_jd;

use crate::celestial::{
    format_jd_utc_seconds, jd_to_calendar, normalize_western_body_name, CalendarDateTime,
    GeoCoordinate,
};
use crate::ephemeris::{compute_ephemeris, EphemerisSnapshot};
use crate::timezone::resolve_local_to_utc_with_tz;
use crate::western::aspects::is_aspect_applying;
use super::{
    body_from_name, find_aspect_with_target, get_body_tropical_lon,
    get_body_tropical_lon_and_speed, separation_error,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitContact {
    pub transiting_planet: String,
    pub transiting_longitude: f64,
    pub natal_planet: String,
    pub natal_longitude: f64,
    pub aspect_type: String,
    pub orb: f64,
    #[serde(default)]
    pub is_applying: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitPlanetSnapshot {
    pub name: String,
    pub tropical_longitude: f64,
    pub speed: f64,
    pub is_retrograde: bool,
    pub sign_index: u32,
    pub sign_degree: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitEventHit {
    pub event_jd: f64,
    pub utc_date_time: String,
    pub moving_body: String,
    pub moving_longitude: f64,
    pub moving_speed: f64,
    pub is_retrograde: bool,
    pub natal_point: String,
    pub natal_longitude: f64,
    pub aspect_type: String,
    pub orb_deg: f64,
    pub snapshot_planets: Vec<TransitPlanetSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InOrbTransitDay {
    pub date_formatted: String,
    pub jd: f64,
    pub trigger_contacts: Vec<TransitContact>,
    pub snapshot_planets: Vec<TransitPlanetSnapshot>,
    pub all_active_transits: Vec<TransitContact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitSearchQuery {
    pub start_year: i32,
    pub start_month: u32,
    pub start_day: u32,
    pub end_year: i32,
    pub end_month: u32,
    pub end_day: u32,
    pub moving_bodies: Option<Vec<String>>,
    pub natal_targets: Option<Vec<String>>,
    pub aspect_types: Option<Vec<String>>,
}

pub const NATAL_CUSP_NAMES: [&str; 12] = [
    "Cusp 1", "Cusp 2", "Cusp 3", "Cusp 4", "Cusp 5", "Cusp 6",
    "Cusp 7", "Cusp 8", "Cusp 9", "Cusp 10", "Cusp 11", "Cusp 12",
];

const DEFAULT_MOVING_BODIES: [&str; 6] = ["Saturn", "Jupiter", "Mars", "Sun", "Venus", "Mercury"];

const ASPECT_TARGET_DEFS: [(&str, &[f64]); 5] = [
    ("Conjunction", &[0.0]),
    ("Sextile", &[60.0, 300.0]),
    ("Square", &[90.0, 270.0]),
    ("Trine", &[120.0, 240.0]),
    ("Opposition", &[180.0]),
];

/// Flattened (natal_point_name, natal_lon, aspect_name, target_separation) entry.
struct TargetAspectSpec {
    natal_name: String,
    natal_lon: f64,
    aspect_name: &'static str,
    target_sep: f64,
}

struct PreparedTransitSearch {
    start_jd: f64,
    end_jd: f64,
    moving_bodies: Vec<(String, Body)>,
    target_specs: Vec<TargetAspectSpec>,
}

fn prepare_transit_search(
    query: &TransitSearchQuery,
    natal_points: &[(&str, f64)],
) -> Result<PreparedTransitSearch, String> {
    let start_jd = calendar_to_jd(query.start_year, query.start_month, query.start_day as f64);
    let end_jd = calendar_to_jd(query.end_year, query.end_month, (query.end_day as f64) + 0.9999);
    if end_jd <= start_jd {
        return Err("End date must be after start date".to_string());
    }

    let moving_names: Vec<&str> = match query.moving_bodies {
        Some(ref list) => list.iter().map(String::as_str).collect(),
        None => DEFAULT_MOVING_BODIES.to_vec(),
    };
    let moving_bodies = moving_names
        .into_iter()
        .filter_map(|name| body_from_name(name).map(|b| (normalize_western_body_name(name), b)))
        .collect();

    let filtered_natal = filter_natal_points(natal_points, query.natal_targets.as_deref());
    let mut target_specs = Vec::new();
    for (nat_name, nat_lon) in filtered_natal {
        let norm_natal = normalize_western_body_name(nat_name);
        for &(asp_name, targets) in &ASPECT_TARGET_DEFS {
            let active = query
                .aspect_types
                .as_ref()
                .is_none_or(|sel| sel.iter().any(|s| s.eq_ignore_ascii_case(asp_name)));
            if active {
                for &target_sep in targets {
                    target_specs.push(TargetAspectSpec {
                        natal_name: norm_natal.clone(),
                        natal_lon: nat_lon,
                        aspect_name: asp_name,
                        target_sep,
                    });
                }
            }
        }
    }

    Ok(PreparedTransitSearch {
        start_jd,
        end_jd,
        moving_bodies,
        target_specs,
    })
}

fn build_transit_snapshots(ephem: &EphemerisSnapshot) -> Vec<TransitPlanetSnapshot> {
    ephem
        .bodies
        .iter()
        .map(|b| {
            let sign_idx = ((b.tropical_longitude / 30.0).floor() as u32) % 12;
            TransitPlanetSnapshot {
                name: normalize_western_body_name(&b.name),
                tropical_longitude: b.tropical_longitude,
                speed: b.speed,
                is_retrograde: b.is_retrograde,
                sign_index: sign_idx,
                sign_degree: b.tropical_longitude - (sign_idx as f64 * 30.0),
            }
        })
        .collect()
}

fn collect_ephem_transit_contacts(
    ephem: &EphemerisSnapshot,
    natal_planets: &[(&str, f64)],
    max_orb: f64,
) -> Vec<TransitContact> {
    let mut contacts = Vec::new();
    for tb in &ephem.bodies {
        for &(natal_name, natal_lon) in natal_planets {
            if let Some((asp, target_angle, orb)) =
                find_aspect_with_target(tb.tropical_longitude, natal_lon, max_orb)
            {
                contacts.push(TransitContact {
                    transiting_planet: normalize_western_body_name(&tb.name),
                    transiting_longitude: tb.tropical_longitude,
                    natal_planet: normalize_western_body_name(natal_name),
                    natal_longitude: natal_lon,
                    aspect_type: asp.to_string(),
                    orb,
                    is_applying: is_aspect_applying(tb.tropical_longitude, tb.speed, natal_lon, 0.0, target_angle),
                });
            }
        }
    }
    contacts
}

/// Calculate transits for a target local moment and coordinates, propagating timezone errors explicitly.
pub fn calculate_transits_with_tz(
    target_dt: CalendarDateTime,
    loc: GeoCoordinate,
    natal_planets: &[(&str, f64)],
    timezone_id: Option<&str>,
) -> Result<Vec<TransitContact>, String> {
    let tz_res = resolve_local_to_utc_with_tz(target_dt, loc, timezone_id)
        .map_err(|e| format!("Transit timezone resolution failed: {}", e))?;
    let ephem = compute_ephemeris(tz_res.utc_datetime(), loc)
        .map_err(|e| format!("Transit ephemeris failed: {}", e))?;
    Ok(collect_ephem_transit_contacts(&ephem, natal_planets, 2.0))
}

pub fn matches_natal_target(target_spec: &str, point_name: &str) -> bool {
    let t = target_spec.trim();
    if t.eq_ignore_ascii_case("cusps") || t.eq_ignore_ascii_case("all_cusps") {
        return point_name.starts_with("Cusp")
            || point_name.starts_with('H')
            || matches!(point_name, "ASC" | "MC" | "IC" | "DSC");
    }
    t.eq_ignore_ascii_case(point_name)
        || (t.eq_ignore_ascii_case("ASC") && (point_name == "Cusp 1" || point_name == "H1"))
        || (t.eq_ignore_ascii_case("MC") && (point_name == "Cusp 10" || point_name == "H10"))
        || (t.eq_ignore_ascii_case("IC") && (point_name == "Cusp 4" || point_name == "H4"))
        || (t.eq_ignore_ascii_case("DSC") && (point_name == "Cusp 7" || point_name == "H7"))
        || t.replace(' ', "").eq_ignore_ascii_case(&point_name.replace(' ', ""))
}

fn filter_natal_points<'a>(
    natal_points: &[(&'a str, f64)],
    targets_opt: Option<&[String]>,
) -> Vec<(&'a str, f64)> {
    natal_points
        .iter()
        .copied()
        .filter(|(name, _)| {
            targets_opt.is_none_or(|targets| targets.iter().any(|t| matches_natal_target(t, name)))
        })
        .collect()
}

fn bisect_exact_transit_jd(
    vbody: Body,
    mut left_t: f64,
    mut right_t: f64,
    mut left_e: f64,
    nat_lon: f64,
    target_sep: f64,
) -> f64 {
    for _ in 0..12 {
        let mid_t = (left_t + right_t) * 0.5;
        let Ok(mid_lon) = get_body_tropical_lon(vbody, mid_t) else {
            break;
        };
        let mid_e = separation_error(mid_lon, nat_lon, target_sep);
        if mid_e.abs() < 1e-4 {
            return mid_t;
        }
        if left_e * mid_e <= 0.0 {
            right_t = mid_t;
        } else {
            left_t = mid_t;
            left_e = mid_e;
        }
    }
    (left_t + right_t) * 0.5
}

type RawTransitHit = (f64, String, String, f64, String, f64);

fn scan_body_transit_hits(
    moving_name: &str,
    vbody: Body,
    step_days: f64,
    prep: &PreparedTransitSearch,
) -> Vec<RawTransitHit> {
    let Ok(mut prev_lon) = get_body_tropical_lon(vbody, prep.start_jd) else {
        return Vec::new();
    };
    let mut hits = Vec::new();
    let mut cur_t = prep.start_jd;

    while cur_t < prep.end_jd {
        let next_t = (cur_t + step_days).min(prep.end_jd);
        let Ok(next_lon) = get_body_tropical_lon(vbody, next_t) else {
            cur_t = next_t;
            continue;
        };
        for spec in &prep.target_specs {
            let e0 = separation_error(prev_lon, spec.natal_lon, spec.target_sep);
            let e1 = separation_error(next_lon, spec.natal_lon, spec.target_sep);
            if e0.abs() < 12.0 && e1.abs() < 12.0 && (e0 * e1 <= 0.0) && (e0 != e1) {
                let hit_jd = bisect_exact_transit_jd(vbody, cur_t, next_t, e0, spec.natal_lon, spec.target_sep);
                hits.push((
                    hit_jd,
                    moving_name.to_string(),
                    spec.natal_name.clone(),
                    spec.natal_lon,
                    spec.aspect_name.to_string(),
                    spec.target_sep,
                ));
            }
        }
        cur_t = next_t;
        prev_lon = next_lon;
    }
    hits
}

/// Search exact transit aspect hits within a bounded time interval.
pub fn find_transit_events(
    query: &TransitSearchQuery,
    natal_points: &[(&str, f64)],
) -> Result<Vec<TransitEventHit>, String> {
    let prep = prepare_transit_search(query, natal_points)?;
    let has_moon = prep.moving_bodies.iter().any(|(n, _)| n == "Moon");
    let step_days = if has_moon { 0.25 } else { 0.5 };

    let mut raw_hits = Vec::new();
    for (moving_name, vbody) in &prep.moving_bodies {
        raw_hits.extend(scan_body_transit_hits(moving_name, *vbody, step_days, &prep));
    }
    raw_hits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut unique_hits: Vec<RawTransitHit> = Vec::new();
    for hit in raw_hits {
        let is_dup = unique_hits.iter().any(|u| {
            u.1 == hit.1 && u.2 == hit.2 && u.4 == hit.4 && (u.0 - hit.0).abs() < 0.15
        });
        if !is_dup {
            unique_hits.push(hit);
        }
    }

    let origin = GeoCoordinate::new(0.0, 0.0);
    let mut hits = Vec::with_capacity(unique_hits.len());
    for (hit_jd, moving_name, nat_name, nat_lon, asp_name, target_sep) in unique_hits {
        let dt = jd_to_calendar(hit_jd);
        if let Ok(ephem) = compute_ephemeris(dt, origin) {
            let snapshots = build_transit_snapshots(&ephem);
            let m_snap = snapshots.iter().find(|s| s.name == moving_name);
            let (moving_lon, moving_spd, moving_rx) =
                m_snap.map_or((0.0, 0.0, false), |s| (s.tropical_longitude, s.speed, s.is_retrograde));
            hits.push(TransitEventHit {
                event_jd: hit_jd,
                utc_date_time: format_jd_utc_seconds(hit_jd),
                moving_body: moving_name,
                moving_longitude: moving_lon,
                moving_speed: moving_spd,
                is_retrograde: moving_rx,
                natal_point: nat_name,
                natal_longitude: nat_lon,
                aspect_type: asp_name,
                orb_deg: separation_error(moving_lon, nat_lon, target_sep).abs(),
                snapshot_planets: snapshots,
            });
        }
    }
    Ok(hits)
}

fn collect_day_triggers(
    cur_jd: f64,
    prep: &PreparedTransitSearch,
    orb_limit: f64,
) -> Vec<TransitContact> {
    let mut triggers = Vec::new();
    for (m_name, vbody) in &prep.moving_bodies {
        let Ok((m_lon, m_speed)) = get_body_tropical_lon_and_speed(*vbody, cur_jd) else {
            continue;
        };
        for spec in &prep.target_specs {
            let err = separation_error(m_lon, spec.natal_lon, spec.target_sep).abs();
            if err <= orb_limit {
                let base_angle = if spec.target_sep > 180.0 { 360.0 - spec.target_sep } else { spec.target_sep };
                triggers.push(TransitContact {
                    transiting_planet: m_name.clone(),
                    transiting_longitude: m_lon,
                    natal_planet: spec.natal_name.clone(),
                    natal_longitude: spec.natal_lon,
                    aspect_type: spec.aspect_name.to_string(),
                    orb: err,
                    is_applying: is_aspect_applying(m_lon, m_speed, spec.natal_lon, 0.0, base_angle),
                });
            }
        }
    }
    triggers
}

/// Scan every day within the bounded range and return transit snapshots where transiting planets touch targets.
pub fn find_in_orb_transit_days(
    query: &TransitSearchQuery,
    natal_points: &[(&str, f64)],
    orb_limit: f64,
) -> Result<Vec<InOrbTransitDay>, String> {
    let prep = prepare_transit_search(query, natal_points)?;
    let origin = GeoCoordinate::new(0.0, 0.0);
    let mut result_days = Vec::new();
    let mut cur_jd = prep.start_jd + 0.5;

    while cur_jd <= prep.end_jd {
        let triggers = collect_day_triggers(cur_jd, &prep, orb_limit);
        if !triggers.is_empty() {
            let dt = jd_to_calendar(cur_jd);
            let noon_dt = CalendarDateTime::new(dt.year, dt.month, dt.day, 12, 0, 0.0);
            if let Ok(ephem) = compute_ephemeris(noon_dt, origin) {
                result_days.push(InOrbTransitDay {
                    date_formatted: format!("{:04}-{:02}-{:02}", dt.year, dt.month, dt.day),
                    jd: cur_jd,
                    trigger_contacts: triggers,
                    snapshot_planets: build_transit_snapshots(&ephem),
                    all_active_transits: collect_ephem_transit_contacts(&ephem, natal_points, 2.0),
                });
            }
        }
        cur_jd += 1.0;
    }

    Ok(result_days)
}
