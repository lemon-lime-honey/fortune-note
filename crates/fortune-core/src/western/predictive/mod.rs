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

pub use primary_directions::{
    ascensional_difference, calculate_primary_directions, ecliptic_to_equatorial,
    oblique_ascension, PrimaryDirectionContact,
};
pub use profection::{calculate_annual_profection, AnnualProfectionResult};
pub use progressions::{
    calculate_secondary_progressions, calculate_secondary_progressions_with_tz,
    calculate_solar_arc, calculate_solar_arc_detailed, PredictiveAspect, ProgressedBody,
    SecondaryProgressionResult, SolarArcBody, SolarArcResult,
};
pub use returns::{
    calculate_planetary_return, calculate_solar_return, calculate_solar_return_with_natal_sun,
    calculate_solar_return_with_natal_sun_detailed,
    calculate_solar_return_with_natal_sun_detailed_tz, compute_return_natal_aspects,
    PlanetaryReturnResult, ReturnAspectContact, SolarReturnResult,
};
pub use transits::{
    calculate_transits, calculate_transits_with_tz, find_in_orb_transit_days,
    find_transit_events, matches_natal_target, InOrbTransitDay, TransitContact, TransitEventHit,
    TransitPlanetSnapshot, TransitSearchQuery, NATAL_CUSP_NAMES,
};

use serde::{Deserialize, Serialize};
use crate::timezone::resolve_local_to_utc_with_tz;
use returns::calculate_planetary_return_from_natal;
use vedaksha::ephem::julian::calendar_to_jd;

pub(crate) const ZODIAC_SIGNS: [&str; 12] = [
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces",
];

pub fn normalize_western_body_name(name: &str) -> String {
    match name {
        "Rahu" => "North Node".to_string(),
        "Ketu" => "South Node".to_string(),
        other => other.to_string(),
    }
}

/// Calendar date to Julian Day conversion helper
pub fn jd_to_calendar(jd: f64) -> (i32, u32, u32, u32, u32, f64) {
    let z = (jd + 0.5).floor() as i64;
    let f = (jd + 0.5 - z as f64).clamp(0.0, 1.0 - 1e-12);
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
    let day_sec = (f * 86400.0).clamp(0.0, 86399.999999);
    let hour = (day_sec / 3600.0).floor() as u32 % 24;
    let rem_sec = day_sec - (hour as f64 * 3600.0);
    let minute = (rem_sec / 60.0).floor() as u32 % 60;
    let second = (rem_sec - (minute as f64 * 60.0)).max(0.0);
    (year, month, day.max(1) as u32, hour, minute, second)
}

pub(crate) fn format_jd_utc_seconds(jd: f64) -> String {
    let (y, m, d, h, min, s) = jd_to_calendar(jd + 0.5 / 86400.0);
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC", y, m, d, h, min, s.floor() as u32)
}

pub(crate) fn find_aspect(lon1: f64, lon2: f64, max_orb: f64) -> Option<(&'static str, f64)> {
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
    let provider = vedaksha::ephem::analytical::AnalyticalProvider;
    let pos = vedaksha::ephem::coordinates::apparent_position(&provider, body, jd)
        .map_err(|e| format!("Body position failed: {:?}", e))?;
    Ok(vedaksha::math::angle::normalize_degrees(pos.ecliptic.longitude.to_degrees()))
}

pub(crate) fn separation_error(lon_tr: f64, lon_nat: f64, target_sep: f64) -> f64 {
    let sep = (lon_tr - lon_nat).rem_euclid(360.0);
    (sep - target_sep + 540.0).rem_euclid(360.0) - 180.0
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
    #[serde(default)]
    pub planetary_returns: Vec<PlanetaryReturnResult>,
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
    calculate_predictive_suite_with_timezone(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude, target_year, target_month, target_day, target_hour, target_minute,
        natal_ascendant, natal_midheaven, natal_planets, house_system, true_obliquity_deg, None,
    )
}

pub fn calculate_predictive_suite_with_timezone(
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
    timezone_id: Option<&str>,
) -> PredictiveSuite {
    let profection = calculate_annual_profection(
        birth_year, birth_month, birth_day,
        target_year, target_month, target_day,
        natal_ascendant, natal_planets,
    );
    let solar_arc = calculate_solar_arc_detailed(
        birth_year, birth_month, birth_day,
        target_year, target_month, target_day,
        natal_planets,
    );
    let secondary = calculate_secondary_progressions_with_tz(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude,
        target_year, target_month, target_day,
        natal_planets,
        timezone_id,
    ).ok();

    let mut all_natal_for_return = Vec::with_capacity(natal_planets.len() + 2);
    all_natal_for_return.push(("ASC", natal_ascendant));
    all_natal_for_return.push(("MC", natal_midheaven));
    for &(name, lon) in natal_planets {
        all_natal_for_return.push((name, lon));
    }

    let natal_sun_lon = natal_planets.iter().find(|(name, _)| *name == "Sun").map(|(_, lon)| *lon);
    let solar_return = if let Some(sun_lon) = natal_sun_lon {
        calculate_solar_return_with_natal_sun_detailed_tz(
            birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
            latitude, longitude, target_year, sun_lon, house_system, Some(&all_natal_for_return), timezone_id,
        ).ok()
    } else {
        calculate_solar_return(
            birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
            latitude, longitude, target_year, house_system,
        ).ok()
    };

    let h_sys = house_system.unwrap_or(vedaksha::astro::houses::HouseSystem::Regiomontanus);
    let birth_utc_jd = resolve_local_to_utc_with_tz(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude, timezone_id,
    ).map(|tz_res| {
        let frac = (tz_res.utc_hour as f64 + (tz_res.utc_minute as f64 + tz_res.utc_second / 60.0) / 60.0) / 24.0;
        calendar_to_jd(tz_res.utc_year, tz_res.utc_month, tz_res.utc_day as f64 + frac)
    }).unwrap_or_else(|_| calendar_to_jd(birth_year, birth_month, birth_day as f64));

    let mut planetary_returns = Vec::new();
    for body_name in ["Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn"] {
        let Some(&(_, natal_lon)) = natal_planets.iter().find(|(n, _)| n.eq_ignore_ascii_case(body_name)) else {
            continue;
        };
        if let Ok(mut rets) = calculate_planetary_return_from_natal(
            birth_year,
            birth_utc_jd,
            latitude,
            longitude,
            target_year,
            body_name,
            natal_lon,
            &all_natal_for_return,
            h_sys,
            2.0,
        ) {
            planetary_returns.append(&mut rets);
        }
    }

    let actual_target_hour = target_hour.unwrap_or(birth_hour);
    let actual_target_minute = target_minute.unwrap_or(birth_minute);

    let transits = calculate_transits_with_tz(
        target_year, target_month, target_day, actual_target_hour, actual_target_minute,
        latitude, longitude, natal_planets, timezone_id,
    ).unwrap_or_default();

    let age = if (target_month, target_day) >= (birth_month, birth_day) {
        (target_year - birth_year).max(0) as u32
    } else {
        (target_year - birth_year - 1).max(0) as u32
    };

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
        planetary_returns,
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
            ("Rahu", 190.0),
            ("Ketu", 10.0),
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

        // Ensure Rahu/Ketu were normalized to North Node/South Node across ALL Western techniques
        assert!(suite.profection.active_natal_planets.contains(&"North Node".to_string()));
        assert!(!suite.profection.active_natal_planets.contains(&"Rahu".to_string()));

        assert!(suite.solar_arc.directed_planets.iter().any(|p| p.name == "North Node"));
        assert!(suite.solar_arc.directed_planets.iter().any(|p| p.name == "South Node"));
        assert!(!suite.solar_arc.directed_planets.iter().any(|p| p.name == "Rahu" || p.name == "Ketu"));

        let sec = suite.secondary_progression.as_ref().unwrap();
        assert!(sec.planets.iter().any(|p| p.name == "North Node"));
        assert!(sec.planets.iter().any(|p| p.name == "South Node"));
        assert!(!sec.planets.iter().any(|p| p.name == "Rahu" || p.name == "Ketu"));

        let sr = suite.solar_return.as_ref().unwrap();
        assert!(sr.planets.iter().any(|p| p.name == "North Node"));
        assert!(sr.planets.iter().any(|p| p.name == "South Node"));
        assert!(!sr.planets.iter().any(|p| p.name == "Rahu" || p.name == "Ketu"));

        assert!(!suite.transits.iter().any(|t| {
            t.transiting_planet == "Rahu"
                || t.transiting_planet == "Ketu"
                || t.natal_planet == "Rahu"
                || t.natal_planet == "Ketu"
        }));

        let rahu_found = suite.primary_directions.iter().any(|d| d.promittor == "Rahu" || d.promittor == "Ketu");
        assert!(!rahu_found, "Rahu/Ketu must not be present in western primary directions");
        let north_node_found = suite.primary_directions.iter().any(|d| d.promittor == "North Node");
        assert!(north_node_found, "North Node must be present in western primary directions");

        // Verify that Converse primary directions are present
        let converse_found = suite.primary_directions.iter().any(|d| d.aspect_type.contains("(Converse)"));
        assert!(converse_found, "Converse primary directions must be calculated alongside direct directions");
    }

    #[test]
    fn test_annual_profection_birthday_boundary() {
        let natal_planets = [("Sun", 50.0), ("Moon", 120.0)];
        let asc = 15.0; // Aries ASC (sign index 0)

        // Case 1: Birthday is May 15, 1990.
        // Before birthday (May 14, 2026) -> age 35 -> shift = 35 % 12 = 11 -> Pisces (12th house)
        let prof_before = calculate_annual_profection(1990, 5, 15, 2026, 5, 14, asc, &natal_planets);
        assert_eq!(prof_before.target_age, 35);
        assert_eq!(prof_before.profected_house, 12);
        assert_eq!(prof_before.profected_sign_name, "Pisces");

        // Case 2: On birthday (May 15, 2026) -> age 36 -> shift = 36 % 12 = 0 -> Aries (1st house)
        let prof_on = calculate_annual_profection(1990, 5, 15, 2026, 5, 15, asc, &natal_planets);
        assert_eq!(prof_on.target_age, 36);
        assert_eq!(prof_on.profected_house, 1);
        assert_eq!(prof_on.profected_sign_name, "Aries");

        // Case 3: After birthday (May 16, 2026) -> age 36 -> shift = 0 -> Aries (1st house)
        let prof_after = calculate_annual_profection(1990, 5, 15, 2026, 5, 16, asc, &natal_planets);
        assert_eq!(prof_after.target_age, 36);
        assert_eq!(prof_after.profected_house, 1);
        assert_eq!(prof_after.profected_sign_name, "Aries");
    }

    #[test]
    fn test_true_solar_arc_calculation() {
        // True Solar Arc should use actual progression of the Sun between birth and target
        let natal_planets = [
            ("Sun", 54.32),
            ("Moon", 120.0),
            ("Venus", 80.0),
        ];

        let (arc, directed) = calculate_solar_arc(1990, 5, 15, 2026, 5, 15, &natal_planets);

        // Over 36 years, arc should be roughly ~35.48° (roughly 36 days of Sun motion)
        assert!(arc > 34.0 && arc < 37.0, "True solar arc should be around ~35-36 degrees, got {}", arc);

        let dir_sun = directed.iter().find(|p| p.name == "Sun").unwrap();
        assert!((dir_sun.directed_longitude - (54.32 + arc).rem_euclid(360.0)).abs() < 1e-4);

        let dir_moon = directed.iter().find(|p| p.name == "Moon").unwrap();
        assert!((dir_moon.directed_longitude - (120.0 + arc).rem_euclid(360.0)).abs() < 1e-4);
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
            assert!(hit.snapshot_planets.iter().any(|p| p.name == "North Node"));
            assert!(!hit.snapshot_planets.iter().any(|p| p.name == "Rahu" || p.name == "Ketu"));
        }
    }

    #[test]
    fn test_find_in_orb_transit_days() {
        let query = TransitSearchQuery {
            start_year: 2026,
            start_month: 1,
            start_day: 1,
            end_year: 2026,
            end_month: 3,
            end_day: 31,
            moving_bodies: Some(vec!["Sun".to_string(), "Mars".to_string()]),
            natal_targets: Some(vec!["Jupiter".to_string()]),
            aspect_types: Some(vec!["Conjunction".to_string(), "Trine".to_string()]),
        };

        let natal_points = [
            ("Jupiter", 100.0),
            ("Sun", 50.0),
        ];

        let days = find_in_orb_transit_days(&query, &natal_points, 1.0).expect("Search should succeed");
        assert!(!days.is_empty(), "Expected qualifying days for Sun/Mars in 3 months within 1 degree");

        for day in &days {
            assert!(!day.trigger_contacts.is_empty());
            for c in &day.trigger_contacts {
                assert!(c.orb <= 1.0);
                assert_eq!(c.natal_planet, "Jupiter");
                assert!(c.transiting_planet == "Sun" || c.transiting_planet == "Mars");
            }
            assert!(!day.snapshot_planets.is_empty());
            assert!(day.snapshot_planets.iter().any(|p| p.name == "North Node"));
            assert!(!day.snapshot_planets.iter().any(|p| p.name == "Rahu" || p.name == "Ketu"));
        }
    }

    #[test]
    fn test_planetary_return_inter_chart_aspects() {
        let returns = calculate_planetary_return(
            1990, 5, 15, 14, 30, 0.0,
            37.5665, 126.9780,
            2026, "Sun",
            None, Some(2.0),
        ).expect("Solar return calculation should succeed");

        assert_eq!(returns.len(), 1);
        let sr = &returns[0];
        assert_eq!(sr.return_body, "Sun");
        assert_eq!(sr.target_year, 2026);
        assert!(!sr.return_planets.is_empty());
        assert!(sr.return_planets.iter().any(|p| p.name == "North Node"));
        assert!(!sr.return_planets.iter().any(|p| p.name == "Rahu" || p.name == "Ketu"));
        assert!(!sr.natal_aspects.is_empty(), "Inter-chart aspects between Return and Natal must be computed");

        let sun_conjunction = sr.natal_aspects.iter().find(|a| {
            a.return_point == "Sun" && a.natal_point == "Sun" && a.aspect_type == "Conjunction"
        });
        assert!(sun_conjunction.is_some(), "Solar return must contain Return Sun Conjunction Natal Sun");
        assert!(sun_conjunction.unwrap().orb < 0.01, "Sun conjunction orb must be near zero");

        let has_axis_aspect = sr.natal_aspects.iter().any(|a| {
            a.return_point == "ASC" || a.return_point == "MC" || a.natal_point == "ASC" || a.natal_point == "MC"
        });
        assert!(has_axis_aspect, "Inter-chart aspects must include ASC and MC contacts");
    }

    #[test]
    fn test_find_transit_events_with_cusps() {
        let natal_cusps = [
            ("Cusp 1", 15.0),
            ("Cusp 2", 45.0),
            ("Cusp 3", 75.0),
            ("Cusp 4", 105.0),
            ("Cusp 5", 135.0),
            ("Cusp 6", 165.0),
            ("Cusp 7", 195.0),
            ("Cusp 8", 225.0),
            ("Cusp 9", 255.0),
            ("Cusp 10", 285.0),
            ("Cusp 11", 315.0),
            ("Cusp 12", 345.0),
        ];

        // 1. Search with "cusps" wildcard target
        let query_all_cusps = TransitSearchQuery {
            start_year: 2026,
            start_month: 1,
            start_day: 1,
            end_year: 2026,
            end_month: 3,
            end_day: 31,
            moving_bodies: Some(vec!["Sun".to_string(), "Mars".to_string()]),
            natal_targets: Some(vec!["cusps".to_string()]),
            aspect_types: Some(vec!["Conjunction".to_string(), "Opposition".to_string()]),
        };

        let hits_all = find_transit_events(&query_all_cusps, &natal_cusps).expect("Search all cusps should succeed");
        assert!(!hits_all.is_empty(), "Expected transit hits to natal cusps");
        for hit in &hits_all {
            assert!(hit.natal_point.starts_with("Cusp "), "All targets must be cusps: {}", hit.natal_point);
            assert!(hit.orb_deg < 0.05);
        }

        // 2. Search specific cusps: Cusp 4 and Cusp 7
        let query_specific = TransitSearchQuery {
            start_year: 2026,
            start_month: 1,
            start_day: 1,
            end_year: 2026,
            end_month: 6,
            end_day: 30,
            moving_bodies: Some(vec!["Mars".to_string()]),
            natal_targets: Some(vec!["Cusp 4".to_string(), "Cusp 7".to_string()]),
            aspect_types: Some(vec!["Conjunction".to_string()]),
        };

        let hits_specific = find_transit_events(&query_specific, &natal_cusps).expect("Search specific cusps should succeed");
        for hit in &hits_specific {
            assert!(hit.natal_point == "Cusp 4" || hit.natal_point == "Cusp 7");
            assert_eq!(hit.aspect_type, "Conjunction");
        }

        // 3. Test alias matching: "IC" matches "Cusp 4", "DSC" matches "Cusp 7"
        assert!(matches_natal_target("cusps", "Cusp 3"));
        assert!(matches_natal_target("all_cusps", "Cusp 9"));
        assert!(matches_natal_target("ASC", "Cusp 1"));
        assert!(matches_natal_target("MC", "Cusp 10"));
        assert!(matches_natal_target("IC", "Cusp 4"));
        assert!(matches_natal_target("DSC", "Cusp 7"));
        assert!(matches_natal_target("Cusp 4", "Cusp 4"));
        assert!(matches_natal_target("cusp4", "Cusp 4"));
    }
}
