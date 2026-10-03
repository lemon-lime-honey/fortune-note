//! Condition of the Moon in Traditional Horary and Natal Astrology.
//!
//! Implements:
//! - Void of Course (VOC) detection (Lilly CA p. 112)
//! - Via Combusta (15° Libra to 15° Scorpio)
//! - Lunar phases (8 phases, waxing/waning)
//! - Next applying aspect before leaving the sign

use serde::{Deserialize, Serialize};

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

/// Evaluates Moon's condition according to William Lilly.
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

    // Lunar Phase & Elongation
    let mut elongation = moon_longitude - sun_longitude;
    if elongation < 0.0 { elongation += 360.0; }
    let is_waxing = elongation < 180.0;

    let lunar_phase_name = match elongation {
        e if e < 45.0 => "New Moon (신월)",
        e if e < 90.0 => "Waxing Crescent (초승달)",
        e if e < 135.0 => "First Quarter (상현달)",
        e if e < 180.0 => "Waxing Gibbous (차오르는 달)",
        e if e < 225.0 => "Full Moon (보름달 / 만월)",
        e if e < 270.0 => "Waning Gibbous (기우는 달)",
        e if e < 315.0 => "Third Quarter (하현달)",
        _ => "Waning Crescent (그믐달)",
    }.to_string();

    // Check for applying major aspects (0°, 60°, 90°, 120°, 180°) before leaving the sign
    let aspect_angles = [0.0, 60.0, 90.0, 120.0, 180.0];
    let mut next_aspect: Option<(f64, String)> = None; // (degrees_to_perfection, description)

    for &(p_name, p_lon, p_speed) in other_planets {
        if p_name == "Moon" { continue; }

        let rel_speed = moon_speed - p_speed;
        if rel_speed <= 0.0 {
            // Moon must be faster to apply
            continue;
        }

        // Test each aspect angle
        for &target in &aspect_angles {
            // Target positions where aspect is exact: p_lon + target or p_lon - target
            for &sign_mult in &[1.0, -1.0] {
                let exact_target_pos = (p_lon + sign_mult * target).rem_euclid(360.0);

                // Check if exact position is between Moon and sign end (in direction of Moon's motion)
                let mut dist_to_exact = exact_target_pos - moon_longitude;
                if dist_to_exact < 0.0 { dist_to_exact += 360.0; }

                // The aspect must be perfected before Moon reaches sign_end
                let dist_to_sign_end = 30.0 - sign_degree;

                if dist_to_exact > 0.001 && dist_to_exact < dist_to_sign_end {
                    let desc = format!("{} to {} in {:.2}°", aspect_name(target), p_name, dist_to_exact);
                    if next_aspect.as_ref().map_or(true, |(d, _)| dist_to_exact < *d) {
                        next_aspect = Some((dist_to_exact, desc));
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
