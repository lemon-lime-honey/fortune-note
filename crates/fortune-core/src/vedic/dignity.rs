//! Classical Parashari planetary dignities (Panchadha Maitri), combustion (Moudhya),
//! and Jaimini co-lordship strength evaluation.

use std::collections::HashMap;
use vedaksha::math::angle::normalize_degrees;
use vedaksha::vedic::graha::Graha;

pub(crate) fn parse_graha(name: &str) -> Option<Graha> {
    match name {
        "Sun" => Some(Graha::Sun),
        "Moon" => Some(Graha::Moon),
        "Mars" => Some(Graha::Mars),
        "Mercury" => Some(Graha::Mercury),
        "Jupiter" => Some(Graha::Jupiter),
        "Venus" => Some(Graha::Venus),
        "Saturn" => Some(Graha::Saturn),
        "Rahu" => Some(Graha::Rahu),
        "Ketu" => Some(Graha::Ketu),
        _ => None,
    }
}

pub(crate) fn sign_lord(sign_index: u8) -> Graha {
    const LORDS: [Graha; 12] = [
        Graha::Mars, Graha::Venus, Graha::Mercury, Graha::Moon,
        Graha::Sun, Graha::Mercury, Graha::Venus, Graha::Mars,
        Graha::Jupiter, Graha::Saturn, Graha::Saturn, Graha::Jupiter,
    ];
    LORDS[(sign_index % 12) as usize]
}

fn graha_index(g: Graha) -> usize {
    match g {
        Graha::Sun => 0,
        Graha::Moon => 1,
        Graha::Mars => 2,
        Graha::Mercury => 3,
        Graha::Jupiter => 4,
        Graha::Venus => 5,
        Graha::Saturn => 6,
        Graha::Rahu => 7,
        Graha::Ketu => 8,
    }
}

/// Natural relationship (Naisargika Maitri) per BPHS Ch. 3: +1 Friend, 0 Neutral, -1 Enemy.
fn natural_relationship(planet: Graha, other: Graha) -> i8 {
    const MATRIX: [[i8; 9]; 9] = [
        [-1,  1,  1,  0,  1, -1, -1, -1, -1],
        [ 1,  0,  0,  1,  0,  0,  0,  0,  0],
        [ 1,  1, -1, -1,  1,  0,  0, -1, -1],
        [ 1, -1,  0, -1,  0,  1,  0, -1, -1],
        [ 1,  1,  1, -1, -1, -1,  0, -1, -1],
        [-1, -1,  0,  1,  0, -1,  1, -1, -1],
        [-1, -1, -1,  1,  0,  1, -1, -1, -1],
        [-1, -1, -1,  1,  0,  1,  1, -1, -1],
        [-1, -1, -1,  1,  0,  1,  1, -1, -1],
    ];
    MATRIX[graha_index(planet)][graha_index(other)]
}

struct EssentialDignityRule {
    exalt_sign: u8,
    exalt_max_deg: f64,
    debil_signs: &'static [u8],
    moola_sign: Option<u8>,
    moola_max_deg: f64,
    own_signs: &'static [u8],
}

fn essential_rule(graha: Graha) -> EssentialDignityRule {
    match graha {
        Graha::Sun => EssentialDignityRule { exalt_sign: 0, exalt_max_deg: 31.0, debil_signs: &[6], moola_sign: Some(4), moola_max_deg: 20.0, own_signs: &[4] },
        Graha::Moon => EssentialDignityRule { exalt_sign: 1, exalt_max_deg: 3.000_001, debil_signs: &[7], moola_sign: Some(1), moola_max_deg: 31.0, own_signs: &[3] },
        Graha::Mars => EssentialDignityRule { exalt_sign: 9, exalt_max_deg: 31.0, debil_signs: &[3], moola_sign: Some(0), moola_max_deg: 12.0, own_signs: &[0, 7] },
        Graha::Mercury => EssentialDignityRule { exalt_sign: 5, exalt_max_deg: 15.0, debil_signs: &[11], moola_sign: Some(5), moola_max_deg: 20.0, own_signs: &[2, 5] },
        Graha::Jupiter => EssentialDignityRule { exalt_sign: 3, exalt_max_deg: 31.0, debil_signs: &[9], moola_sign: Some(8), moola_max_deg: 10.0, own_signs: &[8, 11] },
        Graha::Venus => EssentialDignityRule { exalt_sign: 11, exalt_max_deg: 31.0, debil_signs: &[5], moola_sign: Some(6), moola_max_deg: 15.0, own_signs: &[1, 6] },
        Graha::Saturn => EssentialDignityRule { exalt_sign: 6, exalt_max_deg: 31.0, debil_signs: &[0], moola_sign: Some(10), moola_max_deg: 20.0, own_signs: &[9, 10] },
        Graha::Rahu => EssentialDignityRule { exalt_sign: 1, exalt_max_deg: 31.0, debil_signs: &[7], moola_sign: Some(2), moola_max_deg: 31.0, own_signs: &[10] },
        Graha::Ketu => EssentialDignityRule { exalt_sign: 8, exalt_max_deg: 31.0, debil_signs: &[1, 2], moola_sign: None, moola_max_deg: 0.0, own_signs: &[7] },
    }
}

/// Computes classical Vedic dignity (BPHS Ch. 3): Exalted, Moolatrikona, Own Sign,
/// Debilitated, or 5-fold Panchadha Maitri.
pub(crate) fn compute_vedic_dignity(
    graha: Graha,
    sign_index: u8,
    sign_degree: f64,
    planet_signs: &HashMap<Graha, u8>,
) -> String {
    let rule = essential_rule(graha);
    if sign_index == rule.exalt_sign && sign_degree < rule.exalt_max_deg {
        return "Exalted".to_string();
    }
    if rule.debil_signs.contains(&sign_index) {
        return "Debilitated".to_string();
    }
    if rule.moola_sign == Some(sign_index) && sign_degree < rule.moola_max_deg {
        return "Moolatrikona".to_string();
    }
    if rule.own_signs.contains(&sign_index) {
        return "Own Sign".to_string();
    }

    let disp = sign_lord(sign_index);
    let naisargika = natural_relationship(graha, disp);
    let disp_sign = planet_signs.get(&disp).copied().unwrap_or(sign_index);
    let house_dist = ((disp_sign as i16 - sign_index as i16 + 12) % 12 + 1) as u8;
    let tatkalika: i8 = if matches!(house_dist, 2 | 3 | 4 | 10 | 11 | 12) { 1 } else { -1 };

    match naisargika + tatkalika {
        2 => "Great Friend",
        1 => "Friend",
        0 => "Neutral",
        -1 => "Enemy",
        _ => "Great Enemy",
    }
    .to_string()
}

/// Classical Vedic combustion (Moudhya) evaluation per BPHS & Saravali Ch. 36.
pub(crate) fn compute_classical_vedic_combustion(
    graha: Graha,
    sid_lon: f64,
    sun_sid_lon: f64,
    is_retrograde: bool,
) -> String {
    if matches!(graha, Graha::Sun | Graha::Rahu | Graha::Ketu) {
        return "None".to_string();
    }
    let diff = (normalize_degrees(sid_lon) - normalize_degrees(sun_sid_lon)).abs();
    let sep = if diff > 180.0 { 360.0 - diff } else { diff };

    let max_orb = match graha {
        Graha::Moon => 12.0,
        Graha::Mars => if is_retrograde { 8.0 } else { 17.0 },
        Graha::Mercury => if is_retrograde { 12.0 } else { 14.0 },
        Graha::Jupiter => 11.0,
        Graha::Venus => if is_retrograde { 8.0 } else { 10.0 },
        Graha::Saturn => 16.0,
        _ => 0.0,
    };

    if sep <= max_orb {
        if matches!(graha, Graha::Venus | Graha::Saturn) {
            "Combust (Unimpaired per Saravali)".to_string()
        } else if sep <= 1.0 {
            "DeeplyCombust".to_string()
        } else {
            "Combust".to_string()
        }
    } else {
        "None".to_string()
    }
}

fn dignity_rank(label: &str) -> i8 {
    match label {
        "Exalted" => 5,
        "Moolatrikona" => 4,
        "Own Sign" => 3,
        "Great Friend" => 2,
        "Friend" => 1,
        "Neutral" => 0,
        "Enemy" => -1,
        "Great Enemy" => -2,
        "Debilitated" => -3,
        _ => 0,
    }
}

fn effective_degree(g: Graha, planet_lons: &HashMap<Graha, f64>) -> f64 {
    let deg = normalize_degrees(planet_lons.get(&g).copied().unwrap_or(0.0)) % 30.0;
    if matches!(g, Graha::Rahu | Graha::Ketu) {
        if deg < 1e-9 { 0.0 } else { 30.0 - deg }
    } else {
        deg
    }
}

/// Resolves effective Arudha lord of a Rasi (applying Scorpio Mars/Ketu and Aquarius Saturn/Rahu rules).
pub(crate) fn arudha_sign_lord(
    sign_index: u8,
    planet_signs: &HashMap<Graha, u8>,
    planet_lons: &HashMap<Graha, f64>,
) -> Graha {
    let rashi = sign_index % 12;
    let (lord_a, lord_b) = match rashi {
        7 => (Graha::Mars, Graha::Ketu),
        10 => (Graha::Saturn, Graha::Rahu),
        _ => return sign_lord(rashi),
    };

    let sign_a = planet_signs.get(&lord_a).copied().unwrap_or(rashi);
    let sign_b = planet_signs.get(&lord_b).copied().unwrap_or(rashi);
    if sign_a == rashi && sign_b != rashi { return lord_b; }
    if sign_b == rashi && sign_a != rashi { return lord_a; }

    let count_a = planet_signs.values().filter(|&&s| s == sign_a).count();
    let count_b = planet_signs.values().filter(|&&s| s == sign_b).count();
    if count_a != count_b {
        return if count_a > count_b { lord_a } else { lord_b };
    }

    let deg_a = normalize_degrees(planet_lons.get(&lord_a).copied().unwrap_or(0.0)) % 30.0;
    let deg_b = normalize_degrees(planet_lons.get(&lord_b).copied().unwrap_or(0.0)) % 30.0;
    let dig_a = dignity_rank(&compute_vedic_dignity(lord_a, sign_a, deg_a, planet_signs));
    let dig_b = dignity_rank(&compute_vedic_dignity(lord_b, sign_b, deg_b, planet_signs));
    if dig_a != dig_b {
        return if dig_a > dig_b { lord_a } else { lord_b };
    }

    if effective_degree(lord_a, planet_lons) >= effective_degree(lord_b, planet_lons) {
        lord_a
    } else {
        lord_b
    }
}
