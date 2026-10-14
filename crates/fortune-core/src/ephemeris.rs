//! Pure astronomical ephemeris calculation layer.
//!
//! Wraps Vedaksha analytical engine to compute:
//! - Julian Day (UT1) and Terrestrial Time (TT)
//! - Earth orientation: Nutation, True Obliquity, and Local Sidereal Time (RAMC)
//! - Lahiri True Ayanamsha
//! - 7 classical planets + True Osculating Lunar Nodes (Rahu/Ketu)
//! - Exact solar longitude arrival moments (Newton-Raphson solver)

use serde::{Deserialize, Serialize};
use thiserror::Error;

use vedaksha::astro::sidereal::{true_ayanamsha_value, Ayanamsha};
use vedaksha::ephem::analytical::AnalyticalProvider;
use vedaksha::ephem::bodies::Body;
use vedaksha::ephem::coordinates::apparent_position;
use vedaksha::ephem::delta_t::ut1_to_tt;
use vedaksha::ephem::julian::calendar_to_jd;
use vedaksha::ephem::nutation::nutation;
use vedaksha::ephem::obliquity::true_obliquity;
use vedaksha::ephem::sidereal_time::local_sidereal_time;
use vedaksha::math::angle::normalize_degrees;

#[derive(Error, Debug)]
pub enum EphemerisError {
    #[error("Calculation failure for body {0}: {1}")]
    BodyCalculationFailed(String, String),
    #[error("Solar solver failed to converge for angle {0}")]
    SolarConvergenceFailed(f64),
}

/// Planetary body observation data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelestialBody {
    pub name: String,
    pub tropical_longitude: f64,
    pub sidereal_longitude: f64,
    pub latitude: f64,
    pub speed: f64,
    pub is_retrograde: bool,
}

/// Complete astronomical observation snapshot at a given time and location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemerisSnapshot {
    pub jd_ut1: f64,
    pub jd_tt: f64,
    pub ramc: f64,
    pub true_obliquity_deg: f64,
    pub true_ayanamsha_deg: f64,
    pub bodies: Vec<CelestialBody>,
}

impl EphemerisSnapshot {
    pub fn get_body(&self, name: &str) -> Option<&CelestialBody> {
        self.bodies.iter().find(|b| b.name == name)
    }
}

/// Compute astronomical snapshot for a given UTC calendar date, time, and observer coordinates.
pub fn compute_ephemeris(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: f64,
    latitude: f64,
    longitude: f64,
) -> Result<EphemerisSnapshot, EphemerisError> {
    let _ = latitude; // Reserved for topocentric corrections if needed
    let day_fraction = day as f64 + (hour as f64 + (minute as f64 + second / 60.0) / 60.0) / 24.0;
    let jd_ut1 = calendar_to_jd(year, month, day_fraction);

    let jd_tt = ut1_to_tt(jd_ut1);
    let (dpsi, deps) = nutation(jd_tt);
    let eps_true = true_obliquity(jd_tt, deps);
    let true_obliquity_deg = eps_true.to_degrees();

    let ramc = local_sidereal_time(jd_ut1, longitude.to_radians(), dpsi, eps_true).to_degrees();
    let true_ayanamsha_deg = true_ayanamsha_value(Ayanamsha::IndianOfficial, jd_ut1);

    let provider = AnalyticalProvider;
    let target_bodies = [
        ("Sun", Body::Sun),
        ("Moon", Body::Moon),
        ("Mercury", Body::Mercury),
        ("Venus", Body::Venus),
        ("Mars", Body::Mars),
        ("Jupiter", Body::Jupiter),
        ("Saturn", Body::Saturn),
        ("Rahu", Body::TrueNode),
        ("Ketu", Body::TrueSouthNode),
    ];

    let mut bodies = Vec::with_capacity(target_bodies.len());
    for (name, body) in target_bodies {
        let pos = apparent_position(&provider, body, jd_ut1).map_err(|e| {
            EphemerisError::BodyCalculationFailed(name.to_string(), format!("{:?}", e))
        })?;

        let trop_lon = normalize_degrees(pos.ecliptic.longitude.to_degrees());
        let sid_lon = normalize_degrees(trop_lon - true_ayanamsha_deg);
        let speed = pos.longitude_speed;
        let is_retrograde = speed < 0.0;

        bodies.push(CelestialBody {
            name: name.to_string(),
            tropical_longitude: trop_lon,
            sidereal_longitude: sid_lon,
            latitude: pos.ecliptic.latitude.to_degrees(),
            speed,
            is_retrograde,
        });
    }

    Ok(EphemerisSnapshot {
        jd_ut1,
        jd_tt,
        ramc,
        true_obliquity_deg,
        true_ayanamsha_deg,
        bodies,
    })
}

/// Find the exact Julian Day when the Sun reaches a target tropical longitude (0..360 deg).
pub fn find_solar_longitude_moment(
    approx_jd: f64,
    target_lon_deg: f64,
) -> Result<f64, EphemerisError> {
    let provider = AnalyticalProvider;
    let mut jd = approx_jd;

    for _ in 0..20 {
        let pos = apparent_position(&provider, Body::Sun, jd)
            .map_err(|e| EphemerisError::BodyCalculationFailed("Sun".to_string(), format!("{:?}", e)))?;
        let current_lon = normalize_degrees(pos.ecliptic.longitude.to_degrees());
        let mut diff = target_lon_deg - current_lon;
        if diff > 180.0 {
            diff -= 360.0;
        } else if diff < -180.0 {
            diff += 360.0;
        }

        if diff.abs() < 1e-7 {
            return Ok(jd);
        }

        let speed = pos.longitude_speed.max(0.95);
        let step = diff / speed;
        jd += step;
    }

    Ok(jd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vedaksha::ephem::julian::jd_to_calendar;

    #[test]
    fn test_ephemeris_basic_sanity() {
        let ephem = compute_ephemeris(2024, 3, 20, 12, 0, 0.0, 37.5665, 126.9780)
            .expect("ephemeris computation should succeed");

        assert_eq!(ephem.bodies.len(), 9);
        assert!((23.0..25.0).contains(&ephem.true_ayanamsha_deg));

        let rahu = ephem.get_body("Rahu").unwrap();
        let ketu = ephem.get_body("Ketu").unwrap();
        let node_diff = ((rahu.sidereal_longitude - ketu.sidereal_longitude).abs() - 180.0).abs();
        assert!(node_diff < 0.01, "Nodes must be opposed by 180 degrees");
    }

    #[test]
    fn test_solar_equinox_moment() {
        // 2024 Vernal Equinox (0 deg) occurred on March 20 around 03:06 UTC
        let approx_jd = calendar_to_jd(2024, 3, 20.0);
        let equinox_jd = find_solar_longitude_moment(approx_jd, 0.0)
            .expect("solver should find equinox");

        let (y, m, d_frac) = jd_to_calendar(equinox_jd);
        assert_eq!(y, 2024);
        assert_eq!(m, 3);
        assert_eq!(d_frac.floor() as u32, 20);
    }

    #[test]
    fn test_kasi_2024_solar_terms_precision() {
        let test_cases = [
            ("입춘", 315.0, 2024, 2, 4, 8, 27),
            ("춘분", 0.0,   2024, 3, 20, 3, 6),
            ("하지", 90.0,  2024, 6, 20, 20, 51),
            ("추분", 180.0, 2024, 9, 22, 12, 44),
            ("동지", 270.0, 2024, 12, 21, 9, 20),
        ];

        for (name, target_deg, exp_y, exp_m, exp_d, exp_h, exp_min) in test_cases {
            let approx_jd = calendar_to_jd(exp_y, exp_m, exp_d as f64);
            let calc_jd = find_solar_longitude_moment(approx_jd, target_deg)
                .expect("solar moment should converge");

            let (y, m, d_frac) = jd_to_calendar(calc_jd);
            let d = d_frac.floor() as u32;
            let day_remainder = d_frac - d as f64;
            let total_minutes = day_remainder * 1440.0;
            let h = (total_minutes / 60.0).floor() as u32;
            let min = (total_minutes % 60.0).round() as u32;

            let diff_minutes = ((h as i32 * 60 + min as i32) - (exp_h * 60 + exp_min)).abs();

            assert_eq!(y, exp_y);
            assert_eq!(m, exp_m);
            assert_eq!(d, exp_d);
            assert!(
                diff_minutes <= 1,
                "Solar term {} expected {:02}:{:02} UTC, got {:02}:{:02} UTC",
                name, exp_h, exp_min, h, min
            );
        }
    }
}

