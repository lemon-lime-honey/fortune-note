//! Arabic Parts (Lots) in Traditional Western Astrology.
//!
//! Based on Christian Astrology (1647) and classical Arabic/Hellenistic sources.

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
    #[serde(default)]
    pub formula: String,
}

fn planet_short_code(planet: super::dignity::ClassicalPlanet) -> &'static str {
    match planet {
        super::dignity::ClassicalPlanet::Sun => "SUN",
        super::dignity::ClassicalPlanet::Moon => "MON",
        super::dignity::ClassicalPlanet::Mercury => "MER",
        super::dignity::ClassicalPlanet::Venus => "VEN",
        super::dignity::ClassicalPlanet::Mars => "MAR",
        super::dignity::ClassicalPlanet::Jupiter => "JUP",
        super::dignity::ClassicalPlanet::Saturn => "SAT",
    }
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
    let (fortune_lon, fortune_formula) = if is_day_chart {
        ((ascendant + moon_lon - sun_lon).rem_euclid(360.0), "ASC + MON - SUN")
    } else {
        ((ascendant + sun_lon - moon_lon).rem_euclid(360.0), "ASC + SUN - MON")
    };
    parts.push(create_part("Part of Fortune", fortune_lon, cusps, fortune_formula));

    // 2. Part of Spirit (Lot of Daimon - Sun's Lot)
    let (spirit_lon, spirit_formula) = if is_day_chart {
        ((ascendant + sun_lon - moon_lon).rem_euclid(360.0), "ASC + SUN - MON")
    } else {
        ((ascendant + moon_lon - sun_lon).rem_euclid(360.0), "ASC + MON - SUN")
    };
    parts.push(create_part("Part of Spirit", spirit_lon, cusps, spirit_formula));

    // 3. Lot of Eros (Venus's Lot - Love, Passion, Desire)
    // Paulus / Valens: Day: ASC + Venus - Spirit, Night: ASC + Spirit - Venus
    let (eros_lon, eros_formula) = if is_day_chart {
        ((ascendant + venus_lon - spirit_lon).rem_euclid(360.0), "ASC + VEN - POS")
    } else {
        ((ascendant + spirit_lon - venus_lon).rem_euclid(360.0), "ASC + POS - VEN")
    };
    parts.push(create_part("Lot of Eros", eros_lon, cusps, eros_formula));

    // 4. Lot of Victory (Jupiter's Lot - Triumph, Success, Justice)
    // Paulus / Valens: Day: ASC + Jupiter - Spirit, Night: ASC + Spirit - Jupiter
    let (victory_lon, victory_formula) = if is_day_chart {
        ((ascendant + jupiter_lon - spirit_lon).rem_euclid(360.0), "ASC + JUP - POS")
    } else {
        ((ascendant + spirit_lon - jupiter_lon).rem_euclid(360.0), "ASC + POS - JUP")
    };
    parts.push(create_part("Lot of Victory", victory_lon, cusps, victory_formula));

    // 5. Lot of Necessity (Mercury's Lot - Constraints, Fate, Obligations)
    // Paulus / Valens: Day: ASC + Fortune - Mercury, Night: ASC + Mercury - Fortune
    let (necessity_lon, necessity_formula) = if is_day_chart {
        ((ascendant + fortune_lon - mercury_lon).rem_euclid(360.0), "ASC + POF - MER")
    } else {
        ((ascendant + mercury_lon - fortune_lon).rem_euclid(360.0), "ASC + MER - POF")
    };
    parts.push(create_part("Lot of Necessity", necessity_lon, cusps, necessity_formula));

    // 6. Lot of Courage (Mars's Lot - Boldness, Bravery, War)
    // Paulus / Valens: Day: ASC + Fortune - Mars, Night: ASC + Mars - Fortune
    let (courage_lon, courage_formula) = if is_day_chart {
        ((ascendant + fortune_lon - mars_lon).rem_euclid(360.0), "ASC + POF - MAR")
    } else {
        ((ascendant + mars_lon - fortune_lon).rem_euclid(360.0), "ASC + MAR - POF")
    };
    parts.push(create_part("Lot of Courage", courage_lon, cusps, courage_formula));

    // 7. Lot of Nemesis (Saturn's Lot - Retribution, Hidden Pitfalls)
    // Paulus / Valens: Day: ASC + Fortune - Saturn, Night: ASC + Saturn - Fortune
    let (nemesis_lon, nemesis_formula) = if is_day_chart {
        ((ascendant + fortune_lon - saturn_lon).rem_euclid(360.0), "ASC + POF - SAT")
    } else {
        ((ascendant + saturn_lon - fortune_lon).rem_euclid(360.0), "ASC + SAT - POF")
    };
    parts.push(create_part("Lot of Nemesis", nemesis_lon, cusps, nemesis_formula));

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
    let substance_formula = format!("ASC + C2 - {}", planet_short_code(ruler_cusp2_planet));
    parts.push(create_part("Part of Substance", substance_lon, cusps, &substance_formula));

    // 9. Part of Marriage
    let (marriage_lon, marriage_formula) = if is_male {
        ((ascendant + venus_lon - saturn_lon).rem_euclid(360.0), "ASC + VEN - SAT")
    } else {
        ((ascendant + saturn_lon - venus_lon).rem_euclid(360.0), "ASC + SAT - VEN")
    };
    parts.push(create_part("Part of Marriage", marriage_lon, cusps, marriage_formula));

    // 10. Part of Sickness (Day: ASC + Mars - Saturn, Night: ASC + Saturn - Mars)
    let (sickness_lon, sickness_formula) = if is_day_chart {
        ((ascendant + mars_lon - saturn_lon).rem_euclid(360.0), "ASC + MAR - SAT")
    } else {
        ((ascendant + saturn_lon - mars_lon).rem_euclid(360.0), "ASC + SAT - MAR")
    };
    parts.push(create_part("Part of Sickness", sickness_lon, cusps, sickness_formula));

    parts
}

/// Calculate authentic Hellenistic Lots for Whole Sign charts according to Vettius Valens
/// (Anthologies Book II Ch. 18–38 & Book IV).
///
/// Implements the complete Valens Lot suite:
/// 1. Lot of Fortune (`ASC + Moon - Sun` Day / `ASC + Sun - Moon` Night)
/// 2. Lot of Spirit / Daimon (`ASC + Sun - Moon` Day / `ASC + Moon - Sun` Night)
/// 3. Lot of Eros (`ASC + Spirit - Fortune` Day / `ASC + Fortune - Spirit` Night)
/// 4. Lot of Necessity (`ASC + Fortune - Spirit` Day / `ASC + Spirit - Fortune` Night)
/// 5. Lot of Courage / Deceit (`ASC + Mars - Sun` Day / `ASC + Sun - Mars` Night)
/// 6. Lot of Victory (11th Whole Sign place from Lot of Fortune: `Fortune + 300°`)
/// 7. Lot of Nemesis (`ASC + Fortune - Saturn` Day / `ASC + Saturn - Fortune` Night)
/// 8. Part of Substance (`ASC + 2H Whole Sign Cusp - Ruler of 2nd Sign`, unreversed)
/// 9. Lot of Marriage (Men) (`ASC + Venus - Sun`, unreversed per Valens II.38)
/// 10. Lot of Marriage (Women) (`ASC + Mars - Moon`, unreversed per Valens II.38)
/// 11. Lot of Marriage (General) (`ASC + Venus - Jupiter` Day / `ASC + Jupiter - Venus` Night per Valens II.37)
/// 12. Lot of Basis (`ASC + min_arc(Fortune, Spirit)` where short arc < 180° is always projected from ASC)
/// 13. Lot of Exaltation (`ASC + 19° Aries - Sun` Day / `ASC + 3° Taurus - Moon` Night)
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
    _is_male: bool,
) -> Vec<ArabicPart> {
    let mut lots = Vec::with_capacity(13);

    // 1. Lot of Fortune
    let (fortune_lon, fortune_formula) = if is_day_chart {
        ((ascendant + moon_lon - sun_lon).rem_euclid(360.0), "ASC + MON - SUN")
    } else {
        ((ascendant + sun_lon - moon_lon).rem_euclid(360.0), "ASC + SUN - MON")
    };
    lots.push(create_part("Part of Fortune", fortune_lon, cusps, fortune_formula));

    // 2. Lot of Spirit (Daimon)
    // Valens syzygy exception: when Sun and Moon are in the same sign (or exact opposition) causing
    // Fortune and Spirit to fall in the same sign, Spirit is still recorded at its mathematical degree
    // while Valens notes the next sign can act as the secondary action place.
    let (spirit_lon, spirit_formula) = if is_day_chart {
        ((ascendant + sun_lon - moon_lon).rem_euclid(360.0), "ASC + SUN - MON")
    } else {
        ((ascendant + moon_lon - sun_lon).rem_euclid(360.0), "ASC + MON - SUN")
    };
    lots.push(create_part("Part of Spirit", spirit_lon, cusps, spirit_formula));

    // 3. Lot of Eros (Valens Book II & IV: Day: ASC + Spirit - Fortune, Night: ASC + Fortune - Spirit)
    let (eros_lon, eros_formula) = if is_day_chart {
        ((ascendant + spirit_lon - fortune_lon).rem_euclid(360.0), "ASC + POS - POF")
    } else {
        ((ascendant + fortune_lon - spirit_lon).rem_euclid(360.0), "ASC + POF - POS")
    };
    lots.push(create_part("Lot of Eros", eros_lon, cusps, eros_formula));

    // 4. Lot of Necessity (Valens Book II & IV: Day: ASC + Fortune - Spirit, Night: ASC + Spirit - Fortune)
    let (necessity_lon, necessity_formula) = if is_day_chart {
        ((ascendant + fortune_lon - spirit_lon).rem_euclid(360.0), "ASC + POF - POS")
    } else {
        ((ascendant + spirit_lon - fortune_lon).rem_euclid(360.0), "ASC + POS - POF")
    };
    lots.push(create_part("Lot of Necessity", necessity_lon, cusps, necessity_formula));

    // 5. Lot of Courage / Deceit (Valens: Day: ASC + Mars - Sun, Night: ASC + Sun - Mars)
    let (courage_lon, courage_formula) = if is_day_chart {
        ((ascendant + mars_lon - sun_lon).rem_euclid(360.0), "ASC + MAR - SUN")
    } else {
        ((ascendant + sun_lon - mars_lon).rem_euclid(360.0), "ASC + SUN - MAR")
    };
    lots.push(create_part("Lot of Courage", courage_lon, cusps, courage_formula));

    // 6. Lot of Victory (Valens: 11th Whole Sign place of Accomplishment from Lot of Fortune)
    let victory_lon = (fortune_lon + 300.0).rem_euclid(360.0);
    lots.push(create_part("Lot of Victory", victory_lon, cusps, "POF + 300° (11th from Fortune)"));

    // 7. Lot of Nemesis (Day: ASC + Fortune - Saturn, Night: ASC + Saturn - Fortune)
    let (nemesis_lon, nemesis_formula) = if is_day_chart {
        ((ascendant + fortune_lon - saturn_lon).rem_euclid(360.0), "ASC + POF - SAT")
    } else {
        ((ascendant + saturn_lon - fortune_lon).rem_euclid(360.0), "ASC + SAT - POF")
    };
    lots.push(create_part("Lot of Nemesis", nemesis_lon, cusps, nemesis_formula));

    // 8. Part of Substance (Valens Whole Sign 2nd House Lot)
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
    let ws_formula = format!("ASC + H2 - {}", planet_short_code(ruler_second_planet));
    lots.push(create_part("Part of Substance", ws_substance_lon, cusps, &ws_formula));

    // 9. Lot of Marriage (Men) — Valens II.38: ASC + Venus - Sun (Day & Night)
    let marriage_men_lon = (ascendant + venus_lon - sun_lon).rem_euclid(360.0);
    lots.push(create_part("Lot of Marriage (Men)", marriage_men_lon, cusps, "ASC + VEN - SUN"));

    // 10. Lot of Marriage (Women) — Valens II.38: ASC + Mars - Moon (Day & Night)
    let marriage_women_lon = (ascendant + mars_lon - moon_lon).rem_euclid(360.0);
    lots.push(create_part("Lot of Marriage (Women)", marriage_women_lon, cusps, "ASC + MAR - MON"));

    // 11. Lot of Marriage (General) — Valens II.37: Day: ASC + Venus - Jupiter, Night: ASC + Jupiter - Venus
    let (marriage_gen_lon, marriage_gen_formula) = if is_day_chart {
        ((ascendant + venus_lon - jupiter_lon).rem_euclid(360.0), "ASC + VEN - JUP")
    } else {
        ((ascendant + jupiter_lon - venus_lon).rem_euclid(360.0), "ASC + JUP - VEN")
    };
    lots.push(create_part("Part of Marriage", marriage_gen_lon, cusps, marriage_gen_formula));

    // 12. Lot of Basis (Valens: always projects the short arc < 180° between Fortune and Spirit from ASC)
    let arc_f_to_s = (spirit_lon - fortune_lon).rem_euclid(360.0);
    let (basis_lon, basis_formula) = if arc_f_to_s <= 180.0 {
        ((ascendant + arc_f_to_s).rem_euclid(360.0), "ASC + POS - POF (<180°)")
    } else {
        let arc_s_to_f = (fortune_lon - spirit_lon).rem_euclid(360.0);
        ((ascendant + arc_s_to_f).rem_euclid(360.0), "ASC + POF - POS (<180°)")
    };
    lots.push(create_part("Lot of Basis", basis_lon, cusps, basis_formula));

    // 13. Lot of Exaltation (Valens: Day: ASC + 19° Aries - Sun, Night: ASC + 3° Taurus - Moon)
    let (exalt_lon, exalt_formula) = if is_day_chart {
        ((ascendant + 19.0 - sun_lon).rem_euclid(360.0), "ASC + 19°Ari - SUN")
    } else {
        ((ascendant + 33.0 - moon_lon).rem_euclid(360.0), "ASC + 3°Tau - MON")
    };
    lots.push(create_part("Lot of Exaltation", exalt_lon, cusps, exalt_formula));

    lots
}

fn create_part(name: &str, lon: f64, cusps: &[f64; 12], formula: &str) -> ArabicPart {
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
        formula: formula.to_string(),
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
        assert_eq!(substance.formula, "ASC + C2 - VEN");

        let sickness_day = parts_day.iter().find(|p| p.name == "Part of Sickness").unwrap();
        let sickness_night = parts_night.iter().find(|p| p.name == "Part of Sickness").unwrap();
        // Day sickness: ASC (10) + Mars (80) - Saturn (200) = -110 -> 250
        // Night sickness: ASC (10) + Saturn (200) - Mars (80) = 130
        assert!((sickness_day.longitude - 250.0).abs() < 1e-6);
        assert_eq!(sickness_day.formula, "ASC + MAR - SAT");
        assert!((sickness_night.longitude - 130.0).abs() < 1e-6);
        assert_eq!(sickness_night.formula, "ASC + SAT - MAR");
        assert_ne!(sickness_day.longitude, sickness_night.longitude);
    }

    #[test]
    fn test_valens_13_hellenistic_lots() {
        let cusps = [
            0.0, 30.0, 60.0, 90.0, 120.0, 150.0,
            180.0, 210.0, 240.0, 270.0, 300.0, 330.0,
        ];
        let ascendant = 15.0; // Aries 15°
        let sun_lon = 45.0;   // Taurus 15°
        let moon_lon = 105.0; // Cancer 15°
        let mercury_lon = 50.0;
        let venus_lon = 30.0;
        let mars_lon = 90.0;
        let jupiter_lon = 120.0;
        let saturn_lon = 210.0;

        let lots_day = calculate_hellenistic_lots(
            ascendant, sun_lon, moon_lon, mercury_lon, venus_lon, mars_lon, jupiter_lon, saturn_lon,
            &cusps, true, true,
        );
        let lots_night = calculate_hellenistic_lots(
            ascendant, sun_lon, moon_lon, mercury_lon, venus_lon, mars_lon, jupiter_lon, saturn_lon,
            &cusps, false, true,
        );

        assert_eq!(lots_day.len(), 13);
        assert_eq!(lots_night.len(), 13);

        // Fortune Day: 15 + 105 - 45 = 75.0, Spirit Day: 15 + 45 - 105 = 315.0
        let pof_day = lots_day.iter().find(|p| p.name == "Part of Fortune").unwrap();
        let pos_day = lots_day.iter().find(|p| p.name == "Part of Spirit").unwrap();
        assert!((pof_day.longitude - 75.0).abs() < 1e-6);
        assert!((pos_day.longitude - 315.0).abs() < 1e-6);

        // Lot of Eros (Valens Day: ASC + Spirit - Fortune = 15 + 315 - 75 = 255.0)
        let eros_day = lots_day.iter().find(|p| p.name == "Lot of Eros").unwrap();
        assert!((eros_day.longitude - 255.0).abs() < 1e-6);

        // Lot of Victory (Valens: 11th sign from Fortune = POF + 300° = 75 + 300 = 15.0)
        let victory_day = lots_day.iter().find(|p| p.name == "Lot of Victory").unwrap();
        assert!((victory_day.longitude - 15.0).abs() < 1e-6);

        // Lot of Basis (shortest arc between Fortune 75° and Spirit 315° is 120° from Spirit to Fortune -> ASC + 120° = 135.0)
        let basis_day = lots_day.iter().find(|p| p.name == "Lot of Basis").unwrap();
        assert!((basis_day.longitude - 135.0).abs() < 1e-6);

        // Lot of Exaltation (Day: ASC + 19°Ari - Sun = 15 + 19 - 45 = 349.0; Night: ASC + 3°Tau (33°) - Moon = 15 + 33 - 105 = 303.0)
        let exalt_day = lots_day.iter().find(|p| p.name == "Lot of Exaltation").unwrap();
        let exalt_night = lots_night.iter().find(|p| p.name == "Lot of Exaltation").unwrap();
        assert!((exalt_day.longitude - 349.0).abs() < 1e-6);
        assert!((exalt_night.longitude - 303.0).abs() < 1e-6);
    }
}
