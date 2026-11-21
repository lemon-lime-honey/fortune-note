use serde::{Deserialize, Serialize};

use crate::celestial::{
    completed_age, jd_to_calendar, normalize_western_body_name, CalendarDate, CalendarDateTime,
    GeoCoordinate, NAIBOD_DAILY_RATE_DEG,
};
use crate::ephemeris::compute_ephemeris;
use crate::timezone::resolve_local_to_utc_with_tz;
use crate::western::aspects::is_aspect_applying;
use super::{find_aspect_with_target, get_body_tropical_lon_and_speed};

/// Maximum aspect orb (degrees) for Solar Arc & Secondary Progression contacts.
pub const PREDICTIVE_CONTACT_ORB_DEG: f64 = 1.0;

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
    #[serde(default)]
    pub is_applying: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarArcResult {
    pub arc_degrees: f64,
    pub directed_planets: Vec<SolarArcBody>,
    pub contacts: Vec<PredictiveAspect>,
}

fn compute_elapsed_years_and_prog_jd(
    birth: CalendarDate,
    birth_utc_jd: f64,
    target: CalendarDate,
) -> (f64, f64) {
    let (completed_years, last_bday_year) = completed_age(birth, target);
    let last_bday_jd = CalendarDate::new(last_bday_year, birth.month, birth.day).to_julian_day();
    let next_bday_jd = CalendarDate::new(last_bday_year + 1, birth.month, birth.day).to_julian_day();
    let target_day_jd = target.to_julian_day();

    let year_span = (next_bday_jd - last_bday_jd).max(1.0);
    let year_fraction = ((target_day_jd - last_bday_jd) / year_span).clamp(0.0, 1.0);
    let elapsed_years = completed_years as f64 + year_fraction;
    (elapsed_years, birth_utc_jd + elapsed_years)
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
    let birth = CalendarDate::new(birth_year, birth_month, birth_day);
    let target = CalendarDate::new(target_year, target_month, target_day);
    let (arc, _, directed) =
        calculate_solar_arc_from_birth_jd(birth, birth.to_julian_day(), target, natal_planets);
    (arc, directed)
}

pub(crate) fn calculate_solar_arc_from_birth_jd(
    birth: CalendarDate,
    birth_utc_jd: f64,
    target: CalendarDate,
    natal_planets: &[(&str, f64)],
) -> (f64, f64, Vec<SolarArcBody>) {
    let (elapsed_years, prog_jd) = compute_elapsed_years_and_prog_jd(birth, birth_utc_jd, target);
    let natal_sun_lon = natal_planets.iter().find(|(n, _)| *n == "Sun").map(|(_, lon)| *lon);

    let (arc, sun_prog_speed) = match (
        natal_sun_lon,
        get_body_tropical_lon_and_speed(vedaksha::ephem::bodies::Body::Sun, prog_jd),
    ) {
        (Some(natal_sun), Ok((prog_sun, spd))) => ((prog_sun - natal_sun).rem_euclid(360.0), spd),
        _ => ((elapsed_years * NAIBOD_DAILY_RATE_DEG).rem_euclid(360.0), NAIBOD_DAILY_RATE_DEG),
    };

    let directed = natal_planets
        .iter()
        .map(|&(name, lon)| {
            let dir_lon = (lon + arc).rem_euclid(360.0);
            let sign_index = (dir_lon / 30.0).floor() as u8 % 12;
            let sign_degree = (dir_lon - (sign_index as f64 * 30.0)).clamp(0.0, 29.999_999_999);
            SolarArcBody {
                name: normalize_western_body_name(name),
                natal_longitude: lon,
                directed_longitude: dir_lon,
                sign_index,
                sign_degree,
            }
        })
        .collect();

    (arc, sun_prog_speed, directed)
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
    let birth = CalendarDate::new(birth_year, birth_month, birth_day);
    let target = CalendarDate::new(target_year, target_month, target_day);
    calculate_solar_arc_detailed_from_birth_jd(
        birth,
        birth.to_julian_day(),
        target,
        natal_planets,
        natal_planets,
    )
}

pub(crate) fn calculate_solar_arc_detailed_from_birth_jd(
    birth: CalendarDate,
    birth_utc_jd: f64,
    target: CalendarDate,
    directed_targets: &[(&str, f64)],
    natal_targets: &[(&str, f64)],
) -> SolarArcResult {
    let (arc, sun_prog_speed, directed_planets) =
        calculate_solar_arc_from_birth_jd(birth, birth_utc_jd, target, directed_targets);

    let moving: Vec<(String, f64, f64)> = directed_planets
        .iter()
        .map(|dp| (dp.name.clone(), dp.directed_longitude, sun_prog_speed))
        .collect();
    let skip_self = arc < 1.0;
    let contacts = collect_predictive_contacts(&moving, natal_targets, skip_self);

    SolarArcResult {
        arc_degrees: arc,
        directed_planets,
        contacts,
    }
}

fn collect_predictive_contacts(
    moving_bodies: &[(String, f64, f64)],
    natal_targets: &[(&str, f64)],
    skip_same_body: bool,
) -> Vec<PredictiveAspect> {
    let mut contacts = Vec::new();
    for (moving_name, moving_lon, moving_speed) in moving_bodies {
        for &(natal_name, natal_lon) in natal_targets {
            let norm_natal = normalize_western_body_name(natal_name);
            if skip_same_body && *moving_name == norm_natal {
                continue;
            }
            if let Some((asp, target_angle, orb)) =
                find_aspect_with_target(*moving_lon, natal_lon, PREDICTIVE_CONTACT_ORB_DEG)
            {
                let is_applying = is_aspect_applying(*moving_lon, *moving_speed, natal_lon, 0.0, target_angle);
                contacts.push(PredictiveAspect {
                    moving_body: moving_name.clone(),
                    natal_body: norm_natal,
                    aspect_type: asp.to_string(),
                    orb,
                    is_applying,
                });
            }
        }
    }
    contacts
}

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

const PROGRESSED_PHASE_NAMES: [&str; 8] = [
    "Progressed New Moon",
    "Progressed Crescent",
    "Progressed First Quarter",
    "Progressed Gibbous",
    "Progressed Full Moon",
    "Progressed Disseminating",
    "Progressed Last Quarter",
    "Progressed Balsamic",
];

fn progressed_lunar_phase_label(sun_lon: f64, moon_lon: f64) -> String {
    let elongation = (moon_lon - sun_lon).rem_euclid(360.0);
    let idx = (((elongation + 22.5).rem_euclid(360.0)) / 45.0).floor() as usize % 8;
    PROGRESSED_PHASE_NAMES[idx].to_string()
}

/// Calculate Secondary Progressions (1 Ephemeris Day = 1 Tropical Year) with explicit timezone support.
pub fn calculate_secondary_progressions(
    birth_dt: CalendarDateTime,
    loc: GeoCoordinate,
    target_date: CalendarDate,
    natal_planets: &[(&str, f64)],
    timezone_id: Option<&str>,
) -> Result<SecondaryProgressionResult, String> {
    let tz_res = resolve_local_to_utc_with_tz(birth_dt, loc, timezone_id)
        .map_err(|e| format!("Timezone resolution failed: {}", e))?;
    let birth_utc_jd = tz_res.utc_datetime().to_julian_day();

    let (elapsed_years, progressed_jd) =
        compute_elapsed_years_and_prog_jd(birth_dt.date(), birth_utc_jd, target_date);
    let prog_dt = jd_to_calendar(progressed_jd);

    let ephem = compute_ephemeris(prog_dt, loc)
        .map_err(|e| format!("Secondary progression ephemeris failed: {}", e))?;

    let mut planets = Vec::with_capacity(ephem.bodies.len());
    let mut planet_speeds = Vec::with_capacity(ephem.bodies.len());
    for b in &ephem.bodies {
        let lon = b.tropical_longitude;
        let sign_index = (lon / 30.0).floor() as u8 % 12;
        let sign_degree = (lon - (sign_index as f64 * 30.0)).clamp(0.0, 29.999_999_999);
        let norm_name = normalize_western_body_name(&b.name);
        planet_speeds.push((norm_name.clone(), lon, b.speed));
        planets.push(ProgressedBody {
            name: norm_name,
            progressed_longitude: lon,
            sign_index,
            sign_degree,
            is_retrograde: b.is_retrograde,
        });
    }

    let prog_sun_lon = ephem.get_body("Sun").map_or(0.0, |b| b.tropical_longitude);
    let prog_moon_lon = ephem.get_body("Moon").map_or(0.0, |b| b.tropical_longitude);
    let contacts = collect_predictive_contacts(&planet_speeds, natal_planets, false);

    Ok(SecondaryProgressionResult {
        progressed_date_utc: format!(
            "{:04}-{:02}-{:02} {:02}:{:02} UTC",
            prog_dt.year, prog_dt.month, prog_dt.day, prog_dt.hour, prog_dt.minute
        ),
        elapsed_tropical_years: elapsed_years,
        planets,
        progressed_lunar_phase: progressed_lunar_phase_label(prog_sun_lon, prog_moon_lon),
        contacts,
    })
}
