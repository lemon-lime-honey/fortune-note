//! Arabic Parts (Lots) in Traditional Western Astrology.
//!
//! Based on Christian Astrology (1647) and classical Arabic sources.

use serde::{Deserialize, Serialize};
use super::dignity::DOMICILE_RULERS;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArabicPart {
    pub name: String,
    pub longitude: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub house_number: u8,
    pub ruler: String,
}

pub fn calculate_arabic_parts(
    ascendant: f64,
    sun_lon: f64,
    moon_lon: f64,
    venus_lon: f64,
    mars_lon: f64,
    saturn_lon: f64,
    cusps: &[f64; 12],
    is_day_chart: bool,
    is_male: bool,
) -> Vec<ArabicPart> {
    let mut parts = Vec::new();

    // 1. Part of Fortune (Lot of Fortune)
    let fortune_lon = if is_day_chart {
        (ascendant + moon_lon - sun_lon).rem_euclid(360.0)
    } else {
        (ascendant + sun_lon - moon_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Part of Fortune (행운의 점)", fortune_lon, cusps));

    // 2. Part of Spirit (Lot of Daimon)
    let spirit_lon = if is_day_chart {
        (ascendant + sun_lon - moon_lon).rem_euclid(360.0)
    } else {
        (ascendant + moon_lon - sun_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Part of Spirit (정신의 점)", spirit_lon, cusps));

    // 3. Part of Substance / Wealth (2nd House cusp - 2nd House ruler)
    let cusp2 = cusps[1];
    let cusp2_sign = (cusp2 / 30.0).floor() as usize % 12;
    let ruler_cusp2_planet = DOMICILE_RULERS[cusp2_sign];
    // Use planetary position or default to cusp2
    let substance_lon = (ascendant + cusp2 - (ruler_cusp2_planet as usize as f64 * 30.0 + 15.0)).rem_euclid(360.0);
    parts.push(create_part("Part of Substance (재물의 점)", substance_lon, cusps));

    // 4. Part of Marriage
    let marriage_lon = if is_male {
        (ascendant + venus_lon - saturn_lon).rem_euclid(360.0)
    } else {
        (ascendant + saturn_lon - venus_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Part of Marriage (결혼의 점)", marriage_lon, cusps));

    // 5. Part of Sickness / Chronic Disease
    let sickness_lon = (ascendant + mars_lon - saturn_lon).rem_euclid(360.0);
    parts.push(create_part("Part of Sickness (질병의 점)", sickness_lon, cusps));

    parts
}

fn create_part(name: &str, lon: f64, cusps: &[f64; 12]) -> ArabicPart {
    let sign_index = (lon / 30.0).floor() as u8 % 12;
    let sign_degree = lon - (sign_index as f64 * 30.0);
    let ruler = DOMICILE_RULERS[sign_index as usize].name().to_string();

    // Determine house
    let mut house_number = 1;
    for h in 0..12 {
        let curr = cusps[h];
        let next = cusps[(h + 1) % 12];
        let inside = if curr <= next {
            lon >= curr && lon < next
        } else {
            lon >= curr || lon < next
        };
        if inside {
            house_number = (h + 1) as u8;
            break;
        }
    }

    ArabicPart {
        name: name.to_string(),
        longitude: lon,
        sign_index,
        sign_degree,
        house_number,
        ruler,
    }
}
