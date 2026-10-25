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
    pub is_partile: bool, // same integer degree in sign (Lilly CA p. 106)
    pub degrees_to_perfection: f64,
    #[serde(default)]
    pub will_perfect_in_sign: bool,
}

/// Analytical kinematic evaluation of an aspect between two moving bodies.
#[derive(Debug, Clone, Copy)]
pub struct AspectKinematics {
    pub is_applying: bool,
    pub approach_rate: f64,
    pub time_to_perfection: f64,
    pub elapsed_since_separation: f64,
    pub degrees_to_perfection: f64,
    pub will_perfect_in_sign: bool,
}

/// Analytically evaluate whether two bodies are applying (approaching) or separating
/// relative to `target_angle` (0, 60, 90, 120, 180) without finite-difference overshooting.
pub fn evaluate_aspect_kinematics(
    lon1: f64,
    speed1: f64,
    lon2: f64,
    speed2: f64,
    target_angle: f64,
) -> AspectKinematics {
    let d_lon = (lon1 - lon2).rem_euclid(360.0);
    let rel_speed = speed1 - speed2;

    // Determine which branch of target_angle (+target_angle or 360 - target_angle) is closest
    let signed_err = if target_angle == 0.0 {
        if d_lon <= 180.0 { d_lon } else { d_lon - 360.0 }
    } else if (target_angle - 180.0).abs() < 1e-9 {
        d_lon - 180.0
    } else {
        let mut e_pos = (d_lon - target_angle + 540.0).rem_euclid(360.0) - 180.0;
        let mut e_neg = (d_lon - (360.0 - target_angle) + 540.0).rem_euclid(360.0) - 180.0;
        if e_pos == -180.0 { e_pos = 180.0; }
        if e_neg == -180.0 { e_neg = 180.0; }
        if e_pos.abs() <= e_neg.abs() { e_pos } else { e_neg }
    };

    // d(|e|)/dt = sign(e) * (speed1 - speed2)
    // Therefore approach_rate = -d(|e|)/dt:
    let approach_rate = if signed_err > 1e-12 {
        -rel_speed
    } else if signed_err < -1e-12 {
        rel_speed
    } else {
        0.0
    };

    let is_applying = approach_rate > 1e-9;

    if is_applying {
        let t_perf = signed_err.abs() / approach_rate;
        let deg_to_perf = speed1.abs() * t_perf;

        let sign1_now = ((lon1.rem_euclid(360.0) / 30.0).floor() as u8) % 12;
        let sign2_now = ((lon2.rem_euclid(360.0) / 30.0).floor() as u8) % 12;

        let lon1_perf = (lon1 + speed1 * t_perf).rem_euclid(360.0);
        let lon2_perf = (lon2 + speed2 * t_perf).rem_euclid(360.0);
        let sign1_perf = ((lon1_perf / 30.0).floor() as u8) % 12;
        let sign2_perf = ((lon2_perf / 30.0).floor() as u8) % 12;

        let will_perfect_in_sign = sign1_now == sign1_perf
            && sign2_now == sign2_perf
            && is_valid_sign_aspect(sign1_perf, sign2_perf, target_angle);

        AspectKinematics {
            is_applying: true,
            approach_rate,
            time_to_perfection: t_perf,
            elapsed_since_separation: f64::INFINITY,
            degrees_to_perfection: deg_to_perf,
            will_perfect_in_sign,
        }
    } else {
        let sep_rate = (-approach_rate).max(0.0);
        let elapsed_sep = if sep_rate > 1e-9 {
            signed_err.abs() / sep_rate
        } else {
            f64::INFINITY
        };

        AspectKinematics {
            is_applying: false,
            approach_rate,
            time_to_perfection: f64::INFINITY,
            elapsed_since_separation: elapsed_sep,
            degrees_to_perfection: 0.0,
            will_perfect_in_sign: false,
        }
    }
}

/// Convenience helper returning whether two bodies are analytically applying toward `target_angle`.
pub fn is_aspect_applying(
    lon1: f64,
    speed1: f64,
    lon2: f64,
    speed2: f64,
    target_angle: f64,
) -> bool {
    evaluate_aspect_kinematics(lon1, speed1, lon2, speed2, target_angle).is_applying
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
                    // In classical astrology (Lilly CA p. 107), the lighter planet (faster mean motion
                    // in Chaldean order) applies to or separates from the heavier planet.
                    let (lighter_name, heavier_name, l_lon, h_lon, l_speed, h_speed) =
                        if p1.chaldean_speed_rank() <= p2.chaldean_speed_rank() {
                            (name1, name2, lon1, lon2, speed1, speed2)
                        } else {
                            (name2, name1, lon2, lon1, speed2, speed1)
                        };

                    let kin = evaluate_aspect_kinematics(l_lon, l_speed, h_lon, h_speed, target_angle);

                    // In traditional astrology (Lilly CA p. 106), an aspect is partile strictly when
                    // the planets are in the same integer degree of their respective signs
                    // (e.g. 10° Aries and 10° Leo). Different integer degrees within moiety orb are platic.
                    let deg1_int = (lon1.rem_euclid(30.0)).floor() as u32;
                    let deg2_int = (lon2.rem_euclid(30.0)).floor() as u32;
                    let is_partile = deg1_int == deg2_int;

                    results.push(TraditionalAspect {
                        planet1: lighter_name.to_string(),
                        planet2: heavier_name.to_string(),
                        aspect_type: aspect_name.to_string(),
                        exact_angle: target_angle,
                        actual_angle: shortest,
                        orb,
                        max_allowed_orb: max_orb,
                        is_applying: kin.is_applying,
                        is_partile,
                        degrees_to_perfection: kin.degrees_to_perfection,
                        will_perfect_in_sign: kin.will_perfect_in_sign,
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

    #[test]
    fn test_tiny_orb_applying_no_overshoot() {
        // Moon at 9.9999° Aries applying to Sun at 10.0000° Aries (orb 0.0001°)
        let planets = [
            ("Moon", 9.9999, 13.0),
            ("Sun", 10.0000, 1.0),
        ];
        let aspects = calculate_traditional_aspects(&planets);
        assert_eq!(aspects.len(), 1);
        let asp = &aspects[0];
        assert!(asp.is_applying, "Moon must be correctly classified as applying even with infinitesimal orb");
        // 9.9999° is in integer degree 9, whereas 10.0000° is in integer degree 10 -> Platic, not Partile
        assert!(!asp.is_partile, "Different integer degrees (9°59' vs 10°00') are platic, not partile");
        assert!(asp.will_perfect_in_sign);
    }

    #[test]
    fn test_partile_integer_degree_aspect() {
        // Mars at 15.8° Aries and Jupiter at 15.1° Leo (orb 0.7°):
        // Both are in the 15th degree (integer degree 15), thus partile
        let planets = [
            ("Mars", 15.8, 0.6),
            ("Jupiter", 135.1, 0.1),
        ];
        let aspects = calculate_traditional_aspects(&planets);
        assert_eq!(aspects.len(), 1);
        let asp = &aspects[0];
        assert_eq!(asp.aspect_type, "Trine");
        assert!(asp.is_partile, "Same integer degree in sign must be partile");

        // Different integer degrees within 1.0° orb (15.8° Aries vs 16.2° Leo, orb 0.4°):
        // Must be platic (is_partile == false)
        let planets_platic = [
            ("Mars", 15.8, 0.6),
            ("Jupiter", 136.2, 0.1),
        ];
        let aspects_platic = calculate_traditional_aspects(&planets_platic);
        assert_eq!(aspects_platic.len(), 1);
        assert!(!aspects_platic[0].is_partile, "15°50' and 16°12' are in different integer degrees and thus platic");
    }

    #[test]
    fn test_chaldean_order_and_sign_exit_refranation() {
        // Stationary Mercury (speed 0.01°/d at 29.5° Aries) and Sun (speed 1.0°/d at 28.0° Aries)
        // Even though Sun has higher instantaneous speed, Mercury is the lighter planet in Chaldean order
        let planets = [
            ("Sun", 28.0, 1.0),
            ("Mercury", 29.5, 0.1),
        ];
        let aspects = calculate_traditional_aspects(&planets);
        assert_eq!(aspects.len(), 1);
        let asp = &aspects[0];
        assert_eq!(asp.planet1, "Mercury", "Lighter planet (Mercury) must be planet1");
        assert_eq!(asp.planet2, "Sun", "Heavier planet (Sun) must be planet2");
        assert!(asp.is_applying, "Sun catching up to slow Mercury is applying");
        // Gap is 1.5°, rel_speed is 0.9°/d -> t = 1.6667 d -> Mercury moves 29.5 + 0.1667 = 29.6667° (still in Aries)
        assert!(asp.will_perfect_in_sign);

        // Refranation case: Mercury at 29.5° Aries (speed 0.5°/d), Sun at 27.0° Aries (speed 1.0°/d)
        // Gap is 2.5°, rel_speed = 0.5°/d -> t = 5.0 d -> Mercury reaches 32.0° (Taurus 2°) before perfection!
        let planets_refran = [
            ("Sun", 27.0, 1.0),
            ("Mercury", 29.5, 0.5),
        ];
        let aspects_refran = calculate_traditional_aspects(&planets_refran);
        assert_eq!(aspects_refran.len(), 1);
        assert!(aspects_refran[0].is_applying);
        assert!(!aspects_refran[0].will_perfect_in_sign, "Aspect must be marked as not perfecting in sign (Refranation)");
    }
}

