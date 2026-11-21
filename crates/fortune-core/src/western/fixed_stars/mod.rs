//! 31 Elite Traditional Fixed Stars with Astrometry, Precession, and Geographic Visibility.
//!
//! Propagated dynamically via Hipparcos astrometry (`I/311/hip2`) and filtered by horizon
//! visibility, ecliptic latitude, and stellar magnitude.

mod catalog;

use serde::{Deserialize, Serialize};
use vedaksha::ephem::stars::mean_ecliptic_of_date;

use catalog::TRADITIONAL_STARS;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedStarInfo {
    pub name: String,
    pub korean_name: String,
    pub longitude: f64,
    pub latitude: f64,
    pub magnitude: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub declination: f64,
    pub is_visible: bool,
    pub max_altitude: f64,
    pub nature: String,
    pub meaning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedStarConjunction {
    pub star_name: String,
    pub korean_name: String,
    pub body_name: String,
    pub star_longitude: f64,
    pub star_latitude: f64,
    pub star_magnitude: f64,
    pub body_longitude: f64,
    pub orb: f64,
    pub is_visible: bool,
    pub max_altitude: f64,
    pub is_qualified: bool,
    pub qualification_reason: String,
    pub nature: String,
    pub meaning: String,
}

/// Compute the 31 elite traditional fixed star positions and horizon visibility at a given JD (TT).
pub fn calculate_traditional_fixed_stars(
    jd_tt: f64,
    observer_lat: f64,
    obliquity_deg: f64,
) -> Vec<FixedStarInfo> {
    let eps_rad = obliquity_deg.to_radians();

    TRADITIONAL_STARS
        .iter()
        .map(|def| {
            let (raw_lon, lat) = mean_ecliptic_of_date(&def.star, jd_tt);
            let lon = raw_lon.rem_euclid(360.0);
            let sign_index = (lon / 30.0).floor() as u8 % 12;
            let sign_degree = lon - (sign_index as f64 * 30.0);

            let sin_dec = lat.to_radians().sin() * eps_rad.cos()
                + lat.to_radians().cos() * eps_rad.sin() * lon.to_radians().sin();
            let dec = sin_dec.clamp(-1.0, 1.0).asin().to_degrees();
            let max_altitude = 90.0 - (observer_lat - dec).abs();

            FixedStarInfo {
                name: def.name.to_string(),
                korean_name: def.korean_name.to_string(),
                longitude: lon,
                latitude: lat,
                magnitude: def.magnitude,
                sign_index,
                sign_degree,
                declination: dec,
                is_visible: max_altitude > 0.0,
                max_altitude,
                nature: def.nature.to_string(),
                meaning: def.meaning.to_string(),
            }
        })
        .collect()
}

fn evaluate_star_qualification(star: &FixedStarInfo, orb: f64) -> (bool, &'static str) {
    if !star.is_visible {
        return (false, "지평선 미출몰 (관측 불가)");
    }
    let abs_lat = star.latitude.abs();
    let is_first_mag = star.magnitude <= 1.5;
    let is_algol = star.name == "Algol";

    if abs_lat <= 10.0 {
        if orb <= 1.5 {
            (true, "유효 (황도대 결합)")
        } else {
            (false, "허용오차 초과 (황도대 기준 1.5°)")
        }
    } else if abs_lat <= 30.0 {
        if is_first_mag || is_algol {
            if orb <= 1.2 {
                (true, "유효 (중황위 1등성/핵심성)")
            } else {
                (false, "허용오차 초과 (중황위 기준 1.2°)")
            }
        } else if orb <= 1.0 {
            (true, "유효 (중황위 2등성)")
        } else {
            (false, "허용오차 초과 (중황위 2등성 기준 1.0°)")
        }
    } else if is_first_mag {
        if orb <= 1.0 {
            (true, "유효 (고황위 1등성 제한 인정)")
        } else {
            (false, "허용오차 초과 (고황위 1등성 기준 1.0°)")
        }
    } else {
        (false, "단순 투영 배제 (고황위 2등성 이하)")
    }
}

/// Detect conjunctions between chart bodies and fixed stars within `max_orb`.
pub fn detect_fixed_star_conjunctions(
    stars: &[FixedStarInfo],
    bodies: &[(&str, f64)],
    max_orb: f64,
) -> Vec<FixedStarConjunction> {
    let mut conjunctions = Vec::new();

    for star in stars {
        for &(body_name, body_lon) in bodies {
            let diff = (star.longitude - body_lon.rem_euclid(360.0)).abs();
            let orb = if diff > 180.0 { 360.0 - diff } else { diff };

            if orb <= max_orb {
                let (is_qualified, reason) = evaluate_star_qualification(star, orb);
                conjunctions.push(FixedStarConjunction {
                    star_name: star.name.clone(),
                    korean_name: star.korean_name.clone(),
                    body_name: body_name.to_string(),
                    star_longitude: star.longitude,
                    star_latitude: star.latitude,
                    star_magnitude: star.magnitude,
                    body_longitude: body_lon,
                    orb,
                    is_visible: star.is_visible,
                    max_altitude: star.max_altitude,
                    is_qualified,
                    qualification_reason: reason.to_string(),
                    nature: star.nature.clone(),
                    meaning: star.meaning.clone(),
                });
            }
        }
    }

    conjunctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elite_fixed_stars_catalog() {
        let stars = calculate_traditional_fixed_stars(2_447_965.677_778, 37.5665, 23.4425);
        assert_eq!(stars.len(), 31);

        let behenian = [
            "Algol", "Alcyone (Pleiades)", "Aldebaran", "Capella", "Sirius",
            "Procyon", "Regulus", "Algorab", "Spica", "Alkaid",
            "Arcturus", "Alphecca", "Antares", "Vega", "Deneb Algedi",
        ];
        for b in behenian {
            assert!(stars.iter().any(|s| s.name == b), "Missing Behenian star: {}", b);
        }

        let sirius = stars.iter().find(|s| s.name == "Sirius").unwrap();
        assert!(sirius.is_visible && sirius.max_altitude > 30.0);
        let canopus = stars.iter().find(|s| s.name == "Canopus").unwrap();
        assert!(!canopus.is_visible && canopus.max_altitude < 0.0);
        let achernar = stars.iter().find(|s| s.name == "Achernar").unwrap();
        assert!(!achernar.is_visible && achernar.max_altitude < 0.0);
    }

    #[test]
    fn test_fixed_star_qualification_filters_and_zodiacal_orb_limit() {
        let stars = calculate_traditional_fixed_stars(2_447_965.677_778, 37.5665, 23.4425);

        let alkaid = stars.iter().find(|s| s.name == "Alkaid").unwrap();
        let conj = detect_fixed_star_conjunctions(&stars, &[("Imum Coeli", alkaid.longitude + 0.3)], 1.5);
        let alkaid_c = conj.iter().find(|c| c.star_name == "Alkaid").unwrap();
        assert!(!alkaid_c.is_qualified);
        assert_eq!(alkaid_c.qualification_reason, "단순 투영 배제 (고황위 2등성 이하)");

        let canopus = stars.iter().find(|s| s.name == "Canopus").unwrap();
        let canopus_c = detect_fixed_star_conjunctions(&stars, &[("Ascendant", canopus.longitude + 0.2)], 1.5)
            .into_iter().find(|c| c.star_name == "Canopus").unwrap();
        assert!(!canopus_c.is_qualified);
        assert_eq!(canopus_c.qualification_reason, "지평선 미출몰 (관측 불가)");

        let regulus = stars.iter().find(|s| s.name == "Regulus").unwrap();
        let reg_ok = detect_fixed_star_conjunctions(&stars, &[("Midheaven", regulus.longitude + 0.8)], 2.0)
            .into_iter().find(|c| c.star_name == "Regulus").unwrap();
        assert!(reg_ok.is_qualified);
        assert_eq!(reg_ok.qualification_reason, "유효 (황도대 결합)");

        // Regression: zodiacal stars with 1.5° < orb <= 2.0° must be disqualified, not qualified.
        let reg_wide = detect_fixed_star_conjunctions(&stars, &[("Midheaven", regulus.longitude + 1.8)], 2.0)
            .into_iter().find(|c| c.star_name == "Regulus").unwrap();
        assert!(!reg_wide.is_qualified);
        assert_eq!(reg_wide.qualification_reason, "허용오차 초과 (황도대 기준 1.5°)");

        let sirius = stars.iter().find(|s| s.name == "Sirius").unwrap();
        let sirius_c = detect_fixed_star_conjunctions(&stars, &[("Ascendant", sirius.longitude + 0.5)], 1.5)
            .into_iter().find(|c| c.star_name == "Sirius").unwrap();
        assert!(sirius_c.is_qualified);
        assert_eq!(sirius_c.qualification_reason, "유효 (고황위 1등성 제한 인정)");
    }
}
