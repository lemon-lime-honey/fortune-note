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

    let house_result = vedaksha::astro::houses::compute_houses(
        final_ephem.ramc,
        latitude,
        final_ephem.true_obliquity_deg,
        vedaksha::astro::houses::HouseSystem::Placidus,
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

// ---------------------------------------------------------------------------
// 6. Primary Directions (Ptolemaic / Naibod Key)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryDirectionContact {
    pub promittor: String,
    pub significator: String,
    pub aspect_type: String,
    pub direction_arc_deg: f64,
    pub realization_age: f64,
}

pub fn calculate_primary_directions(
    natal_planets: &[(&str, f64)],
    natal_ascendant: f64,
    natal_midheaven: f64,
    max_age: f64,
) -> Vec<PrimaryDirectionContact> {
    let significators = [
        ("ASC", natal_ascendant),
        ("MC", natal_midheaven),
    ];

    let naibod_key = 59.0 / 60.0 + 8.0 / 3600.0; // 0.98565 deg per year
    let max_arc = max_age * naibod_key;

    let mut directions = Vec::new();

    for &(prom_name, prom_lon) in natal_planets {
        for &(sig_name, sig_lon) in &significators {
            let aspects = [
                (0.0, "Conjunction"),
                (60.0, "Sextile"),
                (90.0, "Square"),
                (120.0, "Trine"),
                (180.0, "Opposition"),
            ];

            for (angle, asp_name) in aspects {
                let target_point = (sig_lon + angle).rem_euclid(360.0);
                let arc = (target_point - prom_lon).rem_euclid(360.0);

                if arc > 0.01 && arc <= max_arc {
                    let realization_age = arc / naibod_key;
                    directions.push(PrimaryDirectionContact {
                        promittor: prom_name.to_string(),
                        significator: sig_name.to_string(),
                        aspect_type: asp_name.to_string(),
                        direction_arc_deg: arc,
                        realization_age,
                    });
                }
            }
        }
    }

    directions.sort_by(|a, b| a.realization_age.partial_cmp(&b.realization_age).unwrap());
    directions
}

// ---------------------------------------------------------------------------
// Unified Predictive Suite
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveSuite {
    pub target_year: i32,
    pub target_month: u32,
    pub target_day: u32,
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
    natal_ascendant: f64,
    natal_midheaven: f64,
    natal_planets: &[(&str, f64)],
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
            latitude, longitude, target_year, sun_lon,
        ).ok()
    } else {
        calculate_solar_return(
            birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
            latitude, longitude, target_year,
        ).ok()
    };

    let transits = calculate_transits(
        target_year, target_month, target_day, 12, 0,
        latitude, longitude, natal_planets,
    ).unwrap_or_default();

    let primary = calculate_primary_directions(
        natal_planets, natal_ascendant, natal_midheaven, 100.0,
    );

    let age = if target_year >= birth_year { (target_year - birth_year) as u32 } else { 0 };

    PredictiveSuite {
        target_year,
        target_month,
        target_day,
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
        ];

        let suite = calculate_predictive_suite(
            1990, 5, 15, 14, 30, 0.0,
            37.5665, 126.9780,
            2026, 5, 15,
            180.0, 90.0,
            &natal_planets,
        );

        assert_eq!(suite.age, 36);
        assert_eq!(suite.profection.profected_house, 1);
        assert!(suite.solar_arc.arc_degrees > 30.0);
        assert!(suite.solar_return.is_some());
    }
}
