//! William Lilly's 5-Fold Essential Dignities and Accidental Dignity Scoring.
//!
//! Based on Christian Astrology (1647), Chapter XIX (p. 101-115).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassicalPlanet {
    Sun,
    Moon,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
}

impl ClassicalPlanet {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Sun" => Some(Self::Sun),
            "Moon" => Some(Self::Moon),
            "Mercury" => Some(Self::Mercury),
            "Venus" => Some(Self::Venus),
            "Mars" => Some(Self::Mars),
            "Jupiter" => Some(Self::Jupiter),
            "Saturn" => Some(Self::Saturn),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Sun => "Sun",
            Self::Moon => "Moon",
            Self::Mercury => "Mercury",
            Self::Venus => "Venus",
            Self::Mars => "Mars",
            Self::Jupiter => "Jupiter",
            Self::Saturn => "Saturn",
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::Sun => "☉",
            Self::Moon => "☽",
            Self::Mercury => "☿",
            Self::Venus => "♀",
            Self::Mars => "♂",
            Self::Jupiter => "♃",
            Self::Saturn => "♄",
        }
    }

    /// Average daily motion in degrees.
    pub fn average_daily_motion(self) -> f64 {
        match self {
            Self::Sun => 59.0 / 60.0 + 8.0 / 3600.0, // 0°59'08"
            Self::Moon => 13.0 + 10.0 / 60.0 + 36.0 / 3600.0, // 13°10'36"
            Self::Mercury => 59.0 / 60.0 + 8.0 / 3600.0,
            Self::Venus => 59.0 / 60.0 + 8.0 / 3600.0,
            Self::Mars => 31.0 / 60.0 + 27.0 / 3600.0, // 0°31'27"
            Self::Jupiter => 4.0 / 60.0 + 59.0 / 3600.0, // 0°04'59"
            Self::Saturn => 2.0 / 60.0 + 0.0 / 3600.0, // 0°02'00"
        }
    }

    /// Lilly's planetary moiety of orb (half orb in degrees).
    pub fn moiety(self) -> f64 {
        match self {
            Self::Sun => 7.5,      // Full: 15°
            Self::Moon => 6.0,     // Full: 12°
            Self::Mercury => 3.5,  // Full: 7°
            Self::Venus => 3.5,    // Full: 7°
            Self::Mars => 4.0,     // Full: 8°
            Self::Jupiter => 4.5,  // Full: 9°
            Self::Saturn => 4.5,   // Full: 9°
        }
    }
}

/// Domicile rulers for 12 zodiac signs (0 = Aries .. 11 = Pisces).
pub const DOMICILE_RULERS: [ClassicalPlanet; 12] = [
    ClassicalPlanet::Mars,    // Aries
    ClassicalPlanet::Venus,   // Taurus
    ClassicalPlanet::Mercury, // Gemini
    ClassicalPlanet::Moon,    // Cancer
    ClassicalPlanet::Sun,     // Leo
    ClassicalPlanet::Mercury, // Virgo
    ClassicalPlanet::Venus,   // Libra
    ClassicalPlanet::Mars,    // Scorpio
    ClassicalPlanet::Jupiter, // Sagittarius
    ClassicalPlanet::Saturn,  // Capricorn
    ClassicalPlanet::Saturn,  // Aquarius
    ClassicalPlanet::Jupiter, // Pisces
];

/// Exaltation: (Sign index, Exaltation planet, Exaltation degree).
pub const EXALTATIONS: [(u8, ClassicalPlanet, f64); 7] = [
    (0, ClassicalPlanet::Sun, 19.0),      // Aries 19°
    (1, ClassicalPlanet::Moon, 3.0),      // Taurus 3°
    (3, ClassicalPlanet::Jupiter, 15.0),  // Cancer 15°
    (5, ClassicalPlanet::Mercury, 15.0),  // Virgo 15°
    (6, ClassicalPlanet::Saturn, 21.0),   // Libra 21°
    (9, ClassicalPlanet::Mars, 28.0),     // Capricorn 28°
    (11, ClassicalPlanet::Venus, 27.0),   // Pisces 27°
];

/// Triplicity rulers: (Day Ruler, Night Ruler).
/// Fire (Aries, Leo, Sag): Sun (Day), Jupiter (Night)
/// Earth (Taurus, Virgo, Cap): Venus (Day), Moon (Night)
/// Air (Gemini, Libra, Aqu): Saturn (Day), Mercury (Night)
/// Water (Cancer, Scorpio, Pis): Mars (Day), Mars (Night)
pub fn triplicity_rulers(sign_index: u8) -> (ClassicalPlanet, ClassicalPlanet) {
    match sign_index % 4 {
        0 => (ClassicalPlanet::Sun, ClassicalPlanet::Jupiter),       // Fire: Aries(0), Leo(4), Sag(8)
        1 => (ClassicalPlanet::Venus, ClassicalPlanet::Moon),        // Earth: Taurus(1), Virgo(5), Cap(9)
        2 => (ClassicalPlanet::Saturn, ClassicalPlanet::Mercury),     // Air: Gemini(2), Libra(6), Aqu(10)
        _ => (ClassicalPlanet::Mars, ClassicalPlanet::Mars),         // Water: Cancer(3), Scorpio(7), Pis(11)
    }
}

/// Egyptian Terms: (Max degree, Planet) for each of the 12 signs.
/// Corrected standard Egyptian bounds from Hellenistic tradition.
pub fn egyptian_term(sign_index: u8, degree_in_sign: f64) -> ClassicalPlanet {
    let terms: &[(f64, ClassicalPlanet)] = match sign_index % 12 {
        0 => &[ // Aries
            (6.0, ClassicalPlanet::Jupiter),
            (12.0, ClassicalPlanet::Venus),
            (20.0, ClassicalPlanet::Mercury),
            (25.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        1 => &[ // Taurus (standard Egyptian: Ven 8, Mer 14, Jup 22, Sat 27, Mars 30)
            (8.0, ClassicalPlanet::Venus),
            (14.0, ClassicalPlanet::Mercury),
            (22.0, ClassicalPlanet::Jupiter),
            (27.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        2 => &[ // Gemini
            (6.0, ClassicalPlanet::Mercury),
            (12.0, ClassicalPlanet::Jupiter),
            (17.0, ClassicalPlanet::Venus),
            (24.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        3 => &[ // Cancer
            (7.0, ClassicalPlanet::Mars),
            (13.0, ClassicalPlanet::Venus),
            (19.0, ClassicalPlanet::Mercury),
            (26.0, ClassicalPlanet::Jupiter),
            (30.0, ClassicalPlanet::Saturn),
        ],
        4 => &[ // Leo
            (6.0, ClassicalPlanet::Jupiter),
            (11.0, ClassicalPlanet::Venus),
            (18.0, ClassicalPlanet::Saturn),
            (24.0, ClassicalPlanet::Mercury),
            (30.0, ClassicalPlanet::Mars),
        ],
        5 => &[ // Virgo
            (7.0, ClassicalPlanet::Mercury),
            (17.0, ClassicalPlanet::Venus),
            (21.0, ClassicalPlanet::Jupiter),
            (28.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        6 => &[ // Libra (standard Egyptian: Sat 6, Mer 14, Ven 21, Jup 28, Mars 30)
            (6.0, ClassicalPlanet::Saturn),
            (14.0, ClassicalPlanet::Mercury),
            (21.0, ClassicalPlanet::Venus),
            (28.0, ClassicalPlanet::Jupiter),
            (30.0, ClassicalPlanet::Mars),
        ],
        7 => &[ // Scorpio
            (7.0, ClassicalPlanet::Mars),
            (11.0, ClassicalPlanet::Venus),
            (19.0, ClassicalPlanet::Mercury),
            (24.0, ClassicalPlanet::Jupiter),
            (30.0, ClassicalPlanet::Saturn),
        ],
        8 => &[ // Sagittarius
            (12.0, ClassicalPlanet::Jupiter),
            (17.0, ClassicalPlanet::Venus),
            (21.0, ClassicalPlanet::Mercury),
            (26.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        9 => &[ // Capricorn
            (7.0, ClassicalPlanet::Mercury),
            (14.0, ClassicalPlanet::Jupiter),
            (22.0, ClassicalPlanet::Venus),
            (26.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        10 => &[ // Aquarius
            (7.0, ClassicalPlanet::Mercury),
            (13.0, ClassicalPlanet::Venus),
            (20.0, ClassicalPlanet::Jupiter),
            (25.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        _ => &[ // Pisces
            (12.0, ClassicalPlanet::Venus),
            (16.0, ClassicalPlanet::Jupiter),
            (19.0, ClassicalPlanet::Mercury),
            (28.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
    };

    for &(limit, planet) in terms {
        if degree_in_sign < limit {
            return planet;
        }
    }
    ClassicalPlanet::Saturn
}

/// Ptolemaic Terms: (Max degree, Planet) for each of the 12 signs according to Claudius Ptolemy (Tetrabiblos)
/// and William Lilly (Christian Astrology, 1647, p. 104).
pub fn ptolemaic_term(sign_index: u8, degree_in_sign: f64) -> ClassicalPlanet {
    let terms: &[(f64, ClassicalPlanet)] = match sign_index % 12 {
        0 => &[ // Aries
            (6.0, ClassicalPlanet::Jupiter),
            (12.0, ClassicalPlanet::Venus),
            (20.0, ClassicalPlanet::Mercury),
            (25.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        1 => &[ // Taurus
            (8.0, ClassicalPlanet::Venus),
            (14.0, ClassicalPlanet::Mercury),
            (22.0, ClassicalPlanet::Jupiter),
            (27.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        2 => &[ // Gemini
            (6.0, ClassicalPlanet::Mercury),
            (12.0, ClassicalPlanet::Jupiter),
            (17.0, ClassicalPlanet::Venus),
            (24.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        3 => &[ // Cancer
            (7.0, ClassicalPlanet::Mars),
            (13.0, ClassicalPlanet::Venus),
            (19.0, ClassicalPlanet::Mercury),
            (26.0, ClassicalPlanet::Jupiter),
            (30.0, ClassicalPlanet::Saturn),
        ],
        4 => &[ // Leo
            (6.0, ClassicalPlanet::Jupiter),
            (11.0, ClassicalPlanet::Venus),
            (18.0, ClassicalPlanet::Saturn),
            (24.0, ClassicalPlanet::Mercury),
            (30.0, ClassicalPlanet::Mars),
        ],
        5 => &[ // Virgo
            (7.0, ClassicalPlanet::Mercury),
            (17.0, ClassicalPlanet::Venus),
            (21.0, ClassicalPlanet::Jupiter),
            (28.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        6 => &[ // Libra (Ptolemy / Lilly CA p. 104: Sat 6, Ven 14, Jup 21, Mer 28, Mars 30)
            (6.0, ClassicalPlanet::Saturn),
            (14.0, ClassicalPlanet::Venus),
            (21.0, ClassicalPlanet::Jupiter),
            (28.0, ClassicalPlanet::Mercury),
            (30.0, ClassicalPlanet::Mars),
        ],
        7 => &[ // Scorpio
            (7.0, ClassicalPlanet::Mars),
            (11.0, ClassicalPlanet::Venus),
            (19.0, ClassicalPlanet::Mercury),
            (24.0, ClassicalPlanet::Jupiter),
            (30.0, ClassicalPlanet::Saturn),
        ],
        8 => &[ // Sagittarius
            (12.0, ClassicalPlanet::Jupiter),
            (17.0, ClassicalPlanet::Venus),
            (21.0, ClassicalPlanet::Mercury),
            (26.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        9 => &[ // Capricorn
            (7.0, ClassicalPlanet::Mercury),
            (14.0, ClassicalPlanet::Jupiter),
            (22.0, ClassicalPlanet::Venus),
            (26.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        10 => &[ // Aquarius
            (7.0, ClassicalPlanet::Mercury),
            (13.0, ClassicalPlanet::Venus),
            (20.0, ClassicalPlanet::Jupiter),
            (25.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        _ => &[ // Pisces
            (12.0, ClassicalPlanet::Venus),
            (16.0, ClassicalPlanet::Jupiter),
            (19.0, ClassicalPlanet::Mercury),
            (28.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
    };

    for &(limit, planet) in terms {
        if degree_in_sign < limit {
            return planet;
        }
    }
    ClassicalPlanet::Saturn
}

/// Chaldean Decans / Faces: 10 degrees each, cycling through Chaldean order.
pub fn chaldean_face(sign_index: u8, degree_in_sign: f64) -> ClassicalPlanet {
    let chaldean_order = [
        ClassicalPlanet::Mars,
        ClassicalPlanet::Sun,
        ClassicalPlanet::Venus,
        ClassicalPlanet::Mercury,
        ClassicalPlanet::Moon,
        ClassicalPlanet::Saturn,
        ClassicalPlanet::Jupiter,
    ];
    let decan_index = (degree_in_sign / 10.0).floor().clamp(0.0, 2.0) as usize;
    let total_decan = (sign_index as usize * 3) + decan_index;
    chaldean_order[total_decan % 7]
}

/// 5-Fold Essential Dignity evaluation for a specific point on the zodiac.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssentialDignities {
    pub domicile: ClassicalPlanet,
    pub exaltation: Option<ClassicalPlanet>,
    pub triplicity: ClassicalPlanet,
    pub term: ClassicalPlanet,
    pub face: ClassicalPlanet,
}

pub fn evaluate_essential_dignities(
    longitude: f64,
    is_day_chart: bool,
) -> EssentialDignities {
    let sign_index = (longitude / 30.0).floor() as u8 % 12;
    let degree_in_sign = longitude - (sign_index as f64 * 30.0);

    let domicile = DOMICILE_RULERS[sign_index as usize];
    let exaltation = EXALTATIONS.iter().find(|(s, _, _)| *s == sign_index).map(|(_, p, _)| *p);
    let (day_trip, night_trip) = triplicity_rulers(sign_index);
    let triplicity = if is_day_chart { day_trip } else { night_trip };
    let term = ptolemaic_term(sign_index, degree_in_sign);
    let face = chaldean_face(sign_index, degree_in_sign);

    EssentialDignities {
        domicile,
        exaltation,
        triplicity,
        term,
        face,
    }
}

/// Planetary dignity status and William Lilly score.
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

/// Calculate essential & accidental dignity scores for a planet according to William Lilly.
pub fn calculate_planet_dignity(
    planet: ClassicalPlanet,
    longitude: f64,
    speed: f64,
    house_number: u8,
    sun_longitude: f64,
    is_day_chart: bool,
    north_node_lon: f64,
    south_node_lon: f64,
) -> PlanetDignityReport {
    let sign_index = (longitude / 30.0).floor() as u8 % 12;
    let sign_degree = longitude - (sign_index as f64 * 30.0);
    let opp_sign = (sign_index + 6) % 12;

    let dignities = evaluate_essential_dignities(longitude, is_day_chart);

    let is_in_domicile = dignities.domicile == planet;
    let is_in_exaltation = dignities.exaltation == Some(planet);
    let is_in_triplicity = dignities.triplicity == planet;
    let is_in_term = dignities.term == planet;
    let is_in_face = dignities.face == planet;

    let is_in_detriment = DOMICILE_RULERS[opp_sign as usize] == planet;
    let is_in_fall = EXALTATIONS.iter().find(|(s, _, _)| *s == opp_sign).map(|(_, p, _)| *p) == Some(planet);

    // In classical astrology (Lilly CA p. 112), a planet is Peregrine when it has NO essential dignity
    // (no domicile, exaltation, triplicity, term, nor face), even if it is in detriment or fall.
    let has_any_dignity = is_in_domicile || is_in_exaltation || is_in_triplicity || is_in_term || is_in_face;
    let is_peregrine = !has_any_dignity;

    let mut essential_score = 0;
    if is_in_domicile { essential_score += 5; }
    if is_in_exaltation { essential_score += 4; }
    if is_in_triplicity { essential_score += 3; }
    if is_in_term { essential_score += 2; }
    if is_in_face { essential_score += 1; }
    if is_in_detriment { essential_score -= 5; }
    if is_in_fall { essential_score -= 4; }
    if is_peregrine { essential_score -= 5; }

    // Accidental dignities (William Lilly Table CA p. 115)
    let mut accidental_score = 0;
    let mut factors = Vec::new();

    // House placement
    match house_number {
        1 | 10 => { accidental_score += 5; factors.push("In 1st/10th House (+5)".into()); }
        4 | 7 => { accidental_score += 4; factors.push("In 4th/7th House (+4)".into()); }
        11 => { accidental_score += 4; factors.push("In 11th House (+4)".into()); }
        5 => { accidental_score += 3; factors.push("In 5th House (+3)".into()); }
        2 => { accidental_score += 3; factors.push("In 2nd House (+3)".into()); }
        9 => { accidental_score += 2; factors.push("In 9th House (+2)".into()); }
        3 => { accidental_score += 1; factors.push("In 3rd House (+1)".into()); }
        8 => { accidental_score -= 2; factors.push("In 8th House (-2)".into()); }
        6 | 12 => { accidental_score -= 5; factors.push("In 6th/12th House (-5)".into()); }
        _ => {}
    }

    // Motion: Direct / Retrograde
    if planet != ClassicalPlanet::Sun && planet != ClassicalPlanet::Moon {
        if speed < 0.0 {
            accidental_score -= 5;
            factors.push("Retrograde (-5)".into());
        } else {
            accidental_score += 4;
            factors.push("Direct in Motion (+4)".into());
        }
    }

    // Speed: Swift / Slow
    let avg_speed = planet.average_daily_motion();
    if speed.abs() > avg_speed {
        accidental_score += 2;
        factors.push("Swift in Motion (+2)".into());
    } else {
        accidental_score -= 2;
        factors.push("Slow in Motion (-2)".into());
    }

    // Relation to Sun (Cazimi / Combust / Under Sunbeams)
    if planet != ClassicalPlanet::Sun {
        let diff = (longitude - sun_longitude).abs();
        let sep = if diff > 180.0 { 360.0 - diff } else { diff };

        if sep <= 17.0 / 60.0 {
            accidental_score += 5;
            factors.push("Cazimi / Heart of the Sun (+5)".into());
        } else if sep <= 8.5 {
            accidental_score -= 5;
            factors.push("Combust (-5)".into());
        } else if sep <= 17.0 {
            accidental_score -= 4;
            factors.push("Under the Sunbeams (-4)".into());
        } else {
            accidental_score += 5;
            factors.push("Free from Sun's Beams (+5)".into());
        }
    }

    // Lunar Nodes Conjunction (orb <= 1.5°)
    let diff_nn = (longitude - north_node_lon).abs();
    let sep_nn = if diff_nn > 180.0 { 360.0 - diff_nn } else { diff_nn };
    if sep_nn <= 1.5 {
        accidental_score += 4;
        factors.push("Conjunct North Node (+4)".into());
    }

    let diff_sn = (longitude - south_node_lon).abs();
    let sep_sn = if diff_sn > 180.0 { 360.0 - diff_sn } else { diff_sn };
    if sep_sn <= 1.5 {
        accidental_score -= 4;
        factors.push("Conjunct South Node (-4)".into());
    }

    // Joy of planets in houses
    let is_joy = match (planet, house_number) {
        (ClassicalPlanet::Mercury, 1) => true,
        (ClassicalPlanet::Moon, 3) => true,
        (ClassicalPlanet::Venus, 5) => true,
        (ClassicalPlanet::Mars, 6) => true,
        (ClassicalPlanet::Sun, 9) => true,
        (ClassicalPlanet::Jupiter, 11) => true,
        (ClassicalPlanet::Saturn, 12) => true,
        _ => false,
    };
    if is_joy {
        accidental_score += 2;
        factors.push("Joy in House (+2)".into());
    }

    let total_lilly_score = essential_score + accidental_score;

    PlanetDignityReport {
        planet,
        longitude,
        sign_index,
        sign_degree,
        house_number,
        is_in_domicile,
        is_in_exaltation,
        is_in_triplicity,
        is_in_term,
        is_in_face,
        is_in_detriment,
        is_in_fall,
        is_peregrine,
        essential_score,
        accidental_score,
        total_lilly_score,
        accidental_factors: factors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_egyptian_vs_ptolemaic_terms() {
        // Taurus (sign_index 1):
        // 14.5° Taurus:
        // Egyptian: Jupiter (14-22°)
        assert_eq!(egyptian_term(1, 14.5), ClassicalPlanet::Jupiter);
        // Ptolemaic: Jupiter (14-22°)
        assert_eq!(ptolemaic_term(1, 14.5), ClassicalPlanet::Jupiter);

        // 26.5° Taurus:
        // Egyptian: Saturn (22-27°)
        assert_eq!(egyptian_term(1, 26.5), ClassicalPlanet::Saturn);
        // Ptolemaic: Saturn (22-27°)
        assert_eq!(ptolemaic_term(1, 26.5), ClassicalPlanet::Saturn);

        // Libra (sign_index 6):
        // 10.0° Libra:
        // Egyptian: Mercury (6-14°)
        assert_eq!(egyptian_term(6, 10.0), ClassicalPlanet::Mercury);
        // Ptolemaic / Lilly CA p. 104: Venus (6-14°)
        assert_eq!(ptolemaic_term(6, 10.0), ClassicalPlanet::Venus);

        // 18.0° Libra:
        // Egyptian: Venus (14-21°)
        assert_eq!(egyptian_term(6, 18.0), ClassicalPlanet::Venus);
        // Ptolemaic / Lilly CA p. 104: Jupiter (14-21°)
        assert_eq!(ptolemaic_term(6, 18.0), ClassicalPlanet::Jupiter);
    }

    #[test]
    fn test_peregrine_in_detriment_and_fall() {
        // Mars in Taurus 2° (32.0°):
        // Domicile: Venus, Exaltation: Moon (3°), Triplicity: Venus/Moon, Term: Venus (0-8°), Face: Mercury (0-10°)
        // Mars has no dignity in Taurus 2°.
        // Taurus is detriment for Mars (opposite Scorpio).
        // Under classical astrology rules, Mars is BOTH in detriment (-5) AND peregrine (-5).
        let report = calculate_planet_dignity(
            ClassicalPlanet::Mars,
            32.0, // 2° Taurus
            0.5,
            7, // 7th house
            90.0,
            true, // day chart
            120.0,
            300.0,
        );

        assert!(report.is_in_detriment);
        assert!(report.is_peregrine, "Planet in detriment with no dignities must be marked peregrine");
        // Essential score: Detriment (-5) + Peregrine (-5) = -10
        assert_eq!(report.essential_score, -10);
    }
}
