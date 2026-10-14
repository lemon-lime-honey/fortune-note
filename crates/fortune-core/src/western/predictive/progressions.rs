use serde::{Deserialize, Serialize};
use crate::ephemeris::compute_ephemeris;
use crate::timezone::resolve_local_to_utc_with_tz;
use vedaksha::ephem::julian::calendar_to_jd;
use super::{find_aspect, get_body_tropical_lon, jd_to_calendar, normalize_western_body_name};

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

    let birth_jd = calendar_to_jd(birth_year, birth_month, birth_day as f64);
    let prog_jd = birth_jd + elapsed_years;

    let natal_sun_lon = natal_planets
        .iter()
        .find(|(name, _)| *name == "Sun")
        .map(|(_, lon)| *lon);

    let arc = match (natal_sun_lon, get_body_tropical_lon(vedaksha::ephem::bodies::Body::Sun, prog_jd)) {
        (Some(natal_sun), Ok(prog_sun)) => (prog_sun - natal_sun).rem_euclid(360.0),
        _ => (elapsed_years * (59.0 / 60.0 + 8.0 / 3600.0)).rem_euclid(360.0),
    };

    let directed = natal_planets.iter().map(|&(name, lon)| {
        let dir_lon = (lon + arc).rem_euclid(360.0);
        let sign_index = (dir_lon / 30.0).floor() as u8 % 12;
        let sign_degree = dir_lon - (sign_index as f64 * 30.0);

        SolarArcBody {
            name: normalize_western_body_name(name),
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
                    natal_body: format!("Nat {}", normalize_western_body_name(natal_name)),
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
    calculate_secondary_progressions_with_tz(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude, target_year, target_month, target_day, natal_planets, None,
    )
}

pub fn calculate_secondary_progressions_with_tz(
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
    timezone_id: Option<&str>,
) -> Result<SecondaryProgressionResult, String> {
    let tz_res = resolve_local_to_utc_with_tz(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude, timezone_id,
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
            name: normalize_western_body_name(&b.name),
            progressed_longitude: lon,
            sign_index,
            sign_degree,
            is_retrograde: b.is_retrograde,
        });
    }

    let mut elongation = prog_moon_lon - prog_sun_lon;
    if elongation < 0.0 { elongation += 360.0; }
    let progressed_lunar_phase = match elongation {
        e if !(22.5..337.5).contains(&e) => "Progressed New Moon",
        e if (22.5..67.5).contains(&e) => "Progressed Crescent",
        e if (67.5..112.5).contains(&e) => "Progressed First Quarter",
        e if (112.5..157.5).contains(&e) => "Progressed Gibbous",
        e if (157.5..202.5).contains(&e) => "Progressed Full Moon",
        e if (202.5..247.5).contains(&e) => "Progressed Disseminating",
        e if (247.5..292.5).contains(&e) => "Progressed Last Quarter",
        _ => "Progressed Balsamic",
    }.to_string();

    let mut contacts = Vec::new();
    for pb in &planets {
        for &(natal_name, natal_lon) in natal_planets {
            if let Some((asp, orb)) = find_aspect(pb.progressed_longitude, natal_lon, 1.0) {
                contacts.push(PredictiveAspect {
                    moving_body: format!("Prog {}", pb.name),
                    natal_body: format!("Nat {}", normalize_western_body_name(natal_name)),
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
