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

            // Combined moiety orb
            let max_orb = p1.moiety() + p2.moiety();

            let diff = (lon1 - lon2).abs();
            let shortest = if diff > 180.0 { 360.0 - diff } else { diff };

            for &(aspect_name, target_angle) in &aspect_defs {
                let orb = (shortest - target_angle).abs();
                if orb <= max_orb {
                    // Determine which planet is faster
                    let (faster_name, slower_name, f_lon, s_lon, f_speed, s_speed) = if speed1.abs() >= speed2.abs() {
                        (name1, name2, lon1, lon2, speed1, speed2)
                    } else {
                        (name2, name1, lon2, lon1, speed2, speed1)
                    };

                    let rel_speed = f_speed - s_speed;

                    // Compute future angle distance in 1 day to see if gap to target_angle is closing
                    let future_f_lon = (f_lon + f_speed).rem_euclid(360.0);
                    let future_s_lon = (s_lon + s_speed).rem_euclid(360.0);
                    let future_diff = (future_f_lon - future_s_lon).abs();
                    let future_shortest = if future_diff > 180.0 { 360.0 - future_diff } else { future_diff };
                    let future_orb = (future_shortest - target_angle).abs();

                    let is_applying = future_orb < orb;
                    let is_partile = orb <= 1.0;

                    let degrees_to_perfection = if is_applying && rel_speed.abs() > 1e-4 {
                        orb
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
