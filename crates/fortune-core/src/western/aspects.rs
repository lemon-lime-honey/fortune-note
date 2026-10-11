//! Dynamic Aspect Analysis (Applying vs Separating) and Planetary Moiety of Orbs.
//!
//! Based on Christian Astrology (1647), Chapter XIX (p. 106-112).

use serde::{Deserialize, Serialize};
use super::dignity::ClassicalPlanet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionalAspect {
    pub planet1: String,
    pub planet2: String,
    pub aspect_type: String,
    pub exact_angle: f64,
    pub actual_angle: f64,
    pub orb: f64,
    pub max_allowed_orb: f64,
    pub is_applying: bool,
    pub is_partile: bool, // within 1 degree
    pub degrees_to_perfection: f64,
}

/// Check if two zodiac sign indices (0..12) form a valid traditional sign configuration for an aspect.
///
/// Traditional astrology requires aspects to be by sign (Lilly CA p. 106-112):
/// - Conjunction (0°): Same sign (diff 0)
/// - Sextile (60°): 2 or 10 signs apart
/// - Square (90°): 3 or 9 signs apart
/// - Trine (120°): 4 or 8 signs apart
/// - Opposition (180°): 6 signs apart
pub fn is_valid_sign_aspect(sign1: u8, sign2: u8, target_angle: f64) -> bool {
    let diff = ((sign1 as i16 - sign2 as i16).rem_euclid(12)) as u8;
    match target_angle as u32 {
        0 => diff == 0,
        60 => diff == 2 || diff == 10,
        90 => diff == 3 || diff == 9,
        120 => diff == 4 || diff == 8,
        180 => diff == 6,
        _ => false,
    }
}

pub fn calculate_traditional_aspects(
    planets: &[(&str, f64, f64)], // (name, longitude, speed)
) -> Vec<TraditionalAspect> {
    let aspect_defs = [
        ("Conjunction", 0.0),
        ("Sextile", 60.0),
        ("Square", 90.0),
        ("Trine", 120.0),
        ("Opposition", 180.0),
    ];

    let mut results = Vec::new();

    for i in 0..planets.len() {
        for j in (i + 1)..planets.len() {
            let (name1, lon1, speed1) = planets[i];
            let (name2, lon2, speed2) = planets[j];

            let p1_opt = ClassicalPlanet::from_name(name1);
            let p2_opt = ClassicalPlanet::from_name(name2);
            if p1_opt.is_none() || p2_opt.is_none() {
                continue;
            }
            let p1 = p1_opt.unwrap();
            let p2 = p2_opt.unwrap();

            let sign1 = ((lon1.rem_euclid(360.0) / 30.0).floor() as u8) % 12;
            let sign2 = ((lon2.rem_euclid(360.0) / 30.0).floor() as u8) % 12;

            // Combined moiety orb
            let max_orb = p1.moiety() + p2.moiety();

            let diff = (lon1 - lon2).abs().rem_euclid(360.0);
            let shortest = if diff > 180.0 { 360.0 - diff } else { diff };

            for &(aspect_name, target_angle) in &aspect_defs {
                // Aspect must be valid by whole sign geometry (no out-of-sign aspects in traditional Lilly)
                if !is_valid_sign_aspect(sign1, sign2, target_angle) {
                    continue;
                }

                let orb = (shortest - target_angle).abs();
                if orb <= max_orb {
                    // Determine which planet is faster
                    let (faster_name, slower_name, f_lon, s_lon, f_speed, s_speed) = if speed1.abs() >= speed2.abs() {
                        (name1, name2, lon1, lon2, speed1, speed2)
                    } else {
                        (name2, name1, lon2, lon1, speed2, speed1)
                    };

                    // Compute future angle distance in small time step (0.001 day ~ 86.4s) to evaluate instantaneous approach
                    let dt = 0.001;
                    let future_f_lon = (f_lon + f_speed * dt).rem_euclid(360.0);
                    let future_s_lon = (s_lon + s_speed * dt).rem_euclid(360.0);
                    let future_diff = (future_f_lon - future_s_lon).abs().rem_euclid(360.0);
                    let future_shortest = if future_diff > 180.0 { 360.0 - future_diff } else { future_diff };
                    let future_orb = (future_shortest - target_angle).abs();

                    let approach_rate = (orb - future_orb) / dt; // deg/day
                    let is_applying = approach_rate > 1e-6;
                    let is_partile = orb <= 1.0;

                    // Degrees to perfection: distance the applying (faster) planet travels until perfection
                    let degrees_to_perfection = if is_applying && approach_rate > 1e-6 {
                        let time_to_perfection = orb / approach_rate; // days
                        f_speed.abs() * time_to_perfection
                    } else {
                        0.0
                    };

                    results.push(TraditionalAspect {
                        planet1: faster_name.to_string(),
                        planet2: slower_name.to_string(),
                        aspect_type: aspect_name.to_string(),
                        exact_angle: target_angle,
                        actual_angle: shortest,
                        orb,
                        max_allowed_orb: max_orb,
                        is_applying,
                        is_partile,
                        degrees_to_perfection,
                    });
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_aspect_filtering() {
        // Pisces 29° (359°) and Aries 1° (1°) - distance 2°, but Pisces and Aries are adjacent (no major aspect)
        let planets = [
            ("Venus", 359.0, 1.0),
            ("Mars", 1.0, 0.5),
        ];
        let aspects = calculate_traditional_aspects(&planets);
        assert!(aspects.is_empty(), "Adjacent signs must not form conjunction or any major aspect");

        // Aries 10° and Gemini 10° - exact sextile
        let planets2 = [
            ("Moon", 10.0, 13.0),
            ("Sun", 70.0, 1.0),
        ];
        let aspects2 = calculate_traditional_aspects(&planets2);
        assert_eq!(aspects2.len(), 1);
        assert_eq!(aspects2[0].aspect_type, "Sextile");
    }

    #[test]
    fn test_applying_separating_and_degrees_to_perfection() {
        // Moon (speed 13°/d) at 8° Aries applying to Sun (speed 1°/d) at 10° Aries (orb 2°)
        let planets = [
            ("Moon", 8.0, 13.0),
            ("Sun", 10.0, 1.0),
        ];
        let aspects = calculate_traditional_aspects(&planets);
        assert_eq!(aspects.len(), 1);
        let asp = &aspects[0];
        assert_eq!(asp.aspect_type, "Conjunction");
        assert!(asp.is_applying, "Moon must be applying to Sun");
        // Relative speed is 12°/d, orb 2° -> time 2/12 = 1/6 day -> Moon moves 13 * 1/6 = 2.1667°
        assert!((asp.degrees_to_perfection - 2.1667).abs() < 0.05);

        // Moon at 12° Aries separating from Sun at 10° Aries
        let planets_sep = [
            ("Moon", 12.0, 13.0),
            ("Sun", 10.0, 1.0),
        ];
        let aspects_sep = calculate_traditional_aspects(&planets_sep);
        assert_eq!(aspects_sep.len(), 1);
        assert!(!aspects_sep[0].is_applying, "Moon must be separating from Sun");
        assert_eq!(aspects_sep[0].degrees_to_perfection, 0.0);
    }
}

