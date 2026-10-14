use serde::{Deserialize, Serialize};
use crate::ephemeris::compute_ephemeris;
use crate::timezone::resolve_local_to_utc_with_tz;
use vedaksha::ephem::julian::calendar_to_jd;
use super::{
    body_from_name, find_aspect, get_body_tropical_lon, jd_to_calendar,
    normalize_western_body_name, separation_error,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitContact {
    pub transiting_planet: String,
    pub transiting_longitude: f64,
    pub natal_planet: String,
    pub natal_longitude: f64,
    pub aspect_type: String,
    pub orb: f64,
}

pub fn calculate_transits(
    target_year: i32,
    target_month: u32,
    target_day: u32,
    target_hour: u32,
    target_minute: u32,
    latitude: f64,
    longitude: f64,
    natal_planets: &[(&str, f64)],
) -> Result<Vec<TransitContact>, String> {
    calculate_transits_with_tz(
        target_year, target_month, target_day, target_hour, target_minute,
        latitude, longitude, natal_planets, None,
    )
}

pub fn calculate_transits_with_tz(
    target_year: i32,
    target_month: u32,
    target_day: u32,
    target_hour: u32,
    target_minute: u32,
    latitude: f64,
    longitude: f64,
    natal_planets: &[(&str, f64)],
    timezone_id: Option<&str>,
) -> Result<Vec<TransitContact>, String> {
    let tz_res = resolve_local_to_utc_with_tz(
        target_year, target_month, target_day, target_hour, target_minute, 0.0,
        latitude, longitude, timezone_id,
    );
    let (ty, tm, td, th, tmin, ts) = match tz_res {
        Ok(ref res) => (res.utc_year, res.utc_month, res.utc_day, res.utc_hour, res.utc_minute, res.utc_second),
        Err(_) => (target_year, target_month, target_day, target_hour, target_minute, 0.0),
    };

    let ephem = compute_ephemeris(
        ty, tm, td, th, tmin, ts,
        latitude, longitude,
    ).map_err(|e| format!("Transit ephemeris failed: {}", e))?;

    let mut contacts = Vec::new();
    for tb in &ephem.bodies {
        for &(natal_name, natal_lon) in natal_planets {
            if let Some((asp, orb)) = find_aspect(tb.tropical_longitude, natal_lon, 2.0) {
                contacts.push(TransitContact {
                    transiting_planet: normalize_western_body_name(&tb.name),
                    transiting_longitude: tb.tropical_longitude,
                    natal_planet: normalize_western_body_name(natal_name),
                    natal_longitude: natal_lon,
                    aspect_type: asp.to_string(),
                    orb,
                });
            }
        }
    }

    Ok(contacts)
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

pub fn matches_natal_target(target_spec: &str, point_name: &str) -> bool {
    let t = target_spec.trim();
    if t.eq_ignore_ascii_case("cusps") || t.eq_ignore_ascii_case("all_cusps") {
        return point_name.starts_with("Cusp")
            || point_name.starts_with('H')
            || point_name == "ASC"
            || point_name == "MC"
            || point_name == "IC"
            || point_name == "DSC";
    }
    t.eq_ignore_ascii_case(point_name)
        || (t.eq_ignore_ascii_case("ASC") && (point_name == "Cusp 1" || point_name == "H1"))
        || (t.eq_ignore_ascii_case("MC") && (point_name == "Cusp 10" || point_name == "H10"))
        || (t.eq_ignore_ascii_case("IC") && (point_name == "Cusp 4" || point_name == "H4"))
        || (t.eq_ignore_ascii_case("DSC") && (point_name == "Cusp 7" || point_name == "H7"))
        || (t.replace(' ', "").eq_ignore_ascii_case(&point_name.replace(' ', "")))
}

fn filter_natal_points<'a>(
    natal_points: &[(&'a str, f64)],
    targets_opt: Option<&[String]>,
) -> Vec<(&'a str, f64)> {
    let mut filtered = Vec::new();
    for &(name, lon) in natal_points {
        if let Some(targets) = targets_opt {
            if targets.iter().any(|t| matches_natal_target(t, name)) {
                filtered.push((name, lon));
            }
        } else {
            filtered.push((name, lon));
        }
    }
    filtered
}

/// Search exact transit aspect hits within a bounded time interval.
pub fn find_transit_events(
    query: &TransitSearchQuery,
    natal_points: &[(&str, f64)],
) -> Result<Vec<TransitEventHit>, String> {
    let start_jd = calendar_to_jd(query.start_year, query.start_month, query.start_day as f64);
    let end_jd = calendar_to_jd(query.end_year, query.end_month, (query.end_day as f64) + 0.9999);

    if end_jd <= start_jd {
        return Err("End date must be after start date".to_string());
    }

    let default_moving = vec![
        "Saturn".to_string(),
        "Jupiter".to_string(),
        "Mars".to_string(),
        "Sun".to_string(),
        "Venus".to_string(),
        "Mercury".to_string(),
    ];
    let moving_list = query.moving_bodies.as_ref().unwrap_or(&default_moving);

    let has_moon = moving_list.iter().any(|b| b == "Moon");
    let step_days = if has_moon { 0.25 } else { 0.5 };

    let aspect_defs: &[(&str, &[f64])] = &[
        ("Conjunction", &[0.0]),
        ("Sextile", &[60.0, 300.0]),
        ("Square", &[90.0, 270.0]),
        ("Trine", &[120.0, 240.0]),
        ("Opposition", &[180.0]),
    ];

    let filtered_natal = filter_natal_points(natal_points, query.natal_targets.as_deref());

    let mut active_aspects = Vec::new();
    for &(asp_name, targets) in aspect_defs {
        if let Some(ref selected_asps) = query.aspect_types {
            if selected_asps.iter().any(|s| s.eq_ignore_ascii_case(asp_name)) {
                active_aspects.push((asp_name, targets));
            }
        } else {
            active_aspects.push((asp_name, targets));
        }
    }

    let mut raw_hits = Vec::new();

    for moving_name in moving_list {
        let vbody = match body_from_name(moving_name) {
            Some(b) => b,
            None => continue,
        };

        let mut cur_t = start_jd;
        let mut prev_lon = match get_body_tropical_lon(vbody, cur_t) {
            Ok(l) => l,
            Err(_) => continue,
        };

        while cur_t < end_jd {
            let next_t = (cur_t + step_days).min(end_jd);
            let next_lon = match get_body_tropical_lon(vbody, next_t) {
                Ok(l) => l,
                Err(_) => {
                    cur_t = next_t;
                    continue;
                }
            };

            for &(nat_name, nat_lon) in &filtered_natal {
                for &(asp_name, targets) in &active_aspects {
                    for &target_sep in targets {
                        let e0 = separation_error(prev_lon, nat_lon, target_sep);
                        let e1 = separation_error(next_lon, nat_lon, target_sep);

                        if e0.abs() < 12.0 && e1.abs() < 12.0 && (e0 * e1 <= 0.0) && (e0 != e1) {
                            // Bisection refinement
                            let mut left_t = cur_t;
                            let mut right_t = next_t;
                            let mut left_e = e0;

                            for _ in 0..12 {
                                let mid_t = (left_t + right_t) * 0.5;
                                if let Ok(mid_lon) = get_body_tropical_lon(vbody, mid_t) {
                                    let mid_e = separation_error(mid_lon, nat_lon, target_sep);
                                    if mid_e.abs() < 1e-4 {
                                        left_t = mid_t;
                                        break;
                                    }
                                    if left_e * mid_e <= 0.0 {
                                        right_t = mid_t;
                                    } else {
                                        left_t = mid_t;
                                        left_e = mid_e;
                                    }
                                } else {
                                    break;
                                }
                            }

                            let hit_jd = (left_t + right_t) * 0.5;
                            raw_hits.push((
                                hit_jd,
                                normalize_western_body_name(moving_name),
                                normalize_western_body_name(nat_name),
                                nat_lon,
                                asp_name.to_string(),
                                target_sep,
                            ));
                        }
                    }
                }
            }

            cur_t = next_t;
            prev_lon = next_lon;
        }
    }

    // Sort by JD
    raw_hits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // Deduplicate within 0.15 day (~3.6 hours) for the same planet-target-aspect
    let mut unique_hits = Vec::new();
    for hit in raw_hits {
        let is_dup = unique_hits.iter().any(|u: &(f64, String, String, f64, String, f64)| {
            u.1 == hit.1 && u.2 == hit.2 && u.4 == hit.4 && (u.0 - hit.0).abs() < 0.15
        });
        if !is_dup {
            unique_hits.push(hit);
        }
    }

    // Generate full snapshots for each hit
    let mut hits = Vec::with_capacity(unique_hits.len());
    for (hit_jd, moving_name, nat_name, nat_lon, asp_name, target_sep) in unique_hits {
        let (hy, hm, hd, hhour, hmin, hsec) = jd_to_calendar(hit_jd);
        let date_str = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
            hy, hm, hd, hhour, hmin, hsec.floor() as u32
        );

        if let Ok(ephem) = compute_ephemeris(hy, hm, hd, hhour, hmin, hsec, 0.0, 0.0) {
            let mut snapshots = Vec::new();
            let mut moving_lon = 0.0;
            let mut moving_spd = 0.0;
            let mut moving_rx = false;

            for b in &ephem.bodies {
                let sign_idx = ((b.tropical_longitude / 30.0).floor() as u32) % 12;
                let sign_deg = b.tropical_longitude - (sign_idx as f64 * 30.0);
                let norm_name = normalize_western_body_name(&b.name);
                if norm_name == moving_name {
                    moving_lon = b.tropical_longitude;
                    moving_spd = b.speed;
                    moving_rx = b.is_retrograde;
                }
                snapshots.push(TransitPlanetSnapshot {
                    name: norm_name,
                    tropical_longitude: b.tropical_longitude,
                    speed: b.speed,
                    is_retrograde: b.is_retrograde,
                    sign_index: sign_idx,
                    sign_degree: sign_deg,
                });
            }

            let orb_deg = separation_error(moving_lon, nat_lon, target_sep).abs();

            hits.push(TransitEventHit {
                event_jd: hit_jd,
                utc_date_time: date_str,
                moving_body: moving_name,
                moving_longitude: moving_lon,
                moving_speed: moving_spd,
                is_retrograde: moving_rx,
                natal_point: nat_name,
                natal_longitude: nat_lon,
                aspect_type: asp_name,
                orb_deg,
                snapshot_planets: snapshots,
            });
        }
    }

    Ok(hits)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InOrbTransitDay {
    pub date_formatted: String,
    pub jd: f64,
    pub trigger_contacts: Vec<TransitContact>,
    pub snapshot_planets: Vec<TransitPlanetSnapshot>,
    pub all_active_transits: Vec<TransitContact>,
}

/// Scan every day within the bounded range and return complete transit snapshots
/// for all dates where any transiting planet touches the target natal points within `orb_limit`.
pub fn find_in_orb_transit_days(
    query: &TransitSearchQuery,
    natal_points: &[(&str, f64)],
    orb_limit: f64,
) -> Result<Vec<InOrbTransitDay>, String> {
    let start_jd = calendar_to_jd(query.start_year, query.start_month, query.start_day as f64);
    let end_jd = calendar_to_jd(query.end_year, query.end_month, (query.end_day as f64) + 0.9999);

    if end_jd <= start_jd {
        return Err("End date must be after start date".to_string());
    }

    let default_moving = vec![
        "Saturn".to_string(),
        "Jupiter".to_string(),
        "Mars".to_string(),
        "Sun".to_string(),
        "Venus".to_string(),
        "Mercury".to_string(),
    ];
    let moving_list = query.moving_bodies.as_ref().unwrap_or(&default_moving);

    let aspect_defs: &[(&str, &[f64])] = &[
        ("Conjunction", &[0.0]),
        ("Sextile", &[60.0, 300.0]),
        ("Square", &[90.0, 270.0]),
        ("Trine", &[120.0, 240.0]),
        ("Opposition", &[180.0]),
    ];

    let filtered_natal = filter_natal_points(natal_points, query.natal_targets.as_deref());

    let mut active_aspects = Vec::new();
    for &(asp_name, targets) in aspect_defs {
        if let Some(ref selected_asps) = query.aspect_types {
            if selected_asps.iter().any(|s| s.eq_ignore_ascii_case(asp_name)) {
                active_aspects.push((asp_name, targets));
            }
        } else {
            active_aspects.push((asp_name, targets));
        }
    }

    let mut moving_parsed = Vec::new();
    for name in moving_list {
        if let Some(b) = body_from_name(name) {
            moving_parsed.push((normalize_western_body_name(name), b));
        }
    }

    let mut result_days = Vec::new();
    let mut cur_jd = start_jd + 0.5; // Noon UT1

    while cur_jd <= end_jd {
        let mut triggers = Vec::new();

        for (m_name, vbody) in &moving_parsed {
            if let Ok(m_lon) = get_body_tropical_lon(*vbody, cur_jd) {
                for &(nat_name, nat_lon) in &filtered_natal {
                    for &(asp_name, targets) in &active_aspects {
                        for &target_sep in targets {
                            let err = separation_error(m_lon, nat_lon, target_sep).abs();
                            if err <= orb_limit {
                                triggers.push(TransitContact {
                                    transiting_planet: m_name.clone(),
                                    transiting_longitude: m_lon,
                                    natal_planet: normalize_western_body_name(nat_name),
                                    natal_longitude: nat_lon,
                                    aspect_type: asp_name.to_string(),
                                    orb: err,
                                });
                            }
                        }
                    }
                }
            }
        }

        if !triggers.is_empty() {
            let (y, m, d, _, _, _) = jd_to_calendar(cur_jd);
            if let Ok(ephem) = compute_ephemeris(y, m, d, 12, 0, 0.0, 0.0, 0.0) {
                let mut snapshots = Vec::new();
                for b in &ephem.bodies {
                    let sign_idx = ((b.tropical_longitude / 30.0).floor() as u32) % 12;
                    let sign_deg = b.tropical_longitude - (sign_idx as f64 * 30.0);
                    snapshots.push(TransitPlanetSnapshot {
                        name: normalize_western_body_name(&b.name),
                        tropical_longitude: b.tropical_longitude,
                        speed: b.speed,
                        is_retrograde: b.is_retrograde,
                        sign_index: sign_idx,
                        sign_degree: sign_deg,
                    });
                }

                let mut all_transits = Vec::new();
                for b in &ephem.bodies {
                    for &(nat_name, nat_lon) in natal_points {
                        if let Some((asp, orb)) = find_aspect(b.tropical_longitude, nat_lon, 2.0) {
                            all_transits.push(TransitContact {
                                transiting_planet: normalize_western_body_name(&b.name),
                                transiting_longitude: b.tropical_longitude,
                                natal_planet: normalize_western_body_name(nat_name),
                                natal_longitude: nat_lon,
                                aspect_type: asp.to_string(),
                                orb,
                            });
                        }
                    }
                }

                result_days.push(InOrbTransitDay {
                    date_formatted: format!("{:04}-{:02}-{:02}", y, m, d),
                    jd: cur_jd,
                    trigger_contacts: triggers,
                    snapshot_planets: snapshots,
                    all_active_transits: all_transits,
                });
            }
        }

        cur_jd += 1.0;
    }

    Ok(result_days)
}
