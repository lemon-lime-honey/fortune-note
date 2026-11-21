//! William Lilly's 5-Fold Essential Dignities and Accidental Dignity Scoring (CA p. 101-115).

mod tables;

use serde::{Deserialize, Serialize};
use super::fixed_stars::FixedStarConjunction;

pub use tables::{
    chaldean_face, dorothean_triplicity_rulers, egyptian_term, evaluate_essential_dignities,
    ptolemaic_term, triplicity_rulers, ClassicalPlanet, EssentialDignities, DOMICILE_RULERS,
    EXALTATIONS,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetDignityReport {
    pub planet: ClassicalPlanet,
    pub longitude: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub house_number: u8,
    pub is_in_domicile: bool,
    pub is_in_exaltation: bool,
    pub is_in_triplicity: bool,
    pub is_in_term: bool,
    pub is_in_face: bool,
    pub is_in_detriment: bool,
    pub is_in_fall: bool,
    pub is_peregrine: bool,
    pub essential_score: i32,
    pub accidental_score: i32,
    pub total_lilly_score: i32,
    pub accidental_factors: Vec<String>,
}

/// Bundled context for William Lilly essential and accidental dignity evaluation.
pub struct PlanetDignityInput<'a> {
    pub planet: ClassicalPlanet,
    pub longitude: f64,
    pub speed: f64,
    pub house_number: u8,
    pub sun_longitude: f64,
    pub is_day_chart: bool,
    pub north_node_lon: f64,
    pub south_node_lon: f64,
    pub other_planets: &'a [(&'a str, f64)],
    pub fixed_star_conjunctions: &'a [FixedStarConjunction],
}

struct ScoreAcc {
    score: i32,
    factors: Vec<String>,
}

impl ScoreAcc {
    fn new() -> Self {
        Self { score: 0, factors: Vec::new() }
    }

    fn add(&mut self, pts: i32, label: impl Into<String>) {
        self.score += pts;
        self.factors.push(label.into());
    }
}

fn score_house_and_joy(planet: ClassicalPlanet, house: u8, acc: &mut ScoreAcc) {
    let house_entry: Option<(i32, &str)> = match house {
        1 | 10 => Some((5, "In 1st/10th House (+5)")),
        4 | 7 => Some((4, "In 4th/7th House (+4)")),
        11 => Some((4, "In 11th House (+4)")),
        5 => Some((3, "In 5th House (+3)")),
        2 => Some((3, "In 2nd House (+3)")),
        9 => Some((2, "In 9th House (+2)")),
        3 => Some((1, "In 3rd House (+1)")),
        8 => Some((-2, "In 8th House (-2)")),
        6 | 12 => Some((-5, "In 6th/12th House (-5)")),
        _ => None,
    };
    if let Some((pts, label)) = house_entry {
        acc.add(pts, label);
    }

    let is_joy = matches!(
        (planet, house),
        (ClassicalPlanet::Mercury, 1) | (ClassicalPlanet::Moon, 3) | (ClassicalPlanet::Venus, 5)
            | (ClassicalPlanet::Mars, 6) | (ClassicalPlanet::Sun, 9)
            | (ClassicalPlanet::Jupiter, 11) | (ClassicalPlanet::Saturn, 12)
    );
    if is_joy {
        acc.add(2, "Joy in House (+2)");
    }
}

fn score_motion_and_speed(planet: ClassicalPlanet, speed: f64, acc: &mut ScoreAcc) {
    if !matches!(planet, ClassicalPlanet::Sun | ClassicalPlanet::Moon) {
        if speed < 0.0 {
            acc.add(-5, "Retrograde (-5)");
        } else {
            acc.add(4, "Direct in Motion (+4)");
        }
    }
    // Classical Lilly CA p. 115: Swift in motion requires forward (direct) speed exceeding mean motion.
    if speed > planet.average_daily_motion() {
        acc.add(2, "Swift in Motion (+2)");
    } else {
        acc.add(-2, "Slow in Motion (-2)");
    }
}

fn score_solar_phase_and_beams(
    planet: ClassicalPlanet,
    norm_lon: f64,
    sign_index: u8,
    sun_longitude: f64,
    acc: &mut ScoreAcc,
) {
    if planet == ClassicalPlanet::Sun {
        return;
    }
    let norm_sun = sun_longitude.rem_euclid(360.0);
    let elong = (norm_lon - norm_sun).rem_euclid(360.0);
    match planet {
        ClassicalPlanet::Mars | ClassicalPlanet::Jupiter | ClassicalPlanet::Saturn => {
            if elong > 180.0 { acc.add(2, "Oriental of the Sun (+2)"); }
            else if elong > 1e-6 { acc.add(-2, "Occidental of the Sun (-2)"); }
        }
        ClassicalPlanet::Mercury | ClassicalPlanet::Venus => {
            if elong > 1e-6 && elong < 180.0 { acc.add(2, "Occidental of the Sun (+2)"); }
            else if elong > 180.0 { acc.add(-2, "Oriental of the Sun (-2)"); }
        }
        ClassicalPlanet::Moon => {
            if elong < 180.0 { acc.add(2, "Increasing in Light (+2)"); }
            else { acc.add(-2, "Decreasing in Light (-2)"); }
        }
        ClassicalPlanet::Sun => {}
    }

    let diff = (norm_lon - norm_sun).abs();
    let sep = if diff > 180.0 { 360.0 - diff } else { diff };
    let same_sign = sign_index == (norm_sun / 30.0).floor() as u8 % 12;
    if sep <= 17.0 / 60.0 && same_sign {
        acc.add(5, "Cazimi / Heart of the Sun (+5)");
    } else if sep <= 8.5 && same_sign {
        acc.add(-6, "Combust in Same Sign (-6)");
    } else if sep <= 17.0 {
        acc.add(-4, "Under the Sunbeams (-4)");
    } else {
        acc.add(5, "Free from Sun's Beams (+5)");
    }
}

fn score_nodes_and_fixed_stars(input: &PlanetDignityInput<'_>, norm_lon: f64, acc: &mut ScoreAcc) {
    let sep_to = |target: f64| {
        let d = (norm_lon - target.rem_euclid(360.0)).abs();
        if d > 180.0 { 360.0 - d } else { d }
    };
    if sep_to(input.north_node_lon) <= 1.5 {
        acc.add(4, "Conjunct North Node (+4)");
    }
    if sep_to(input.south_node_lon) <= 1.5 {
        acc.add(-4, "Conjunct South Node (-4)");
    }

    // Only astrologically qualified (visible & within latitude/magnitude rules) fixed stars modify Lilly score.
    for sc in input.fixed_star_conjunctions {
        if !sc.is_qualified || sc.body_name != input.planet.name() || sc.orb > 1.0 {
            continue;
        }
        if sc.star_name.starts_with("Regulus") {
            acc.add(6, "Conjunct Regulus (+6)");
        } else if sc.star_name.starts_with("Spica") {
            acc.add(5, "Conjunct Spica (+5)");
        } else if sc.star_name.starts_with("Algol") {
            acc.add(-5, "Conjunct Caput Algol (-5)");
        }
    }
}

fn partile_aspect_points(other_cp: ClassicalPlanet, sign_diff: u8) -> Option<(i32, &'static str)> {
    match (other_cp, sign_diff) {
        (ClassicalPlanet::Jupiter | ClassicalPlanet::Venus, 0) => Some((5, "Partile Conjunction")),
        (ClassicalPlanet::Jupiter | ClassicalPlanet::Venus, 4 | 8) => Some((4, "Partile Trine")),
        (ClassicalPlanet::Jupiter | ClassicalPlanet::Venus, 2 | 10) => Some((3, "Partile Sextile")),
        (ClassicalPlanet::Mars | ClassicalPlanet::Saturn, 0) => Some((-5, "Partile Conjunction")),
        (ClassicalPlanet::Mars | ClassicalPlanet::Saturn, 6) => Some((-4, "Partile Opposition")),
        (ClassicalPlanet::Mars | ClassicalPlanet::Saturn, 3 | 9) => Some((-3, "Partile Square")),
        _ => None,
    }
}

fn score_partile_aspects(
    planet: ClassicalPlanet,
    sign_index: u8,
    sign_degree: f64,
    other_planets: &[(&str, f64)],
    acc: &mut ScoreAcc,
) {
    let deg_int = sign_degree.floor() as i32;
    for &(other_name, raw_lon) in other_planets {
        let Some(other_cp) = ClassicalPlanet::from_name(other_name) else { continue };
        if other_cp == planet {
            continue;
        }
        let norm_other = raw_lon.rem_euclid(360.0);
        let other_sign = (norm_other / 30.0).floor() as u8 % 12;
        let other_deg = norm_other - (other_sign as f64 * 30.0);
        if deg_int != other_deg.floor() as i32 && (sign_degree - other_deg).abs() > 1.0 {
            continue;
        }
        let sign_diff = (sign_index as i16 - other_sign as i16).rem_euclid(12) as u8;
        if let Some((pts, label)) = partile_aspect_points(other_cp, sign_diff) {
            acc.add(pts, format!("{} with {} ({:+})", label, other_cp.name(), pts));
        }
    }
}

fn score_besiegement(
    planet: ClassicalPlanet,
    norm_lon: f64,
    other_planets: &[(&str, f64)],
    acc: &mut ScoreAcc,
) {
    if matches!(planet, ClassicalPlanet::Mars | ClassicalPlanet::Saturn) {
        return;
    }
    let find_body = |target: ClassicalPlanet| {
        other_planets
            .iter()
            .find(|&&(n, _)| ClassicalPlanet::from_name(n) == Some(target))
            .map(|&(_, lon)| lon.rem_euclid(360.0))
    };
    let (Some(m_lon), Some(s_lon)) = (find_body(ClassicalPlanet::Mars), find_body(ClassicalPlanet::Saturn)) else {
        return;
    };
    let diff_ms = (m_lon - s_lon).abs();
    let span_ms = if diff_ms > 180.0 { 360.0 - diff_ms } else { diff_ms };
    if span_ms > 17.0 {
        return;
    }
    let arc = (s_lon - m_lon).rem_euclid(360.0);
    let is_between = if arc <= 180.0 {
        let d = (norm_lon - m_lon).rem_euclid(360.0);
        d > 0.001 && d < arc - 0.001
    } else {
        let d = (norm_lon - s_lon).rem_euclid(360.0);
        d > 0.001 && d < (m_lon - s_lon).rem_euclid(360.0) - 0.001
    };
    if is_between {
        acc.add(-5, "Besieged by Mars and Saturn (-5)");
    }
}

/// Calculate complete William Lilly essential and accidental dignity scores (CA p. 115).
pub fn calculate_planet_dignity_with_context(input: &PlanetDignityInput<'_>) -> PlanetDignityReport {
    let norm_lon = input.longitude.rem_euclid(360.0);
    let sign_index = (norm_lon / 30.0).floor() as u8 % 12;
    let sign_degree = norm_lon - (sign_index as f64 * 30.0);
    let opp_sign = (sign_index + 6) % 12;
    let d = evaluate_essential_dignities(norm_lon, input.is_day_chart);

    let is_in_domicile = d.domicile == input.planet;
    let is_in_exaltation = d.exaltation == Some(input.planet);
    let is_in_triplicity = d.triplicity == input.planet;
    let is_in_term = d.term == input.planet;
    let is_in_face = d.face == input.planet;
    let is_in_detriment = DOMICILE_RULERS[opp_sign as usize] == input.planet;
    let is_in_fall = EXALTATIONS.iter().find(|(s, _, _)| *s == opp_sign).map(|(_, p, _)| *p) == Some(input.planet);
    let is_peregrine = !(is_in_domicile || is_in_exaltation || is_in_triplicity || is_in_term || is_in_face);

    let essential_score = [
        (is_in_domicile, 5), (is_in_exaltation, 4), (is_in_triplicity, 3),
        (is_in_term, 2), (is_in_face, 1), (is_in_detriment, -5),
        (is_in_fall, -4), (is_peregrine, -5),
    ]
    .iter()
    .filter_map(|&(flag, pts)| flag.then_some(pts))
    .sum();

    let mut acc = ScoreAcc::new();
    score_house_and_joy(input.planet, input.house_number, &mut acc);
    score_motion_and_speed(input.planet, input.speed, &mut acc);
    score_solar_phase_and_beams(input.planet, norm_lon, sign_index, input.sun_longitude, &mut acc);
    score_nodes_and_fixed_stars(input, norm_lon, &mut acc);
    score_partile_aspects(input.planet, sign_index, sign_degree, input.other_planets, &mut acc);
    score_besiegement(input.planet, norm_lon, input.other_planets, &mut acc);

    PlanetDignityReport {
        planet: input.planet,
        longitude: input.longitude,
        sign_index,
        sign_degree,
        house_number: input.house_number,
        is_in_domicile,
        is_in_exaltation,
        is_in_triplicity,
        is_in_term,
        is_in_face,
        is_in_detriment,
        is_in_fall,
        is_peregrine,
        essential_score,
        accidental_score: acc.score,
        total_lilly_score: essential_score + acc.score,
        accidental_factors: acc.factors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input(planet: ClassicalPlanet, longitude: f64, speed: f64, sun_longitude: f64) -> PlanetDignityInput<'static> {
        PlanetDignityInput {
            planet, longitude, speed, house_number: 1, sun_longitude,
            is_day_chart: true, north_node_lon: 120.0, south_node_lon: 300.0,
            other_planets: &[], fixed_star_conjunctions: &[],
        }
    }

    #[test]
    fn test_egyptian_vs_ptolemaic_terms() {
        assert_eq!(egyptian_term(0, 13.0), ClassicalPlanet::Mercury);
        assert_eq!(ptolemaic_term(0, 13.0), ClassicalPlanet::Venus);
        assert_eq!(egyptian_term(1, 27.5), ClassicalPlanet::Saturn);
        assert_eq!(ptolemaic_term(1, 27.5), ClassicalPlanet::Mars);
        assert_eq!(egyptian_term(6, 12.0), ClassicalPlanet::Mercury);
        assert_eq!(ptolemaic_term(6, 12.0), ClassicalPlanet::Jupiter);
    }

    #[test]
    fn test_peregrine_and_combust_rules() {
        let mut inp = sample_input(ClassicalPlanet::Mars, 32.0, 0.5, 90.0);
        inp.house_number = 7;
        let report = calculate_planet_dignity_with_context(&inp);
        assert!(report.is_in_detriment && report.is_peregrine);
        assert_eq!(report.essential_score, -10);

        let same_sign = calculate_planet_dignity_with_context(&sample_input(ClassicalPlanet::Mercury, 25.0, 1.2, 29.0));
        assert!(same_sign.accidental_factors.iter().any(|f| f.contains("Combust in Same Sign (-6)")));

        let diff_sign = calculate_planet_dignity_with_context(&sample_input(ClassicalPlanet::Mercury, 32.0, 1.2, 29.0));
        assert!(diff_sign.accidental_factors.iter().any(|f| f.contains("Under the Sunbeams (-4)")));
        assert!(!diff_sign.accidental_factors.iter().any(|f| f.contains("Combust")));
    }

    #[test]
    fn test_retrograde_is_not_swift_and_unqualified_star_ignored() {
        let retro = calculate_planet_dignity_with_context(&sample_input(ClassicalPlanet::Mercury, 100.0, -1.3, 200.0));
        assert!(retro.accidental_factors.iter().any(|f| f.contains("Retrograde (-5)")));
        assert!(retro.accidental_factors.iter().any(|f| f.contains("Slow in Motion (-2)")));
        assert!(!retro.accidental_factors.iter().any(|f| f.contains("Swift in Motion")));

        let unqual_algol = [FixedStarConjunction {
            star_name: "Algol".into(), korean_name: "알골".into(), body_name: "Mars".into(),
            star_longitude: 56.0, star_latitude: 22.4, star_magnitude: 2.12, body_longitude: 56.2,
            orb: 0.2, is_visible: false, max_altitude: -5.0, is_qualified: false,
            qualification_reason: "지평선 미출몰 (관측 불가)".into(),
            nature: "Saturn / Mars".into(), meaning: "Danger".into(),
        }];
        let mut inp = sample_input(ClassicalPlanet::Mars, 56.2, 0.7, 200.0);
        inp.fixed_star_conjunctions = &unqual_algol;
        let rep = calculate_planet_dignity_with_context(&inp);
        assert!(!rep.accidental_factors.iter().any(|f| f.contains("Algol")));
    }
}
