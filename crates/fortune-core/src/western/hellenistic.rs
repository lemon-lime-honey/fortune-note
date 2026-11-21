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
    pub condition_label: String,
    #[serde(default)]
    pub bound_ruler: String,
    #[serde(default)]
    pub triplicity_day_ruler: String,
    #[serde(default)]
    pub triplicity_night_ruler: String,
    #[serde(default)]
    pub triplicity_participating_ruler: String,
    #[serde(default)]
    pub is_in_triplicity: bool,
}

/// Independent angle points in Hellenistic Whole Sign framework.
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

/// Input parameters for Hellenistic Whole Sign calculation.
pub struct HellenisticInput<'a> {
    pub planet_tuples: &'a [(&'a str, f64, f64)],
    pub planet_pos_tuples: &'a [(&'a str, f64)],
    pub ascendant: f64,
    pub midheaven: f64,
    pub cusps: &'a [f64; 12],
    pub is_day_chart: bool,
    pub is_male: bool,
    pub jd_ut1: f64,
    pub jd_tt: f64,
    pub observer_lat: f64,
    pub obliquity_deg: f64,
    pub fixed_star_max_orb: f64,
}

fn build_angle_points(ascendant: f64, midheaven: f64, cusps: &[f64; 12]) -> HellenisticAnglePoints {
    let asc_sign = ((ascendant / 30.0).floor() as u8) % 12;
    let asc_deg = ascendant - (asc_sign as f64 * 30.0);
    let mc_sign = ((midheaven / 30.0).floor() as u8) % 12;
    let mc_deg = midheaven - (mc_sign as f64 * 30.0);
    let mc_house = super::determine_house(midheaven, cusps);

    HellenisticAnglePoints {
        ascendant,
        ascendant_sign: asc_sign,
        ascendant_degree: asc_deg,
        midheaven,
        midheaven_sign: mc_sign,
        midheaven_degree: mc_deg,
        midheaven_house: mc_house,
        is_mc_in_10th: mc_house == 10,
    }
}

fn build_planet_conditions(
    planet_tuples: &[(&str, f64, f64)],
    cusps: &[f64; 12],
    asc_sign: u8,
) -> Vec<HellenisticPlanetCondition> {
    let mut conditions = Vec::with_capacity(7);
    for &(name, lon, _) in planet_tuples {
        let Some(cp) = ClassicalPlanet::from_name(name) else {
            continue;
        };
        let sign_index = ((lon / 30.0).floor() as u8) % 12;
        let sign_degree = lon - (sign_index as f64 * 30.0);
        let opp_sign = (sign_index + 6) % 12;

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

        let condition_label = if is_in_domicile {
            "Domicile"
        } else if is_in_exaltation {
            "Exaltation"
        } else if is_in_fall {
            "Fall"
        } else {
            "Peregrine"
        }
        .to_string();

        let bound_ruler = super::dignity::egyptian_term(sign_index, sign_degree).name().to_string();
        let (t_day, t_night, t_part) = super::dignity::dorothean_triplicity_rulers(sign_index);

        conditions.push(HellenisticPlanetCondition {
            name: name.to_string(),
            longitude: lon,
            sign_index,
            sign_degree,
            house_number: super::determine_house(lon, cusps),
            is_in_domicile,
            is_in_exaltation,
            is_in_fall,
            is_ruler_of_asc: DOMICILE_RULERS[asc_sign as usize] == cp,
            condition_label,
            bound_ruler,
            triplicity_day_ruler: t_day.name().to_string(),
            triplicity_night_ruler: t_night.name().to_string(),
            triplicity_participating_ruler: t_part.name().to_string(),
            is_in_triplicity: cp == t_day || cp == t_night || cp == t_part,
        });
    }
    conditions
}

fn find_lon(tuples: &[(&str, f64)], name: &str) -> f64 {
    tuples.iter().find(|(n, _)| *n == name).map_or(0.0, |(_, l)| *l)
}

/// Calculate Hellenistic chart dataset for a Whole Sign chart.
pub fn calculate_hellenistic_data(input: &HellenisticInput<'_>) -> HellenisticData {
    let angles = build_angle_points(input.ascendant, input.midheaven, input.cusps);
    let planet_conditions = build_planet_conditions(input.planet_tuples, input.cusps, angles.ascendant_sign);

    let sun_lon = find_lon(input.planet_pos_tuples, "Sun");
    let moon_lon = find_lon(input.planet_pos_tuples, "Moon");
    let lots = parts::calculate_hellenistic_lots(
        input.ascendant,
        sun_lon,
        moon_lon,
        find_lon(input.planet_pos_tuples, "Mercury"),
        find_lon(input.planet_pos_tuples, "Venus"),
        find_lon(input.planet_pos_tuples, "Mars"),
        find_lon(input.planet_pos_tuples, "Jupiter"),
        find_lon(input.planet_pos_tuples, "Saturn"),
        input.cusps,
        input.is_day_chart,
        input.is_male,
    );

    let aspects = aspects::calculate_traditional_aspects(input.planet_tuples);

    let stars = fixed_stars::calculate_traditional_fixed_stars(
        input.jd_tt,
        input.observer_lat,
        input.obliquity_deg,
    );
    let dsc = (input.ascendant + 180.0).rem_euclid(360.0);
    let ic = (input.midheaven + 180.0).rem_euclid(360.0);
    let mut check_bodies = input.planet_pos_tuples.to_vec();
    check_bodies.push(("Ascendant", input.ascendant));
    check_bodies.push(("Midheaven", input.midheaven));
    check_bodies.push(("Descendant", dsc));
    check_bodies.push(("Imum Coeli", ic));
    let fixed_star_conjunctions =
        fixed_stars::detect_fixed_star_conjunctions(&stars, &check_bodies, input.fixed_star_max_orb);

    let prenatal_syzygy = syzygy::calculate_prenatal_syzygy_with_coords(input.jd_ut1, sun_lon, moon_lon).ok();

    HellenisticData {
        planet_conditions,
        lots,
        angles,
        aspects,
        fixed_star_conjunctions,
        prenatal_syzygy,
    }
}
