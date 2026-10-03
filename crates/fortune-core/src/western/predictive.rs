//! 6 Western Predictive Astrology Techniques.
//!
//! 1. Annual Profections (1 year = 1 sign)
//! 2. Solar Return (Exact solar return moment via Newton-Raphson solver)
//! 3. Secondary Progressions (1 day = 1 year)
//! 4. Solar Arc Directions (Arc of progressed Sun added to all natal bodies)
//! 5. Transits (Target date ephemeris against natal)
//! 6. Primary Directions (Classical diurnal motion)

use serde::{Deserialize, Serialize};
use super::dignity::DOMICILE_RULERS;
use crate::ephemeris::{compute_ephemeris, find_solar_longitude_moment};
use crate::timezone::resolve_local_to_utc;
use vedaksha::ephem::julian::calendar_to_jd;

const ZODIAC_SIGNS: [&str; 12] = [
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces",
];

/// Calendar date to Julian Day conversion helper
pub fn jd_to_calendar(jd: f64) -> (i32, u32, u32, u32, u32, f64) {
    let z = (jd + 0.5).floor() as i64;
    let f = jd + 0.5 - z as f64;
    let a = if z < 2_299_161 {
        z
    } else {
        let alpha = (((z as f64 - 1_867_216.25) / 36524.25).floor()) as i64;
        z + 1 + alpha - alpha / 4
    };
    let b = a + 1524;
    let c = (((b as f64 - 122.1) / 365.25).floor()) as i64;
    let d = (365.25 * c as f64).floor() as i64;
    let e = (((b - d) as f64 / 30.6001).floor()) as i64;
    let day = b - d - (30.6001 * e as f64).floor() as i64;
    let month = if e < 14 { e - 1 } else { e - 13 } as u32;
    let year = if month > 2 { c - 4716 } else { c - 4715 } as i32;
    let day_sec = (f * 86400.0).max(0.0);
    let hour = (day_sec / 3600.0).floor() as u32 % 24;
    let rem_sec = day_sec - (hour as f64 * 3600.0);
    let minute = (rem_sec / 60.0).floor() as u32 % 60;
    let second = (rem_sec - (minute as f64 * 60.0)).max(0.0);
    (year, month, day.max(1) as u32, hour, minute, second)
}

fn find_aspect(lon1: f64, lon2: f64, max_orb: f64) -> Option<(&'static str, f64)> {
    let diff = (lon1 - lon2).abs().rem_euclid(360.0);
    let shortest = if diff > 180.0 { 360.0 - diff } else { diff };
    let aspects = [
        (0.0, "Conjunction"),
        (60.0, "Sextile"),
        (90.0, "Square"),
        (120.0, "Trine"),
        (180.0, "Opposition"),
    ];
    for (angle, name) in aspects {
        let orb = (shortest - angle).abs();
        if orb <= max_orb {
            return Some((name, orb));
        }
    }
    None
}

// ---------------------------------------------------------------------------
// 1. Annual Profection
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualProfectionResult {
    pub target_age: u32,
    pub target_year: i32,
    pub profected_house: u8, // 1 .. 12
    pub profected_sign_index: u8,
    pub profected_sign_name: String,
    pub lord_of_the_year: String,
    pub active_natal_planets: Vec<String>,
}

pub fn calculate_annual_profection(
    birth_year: i32,
    target_year: i32,
    natal_ascendant: f64,
    natal_planets: &[(&str, f64)],
) -> AnnualProfectionResult {
    let age = if target_year >= birth_year { (target_year - birth_year) as u32 } else { 0 };
    let shift_signs = (age % 12) as u8;

    let asc_sign = (natal_ascendant / 30.0).floor() as u8 % 12;
    let profected_sign_index = (asc_sign + shift_signs) % 12;
    let profected_house = (shift_signs + 1) as u8;
    let profected_sign_name = ZODIAC_SIGNS[profected_sign_index as usize].to_string();

    let lord_of_the_year = DOMICILE_RULERS[profected_sign_index as usize].name().to_string();

    let mut active_natal_planets = Vec::new();
    for &(name, lon) in natal_planets {
        let p_sign = (lon / 30.0).floor() as u8 % 12;
        if p_sign == profected_sign_index {
            active_natal_planets.push(name.to_string());
        }
    }

    AnnualProfectionResult {
        target_age: age,
        target_year,
        profected_house,
        profected_sign_index,
        profected_sign_name,
        lord_of_the_year,
        active_natal_planets,
    }
}

// ---------------------------------------------------------------------------
// 2. Solar Arc Directions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarArcBody {
    pub name: String,
    pub natal_longitude: f64,
    pub directed_longitude: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveAspect {
    pub moving_body: String,
    pub natal_body: String,
    pub aspect_type: String,
    pub orb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarArcResult {
    pub arc_degrees: f64,
    pub directed_planets: Vec<SolarArcBody>,
    pub contacts: Vec<PredictiveAspect>,
}

pub fn calculate_solar_arc(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    target_year: i32,
    target_month: u32,
    target_day: u32,
    natal_planets: &[(&str, f64)],
) -> (f64, Vec<SolarArcBody>) {
    let fractional_years = (target_year - birth_year) as f64
        + (target_month as f64 - birth_month as f64) / 12.0
        + (target_day as f64 - birth_day as f64) / 365.25;

    let arc = (fractional_years * (59.0 / 60.0 + 8.0 / 3600.0)).rem_euclid(360.0);

    let directed = natal_planets.iter().map(|&(name, lon)| {
        let dir_lon = (lon + arc).rem_euclid(360.0);
        let sign_index = (dir_lon / 30.0).floor() as u8 % 12;
        let sign_degree = dir_lon - (sign_index as f64 * 30.0);

        SolarArcBody {
            name: name.to_string(),
            natal_longitude: lon,
            directed_longitude: dir_lon,
            sign_index,
            sign_degree,
        }
    }).collect();

    (arc, directed)
}

pub fn calculate_solar_arc_detailed(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    target_year: i32,
    target_month: u32,
    target_day: u32,
    natal_planets: &[(&str, f64)],
) -> SolarArcResult {
    let (arc, directed_planets) = calculate_solar_arc(
        birth_year, birth_month, birth_day,
        target_year, target_month, target_day,
        natal_planets,
    );

    let mut contacts = Vec::new();
    for dp in &directed_planets {
        for &(natal_name, natal_lon) in natal_planets {
            if let Some((asp, orb)) = find_aspect(dp.directed_longitude, natal_lon, 1.0) {
                contacts.push(PredictiveAspect {
                    moving_body: format!("Dir {}", dp.name),
                    natal_body: format!("Nat {}", natal_name),
                    aspect_type: asp.to_string(),
                    orb,
                });
            }
        }
    }

    SolarArcResult {
        arc_degrees: arc,
        directed_planets,
        contacts,
    }
}

// ---------------------------------------------------------------------------
// 3. Secondary Progressions (1 day = 1 year)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressedBody {
    pub name: String,
    pub progressed_longitude: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub is_retrograde: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryProgressionResult {
    pub progressed_date_utc: String,
    pub elapsed_tropical_years: f64,
    pub planets: Vec<ProgressedBody>,
    pub progressed_lunar_phase: String,
    pub contacts: Vec<PredictiveAspect>,
}

pub fn calculate_secondary_progressions(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    birth_hour: u32,
    birth_minute: u32,
    birth_second: f64,
    latitude: f64,
    longitude: f64,
    target_year: i32,
    target_month: u32,
    target_day: u32,
    natal_planets: &[(&str, f64)],
) -> Result<SecondaryProgressionResult, String> {
    let tz_res = resolve_local_to_utc(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude,
    ).map_err(|e| format!("Timezone resolution failed: {}", e))?;

    let birth_time_fraction = (tz_res.utc_hour as f64 + (tz_res.utc_minute as f64 + tz_res.utc_second / 60.0) / 60.0) / 24.0;
    let birth_utc_jd = calendar_to_jd(tz_res.utc_year, tz_res.utc_month, tz_res.utc_day as f64 + birth_time_fraction);

    // Standard secondary progression: 1 Ephemeris Day = 1 Tropical/Solar Year
    // Day-for-a-Year milestone mapping aligned to completed birthday years + intra-year fraction
    let (last_bday_year, next_bday_year, completed_years) = if (target_month, target_day) >= (birth_month, birth_day) {
        (target_year, target_year + 1, (target_year - birth_year).max(0) as f64)
    } else {
        (target_year - 1, target_year, (target_year - birth_year - 1).max(0) as f64)
    };

    let last_bday_jd = calendar_to_jd(last_bday_year, birth_month, birth_day as f64);
    let next_bday_jd = calendar_to_jd(next_bday_year, birth_month, birth_day as f64);
    let target_day_jd = calendar_to_jd(target_year, target_month, target_day as f64);

    let year_span = (next_bday_jd - last_bday_jd).max(1.0);
    let year_fraction = ((target_day_jd - last_bday_jd) / year_span).clamp(0.0, 1.0);

    let elapsed_years = completed_years + year_fraction;
    let progressed_jd = birth_utc_jd + elapsed_years;
    let (py, pm, pd, ph, pmin, ps) = jd_to_calendar(progressed_jd);

    let ephem = compute_ephemeris(py, pm, pd, ph, pmin, ps, latitude, longitude)
        .map_err(|e| format!("Secondary progression ephemeris failed: {}", e))?;

    let mut planets = Vec::new();
    let mut prog_sun_lon = 0.0;
    let mut prog_moon_lon = 0.0;

    for b in &ephem.bodies {
        let lon = b.tropical_longitude;
        let sign_index = (lon / 30.0).floor() as u8 % 12;
        let sign_degree = lon - (sign_index as f64 * 30.0);

        if b.name == "Sun" { prog_sun_lon = lon; }
        if b.name == "Moon" { prog_moon_lon = lon; }

        planets.push(ProgressedBody {
            name: b.name.clone(),
            progressed_longitude: lon,
            sign_index,
            sign_degree,
            is_retrograde: b.is_retrograde,
        });
    }

    let mut elongation = prog_moon_lon - prog_sun_lon;
    if elongation < 0.0 { elongation += 360.0; }
    let progressed_lunar_phase = match elongation {
        e if e >= 337.5 || e < 22.5 => "Progressed New Moon",
        e if e >= 22.5 && e < 67.5 => "Progressed Crescent",
        e if e >= 67.5 && e < 112.5 => "Progressed First Quarter",
        e if e >= 112.5 && e < 157.5 => "Progressed Gibbous",
        e if e >= 157.5 && e < 202.5 => "Progressed Full Moon",
        e if e >= 202.5 && e < 247.5 => "Progressed Disseminating",
        e if e >= 247.5 && e < 292.5 => "Progressed Last Quarter",
        _ => "Progressed Balsamic",
    }.to_string();

    let mut contacts = Vec::new();
    for pb in &planets {
        for &(natal_name, natal_lon) in natal_planets {
            if let Some((asp, orb)) = find_aspect(pb.progressed_longitude, natal_lon, 1.0) {
                contacts.push(PredictiveAspect {
                    moving_body: format!("Prog {}", pb.name),
                    natal_body: format!("Nat {}", natal_name),
                    aspect_type: asp.to_string(),
                    orb,
                });
            }
        }
    }

    Ok(SecondaryProgressionResult {
        progressed_date_utc: format!("{:04}-{:02}-{:02} {:02}:{:02} UTC", py, pm, pd, ph, pmin),
        elapsed_tropical_years: elapsed_years,
        planets,
        progressed_lunar_phase,
        contacts,
    })
}

// ---------------------------------------------------------------------------
// 4. Solar Return (Exact Newton-Raphson solver at natal coordinates)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarReturnResult {
    pub return_year: i32,
    pub return_date_utc: String,
    pub exact_jd_ut1: f64,
    pub sun_longitude: f64,
    pub ascendant: f64,
    pub midheaven: f64,
    pub house_cusps: [f64; 12],
    pub planets: Vec<ProgressedBody>,
}

/// Core solar return calculation when natal Sun longitude is already known.
pub fn calculate_solar_return_with_natal_sun(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    birth_hour: u32,
    birth_minute: u32,
    birth_second: f64,
    latitude: f64,
    longitude: f64,
    target_year: i32,
    target_sun_lon: f64,
    house_system: Option<vedaksha::astro::houses::HouseSystem>,
) -> Result<SolarReturnResult, String> {
    // 1. Resolve exact natal UTC timestamp
    let tz_res = resolve_local_to_utc(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude,
    ).map_err(|e| format!("Timezone resolution failed: {}", e))?;

    // 2. Initial guess in target_year using exact birth UTC JD
    let birth_time_frac = (tz_res.utc_hour as f64 + (tz_res.utc_minute as f64 + tz_res.utc_second / 60.0) / 60.0) / 24.0;
    let birth_utc_jd = calendar_to_jd(tz_res.utc_year, tz_res.utc_month, tz_res.utc_day as f64 + birth_time_frac);
    let year_diff = (target_year - birth_year) as f64;
    let approx_jd = birth_utc_jd + year_diff * 365.242_198_78;

    // 3. Newton-Raphson exact solar return solver (1e-7 deg precision)
    let exact_jd = find_solar_longitude_moment(approx_jd, target_sun_lon)
        .map_err(|e| format!("Solar return solver failed: {}", e))?;

    let (ry, rm, rd, rh, rmin, rs) = jd_to_calendar(exact_jd);

    // 4. Return chart observation at natal coordinates
    let final_ephem = compute_ephemeris(ry, rm, rd, rh, rmin, rs, latitude, longitude)
        .map_err(|e| format!("Solar return ephemeris calculation failed: {}", e))?;

    let h_sys = house_system.unwrap_or(vedaksha::astro::houses::HouseSystem::Regiomontanus);
    let house_result = vedaksha::astro::houses::compute_houses(
        final_ephem.ramc,
        latitude,
        final_ephem.true_obliquity_deg,
        h_sys,
    );

    let planets = final_ephem.bodies.iter().map(|b| {
        let lon = b.tropical_longitude;
        let sign_index = (lon / 30.0).floor() as u8 % 12;
        let sign_degree = lon - (sign_index as f64 * 30.0);
        ProgressedBody {
            name: b.name.clone(),
            progressed_longitude: lon,
            sign_index,
            sign_degree,
            is_retrograde: b.is_retrograde,
        }
    }).collect();

    Ok(SolarReturnResult {
        return_year: target_year,
        return_date_utc: format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC", ry, rm, rd, rh, rmin, rs.round() as u32 % 60),
        exact_jd_ut1: exact_jd,
        sun_longitude: target_sun_lon,
        ascendant: house_result.asc,
        midheaven: house_result.mc,
        house_cusps: house_result.cusps,
        planets,
    })
}

pub fn calculate_solar_return(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    birth_hour: u32,
    birth_minute: u32,
    birth_second: f64,
    latitude: f64,
    longitude: f64,
    target_year: i32,
    house_system: Option<vedaksha::astro::houses::HouseSystem>,
) -> Result<SolarReturnResult, String> {
    // 1. Resolve exact natal UTC timestamp
    let tz_res = resolve_local_to_utc(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude,
    ).map_err(|e| format!("Timezone resolution failed: {}", e))?;

    let natal_ephem = compute_ephemeris(
        tz_res.utc_year, tz_res.utc_month, tz_res.utc_day,
        tz_res.utc_hour, tz_res.utc_minute, tz_res.utc_second,
        latitude, longitude,
    ).map_err(|e| format!("Natal ephemeris error: {}", e))?;

    let natal_sun = natal_ephem.get_body("Sun")
        .ok_or_else(|| "Sun not found in natal ephemeris".to_string())?;

    calculate_solar_return_with_natal_sun(
        birth_year,
        birth_month,
        birth_day,
        birth_hour,
        birth_minute,
        birth_second,
        latitude,
        longitude,
        target_year,
        natal_sun.tropical_longitude,
        house_system,
    )
}

// ---------------------------------------------------------------------------
// 5. Transits
// ---------------------------------------------------------------------------

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
    let tz_res = resolve_local_to_utc(
        target_year, target_month, target_day, target_hour, target_minute, 0.0,
        latitude, longitude,
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
                    transiting_planet: tb.name.clone(),
                    transiting_longitude: tb.tropical_longitude,
                    natal_planet: natal_name.to_string(),
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

fn body_from_name(name: &str) -> Option<vedaksha::ephem::bodies::Body> {
    match name {
        "Sun" => Some(vedaksha::ephem::bodies::Body::Sun),
        "Moon" => Some(vedaksha::ephem::bodies::Body::Moon),
        "Mercury" => Some(vedaksha::ephem::bodies::Body::Mercury),
        "Venus" => Some(vedaksha::ephem::bodies::Body::Venus),
        "Mars" => Some(vedaksha::ephem::bodies::Body::Mars),
        "Jupiter" => Some(vedaksha::ephem::bodies::Body::Jupiter),
        "Saturn" => Some(vedaksha::ephem::bodies::Body::Saturn),
        "North Node" | "Rahu" => Some(vedaksha::ephem::bodies::Body::TrueNode),
        "South Node" | "Ketu" => Some(vedaksha::ephem::bodies::Body::TrueSouthNode),
        _ => None,
    }
}

fn get_body_tropical_lon(body: vedaksha::ephem::bodies::Body, jd: f64) -> Result<f64, String> {
    let provider = vedaksha::ephem::analytical::AnalyticalProvider;
    let pos = vedaksha::ephem::coordinates::apparent_position(&provider, body, jd)
        .map_err(|e| format!("Body position failed: {:?}", e))?;
    Ok(vedaksha::math::angle::normalize_degrees(pos.ecliptic.longitude.to_degrees()))
}

fn separation_error(lon_tr: f64, lon_nat: f64, target_sep: f64) -> f64 {
    let sep = (lon_tr - lon_nat).rem_euclid(360.0);
    (sep - target_sep + 540.0).rem_euclid(360.0) - 180.0
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

    let mut filtered_natal = Vec::new();
    for &(name, lon) in natal_points {
        if let Some(ref targets) = query.natal_targets {
            if targets.iter().any(|t| t.eq_ignore_ascii_case(name)) {
                filtered_natal.push((name, lon));
            }
        } else {
            filtered_natal.push((name, lon));
        }
    }

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
                            raw_hits.push((hit_jd, moving_name.clone(), nat_name.to_string(), nat_lon, asp_name.to_string(), target_sep));
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
                if b.name == moving_name {
                    moving_lon = b.tropical_longitude;
                    moving_spd = b.speed;
                    moving_rx = b.is_retrograde;
                }
                snapshots.push(TransitPlanetSnapshot {
                    name: b.name.clone(),
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


// ---------------------------------------------------------------------------
// 6. Primary Directions (Ptolemaic / Placidean Equatorial Arc & Naibod Key)
// ---------------------------------------------------------------------------

/// Convert ecliptic longitude (at ecliptic latitude = 0) to equatorial coordinates (RA, Dec).
pub fn ecliptic_to_equatorial(longitude_deg: f64, obliquity_deg: f64) -> (f64, f64) {
    let l_rad = longitude_deg.to_radians();
    let eps_rad = obliquity_deg.to_radians();

    let sin_delta = eps_rad.sin() * l_rad.sin();
    let delta_rad = sin_delta.clamp(-1.0, 1.0).asin();
    let delta_deg = delta_rad.to_degrees();

    let alpha_rad = (eps_rad.cos() * l_rad.sin()).atan2(l_rad.cos());
    let alpha_deg = alpha_rad.to_degrees().rem_euclid(360.0);

    (alpha_deg, delta_deg)
}

/// Calculate ascensional difference (AD) in degrees: sin(AD) = tan(phi) * tan(delta).
pub fn ascensional_difference(declination_deg: f64, latitude_deg: f64) -> f64 {
    let tan_phi = latitude_deg.to_radians().tan();
    let tan_delta = declination_deg.to_radians().tan();
    let product = (tan_phi * tan_delta).clamp(-1.0, 1.0);
    product.asin().to_degrees()
}

/// Calculate Oblique Ascension (OA): OA = RA - AD.
pub fn oblique_ascension(ra_deg: f64, ad_deg: f64) -> f64 {
    (ra_deg - ad_deg).rem_euclid(360.0)
}

fn normalize_western_body_name(name: &str) -> String {
    match name {
        "Rahu" => "North Node".to_string(),
        "Ketu" => "South Node".to_string(),
        other => other.to_string(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryDirectionContact {
    pub promittor: String,
    pub significator: String,
    pub aspect_type: String,
    pub direction_arc_deg: f64,
    pub realization_age: f64,
}

/// Computes Ptolemaic / Placidean Primary Directions with equatorial arcs and Naibod key.
/// Formatted and windowed around `target_age` to prevent the infant-direction lock defect.
pub fn calculate_primary_directions(
    natal_planets: &[(&str, f64)],
    natal_ascendant: f64,
    natal_midheaven: f64,
    latitude: f64,
    obliquity_deg: f64,
    target_age: f64,
    max_age: f64,
) -> Vec<PrimaryDirectionContact> {
    let obliquity = if obliquity_deg > 0.0 { obliquity_deg } else { 23.4392911 };
    let naibod_key = 59.0 / 60.0 + 8.0 / 3600.0; // 0.985647 deg per tropical year
    let max_arc = max_age * naibod_key;

    // 1. Compute equatorial coordinates for primary significators
    let (ramc, _) = ecliptic_to_equatorial(natal_midheaven, obliquity);
    // In celestial mechanics, OA of Ascendant = RA - AD of natal Ascendant
    let (ra_asc, dec_asc) = ecliptic_to_equatorial(natal_ascendant, obliquity);
    let ad_asc = ascensional_difference(dec_asc, latitude);
    let oa_asc = oblique_ascension(ra_asc, ad_asc);

    let aspects: [(f64, &str); 5] = [
        (0.0, "Conjunction"),
        (60.0, "Sextile"),
        (90.0, "Square"),
        (120.0, "Trine"),
        (180.0, "Opposition"),
    ];

    let mut all_directions = Vec::new();

    for &(prom_raw_name, prom_lon) in natal_planets {
        let prom_name = normalize_western_body_name(prom_raw_name);

        for (angle, asp_name) in aspects {
            // Test both forward and backward aspect rays (except for 0° and 180°)
            let ray_offsets = if angle == 0.0 || angle == 180.0 {
                vec![angle]
            } else {
                vec![angle, 360.0 - angle]
            };

            for ray_offset in ray_offsets {
                let asp_ecliptic_lon = (prom_lon + ray_offset).rem_euclid(360.0);
                let (ra_asp, dec_asp) = ecliptic_to_equatorial(asp_ecliptic_lon, obliquity);
                let ad_asp = ascensional_difference(dec_asp, latitude);
                let oa_asp = oblique_ascension(ra_asp, ad_asp);

                // A. Direct direction to MC: Diurnal motion carries promittor to Meridian
                let arc_mc = (ra_asp - ramc).rem_euclid(360.0);
                if arc_mc > 0.05 && arc_mc <= max_arc && arc_mc <= 110.0 {
                    let age = arc_mc / naibod_key;
                    all_directions.push(PrimaryDirectionContact {
                        promittor: prom_name.clone(),
                        significator: "MC".to_string(),
                        aspect_type: asp_name.to_string(),
                        direction_arc_deg: arc_mc,
                        realization_age: age,
                    });
                }

                // B. Direct direction to ASC: Diurnal motion carries promittor to Horizon
                let arc_asc = (oa_asp - oa_asc).rem_euclid(360.0);
                if arc_asc > 0.05 && arc_asc <= max_arc && arc_asc <= 110.0 {
                    let age = arc_asc / naibod_key;
                    all_directions.push(PrimaryDirectionContact {
                        promittor: prom_name.clone(),
                        significator: "ASC".to_string(),
                        aspect_type: asp_name.to_string(),
                        direction_arc_deg: arc_asc,
                        realization_age: age,
                    });
                }
            }
        }
    }

    // Deduplicate any duplicate contacts that might arise from ray symmetry
    all_directions.sort_by(|a, b| {
        a.realization_age.partial_cmp(&b.realization_age).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut unique_directions: Vec<PrimaryDirectionContact> = Vec::new();
    for d in all_directions {
        let is_dup = unique_directions.iter().any(|u| {
            u.promittor == d.promittor
                && u.significator == d.significator
                && u.aspect_type == d.aspect_type
                && (u.direction_arc_deg - d.direction_arc_deg).abs() < 0.01
        });
        if !is_dup {
            unique_directions.push(d);
        }
    }

    // Window around target_age: prioritize directions within target_age ± 5 years,
    // then sort by proximity to target_age so the user sees relevant life milestones.
    unique_directions.sort_by(|a, b| {
        let diff_a = (a.realization_age - target_age).abs();
        let diff_b = (b.realization_age - target_age).abs();
        diff_a.partial_cmp(&diff_b).unwrap_or(std::cmp::Ordering::Equal)
    });

    unique_directions
}

// ---------------------------------------------------------------------------
// Unified Predictive Suite
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveSuite {
    pub target_year: i32,
    pub target_month: u32,
    pub target_day: u32,
    pub target_hour: u32,
    pub target_minute: u32,
    pub age: u32,
    pub profection: AnnualProfectionResult,
    pub solar_arc: SolarArcResult,
    pub secondary_progression: Option<SecondaryProgressionResult>,
    pub solar_return: Option<SolarReturnResult>,
    pub transits: Vec<TransitContact>,
    pub primary_directions: Vec<PrimaryDirectionContact>,
}

pub fn calculate_predictive_suite(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    birth_hour: u32,
    birth_minute: u32,
    birth_second: f64,
    latitude: f64,
    longitude: f64,
    target_year: i32,
    target_month: u32,
    target_day: u32,
    target_hour: Option<u32>,
    target_minute: Option<u32>,
    natal_ascendant: f64,
    natal_midheaven: f64,
    natal_planets: &[(&str, f64)],
    house_system: Option<vedaksha::astro::houses::HouseSystem>,
    true_obliquity_deg: Option<f64>,
) -> PredictiveSuite {
    let profection = calculate_annual_profection(birth_year, target_year, natal_ascendant, natal_planets);
    let solar_arc = calculate_solar_arc_detailed(
        birth_year, birth_month, birth_day,
        target_year, target_month, target_day,
        natal_planets,
    );
    let secondary = calculate_secondary_progressions(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude,
        target_year, target_month, target_day,
        natal_planets,
    ).ok();

    let natal_sun_lon = natal_planets.iter().find(|(name, _)| *name == "Sun").map(|(_, lon)| *lon);
    let solar_return = if let Some(sun_lon) = natal_sun_lon {
        calculate_solar_return_with_natal_sun(
            birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
            latitude, longitude, target_year, sun_lon, house_system,
        ).ok()
    } else {
        calculate_solar_return(
            birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
            latitude, longitude, target_year, house_system,
        ).ok()
    };

    let actual_target_hour = target_hour.unwrap_or(birth_hour);
    let actual_target_minute = target_minute.unwrap_or(birth_minute);

    let transits = calculate_transits(
        target_year, target_month, target_day, actual_target_hour, actual_target_minute,
        latitude, longitude, natal_planets,
    ).unwrap_or_default();

    let age = if target_year >= birth_year { (target_year - birth_year) as u32 } else { 0 };

    let eps = true_obliquity_deg.unwrap_or(23.4392911);
    let primary = calculate_primary_directions(
        natal_planets, natal_ascendant, natal_midheaven, latitude, eps, age as f64, 100.0,
    );

    PredictiveSuite {
        target_year,
        target_month,
        target_day,
        target_hour: actual_target_hour,
        target_minute: actual_target_minute,
        age,
        profection,
        solar_arc,
        secondary_progression: secondary,
        solar_return,
        transits,
        primary_directions: primary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predictive_suite() {
        let natal_planets = [
            ("Sun", 280.0),
            ("Moon", 45.0),
            ("Mercury", 270.0),
            ("Venus", 310.0),
            ("Mars", 15.0),
            ("Jupiter", 195.0),
            ("Saturn", 120.0),
            ("Rahu", 10.0),
        ];

        let suite = calculate_predictive_suite(
            1990, 5, 15, 14, 30, 0.0,
            37.5665, 126.9780,
            2026, 5, 15,
            Some(14), Some(30),
            180.0, 90.0,
            &natal_planets,
            Some(vedaksha::astro::houses::HouseSystem::Regiomontanus),
            Some(23.44),
        );

        assert_eq!(suite.age, 36);
        assert_eq!(suite.target_hour, 14);
        assert_eq!(suite.target_minute, 30);
        assert_eq!(suite.profection.profected_house, 1);
        assert!(suite.solar_arc.arc_degrees > 30.0);
        assert!(suite.solar_return.is_some());
        assert!(!suite.primary_directions.is_empty());

        // The top direction must be windowed near age 36, not locked at infancy (age 0-4)
        let top_dir = &suite.primary_directions[0];
        assert!((top_dir.realization_age - 36.0).abs() < 10.0, "Top direction realization age was {}", top_dir.realization_age);

        // Ensure Rahu was normalized to North Node
        let rahu_found = suite.primary_directions.iter().any(|d| d.promittor == "Rahu");
        assert!(!rahu_found, "Rahu must not be present in western primary directions");
        let north_node_found = suite.primary_directions.iter().any(|d| d.promittor == "North Node");
        assert!(north_node_found, "North Node must be present in western primary directions");
    }

    #[test]
    fn test_equatorial_coordinates() {
        let eps = 23.4392911;
        // Vernal Equinox 0°: RA = 0°, Dec = 0°
        let (ra0, dec0) = ecliptic_to_equatorial(0.0, eps);
        assert!((ra0 - 0.0).abs() < 1e-5);
        assert!((dec0 - 0.0).abs() < 1e-5);

        // Summer Solstice 90°: RA = 90°, Dec = +23.439°
        let (ra90, dec90) = ecliptic_to_equatorial(90.0, eps);
        assert!((ra90 - 90.0).abs() < 1e-5);
        assert!((dec90 - eps).abs() < 1e-5);

        // Autumnal Equinox 180°: RA = 180°, Dec = 0°
        let (ra180, dec180) = ecliptic_to_equatorial(180.0, eps);
        assert!((ra180 - 180.0).abs() < 1e-5);
        assert!((dec180 - 0.0).abs() < 1e-5);

        // Winter Solstice 270°: RA = 270°, Dec = -23.439°
        let (ra270, dec270) = ecliptic_to_equatorial(270.0, eps);
        assert!((ra270 - 270.0).abs() < 1e-5);
        assert!((dec270 + eps).abs() < 1e-5);
    }

    #[test]
    fn test_ascensional_difference() {
        // At equator (lat = 0), AD is always 0
        let ad_equator = ascensional_difference(20.0, 0.0);
        assert!((ad_equator - 0.0).abs() < 1e-5);

        // At Dec = 0, AD is always 0
        let ad_zero_dec = ascensional_difference(0.0, 37.5);
        assert!((ad_zero_dec - 0.0).abs() < 1e-5);

        // At Seoul (lat = 37.5665), Dec = +23.44 gives positive AD
        let ad_seoul = ascensional_difference(23.439, 37.5665);
        assert!(ad_seoul > 15.0 && ad_seoul < 25.0);
    }

    #[test]
    fn test_find_transit_events_bounded() {
        let query = TransitSearchQuery {
            start_year: 2026,
            start_month: 1,
            start_day: 1,
            end_year: 2026,
            end_month: 6,
            end_day: 30,
            moving_bodies: Some(vec!["Jupiter".to_string(), "Mars".to_string()]),
            natal_targets: Some(vec!["Sun".to_string(), "MC".to_string()]),
            aspect_types: Some(vec!["Conjunction".to_string(), "Trine".to_string(), "Square".to_string()]),
        };

        let natal_points = [
            ("Sun", 45.0),
            ("MC", 120.0),
            ("Moon", 210.0),
        ];

        let hits = find_transit_events(&query, &natal_points).expect("Search should succeed");
        assert!(!hits.is_empty(), "Expected at least 1 transit hit in 6 months for Mars/Jupiter");

        for hit in &hits {
            assert!(hit.orb_deg < 0.1, "Exact hit orb should be very close to zero: {}", hit.orb_deg);
            assert!(hit.moving_body == "Jupiter" || hit.moving_body == "Mars");
            assert!(hit.natal_point == "Sun" || hit.natal_point == "MC");
            assert!(!hit.snapshot_planets.is_empty(), "Snapshot planets must be present");
        }
    }
}
