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
    mercury_lon: f64,
    venus_lon: f64,
    mars_lon: f64,
    jupiter_lon: f64,
    saturn_lon: f64,
    cusps: &[f64; 12],
    is_day_chart: bool,
    is_male: bool,
) -> Vec<ArabicPart> {
    let mut parts = Vec::new();

    // 1. Part of Fortune (Lot of Fortune - Moon's Lot)
    let fortune_lon = if is_day_chart {
        (ascendant + moon_lon - sun_lon).rem_euclid(360.0)
    } else {
        (ascendant + sun_lon - moon_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Part of Fortune", fortune_lon, cusps));

    // 2. Part of Spirit (Lot of Daimon - Sun's Lot)
    let spirit_lon = if is_day_chart {
        (ascendant + sun_lon - moon_lon).rem_euclid(360.0)
    } else {
        (ascendant + moon_lon - sun_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Part of Spirit", spirit_lon, cusps));

    // 3. Lot of Eros (Venus's Lot - Love, Passion, Desire)
    // Paulus / Valens: Day: ASC + Venus - Spirit, Night: ASC + Spirit - Venus
    let eros_lon = if is_day_chart {
        (ascendant + venus_lon - spirit_lon).rem_euclid(360.0)
    } else {
        (ascendant + spirit_lon - venus_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Lot of Eros", eros_lon, cusps));

    // 4. Lot of Victory (Jupiter's Lot - Triumph, Success, Justice)
    // Paulus / Valens: Day: ASC + Jupiter - Spirit, Night: ASC + Spirit - Jupiter
    let victory_lon = if is_day_chart {
        (ascendant + jupiter_lon - spirit_lon).rem_euclid(360.0)
    } else {
        (ascendant + spirit_lon - jupiter_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Lot of Victory", victory_lon, cusps));

    // 5. Lot of Necessity (Mercury's Lot - Constraints, Fate, Obligations)
    // Paulus / Valens: Day: ASC + Fortune - Mercury, Night: ASC + Mercury - Fortune
    let necessity_lon = if is_day_chart {
        (ascendant + fortune_lon - mercury_lon).rem_euclid(360.0)
    } else {
        (ascendant + mercury_lon - fortune_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Lot of Necessity", necessity_lon, cusps));

    // 6. Lot of Courage (Mars's Lot - Boldness, Bravery, War)
    // Paulus / Valens: Day: ASC + Fortune - Mars, Night: ASC + Mars - Fortune
    let courage_lon = if is_day_chart {
        (ascendant + fortune_lon - mars_lon).rem_euclid(360.0)
    } else {
        (ascendant + mars_lon - fortune_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Lot of Courage", courage_lon, cusps));

    // 7. Lot of Nemesis (Saturn's Lot - Retribution, Hidden Pitfalls)
    // Paulus / Valens: Day: ASC + Fortune - Saturn, Night: ASC + Saturn - Fortune
    let nemesis_lon = if is_day_chart {
        (ascendant + fortune_lon - saturn_lon).rem_euclid(360.0)
    } else {
        (ascendant + saturn_lon - fortune_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Lot of Nemesis", nemesis_lon, cusps));

    // 8. Part of Substance / Wealth (2nd House cusp ruler derivative: ASC + Cusp2 - Ruler(Cusp2))
    let cusp2 = cusps[1];
    let cusp2_sign = (cusp2 / 30.0).floor() as usize % 12;
    let ruler_cusp2_planet = DOMICILE_RULERS[cusp2_sign];
    let ruler_cusp2_lon = match ruler_cusp2_planet {
        super::dignity::ClassicalPlanet::Sun => sun_lon,
        super::dignity::ClassicalPlanet::Moon => moon_lon,
        super::dignity::ClassicalPlanet::Mercury => mercury_lon,
        super::dignity::ClassicalPlanet::Venus => venus_lon,
        super::dignity::ClassicalPlanet::Mars => mars_lon,
        super::dignity::ClassicalPlanet::Jupiter => jupiter_lon,
        super::dignity::ClassicalPlanet::Saturn => saturn_lon,
    };
    let substance_lon = (ascendant + cusp2 - ruler_cusp2_lon).rem_euclid(360.0);
    parts.push(create_part("Part of Substance", substance_lon, cusps));

    // 9. Part of Marriage
    let marriage_lon = if is_male {
        (ascendant + venus_lon - saturn_lon).rem_euclid(360.0)
    } else {
        (ascendant + saturn_lon - venus_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Part of Marriage", marriage_lon, cusps));

    // 10. Part of Sickness (Day: ASC + Mars - Saturn, Night: ASC + Saturn - Mars)
    let sickness_lon = if is_day_chart {
        (ascendant + mars_lon - saturn_lon).rem_euclid(360.0)
    } else {
        (ascendant + saturn_lon - mars_lon).rem_euclid(360.0)
    };
    parts.push(create_part("Part of Sickness", sickness_lon, cusps));

    parts
}

/// Calculate authentic Hellenistic Lots for Whole Sign charts
/// (Paulus Alexandrinus Ch. 23 & Vettius Valens Anthologies II.19, II.38).
///
/// Unlike Quadrant Arabic Parts, Whole Sign Lot of Substance uses the Whole Sign
/// 2nd house reference point `(ascendant + 30.0) % 360.0` (measuring from the ruler
/// of the 2nd sign to the 2nd Whole Sign house offset from ASC) rather than a 0° dummy cusp.
pub fn calculate_hellenistic_lots(
    ascendant: f64,
    sun_lon: f64,
    moon_lon: f64,
    mercury_lon: f64,
    venus_lon: f64,
    mars_lon: f64,
    jupiter_lon: f64,
    saturn_lon: f64,
    cusps: &[f64; 12],
    is_day_chart: bool,
    is_male: bool,
) -> Vec<ArabicPart> {
    let mut lots = calculate_arabic_parts(
        ascendant,
        sun_lon,
        moon_lon,
        mercury_lon,
        venus_lon,
        mars_lon,
        jupiter_lon,
        saturn_lon,
        cusps,
        is_day_chart,
        is_male,
    );

    // Replace Part of Substance with authentic Whole Sign Lot of Substance / Abundance:
    // 2nd sign from Ascendant's sign:
    let asc_sign = ((ascendant.rem_euclid(360.0) / 30.0).floor() as usize) % 12;
    let second_sign = (asc_sign + 1) % 12;
    let ruler_second_planet = DOMICILE_RULERS[second_sign];
    let ruler_second_lon = match ruler_second_planet {
        super::dignity::ClassicalPlanet::Sun => sun_lon,
        super::dignity::ClassicalPlanet::Moon => moon_lon,
        super::dignity::ClassicalPlanet::Mercury => mercury_lon,
        super::dignity::ClassicalPlanet::Venus => venus_lon,
        super::dignity::ClassicalPlanet::Mars => mars_lon,
        super::dignity::ClassicalPlanet::Jupiter => jupiter_lon,
        super::dignity::ClassicalPlanet::Saturn => saturn_lon,
    };
    let second_house_point = (ascendant + 30.0).rem_euclid(360.0);
    let ws_substance_lon = (ascendant + second_house_point - ruler_second_lon).rem_euclid(360.0);

    if let Some(sub) = lots.iter_mut().find(|p| p.name == "Part of Substance") {
        *sub = create_part("Part of Substance", ws_substance_lon, cusps);
    }

    lots
}

fn create_part(name: &str, lon: f64, cusps: &[f64; 12]) -> ArabicPart {
    let norm_lon = lon.rem_euclid(360.0);
    let sign_index = (norm_lon / 30.0).floor() as u8 % 12;
    let sign_degree = (norm_lon - (sign_index as f64 * 30.0)).clamp(0.0, 29.999999999);
    let ruler = DOMICILE_RULERS[sign_index as usize].name().to_string();
    let house_number = super::determine_house(norm_lon, cusps);

    ArabicPart {
        name: name.to_string(),
        longitude: norm_lon,
        sign_index,
        sign_degree,
        house_number,
        ruler,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_of_substance_and_sickness() {
        let cusps = [
            0.0, 30.0, 60.0, 90.0, 120.0, 150.0,
            180.0, 210.0, 240.0, 270.0, 300.0, 330.0,
        ];
        // Cusp 2 is 30.0 (Taurus 0°), Domicile ruler of Taurus is Venus
        let ascendant = 10.0;
        let sun_lon = 50.0;
        let moon_lon = 100.0;
        let mercury_lon = 60.0;
        let venus_lon = 45.0; // Ruler of Cusp 2
        let mars_lon = 80.0;
        let jupiter_lon = 120.0;
        let saturn_lon = 200.0;

        let parts_day = calculate_arabic_parts(
            ascendant, sun_lon, moon_lon, mercury_lon, venus_lon, mars_lon, jupiter_lon, saturn_lon,
            &cusps, true, true,
        );
        let parts_night = calculate_arabic_parts(
            ascendant, sun_lon, moon_lon, mercury_lon, venus_lon, mars_lon, jupiter_lon, saturn_lon,
            &cusps, false, true,
        );

        let substance = parts_day.iter().find(|p| p.name == "Part of Substance").unwrap();
        // Substance: ASC (10) + Cusp2 (30) - Venus (45) = -5 -> 355
        assert!((substance.longitude - 355.0).abs() < 1e-6);

        let sickness_day = parts_day.iter().find(|p| p.name == "Part of Sickness").unwrap();
        let sickness_night = parts_night.iter().find(|p| p.name == "Part of Sickness").unwrap();
        // Day sickness: ASC (10) + Mars (80) - Saturn (200) = -110 -> 250
        // Night sickness: ASC (10) + Saturn (200) - Mars (80) = 130
        assert!((sickness_day.longitude - 250.0).abs() < 1e-6);
        assert!((sickness_night.longitude - 130.0).abs() < 1e-6);
        assert_ne!(sickness_day.longitude, sickness_night.longitude);
    }
}
