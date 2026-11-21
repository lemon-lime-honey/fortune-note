//! 6 Western Predictive Astrology Techniques.
//!
//! 1. Annual Profections (1 year = 1 sign) — `profection`
//! 2. Solar Arc Directions (Arc of progressed Sun added to all natal bodies) — `progressions`
//! 3. Secondary Progressions (1 day = 1 year) — `progressions`
//! 4. Solar & Planetary Returns (Exact return moment solver) — `returns`
//! 5. Transits & Bounded Event Search — `transits`
//! 6. Primary Directions (Classical diurnal motion & Naibod key) — `primary_directions`

pub mod primary_directions;
pub mod profection;
pub mod progressions;
pub mod returns;
pub mod transits;

pub use crate::celestial::normalize_western_body_name;
pub use primary_directions::{
    ascensional_difference, calculate_primary_directions, ecliptic_to_equatorial,
    oblique_ascension, PrimaryDirectionContact,
};
pub use profection::{
    calculate_annual_profection, calculate_zodiacal_releasing, zr_sign_minor_years,
    AnnualProfectionResult, ZodiacalReleasingResult, ZrPeriod,
};
pub use progressions::{
    calculate_secondary_progressions, calculate_solar_arc, calculate_solar_arc_detailed,
    PredictiveAspect, ProgressedBody, SecondaryProgressionResult, SolarArcBody, SolarArcResult,
    PREDICTIVE_CONTACT_ORB_DEG,
};
pub use returns::{
    calculate_planetary_return, calculate_solar_return, compute_return_natal_aspects,
    PlanetaryReturnRequest, PlanetaryReturnResult, ReturnAspectContact, SolarReturnResult,
};
pub use transits::{
    calculate_transits_with_tz, find_in_orb_transit_days, find_transit_events,
    matches_natal_target, InOrbTransitDay, TransitContact, TransitEventHit,
    TransitPlanetSnapshot, TransitSearchQuery, NATAL_CUSP_NAMES,
};

use serde::{Deserialize, Serialize};
use vedaksha::astro::houses::HouseSystem;

use crate::celestial::{
    completed_age, CalendarDate, CalendarDateTime, GeoCoordinate, MEAN_OBLIQUITY_J2000_DEG,
};
use crate::timezone::resolve_local_to_utc_with_tz;
use returns::{calculate_planetary_return_from_natal, NatalReturnContext};

pub(crate) use crate::celestial::ZODIAC_SIGNS;

pub(crate) const MAJOR_PTOLEMAIC_ASPECTS: [(f64, &str); 5] = [
    (0.0, "Conjunction"),
    (60.0, "Sextile"),
    (90.0, "Square"),
    (120.0, "Trine"),
    (180.0, "Opposition"),
];

pub(crate) fn find_aspect_with_target(
    lon1: f64,
    lon2: f64,
    max_orb: f64,
) -> Option<(&'static str, f64, f64)> {
    let diff = (lon1 - lon2).abs().rem_euclid(360.0);
    let shortest = if diff > 180.0 { 360.0 - diff } else { diff };
    for (angle, name) in MAJOR_PTOLEMAIC_ASPECTS {
        let orb = (shortest - angle).abs();
        if orb <= max_orb {
            return Some((name, angle, orb));
        }
    }
    None
}

pub(crate) fn get_body_tropical_lon_and_speed(
    body: vedaksha::ephem::bodies::Body,
    jd: f64,
) -> Result<(f64, f64), String> {
    let provider = vedaksha::ephem::analytical::AnalyticalProvider;
    let pos = vedaksha::ephem::coordinates::apparent_position(&provider, body, jd)
        .map_err(|e| format!("Body position failed: {:?}", e))?;
    Ok((
        vedaksha::math::angle::normalize_degrees(pos.ecliptic.longitude.to_degrees()),
        pos.longitude_speed,
    ))
}

pub(crate) fn body_from_name(name: &str) -> Option<vedaksha::ephem::bodies::Body> {
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

pub(crate) fn get_body_tropical_lon(body: vedaksha::ephem::bodies::Body, jd: f64) -> Result<f64, String> {
    get_body_tropical_lon_and_speed(body, jd).map(|(lon, _)| lon)
}

pub(crate) fn separation_error(lon_tr: f64, lon_nat: f64, target_sep: f64) -> f64 {
    let sep = (lon_tr - lon_nat).rem_euclid(360.0);
    (sep - target_sep + 540.0).rem_euclid(360.0) - 180.0
}

/// Natal chart context for unified predictive calculations.
pub struct NatalPredictiveContext<'a> {
    pub birth_dt: CalendarDateTime,
    pub location: GeoCoordinate,
    pub timezone_id: Option<&'a str>,
    pub ascendant: f64,
    pub midheaven: f64,
    pub planets: &'a [(&'a str, f64)],
    pub true_obliquity_deg: Option<f64>,
}

/// Target date/time and house configuration for predictive calculations.
#[derive(Debug, Clone, Copy)]
pub struct PredictiveTarget {
    pub date: CalendarDate,
    pub hour: Option<u32>,
    pub minute: Option<u32>,
    pub house_system: Option<HouseSystem>,
}

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
    #[serde(default)]
    pub planetary_returns: Vec<PlanetaryReturnResult>,
    pub transits: Vec<TransitContact>,
    pub primary_directions: Vec<PrimaryDirectionContact>,
}

const CLASSICAL_RETURN_BODIES: [&str; 7] = [
    "Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn",
];

fn compute_all_returns(
    natal: &NatalPredictiveContext<'_>,
    birth_utc_jd: f64,
    target_year: i32,
    all_natal_points: &[(&str, f64)],
    house_system: HouseSystem,
) -> (Option<SolarReturnResult>, Vec<PlanetaryReturnResult>) {
    let ctx = NatalReturnContext {
        birth_year: natal.birth_dt.year,
        birth_utc_jd,
        location: natal.location,
        target_year,
        all_natal_points,
        house_system,
        orb_limit: 2.0,
    };

    let mut planetary_returns = Vec::new();
    for body_name in CLASSICAL_RETURN_BODIES {
        let Some(&(_, natal_lon)) = natal.planets.iter().find(|(n, _)| n.eq_ignore_ascii_case(body_name)) else {
            continue;
        };
        if let Ok(mut rets) = calculate_planetary_return_from_natal(&ctx, body_name, natal_lon) {
            planetary_returns.append(&mut rets);
        }
    }

    // Derive Solar Return directly from the already-computed Sun return to avoid duplicate solver runs.
    let solar_return = planetary_returns
        .iter()
        .find(|r| r.return_body.eq_ignore_ascii_case("Sun"))
        .map(PlanetaryReturnResult::to_solar_return)
        .or_else(|| {
            calculate_solar_return(&PlanetaryReturnRequest {
                birth_dt: natal.birth_dt,
                location: natal.location,
                timezone_id: natal.timezone_id,
                target_year,
                return_body_name: "Sun",
                house_system: Some(house_system),
                aspect_max_orb: Some(2.0),
            })
            .ok()
        });

    (solar_return, planetary_returns)
}

/// Compute all 6 predictive techniques from structured natal and target contexts.
pub fn calculate_predictive_suite(
    natal: &NatalPredictiveContext<'_>,
    target: PredictiveTarget,
) -> PredictiveSuite {
    let birth_utc_jd = resolve_local_to_utc_with_tz(natal.birth_dt, natal.location, natal.timezone_id)
        .map(|tz_res| tz_res.utc_datetime().to_julian_day())
        .unwrap_or_else(|_| natal.birth_dt.date().to_julian_day());

    let mut all_natal_points = Vec::with_capacity(natal.planets.len() + 2);
    all_natal_points.push(("ASC", natal.ascendant));
    all_natal_points.push(("MC", natal.midheaven));
    for &(name, lon) in natal.planets {
        if !name.eq_ignore_ascii_case("ASC") && !name.eq_ignore_ascii_case("MC") {
            all_natal_points.push((name, lon));
        }
    }

    let birth_date = natal.birth_dt.date();
    let profection = calculate_annual_profection(
        birth_date.year, birth_date.month, birth_date.day,
        target.date.year, target.date.month, target.date.day,
        natal.ascendant, natal.planets,
    );
    let solar_arc = progressions::calculate_solar_arc_detailed_from_birth_jd(
        birth_date, birth_utc_jd, target.date, natal.planets, &all_natal_points,
    );
    let secondary_progression = calculate_secondary_progressions(
        natal.birth_dt, natal.location, target.date, &all_natal_points, natal.timezone_id,
    ).ok();

    let h_sys = target.house_system.unwrap_or(HouseSystem::Regiomontanus);
    let (solar_return, planetary_returns) =
        compute_all_returns(natal, birth_utc_jd, target.date.year, &all_natal_points, h_sys);

    let target_hour = target.hour.unwrap_or(natal.birth_dt.hour);
    let target_minute = target.minute.unwrap_or(natal.birth_dt.minute);
    let transit_dt = CalendarDateTime::new(
        target.date.year, target.date.month, target.date.day, target_hour, target_minute, 0.0,
    );
    let transits = calculate_transits_with_tz(
        transit_dt, natal.location, &all_natal_points, natal.timezone_id,
    ).unwrap_or_default();

    let (age, _) = completed_age(birth_date, target.date);
    let eps = natal.true_obliquity_deg.unwrap_or(MEAN_OBLIQUITY_J2000_DEG);
    let primary_directions = calculate_primary_directions(
        natal.planets, natal.ascendant, natal.midheaven, natal.location.latitude, eps, age as f64, 100.0,
    );

    PredictiveSuite {
        target_year: target.date.year,
        target_month: target.date.month,
        target_day: target.date.day,
        target_hour,
        target_minute,
        age,
        profection,
        solar_arc,
        secondary_progression,
        solar_return,
        planetary_returns,
        transits,
        primary_directions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_PLANETS: [(&str, f64); 9] = [
        ("Sun", 280.0), ("Moon", 45.0), ("Mercury", 270.0), ("Venus", 310.0),
        ("Mars", 15.0), ("Jupiter", 195.0), ("Saturn", 120.0), ("Rahu", 190.0), ("Ketu", 10.0),
    ];

    #[test]
    fn test_predictive_suite() {
        let natal = NatalPredictiveContext {
            birth_dt: CalendarDateTime::new(1990, 5, 15, 14, 30, 0.0),
            location: GeoCoordinate::new(37.5665, 126.9780),
            timezone_id: Some("Asia/Seoul"),
            ascendant: 180.0,
            midheaven: 90.0,
            planets: &SAMPLE_PLANETS,
            true_obliquity_deg: Some(23.44),
        };
        let target = PredictiveTarget {
            date: CalendarDate::new(2026, 5, 15),
            hour: Some(14),
            minute: Some(30),
            house_system: Some(HouseSystem::Regiomontanus),
        };
        let suite = calculate_predictive_suite(&natal, target);

        assert_eq!(suite.age, 36);
        assert_eq!(suite.profection.profected_house, 1);
        assert!(suite.solar_arc.arc_degrees > 30.0);
        assert!(suite.solar_return.is_some());
        assert!(!suite.primary_directions.is_empty());

        for c in &suite.solar_arc.contacts {
            assert!(!c.moving_body.starts_with("Dir "), "Unexpected prefix: {}", c.moving_body);
            assert!(!c.natal_body.starts_with("Nat "), "Unexpected prefix: {}", c.natal_body);
        }
        if let Some(ref sec) = suite.secondary_progression {
            for c in &sec.contacts {
                assert!(!c.moving_body.starts_with("Prog "), "Unexpected prefix: {}", c.moving_body);
                assert!(!c.natal_body.starts_with("Nat "), "Unexpected prefix: {}", c.natal_body);
            }
        }
        let sun_returns: Vec<_> = suite.planetary_returns.iter().filter(|r| r.return_body == "Sun").collect();
        assert_eq!(sun_returns.len(), 1);
        assert!((sun_returns[0].exact_jd_ut1 - suite.solar_return.as_ref().unwrap().exact_jd_ut1).abs() < 1e-9);
    }

    #[test]
    fn test_annual_profection_birthday_boundary() {
        let natal_planets = [("Sun", 50.0), ("Moon", 120.0)];
        let prof_before = calculate_annual_profection(1990, 5, 15, 2026, 5, 14, 15.0, &natal_planets);
        assert_eq!((prof_before.target_age, prof_before.profected_house), (35, 12));
        let prof_on = calculate_annual_profection(1990, 5, 15, 2026, 5, 15, 15.0, &natal_planets);
        assert_eq!((prof_on.target_age, prof_on.profected_house), (36, 1));
    }

    #[test]
    fn test_true_solar_arc_calculation() {
        let natal_planets = [("Sun", 54.32), ("Moon", 120.0), ("Venus", 80.0)];
        let (arc, directed) = calculate_solar_arc(1990, 5, 15, 2026, 5, 15, &natal_planets);
        assert!(arc > 34.0 && arc < 37.0);
        let dir_sun = directed.iter().find(|p| p.name == "Sun").unwrap();
        assert!((dir_sun.directed_longitude - (54.32 + arc).rem_euclid(360.0)).abs() < 1e-4);
    }

    #[test]
    fn test_transits_reject_invalid_timezone() {
        let res = calculate_transits_with_tz(
            CalendarDateTime::new(2026, 5, 15, 12, 0, 0.0),
            GeoCoordinate::new(37.5665, 126.9780),
            &SAMPLE_PLANETS,
            Some("Invalid/Timezone_Name"),
        );
        assert!(res.is_err(), "Invalid timezone must error instead of silently using UTC");
    }

    #[test]
    fn test_equatorial_and_ascensional_difference() {
        let (ra90, dec90) = ecliptic_to_equatorial(90.0, MEAN_OBLIQUITY_J2000_DEG);
        assert!((ra90 - 90.0).abs() < 1e-5);
        assert!((dec90 - MEAN_OBLIQUITY_J2000_DEG).abs() < 1e-5);
        assert!(ascensional_difference(20.0, 0.0).abs() < 1e-5);
        let ad_seoul = ascensional_difference(MEAN_OBLIQUITY_J2000_DEG, 37.5665);
        assert!(ad_seoul > 15.0 && ad_seoul < 25.0);
    }

    #[test]
    fn test_planetary_return_and_transit_search() {
        let returns = calculate_planetary_return(&PlanetaryReturnRequest {
            birth_dt: CalendarDateTime::new(1990, 5, 15, 14, 30, 0.0),
            location: GeoCoordinate::new(37.5665, 126.9780),
            timezone_id: Some("Asia/Seoul"),
            target_year: 2026,
            return_body_name: "Sun",
            house_system: None,
            aspect_max_orb: Some(2.0),
        })
        .expect("Solar return calculation should succeed");
        assert_eq!(returns.len(), 1);
        assert!(returns[0].natal_aspects.iter().any(|a| a.return_point == "Sun" && a.natal_point == "Sun" && a.orb < 0.01));

        let query = TransitSearchQuery {
            start_year: 2026, start_month: 1, start_day: 1,
            end_year: 2026, end_month: 6, end_day: 30,
            moving_bodies: Some(vec!["Jupiter".to_string(), "Mars".to_string()]),
            natal_targets: Some(vec!["Sun".to_string(), "MC".to_string()]),
            aspect_types: Some(vec!["Conjunction".to_string(), "Trine".to_string()]),
        };
        let hits = find_transit_events(&query, &[("Sun", 45.0), ("MC", 120.0)]).expect("Search should succeed");
        assert!(!hits.is_empty());
        assert!(matches_natal_target("IC", "Cusp 4"));
        assert!(matches_natal_target("DSC", "Cusp 7"));
    }
}
