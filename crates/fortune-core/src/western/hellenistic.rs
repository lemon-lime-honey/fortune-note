//! Hellenistic Astrology calculation engine (Whole Sign House system).
//!
//! Based on Vettius Valens (Anthologies), Dorotheus of Sidon, and Paulus Alexandrinus.
//!
//! Excludes 17th-century Renaissance point scoring, Almuten calculations, and Detriment debility,
//! focusing on qualitative essential conditions (Domicile, Exaltation, Fall), Hermetic Lots,
//! sign-based geometry, and independent Midheaven (MC) action points.

use serde::{Deserialize, Serialize};

use super::aspects::{self, TraditionalAspect};
use super::dignity::{ClassicalPlanet, DOMICILE_RULERS, EXALTATIONS};
use super::fixed_stars::{self, FixedStarConjunction};
use super::parts::{self, ArabicPart};
use super::syzygy::{self, PrenatalSyzygy};

/// Qualitative condition of a classical planet in Hellenistic astrology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HellenisticPlanetCondition {
    pub name: String,
    pub longitude: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub house_number: u8,
    pub is_in_domicile: bool,
    pub is_in_exaltation: bool,
    pub is_in_fall: bool,
    pub is_ruler_of_asc: bool,
    pub condition_label: String, // "Domicile", "Exaltation", "Fall", or "Peregrine"
}

/// Independent angle points in Hellenistic Whole Sign framework.
///
/// In Whole Sign astrology, the 1st House starts at 0° of the Ascendant's sign,
/// and the 10th House starts at 0° of the 10th sign from the Ascendant.
/// Midheaven (MC) is an astronomical action point that frequently falls in the
/// 9th, 10th, or 11th sign, independent of the 10th House cusp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HellenisticAnglePoints {
    pub ascendant: f64,
    pub ascendant_sign: u8,
    pub ascendant_degree: f64,
    pub midheaven: f64,
    pub midheaven_sign: u8,
    pub midheaven_degree: f64,
    pub midheaven_house: u8,
    pub is_mc_in_10th: bool,
}

/// Complete Hellenistic astrology dataset for Whole Sign charts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HellenisticData {
    pub planet_conditions: Vec<HellenisticPlanetCondition>,
    pub lots: Vec<ArabicPart>,
    pub angles: HellenisticAnglePoints,
    pub aspects: Vec<TraditionalAspect>,
    pub fixed_star_conjunctions: Vec<FixedStarConjunction>,
    pub prenatal_syzygy: Option<PrenatalSyzygy>,
}

/// Calculate Hellenistic chart dataset for a Whole Sign chart.
pub fn calculate_hellenistic_data(
    planet_tuples: &[(&str, f64, f64)],      // (name, lon, speed)
    planet_pos_tuples: &[(&str, f64)],       // (name, lon)
    ascendant: f64,
    midheaven: f64,
    cusps: &[f64; 12],
    is_day_chart: bool,
    is_male: bool,
    jd_ut1: f64,
    jd_tt: f64,
    observer_lat: f64,
    obliquity_deg: f64,
    fixed_star_max_orb: f64,
) -> HellenisticData {
    let asc_sign = ((ascendant / 30.0).floor() as u8) % 12;
    let asc_deg = ascendant - (asc_sign as f64 * 30.0);

    let mc_sign = ((midheaven / 30.0).floor() as u8) % 12;
    let mc_deg = midheaven - (mc_sign as f64 * 30.0);
    let mc_house = super::determine_house(midheaven, cusps);
    let is_mc_in_10th = mc_house == 10;

    let angles = HellenisticAnglePoints {
        ascendant,
        ascendant_sign: asc_sign,
        ascendant_degree: asc_deg,
        midheaven,
        midheaven_sign: mc_sign,
        midheaven_degree: mc_deg,
        midheaven_house: mc_house,
        is_mc_in_10th,
    };

    // 1. Qualitative planet conditions (Domicile, Exaltation, Fall only - no Detriment or Lilly score)
    let mut planet_conditions = Vec::with_capacity(7);
    for &(name, lon, _) in planet_tuples {
        if let Some(cp) = ClassicalPlanet::from_name(name) {
            let sign_index = ((lon / 30.0).floor() as u8) % 12;
            let sign_degree = lon - (sign_index as f64 * 30.0);
            let opp_sign = (sign_index + 6) % 12;
            let house_number = super::determine_house(lon, cusps);

            let is_in_domicile = DOMICILE_RULERS[sign_index as usize] == cp;
            let is_in_exaltation = EXALTATIONS
                .iter()
                .find(|(s, _, _)| *s == sign_index)
                .map(|(_, p, _)| *p)
                == Some(cp);
            let is_in_fall = EXALTATIONS
                .iter()
                .find(|(s, _, _)| *s == opp_sign)
                .map(|(_, p, _)| *p)
                == Some(cp);
            let is_ruler_of_asc = DOMICILE_RULERS[asc_sign as usize] == cp;

            let condition_label = if is_in_domicile {
                "Domicile".to_string()
            } else if is_in_exaltation {
                "Exaltation".to_string()
            } else if is_in_fall {
                "Fall".to_string()
            } else {
                "Peregrine".to_string()
            };

            planet_conditions.push(HellenisticPlanetCondition {
                name: name.to_string(),
                longitude: lon,
                sign_index,
                sign_degree,
                house_number,
                is_in_domicile,
                is_in_exaltation,
                is_in_fall,
                is_ruler_of_asc,
                condition_label,
            });
        }
    }

    // 2. Hermetic Lots (Paulus Alexandrinus 7 Hermetic Lots)
    let sun_opt = planet_pos_tuples.iter().find(|(n, _)| *n == "Sun");
    let moon_opt = planet_pos_tuples.iter().find(|(n, _)| *n == "Moon");
    let sun_lon = sun_opt.map_or(0.0, |(_, l)| *l);
    let moon_lon = moon_opt.map_or(0.0, |(_, l)| *l);

    let mercury_lon = planet_pos_tuples.iter().find(|(n, _)| *n == "Mercury").map_or(0.0, |(_, l)| *l);
    let venus_lon = planet_pos_tuples.iter().find(|(n, _)| *n == "Venus").map_or(0.0, |(_, l)| *l);
    let mars_lon = planet_pos_tuples.iter().find(|(n, _)| *n == "Mars").map_or(0.0, |(_, l)| *l);
    let jupiter_lon = planet_pos_tuples.iter().find(|(n, _)| *n == "Jupiter").map_or(0.0, |(_, l)| *l);
    let saturn_lon = planet_pos_tuples.iter().find(|(n, _)| *n == "Saturn").map_or(0.0, |(_, l)| *l);

    let all_lots = parts::calculate_arabic_parts(
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

    // Filter to the 7 authentic Hermetic Lots + Part of Marriage / Substance
    let lots = all_lots;

    // 3. Sign-based Aspects with Degree Contacts
    let aspects = aspects::calculate_traditional_aspects(planet_tuples);

    // 4. Fixed Star Conjunctions
    let stars = fixed_stars::calculate_traditional_fixed_stars(jd_tt, observer_lat, obliquity_deg);
    let dsc = (ascendant + 180.0).rem_euclid(360.0);
    let ic = (midheaven + 180.0).rem_euclid(360.0);
    let mut check_bodies = planet_pos_tuples.to_vec();
    check_bodies.push(("Ascendant", ascendant));
    check_bodies.push(("Midheaven", midheaven));
    check_bodies.push(("Descendant", dsc));
    check_bodies.push(("Immum Coeli", ic));
    let fixed_star_conjunctions = fixed_stars::detect_fixed_star_conjunctions(&stars, &check_bodies, fixed_star_max_orb);

    // 5. Prenatal Syzygy (SAN)
    let prenatal_syzygy = syzygy::calculate_prenatal_syzygy_with_coords(jd_ut1, sun_lon, moon_lon).ok();

    HellenisticData {
        planet_conditions,
        lots,
        angles,
        aspects,
        fixed_star_conjunctions,
        prenatal_syzygy,
    }
}
