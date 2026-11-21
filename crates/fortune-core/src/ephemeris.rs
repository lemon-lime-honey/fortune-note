//! Pure astronomical ephemeris calculation layer.
//!
//! Wraps `vedaksha` VSOP87D / ELP2000 analytical ephemerides to produce
//! unified tropical and sidereal positions, nutation, obliquity, RAMC,
//! and Lahiri True Chitra Paksha ayanamsha for a given UTC moment and observer coordinates.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use vedaksha::astro::sidereal::{true_ayanamsha_value, Ayanamsha};
use vedaksha::ephem::analytical::AnalyticalProvider;
use vedaksha::ephem::bodies::Body;
use vedaksha::ephem::coordinates::apparent_position;
use vedaksha::ephem::delta_t::ut1_to_tt;
use vedaksha::ephem::nutation::nutation;
use vedaksha::ephem::obliquity::true_obliquity;
use vedaksha::ephem::sidereal_time::local_sidereal_time;
use vedaksha::math::angle::normalize_degrees;

use crate::celestial::{CalendarDateTime, GeoCoordinate};

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

const TARGET_BODIES: [(&str, Body); 9] = [
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

/// Compute astronomical snapshot for a given UTC calendar date/time and observer coordinates.
pub fn compute_ephemeris(
    utc: CalendarDateTime,
    loc: GeoCoordinate,
) -> Result<EphemerisSnapshot, EphemerisError> {
    let _ = loc.latitude; // Reserved for topocentric corrections if needed
    let jd_ut1 = utc.to_julian_day();

    let jd_tt = ut1_to_tt(jd_ut1);
    let (dpsi, deps) = nutation(jd_tt);
    let eps_true = true_obliquity(jd_tt, deps);
    let true_obliquity_deg = eps_true.to_degrees();

    let ramc = local_sidereal_time(jd_ut1, loc.longitude.to_radians(), dpsi, eps_true).to_degrees();
    let true_ayanamsha_deg = true_ayanamsha_value(Ayanamsha::IndianOfficial, jd_ut1);

    let provider = AnalyticalProvider;
    let mut bodies = Vec::with_capacity(TARGET_BODIES.len());

    for (name, body) in TARGET_BODIES {
        let pos = apparent_position(&provider, body, jd_ut1).map_err(|e| {
            EphemerisError::BodyCalculationFailed(name.to_string(), format!("{:?}", e))
        })?;

        let trop_lon = normalize_degrees(pos.ecliptic.longitude.to_degrees());
        let sid_lon = normalize_degrees(trop_lon - true_ayanamsha_deg);
        let speed = pos.longitude_speed;

        bodies.push(CelestialBody {
            name: name.to_string(),
            tropical_longitude: trop_lon,
            sidereal_longitude: sid_lon,
            latitude: pos.ecliptic.latitude.to_degrees(),
            speed,
            is_retrograde: speed < 0.0,
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
    if !approx_jd.is_finite() || !target_lon_deg.is_finite() {
        return Err(EphemerisError::SolarConvergenceFailed(target_lon_deg));
    }

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
        jd += diff / speed;
    }

    Err(EphemerisError::SolarConvergenceFailed(target_lon_deg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use vedaksha::ephem::julian::calendar_to_jd;

    #[test]
    fn test_compute_ephemeris_vernal_equinox_2024() {
        let ephem = compute_ephemeris(
            CalendarDateTime::new(2024, 3, 20, 12, 0, 0.0),
            GeoCoordinate::new(37.5665, 126.9780),
        )
        .expect("ephemeris calculation should succeed");

        assert_eq!(ephem.bodies.len(), 9);
        assert!(ephem.true_ayanamsha_deg > 24.0 && ephem.true_ayanamsha_deg < 24.3);

        let sun = ephem.get_body("Sun").expect("Sun should exist");
        assert!(sun.tropical_longitude >= 0.0 && sun.tropical_longitude < 1.0);
        assert!(sun.sidereal_longitude > 335.0 && sun.sidereal_longitude < 337.0);

        let rahu = ephem.get_body("Rahu").expect("Rahu should exist");
        let ketu = ephem.get_body("Ketu").expect("Ketu should exist");
        let diff = (rahu.tropical_longitude - ketu.tropical_longitude).abs();
        assert!((diff - 180.0).abs() < 1e-4);
    }

    #[test]
    fn test_find_solar_longitude_ipchun_2024() {
        let approx_jd = calendar_to_jd(2024, 2, 4.0);
        let exact_jd = find_solar_longitude_moment(approx_jd, 315.0)
            .expect("Ipchun solver should converge");

        let expected_jd = calendar_to_jd(2024, 2, 4.0 + (8.0 + 27.0 / 60.0) / 24.0);
        let diff_minutes = (exact_jd - expected_jd).abs() * 1440.0;
        assert!(diff_minutes < 2.0, "Ipchun time error too large: {:.2} min", diff_minutes);
    }

    #[test]
    fn test_find_solar_longitude_rejects_non_finite() {
        let res = find_solar_longitude_moment(f64::NAN, 315.0);
        assert!(matches!(res, Err(EphemerisError::SolarConvergenceFailed(_))));
    }
}
