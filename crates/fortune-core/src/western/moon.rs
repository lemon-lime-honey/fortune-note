//! Condition of the Moon in Traditional Horary and Natal Astrology.
//!
//! Implements:
//! - Void of Course (VOC) detection (Lilly CA p. 112)
//! - Via Combusta (15° Libra to 15° Scorpio)
//! - Lunar phases (8 phases, waxing/waning)
//! - Next applying aspect before leaving the sign

use serde::{Deserialize, Serialize};
use super::aspects::is_valid_sign_aspect;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonCondition {
    pub is_void_of_course: bool,
    pub is_via_combusta: bool,
    pub is_in_tincture_exception: bool, // Taurus, Cancer, Sag, Pisces where VOC rarely hurts
    pub lunar_phase_name: String,
    pub is_waxing: bool,
    pub elongation_deg: f64,
    pub next_applying_aspect: Option<String>,
}

/// Evaluates Moon's phase and elongation for Natal charts without Horary-only VOC or Via Combusta flags.
pub fn evaluate_natal_moon_condition(
    moon_longitude: f64,
    sun_longitude: f64,
) -> MoonCondition {
    let mut elongation = moon_longitude - sun_longitude;
    if elongation < 0.0 {
        elongation += 360.0;
    }
    let is_waxing = elongation < 180.0;

    let lunar_phase_name = match elongation {
        e if !(22.5..337.5).contains(&e) => "New Moon",
        e if (22.5..67.5).contains(&e) => "Waxing Crescent",
        e if (67.5..112.5).contains(&e) => "First Quarter",
        e if (112.5..157.5).contains(&e) => "Waxing Gibbous",
        e if (157.5..202.5).contains(&e) => "Full Moon",
        e if (202.5..247.5).contains(&e) => "Waning Gibbous",
        e if (247.5..292.5).contains(&e) => "Third Quarter",
        _ => "Waning Crescent",
    }
    .to_string();

    MoonCondition {
        is_void_of_course: false,
        is_via_combusta: false,
        is_in_tincture_exception: false,
        lunar_phase_name,
        is_waxing,
        elongation_deg: elongation,
        next_applying_aspect: None,
    }
}

/// Evaluates Moon's condition according to William Lilly (Horary mode).
///
/// In Lilly: Moon is Void of Course when she has separated from a planet,
/// and doth not during her stay in that sign apply to any other planet by major aspect.
pub fn evaluate_moon_condition(
    moon_longitude: f64,
    moon_speed: f64,
    sun_longitude: f64,
    other_planets: &[(&str, f64, f64)], // (name, longitude, speed)
) -> MoonCondition {
    let sign_idx = (moon_longitude / 30.0).floor() as u8 % 12;
    let sign_degree = moon_longitude - (sign_idx as f64 * 30.0);

    // Via Combusta: 15° Libra (195.0°) to 15° Scorpio (225.0°)
    let is_via_combusta = (195.0..=225.0).contains(&moon_longitude);

    // Lilly exception: In Taurus(1), Cancer(3), Sagittarius(8), or Pisces(11), VOC "rarely hurts"
    let is_in_tincture_exception = matches!(sign_idx, 1 | 3 | 8 | 11);

    let natal_base = evaluate_natal_moon_condition(moon_longitude, sun_longitude);
    let elongation = natal_base.elongation_deg;
    let is_waxing = natal_base.is_waxing;
    let lunar_phase_name = natal_base.lunar_phase_name;

    // Check for applying major aspects (0°, 60°, 90°, 120°, 180°) before leaving the sign
    let aspect_angles = [0.0, 60.0, 90.0, 120.0, 180.0];
    let mut next_aspect: Option<(f64, String)> = None; // (degrees_to_perfection, description)
    let dist_to_sign_end = 30.0 - sign_degree;

    for &(p_name, p_lon, p_speed) in other_planets {
        if p_name == "Moon" { continue; }

        let rel_speed = moon_speed - p_speed;
        if rel_speed <= 0.0 {
            // Moon must be faster to apply
            continue;
        }

        let p_sign_idx = ((p_lon.rem_euclid(360.0) / 30.0).floor() as u8) % 12;

        // Test each aspect angle
        for &target in &aspect_angles {
            // Must form a valid aspect by whole sign geometry (Lilly CA p. 112)
            if !is_valid_sign_aspect(sign_idx, p_sign_idx, target) {
                continue;
            }

            let c0 = [p_lon.rem_euclid(360.0)];
            let c180 = [(p_lon + 180.0).rem_euclid(360.0)];
            let c_both = [
                (p_lon - target).rem_euclid(360.0),
                (p_lon + target).rem_euclid(360.0),
            ];
            let candidates: &[f64] = if target == 0.0 {
                &c0
            } else if target == 180.0 {
                &c180
            } else {
                &c_both
            };

            for &exact_target_pos in candidates {
                // Check if exact position falls in Moon's current sign
                let target_sign = ((exact_target_pos / 30.0).floor() as u8) % 12;
                if target_sign != sign_idx {
                    continue;
                }

                let diff = (exact_target_pos - moon_longitude).rem_euclid(360.0);
                if diff <= 0.001 {
                    continue;
                }

                // Days to perfection considering other planet's movement
                let t = diff / rel_speed;
                let dist_moon_moves = moon_speed * t;

                // The aspect must be perfected before Moon reaches sign end
                if dist_moon_moves < dist_to_sign_end {
                    // Check if other planet also stays in its sign until perfection
                    let future_p_lon = (p_lon + p_speed * t).rem_euclid(360.0);
                    let future_p_sign = ((future_p_lon / 30.0).floor() as u8) % 12;
                    if future_p_sign == p_sign_idx {
                        let desc = format!("{} to {} in {:.2}°", aspect_name(target), p_name, dist_moon_moves);
                        if next_aspect.as_ref().is_none_or(|(d, _)| dist_moon_moves < *d) {
                            next_aspect = Some((dist_moon_moves, desc));
                        }
                    }
                }
            }
        }
    }

    let is_void_of_course = next_aspect.is_none();
    let next_applying_aspect = next_aspect.map(|(_, desc)| desc);

    MoonCondition {
        is_void_of_course,
        is_via_combusta,
        is_in_tincture_exception,
        lunar_phase_name,
        is_waxing,
        elongation_deg: elongation,
        next_applying_aspect,
    }
}

fn aspect_name(angle: f64) -> &'static str {
    match angle as u32 {
        0 => "Conjunction",
        60 => "Sextile",
        90 => "Square",
        120 => "Trine",
        180 => "Opposition",
        _ => "Aspect",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moon_next_applying_aspect_dynamics() {
        // Moon in Aries 10° (speed 13°/d), Jupiter in Gemini 15° (speed 0.2°/d)
        // Aries and Gemini form a Sextile (diff 2 signs).
        // Target is Gemini 15° - 60° = Aries 15°.
        // Gap is 5°, rel_speed is 12.8°/d -> t = 5/12.8 = 0.390625 d -> Moon travels 13 * 0.390625 = 5.078125°
        let planets = [
            ("Jupiter", 75.0, 0.2),
        ];
        let cond = evaluate_moon_condition(10.0, 13.0, 280.0, &planets);
        assert!(!cond.is_void_of_course);
        assert!(cond.next_applying_aspect.is_some());
        let desc = cond.next_applying_aspect.unwrap();
        assert!(desc.starts_with("Sextile to Jupiter in 5.08°"));
    }

    #[test]
    fn test_moon_voc_when_no_aspect_before_sign_exit() {
        // Moon at Aries 28° (only 2° until sign exit).
        // Jupiter in Gemini 15° -> target is Aries 15° (already passed).
        // Sun in Leo 20° -> Trine target is Aries 20° (already passed).
        let planets = [
            ("Jupiter", 75.0, 0.2),
            ("Sun", 140.0, 1.0),
        ];
        let cond = evaluate_moon_condition(28.0, 13.0, 140.0, &planets);
        assert!(cond.is_void_of_course);
        assert!(cond.next_applying_aspect.is_none());
    }
}

