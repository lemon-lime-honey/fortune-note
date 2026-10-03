//! Prenatal Syzygy (Syzygy Ante Nativitatem - SAN) calculation engine.
//!
//! Calculates the exact moment, zodiac longitude, and phase type (New Moon or Full Moon)
//! of the last syzygy occurring immediately prior to the moment of birth.
//!
//! Essential for:
//! - Hyleg determination (Ptolemy, Dorotheus, Lilly)
//! - Almuten Figuris (Chart Almuten)
//! - Hermetic Lot calculation baselines

use serde::{Deserialize, Serialize};
use vedaksha::ephem::analytical::AnalyticalProvider;
use vedaksha::ephem::bodies::Body;
use vedaksha::ephem::coordinates::apparent_position;
use vedaksha::ephem::julian::jd_to_calendar;
use vedaksha::math::angle::normalize_degrees;

/// Prenatal Syzygy report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrenatalSyzygy {
    pub syzygy_type: String, // "New Moon" or "Full Moon"
    pub exact_datetime_utc: String,
    pub exact_jd_ut1: f64,
    pub longitude: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub sun_longitude: f64,
    pub moon_longitude: f64,
}

/// Calculate the exact prenatal syzygy immediately prior to the given birth JD.
pub fn calculate_prenatal_syzygy(birth_jd: f64) -> Result<PrenatalSyzygy, String> {
    let provider = AnalyticalProvider;

    // 1. Calculate Sun and Moon at birth
    let sun_pos = apparent_position(&provider, Body::Sun, birth_jd)
        .map_err(|e| format!("Sun position calculation failed at birth JD: {:?}", e))?;
    let moon_pos = apparent_position(&provider, Body::Moon, birth_jd)
        .map_err(|e| format!("Moon position calculation failed at birth JD: {:?}", e))?;

    let birth_sun_lon = normalize_degrees(sun_pos.ecliptic.longitude.to_degrees());
    let birth_moon_lon = normalize_degrees(moon_pos.ecliptic.longitude.to_degrees());

    let mut elongation = birth_moon_lon - birth_sun_lon;
    if elongation < 0.0 {
        elongation += 360.0;
    }

    // Determine target syzygy
    // If elongation is between 0° and 180°, the immediate prior syzygy was a New Moon (0°).
    // If elongation is between 180° and 360°, the immediate prior syzygy was a Full Moon (180°).
    let (is_new_moon, target_angle, days_prior_approx) = if elongation < 180.0 {
        (true, 0.0, elongation / 12.190_749)
    } else {
        (false, 180.0, (elongation - 180.0) / 12.190_749)
    };

    let mut cur_jd = birth_jd - days_prior_approx;

    // 2. Newton-Raphson solver to find exact moment where elongation == target_angle
    for _ in 0..15 {
        let s_pos = apparent_position(&provider, Body::Sun, cur_jd)
            .map_err(|e| format!("Sun calculation failed during syzygy iteration: {:?}", e))?;
        let m_pos = apparent_position(&provider, Body::Moon, cur_jd)
            .map_err(|e| format!("Moon calculation failed during syzygy iteration: {:?}", e))?;

        let s_lon = normalize_degrees(s_pos.ecliptic.longitude.to_degrees());
        let m_lon = normalize_degrees(m_pos.ecliptic.longitude.to_degrees());

        let mut diff = (m_lon - s_lon).rem_euclid(360.0) - target_angle;
        if diff > 180.0 {
            diff -= 360.0;
        } else if diff < -180.0 {
            diff += 360.0;
        }

        if diff.abs() < 1e-6 {
            // Converged
            let (y, m, d_frac) = jd_to_calendar(cur_jd);
            let day = d_frac.floor() as u32;
            let day_rem = (d_frac - day as f64) * 24.0;
            let hour = day_rem.floor() as u32;
            let hour_rem = (day_rem - hour as f64) * 60.0;
            let minute = hour_rem.floor() as u32;
            let second = ((hour_rem - minute as f64) * 60.0).round() as u32;

            let exact_datetime_utc = format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
                y, m, day, hour, minute, second.min(59)
            );

            // In traditional astrology, the SAN degree is taken from the Moon's longitude
            let target_lon = m_lon;
            let sign_index = (target_lon / 30.0).floor() as u8 % 12;
            let sign_degree = target_lon - (sign_index as f64 * 30.0);

            return Ok(PrenatalSyzygy {
                syzygy_type: if is_new_moon { "New Moon".to_string() } else { "Full Moon".to_string() },
                exact_datetime_utc,
                exact_jd_ut1: cur_jd,
                longitude: target_lon,
                sign_index,
                sign_degree,
                sun_longitude: s_lon,
                moon_longitude: m_lon,
            });
        }

        let rel_speed = (m_pos.longitude_speed - s_pos.longitude_speed).abs().max(10.0);
        cur_jd -= diff / rel_speed;
    }

    Err("Prenatal syzygy solver failed to converge within 15 iterations".to_string())
}
