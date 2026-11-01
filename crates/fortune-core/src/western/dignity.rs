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

    /// Traditional Chaldean speed order (0 = Moon fastest/lightest .. 6 = Saturn slowest/heaviest).
    /// In classical astrology (Lilly CA p. 107), lighter planets apply to heavier planets.
    pub fn chaldean_speed_rank(self) -> u8 {
        match self {
            Self::Moon => 0,
            Self::Mercury => 1,
            Self::Venus => 2,
            Self::Sun => 3,
            Self::Mars => 4,
            Self::Jupiter => 5,
            Self::Saturn => 6,
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

/// Triplicity rulers: (Day Ruler, Night Ruler) according to Ptolemy / William Lilly (CA p. 104).
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

/// Dorothean / Hellenistic Triplicity rulers (Vettius Valens, Anthologies Book I & II):
/// Returns `(Day Ruler, Night Ruler, Participating Ruler)`.
/// - Fire (Aries, Leo, Sag): Sun (Day), Jupiter (Night), Saturn (Participating)
/// - Earth (Taurus, Virgo, Cap): Venus (Day), Moon (Night), Mars (Participating)
/// - Air (Gemini, Libra, Aqu): Saturn (Day), Mercury (Night), Jupiter (Participating)
/// - Water (Cancer, Scorpio, Pis): Venus (Day), Mars (Night), Moon (Participating)
pub fn dorothean_triplicity_rulers(sign_index: u8) -> (ClassicalPlanet, ClassicalPlanet, ClassicalPlanet) {
    match sign_index % 4 {
        0 => (ClassicalPlanet::Sun, ClassicalPlanet::Jupiter, ClassicalPlanet::Saturn),   // Fire
        1 => (ClassicalPlanet::Venus, ClassicalPlanet::Moon, ClassicalPlanet::Mars),      // Earth
        2 => (ClassicalPlanet::Saturn, ClassicalPlanet::Mercury, ClassicalPlanet::Jupiter), // Air
        _ => (ClassicalPlanet::Venus, ClassicalPlanet::Mars, ClassicalPlanet::Moon),      // Water
    }
}

/// Egyptian Terms (Bounds): (Max degree, Planet) for each of the 12 signs.
/// Authentic Egyptian bounds per Vettius Valens (Anthologies Book I Ch. 3 & Book IX Ch. 6).
pub fn egyptian_term(sign_index: u8, degree_in_sign: f64) -> ClassicalPlanet {
    let terms: &[(f64, ClassicalPlanet)] = match sign_index % 12 {
        0 => &[ // Aries: Jup 6, Ven 6 (12), Mer 8 (20), Mar 5 (25), Sat 5 (30)
            (6.0, ClassicalPlanet::Jupiter),
            (12.0, ClassicalPlanet::Venus),
            (20.0, ClassicalPlanet::Mercury),
            (25.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        1 => &[ // Taurus (Valens Book IX Ch. 6): Ven 8, Mer 6 (14), Jup 8 (22), Sat 6 (28), Mars 2 (30)
            (8.0, ClassicalPlanet::Venus),
            (14.0, ClassicalPlanet::Mercury),
            (22.0, ClassicalPlanet::Jupiter),
            (28.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        2 => &[ // Gemini: Mer 6, Jup 6 (12), Ven 5 (17), Mar 7 (24), Sat 6 (30)
            (6.0, ClassicalPlanet::Mercury),
            (12.0, ClassicalPlanet::Jupiter),
            (17.0, ClassicalPlanet::Venus),
            (24.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        3 => &[ // Cancer: Mar 7, Ven 6 (13), Mer 6 (19), Jup 7 (26), Sat 4 (30)
            (7.0, ClassicalPlanet::Mars),
            (13.0, ClassicalPlanet::Venus),
            (19.0, ClassicalPlanet::Mercury),
            (26.0, ClassicalPlanet::Jupiter),
            (30.0, ClassicalPlanet::Saturn),
        ],
        4 => &[ // Leo: Jup 6, Ven 5 (11), Sat 7 (18), Mer 6 (24), Mar 6 (30)
            (6.0, ClassicalPlanet::Jupiter),
            (11.0, ClassicalPlanet::Venus),
            (18.0, ClassicalPlanet::Saturn),
            (24.0, ClassicalPlanet::Mercury),
            (30.0, ClassicalPlanet::Mars),
        ],
        5 => &[ // Virgo: Mer 7, Ven 10 (17), Jup 4 (21), Mar 7 (28), Sat 2 (30)
            (7.0, ClassicalPlanet::Mercury),
            (17.0, ClassicalPlanet::Venus),
            (21.0, ClassicalPlanet::Jupiter),
            (28.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        6 => &[ // Libra (Valens Book I Ch. 3 & IX Ch. 6): Sat 6, Mer 8 (14), Jup 8 (22), Ven 6 (28), Mar 2 (30)
            (6.0, ClassicalPlanet::Saturn),
            (14.0, ClassicalPlanet::Mercury),
            (22.0, ClassicalPlanet::Jupiter),
            (28.0, ClassicalPlanet::Venus),
            (30.0, ClassicalPlanet::Mars),
        ],
        7 => &[ // Scorpio: Mar 7, Ven 4 (11), Mer 8 (19), Jup 5 (24), Sat 6 (30)
            (7.0, ClassicalPlanet::Mars),
            (11.0, ClassicalPlanet::Venus),
            (19.0, ClassicalPlanet::Mercury),
            (24.0, ClassicalPlanet::Jupiter),
            (30.0, ClassicalPlanet::Saturn),
        ],
        8 => &[ // Sagittarius: Jup 12, Ven 5 (17), Mer 4 (21), Sat 5 (26), Mar 4 (30)
            (12.0, ClassicalPlanet::Jupiter),
            (17.0, ClassicalPlanet::Venus),
            (21.0, ClassicalPlanet::Mercury),
            (26.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        9 => &[ // Capricorn: Mer 7, Jup 7 (14), Ven 8 (22), Sat 4 (26), Mar 4 (30)
            (7.0, ClassicalPlanet::Mercury),
            (14.0, ClassicalPlanet::Jupiter),
            (22.0, ClassicalPlanet::Venus),
            (26.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        10 => &[ // Aquarius: Mer 7, Ven 6 (13), Jup 7 (20), Mar 5 (25), Sat 5 (30)
            (7.0, ClassicalPlanet::Mercury),
            (13.0, ClassicalPlanet::Venus),
            (20.0, ClassicalPlanet::Jupiter),
            (25.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        _ => &[ // Pisces: Ven 12, Jup 4 (16), Mer 3 (19), Mar 9 (28), Sat 2 (30)
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

/// Ptolemaic Terms: (Max degree, Planet) for each of the 12 signs according to William Lilly
/// (Christian Astrology, 1647, p. 104 "Table of the Essential Dignities of the Planets According to Ptolemy").
pub fn ptolemaic_term(sign_index: u8, degree_in_sign: f64) -> ClassicalPlanet {
    let terms: &[(f64, ClassicalPlanet)] = match sign_index % 12 {
        0 => &[ // Aries: Jup 6, Ven 14, Mer 21, Mar 26, Sat 30
            (6.0, ClassicalPlanet::Jupiter),
            (14.0, ClassicalPlanet::Venus),
            (21.0, ClassicalPlanet::Mercury),
            (26.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        1 => &[ // Taurus: Ven 8, Mer 15, Jup 22, Sat 26, Mar 30
            (8.0, ClassicalPlanet::Venus),
            (15.0, ClassicalPlanet::Mercury),
            (22.0, ClassicalPlanet::Jupiter),
            (26.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        2 => &[ // Gemini: Mer 7, Jup 14, Ven 21, Mar 25, Sat 30
            (7.0, ClassicalPlanet::Mercury),
            (14.0, ClassicalPlanet::Jupiter),
            (21.0, ClassicalPlanet::Venus),
            (25.0, ClassicalPlanet::Mars),
            (30.0, ClassicalPlanet::Saturn),
        ],
        3 => &[ // Cancer: Mer 6, Jup 13, Ven 20, Sat 27, Mar 30
            (6.0, ClassicalPlanet::Mercury),
            (13.0, ClassicalPlanet::Jupiter),
            (20.0, ClassicalPlanet::Venus),
            (27.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        4 => &[ // Leo: Jup 6, Ven 13, Sat 19, Mer 25, Mar 30
            (6.0, ClassicalPlanet::Jupiter),
            (13.0, ClassicalPlanet::Venus),
            (19.0, ClassicalPlanet::Saturn),
            (25.0, ClassicalPlanet::Mercury),
            (30.0, ClassicalPlanet::Mars),
        ],
        5 => &[ // Virgo: Mer 7, Ven 13, Jup 18, Sat 24, Mar 30
            (7.0, ClassicalPlanet::Mercury),
            (13.0, ClassicalPlanet::Venus),
            (18.0, ClassicalPlanet::Jupiter),
            (24.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        6 => &[ // Libra: Sat 6, Mer 11, Jup 19, Ven 24, Mar 30
            (6.0, ClassicalPlanet::Saturn),
            (11.0, ClassicalPlanet::Mercury),
            (19.0, ClassicalPlanet::Jupiter),
            (24.0, ClassicalPlanet::Venus),
            (30.0, ClassicalPlanet::Mars),
        ],
        7 => &[ // Scorpio: Mar 6, Jup 14, Ven 21, Sat 27, Mar 30
            (6.0, ClassicalPlanet::Mars),
            (14.0, ClassicalPlanet::Jupiter),
            (21.0, ClassicalPlanet::Venus),
            (27.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        8 => &[ // Sagittarius: Jup 8, Ven 14, Mer 19, Sat 25, Mar 30
            (8.0, ClassicalPlanet::Jupiter),
            (14.0, ClassicalPlanet::Venus),
            (19.0, ClassicalPlanet::Mercury),
            (25.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        9 => &[ // Capricorn: Ven 6, Mer 12, Jup 19, Sat 25, Mar 30
            (6.0, ClassicalPlanet::Venus),
            (12.0, ClassicalPlanet::Mercury),
            (19.0, ClassicalPlanet::Jupiter),
            (25.0, ClassicalPlanet::Saturn),
            (30.0, ClassicalPlanet::Mars),
        ],
        10 => &[ // Aquarius: Sat 6, Mer 12, Jup 20, Ven 25, Mar 30
            (6.0, ClassicalPlanet::Saturn),
            (12.0, ClassicalPlanet::Mercury),
            (20.0, ClassicalPlanet::Jupiter),
            (25.0, ClassicalPlanet::Venus),
            (30.0, ClassicalPlanet::Mars),
        ],
        _ => &[ // Pisces: Ven 8, Jup 14, Mer 20, Mar 26, Sat 30
            (8.0, ClassicalPlanet::Venus),
            (14.0, ClassicalPlanet::Jupiter),
            (20.0, ClassicalPlanet::Mercury),
            (26.0, ClassicalPlanet::Mars),
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

/// Calculate essential & accidental dignity scores for a planet according to William Lilly (CA p. 115).
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
    calculate_planet_dignity_with_context(
        planet,
        longitude,
        speed,
        house_number,
        sun_longitude,
        is_day_chart,
        north_node_lon,
        south_node_lon,
        &[],
        &[],
    )
}

/// Calculate complete William Lilly essential and accidental dignity scores (CA p. 115),
/// including partile benefic/malefic aspects, besiegement, and fixed star contacts (Regulus, Spica, Algol).
pub fn calculate_planet_dignity_with_context(
    planet: ClassicalPlanet,
    longitude: f64,
    speed: f64,
    house_number: u8,
    sun_longitude: f64,
    is_day_chart: bool,
    north_node_lon: f64,
    south_node_lon: f64,
    other_planets: &[(&str, f64)],
    fixed_star_conjunctions: &[super::fixed_stars::FixedStarConjunction],
) -> PlanetDignityReport {
    let norm_lon = longitude.rem_euclid(360.0);
    let sign_index = (norm_lon / 30.0).floor() as u8 % 12;
    let sign_degree = norm_lon - (sign_index as f64 * 30.0);
    let opp_sign = (sign_index + 6) % 12;

    let dignities = evaluate_essential_dignities(norm_lon, is_day_chart);

    let is_in_domicile = dignities.domicile == planet;
    let is_in_exaltation = dignities.exaltation == Some(planet);
    let is_in_triplicity = dignities.triplicity == planet;
    let is_in_term = dignities.term == planet;
    let is_in_face = dignities.face == planet;

    let is_in_detriment = DOMICILE_RULERS[opp_sign as usize] == planet;
    let is_in_fall = EXALTATIONS.iter().find(|(s, _, _)| *s == opp_sign).map(|(_, p, _)| *p) == Some(planet);

    // In classical astrology (Lilly CA p. 112, p. 115, p. 139), a planet is Peregrine when it has NO
    // positive essential dignity (no domicile, exaltation, triplicity, term, nor face), even if in detriment or fall.
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

    // 1. House placement
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

    // 2. Motion: Direct / Retrograde
    if planet != ClassicalPlanet::Sun && planet != ClassicalPlanet::Moon {
        if speed < 0.0 {
            accidental_score -= 5;
            factors.push("Retrograde (-5)".into());
        } else {
            accidental_score += 4;
            factors.push("Direct in Motion (+4)".into());
        }
    }

    // 3. Speed: Swift / Slow
    let avg_speed = planet.average_daily_motion();
    if speed.abs() > avg_speed {
        accidental_score += 2;
        factors.push("Swift in Motion (+2)".into());
    } else {
        accidental_score -= 2;
        factors.push("Slow in Motion (-2)".into());
    }

    // 4. Oriental / Occidental of the Sun & Moon Increasing/Decreasing in Light (CA p. 115)
    let norm_sun = sun_longitude.rem_euclid(360.0);
    let sun_sign_index = (norm_sun / 30.0).floor() as u8 % 12;
    let elong_from_sun = (norm_lon - norm_sun).rem_euclid(360.0);

    match planet {
        ClassicalPlanet::Mars | ClassicalPlanet::Jupiter | ClassicalPlanet::Saturn => {
            // Superior planets: Oriental (rising before Sun, elong > 180°) = +2, Occidental = -2
            if elong_from_sun > 180.0 {
                accidental_score += 2;
                factors.push("Oriental of the Sun (+2)".into());
            } else if elong_from_sun > 1e-6 {
                accidental_score -= 2;
                factors.push("Occidental of the Sun (-2)".into());
            }
        }
        ClassicalPlanet::Mercury | ClassicalPlanet::Venus => {
            // Inferior planets: Occidental (setting after Sun, elong < 180°) = +2, Oriental = -2
            if elong_from_sun > 1e-6 && elong_from_sun < 180.0 {
                accidental_score += 2;
                factors.push("Occidental of the Sun (+2)".into());
            } else if elong_from_sun > 180.0 {
                accidental_score -= 2;
                factors.push("Oriental of the Sun (-2)".into());
            }
        }
        ClassicalPlanet::Moon => {
            // Moon Waxing (Increasing in Light, elong < 180°) = +2, Waning (Decreasing in Light) = -2
            if elong_from_sun < 180.0 {
                accidental_score += 2;
                factors.push("Increasing in Light (+2)".into());
            } else {
                accidental_score -= 2;
                factors.push("Decreasing in Light (-2)".into());
            }
        }
        ClassicalPlanet::Sun => {}
    }

    // 5. Relation to Sun: Cazimi / Combust (Same Sign only, -6) / Under the Sunbeams (-4) / Free (+5)
    // Lilly CA p. 115 & p. 228: Combustion requires personal conjunction in the same sign within 8°30'.
    if planet != ClassicalPlanet::Sun {
        let diff = (norm_lon - norm_sun).abs();
        let sep = if diff > 180.0 { 360.0 - diff } else { diff };
        let same_sign_as_sun = sign_index == sun_sign_index;

        if sep <= 17.0 / 60.0 && same_sign_as_sun {
            accidental_score += 5;
            factors.push("Cazimi / Heart of the Sun (+5)".into());
        } else if sep <= 8.5 && same_sign_as_sun {
            accidental_score -= 6;
            factors.push("Combust in Same Sign (-6)".into());
        } else if sep <= 17.0 {
            accidental_score -= 4;
            factors.push("Under the Sunbeams (-4)".into());
        } else {
            accidental_score += 5;
            factors.push("Free from Sun's Beams (+5)".into());
        }
    }

    // 6. Lunar Nodes Conjunction (orb <= 1.5°)
    let diff_nn = (norm_lon - north_node_lon.rem_euclid(360.0)).abs();
    let sep_nn = if diff_nn > 180.0 { 360.0 - diff_nn } else { diff_nn };
    if sep_nn <= 1.5 {
        accidental_score += 4;
        factors.push("Conjunct North Node (+4)".into());
    }

    let diff_sn = (norm_lon - south_node_lon.rem_euclid(360.0)).abs();
    let sep_sn = if diff_sn > 180.0 { 360.0 - diff_sn } else { diff_sn };
    if sep_sn <= 1.5 {
        accidental_score -= 4;
        factors.push("Conjunct South Node (-4)".into());
    }

    // 7. Partile Aspects with Benefics (Jupiter, Venus) & Malefics (Mars, Saturn) + Besiegement (CA p. 115)
    let deg_int = sign_degree.floor() as i32;
    let mut mars_lon_opt: Option<f64> = None;
    let mut saturn_lon_opt: Option<f64> = None;

    for &(other_name, other_lon) in other_planets {
        let Some(other_cp) = ClassicalPlanet::from_name(other_name) else { continue };
        let norm_other = other_lon.rem_euclid(360.0);
        if other_cp == ClassicalPlanet::Mars {
            mars_lon_opt = Some(norm_other);
        } else if other_cp == ClassicalPlanet::Saturn {
            saturn_lon_opt = Some(norm_other);
        }
        if other_cp == planet {
            continue;
        }

        let other_sign = (norm_other / 30.0).floor() as u8 % 12;
        let other_deg = norm_other - (other_sign as f64 * 30.0);
        let other_deg_int = other_deg.floor() as i32;
        let is_partile = deg_int == other_deg_int || (sign_degree - other_deg).abs() <= 1.0;
        if !is_partile {
            continue;
        }

        let sign_diff = (sign_index as i16 - other_sign as i16).rem_euclid(12) as u8;
        match other_cp {
            ClassicalPlanet::Jupiter | ClassicalPlanet::Venus => {
                match sign_diff {
                    0 => {
                        accidental_score += 5;
                        factors.push(format!("Partile Conjunction with {} (+5)", other_cp.name()));
                    }
                    4 | 8 => {
                        accidental_score += 4;
                        factors.push(format!("Partile Trine with {} (+4)", other_cp.name()));
                    }
                    2 | 10 => {
                        accidental_score += 3;
                        factors.push(format!("Partile Sextile with {} (+3)", other_cp.name()));
                    }
                    _ => {}
                }
            }
            ClassicalPlanet::Mars | ClassicalPlanet::Saturn => {
                match sign_diff {
                    0 => {
                        accidental_score -= 5;
                        factors.push(format!("Partile Conjunction with {} (-5)", other_cp.name()));
                    }
                    6 => {
                        accidental_score -= 4;
                        factors.push(format!("Partile Opposition with {} (-4)", other_cp.name()));
                    }
                    3 | 9 => {
                        accidental_score -= 3;
                        factors.push(format!("Partile Square with {} (-3)", other_cp.name()));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // Besiegement between Mars and Saturn (-5)
    if planet != ClassicalPlanet::Mars && planet != ClassicalPlanet::Saturn
        && let (Some(m_lon), Some(s_lon)) = (mars_lon_opt, saturn_lon_opt)
    {
        let diff_ms = (m_lon - s_lon).abs();
        let span_ms = if diff_ms > 180.0 { 360.0 - diff_ms } else { diff_ms };
        if span_ms <= 17.0 {
            let arc_m_to_s = (s_lon - m_lon).rem_euclid(360.0);
            let is_between = if arc_m_to_s <= 180.0 {
                let d = (norm_lon - m_lon).rem_euclid(360.0);
                d > 0.001 && d < arc_m_to_s - 0.001
            } else {
                let rev = (m_lon - s_lon).rem_euclid(360.0);
                let d = (norm_lon - s_lon).rem_euclid(360.0);
                d > 0.001 && d < rev - 0.001
            };
            if is_between {
                accidental_score -= 5;
                factors.push("Besieged by Mars and Saturn (-5)".into());
            }
        }
    }

    // 8. Fixed Stars (CA p. 115: Regulus +6, Spica +5, Caput Algol -5 within 1.0° orb)
    for sc in fixed_star_conjunctions {
        if sc.body_name == planet.name() && sc.orb <= 1.0 {
            if sc.star_name.starts_with("Regulus") {
                accidental_score += 6;
                factors.push("Conjunct Regulus (+6)".into());
            } else if sc.star_name.starts_with("Spica") {
                accidental_score += 5;
                factors.push("Conjunct Spica (+5)".into());
            } else if sc.star_name.starts_with("Algol") {
                accidental_score -= 5;
                factors.push("Conjunct Caput Algol (-5)".into());
            }
        }
    }

    // 9. Joy of planets in houses
    let is_joy = matches!(
        (planet, house_number),
        (ClassicalPlanet::Mercury, 1)
            | (ClassicalPlanet::Moon, 3)
            | (ClassicalPlanet::Venus, 5)
            | (ClassicalPlanet::Mars, 6)
            | (ClassicalPlanet::Sun, 9)
            | (ClassicalPlanet::Jupiter, 11)
            | (ClassicalPlanet::Saturn, 12)
    );
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
        // Aries (0): 13° -> Egyptian: Mercury (12-20°), Ptolemaic: Venus (6-14°)
        assert_eq!(egyptian_term(0, 13.0), ClassicalPlanet::Mercury);
        assert_eq!(ptolemaic_term(0, 13.0), ClassicalPlanet::Venus);

        // Taurus (1): 27.5° -> Egyptian (Valens IX.6): Saturn (22-28°), Ptolemaic (CA p. 104): Mars (26-30°)
        assert_eq!(egyptian_term(1, 27.5), ClassicalPlanet::Saturn);
        assert_eq!(ptolemaic_term(1, 27.5), ClassicalPlanet::Mars);

        // Cancer (3): 3° -> Egyptian: Mars (0-7°), Ptolemaic: Mercury (0-6°)
        assert_eq!(egyptian_term(3, 3.0), ClassicalPlanet::Mars);
        assert_eq!(ptolemaic_term(3, 3.0), ClassicalPlanet::Mercury);

        // Libra (6): 12.0° -> Egyptian: Mercury (6-14°), Ptolemaic (CA p. 104): Jupiter (11-19°)
        assert_eq!(egyptian_term(6, 12.0), ClassicalPlanet::Mercury);
        assert_eq!(ptolemaic_term(6, 12.0), ClassicalPlanet::Jupiter);

        // Libra (6): 23.0° -> Egyptian: Venus (22-28°), Ptolemaic (CA p. 104): Venus (19-24°)
        assert_eq!(egyptian_term(6, 23.0), ClassicalPlanet::Venus);
        assert_eq!(ptolemaic_term(6, 23.0), ClassicalPlanet::Venus);

        // Capricorn (9): 4.0° -> Egyptian: Mercury (0-7°), Ptolemaic: Venus (0-6°)
        assert_eq!(egyptian_term(9, 4.0), ClassicalPlanet::Mercury);
        assert_eq!(ptolemaic_term(9, 4.0), ClassicalPlanet::Venus);
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

    #[test]
    fn test_combust_same_sign_vs_out_of_sign() {
        // Sun at 29° Aries (29.0°).
        // Case A: Mercury at 25° Aries (25.0°) -> separation 4° in SAME sign -> Combust (-6)
        let rep_same = calculate_planet_dignity(
            ClassicalPlanet::Mercury,
            25.0,
            1.2,
            1,
            29.0,
            true,
            120.0,
            300.0,
        );
        assert!(rep_same.accidental_factors.iter().any(|f| f.contains("Combust in Same Sign (-6)")));

        // Case B: Mercury at 2° Taurus (32.0°) -> separation 3° across sign boundary -> Under the Sunbeams (-4), NOT Combust
        let rep_diff = calculate_planet_dignity(
            ClassicalPlanet::Mercury,
            32.0,
            1.2,
            1,
            29.0,
            true,
            120.0,
            300.0,
        );
        assert!(rep_diff.accidental_factors.iter().any(|f| f.contains("Under the Sunbeams (-4)")));
        assert!(!rep_diff.accidental_factors.iter().any(|f| f.contains("Combust")));
    }
}
