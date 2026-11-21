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
            Self::Sun | Self::Mercury | Self::Venus => 59.0 / 60.0 + 8.0 / 3600.0,
            Self::Moon => 13.0 + 10.0 / 60.0 + 36.0 / 3600.0,
            Self::Mars => 31.0 / 60.0 + 27.0 / 3600.0,
            Self::Jupiter => 4.0 / 60.0 + 59.0 / 3600.0,
            Self::Saturn => 2.0 / 60.0,
        }
    }

    /// Lilly's planetary moiety of orb (half orb in degrees).
    pub fn moiety(self) -> f64 {
        match self {
            Self::Sun => 7.5,
            Self::Moon => 6.0,
            Self::Mercury | Self::Venus => 3.5,
            Self::Mars => 4.0,
            Self::Jupiter | Self::Saturn => 4.5,
        }
    }

    /// Traditional Chaldean speed order (0 = Moon fastest .. 6 = Saturn slowest).
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

pub const DOMICILE_RULERS: [ClassicalPlanet; 12] = [
    ClassicalPlanet::Mars, ClassicalPlanet::Venus, ClassicalPlanet::Mercury, ClassicalPlanet::Moon,
    ClassicalPlanet::Sun, ClassicalPlanet::Mercury, ClassicalPlanet::Venus, ClassicalPlanet::Mars,
    ClassicalPlanet::Jupiter, ClassicalPlanet::Saturn, ClassicalPlanet::Saturn, ClassicalPlanet::Jupiter,
];

pub const EXALTATIONS: [(u8, ClassicalPlanet, f64); 7] = [
    (0, ClassicalPlanet::Sun, 19.0),
    (1, ClassicalPlanet::Moon, 3.0),
    (3, ClassicalPlanet::Jupiter, 15.0),
    (5, ClassicalPlanet::Mercury, 15.0),
    (6, ClassicalPlanet::Saturn, 21.0),
    (9, ClassicalPlanet::Mars, 28.0),
    (11, ClassicalPlanet::Venus, 27.0),
];

/// Ptolemy / William Lilly triplicity rulers: (Day Ruler, Night Ruler).
pub fn triplicity_rulers(sign_index: u8) -> (ClassicalPlanet, ClassicalPlanet) {
    match sign_index % 4 {
        0 => (ClassicalPlanet::Sun, ClassicalPlanet::Jupiter),
        1 => (ClassicalPlanet::Venus, ClassicalPlanet::Moon),
        2 => (ClassicalPlanet::Saturn, ClassicalPlanet::Mercury),
        _ => (ClassicalPlanet::Mars, ClassicalPlanet::Mars),
    }
}

/// Dorothean / Hellenistic triplicity rulers (Valens): (Day, Night, Participating).
pub fn dorothean_triplicity_rulers(sign_index: u8) -> (ClassicalPlanet, ClassicalPlanet, ClassicalPlanet) {
    match sign_index % 4 {
        0 => (ClassicalPlanet::Sun, ClassicalPlanet::Jupiter, ClassicalPlanet::Saturn),
        1 => (ClassicalPlanet::Venus, ClassicalPlanet::Moon, ClassicalPlanet::Mars),
        2 => (ClassicalPlanet::Saturn, ClassicalPlanet::Mercury, ClassicalPlanet::Jupiter),
        _ => (ClassicalPlanet::Venus, ClassicalPlanet::Mars, ClassicalPlanet::Moon),
    }
}

const EGYPTIAN_TERMS: [[(f64, ClassicalPlanet); 5]; 12] = {
    use ClassicalPlanet::*;
    [
        [(6.0, Jupiter), (12.0, Venus), (20.0, Mercury), (25.0, Mars), (30.0, Saturn)],
        [(8.0, Venus), (14.0, Mercury), (22.0, Jupiter), (28.0, Saturn), (30.0, Mars)],
        [(6.0, Mercury), (12.0, Jupiter), (17.0, Venus), (24.0, Mars), (30.0, Saturn)],
        [(7.0, Mars), (13.0, Venus), (19.0, Mercury), (26.0, Jupiter), (30.0, Saturn)],
        [(6.0, Jupiter), (11.0, Venus), (18.0, Saturn), (24.0, Mercury), (30.0, Mars)],
        [(7.0, Mercury), (17.0, Venus), (21.0, Jupiter), (28.0, Mars), (30.0, Saturn)],
        [(6.0, Saturn), (14.0, Mercury), (22.0, Jupiter), (28.0, Venus), (30.0, Mars)],
        [(7.0, Mars), (11.0, Venus), (19.0, Mercury), (24.0, Jupiter), (30.0, Saturn)],
        [(12.0, Jupiter), (17.0, Venus), (21.0, Mercury), (26.0, Saturn), (30.0, Mars)],
        [(7.0, Mercury), (14.0, Jupiter), (22.0, Venus), (26.0, Saturn), (30.0, Mars)],
        [(7.0, Mercury), (13.0, Venus), (20.0, Jupiter), (25.0, Mars), (30.0, Saturn)],
        [(12.0, Venus), (16.0, Jupiter), (19.0, Mercury), (28.0, Mars), (30.0, Saturn)],
    ]
};

const PTOLEMAIC_TERMS: [[(f64, ClassicalPlanet); 5]; 12] = {
    use ClassicalPlanet::*;
    [
        [(6.0, Jupiter), (14.0, Venus), (21.0, Mercury), (26.0, Mars), (30.0, Saturn)],
        [(8.0, Venus), (15.0, Mercury), (22.0, Jupiter), (26.0, Saturn), (30.0, Mars)],
        [(7.0, Mercury), (14.0, Jupiter), (21.0, Venus), (25.0, Mars), (30.0, Saturn)],
        [(6.0, Mercury), (13.0, Jupiter), (20.0, Venus), (27.0, Saturn), (30.0, Mars)],
        [(6.0, Jupiter), (13.0, Venus), (19.0, Saturn), (25.0, Mercury), (30.0, Mars)],
        [(7.0, Mercury), (13.0, Venus), (18.0, Jupiter), (24.0, Saturn), (30.0, Mars)],
        [(6.0, Saturn), (11.0, Mercury), (19.0, Jupiter), (24.0, Venus), (30.0, Mars)],
        [(6.0, Mars), (14.0, Jupiter), (21.0, Venus), (27.0, Saturn), (30.0, Mars)],
        [(8.0, Jupiter), (14.0, Venus), (19.0, Mercury), (25.0, Saturn), (30.0, Mars)],
        [(6.0, Venus), (12.0, Mercury), (19.0, Jupiter), (25.0, Saturn), (30.0, Mars)],
        [(6.0, Saturn), (12.0, Mercury), (20.0, Jupiter), (25.0, Venus), (30.0, Mars)],
        [(8.0, Venus), (14.0, Jupiter), (20.0, Mercury), (26.0, Mars), (30.0, Saturn)],
    ]
};

fn lookup_term(table: &[[(f64, ClassicalPlanet); 5]; 12], sign_index: u8, degree_in_sign: f64) -> ClassicalPlanet {
    let deg = degree_in_sign.clamp(0.0, 29.999_999);
    for &(limit, planet) in &table[(sign_index % 12) as usize] {
        if deg < limit {
            return planet;
        }
    }
    table[(sign_index % 12) as usize][4].1
}

/// Egyptian Terms / Bounds (Vettius Valens Book I Ch. 3 & Book IX Ch. 6).
pub fn egyptian_term(sign_index: u8, degree_in_sign: f64) -> ClassicalPlanet {
    lookup_term(&EGYPTIAN_TERMS, sign_index, degree_in_sign)
}

/// Ptolemaic Terms (William Lilly, Christian Astrology 1647, p. 104).
pub fn ptolemaic_term(sign_index: u8, degree_in_sign: f64) -> ClassicalPlanet {
    lookup_term(&PTOLEMAIC_TERMS, sign_index, degree_in_sign)
}

/// Chaldean Decans / Faces: 10 degrees each, cycling through Chaldean order.
pub fn chaldean_face(sign_index: u8, degree_in_sign: f64) -> ClassicalPlanet {
    const CHALDEAN_ORDER: [ClassicalPlanet; 7] = [
        ClassicalPlanet::Mars, ClassicalPlanet::Sun, ClassicalPlanet::Venus,
        ClassicalPlanet::Mercury, ClassicalPlanet::Moon, ClassicalPlanet::Saturn,
        ClassicalPlanet::Jupiter,
    ];
    let decan_index = (degree_in_sign / 10.0).floor().clamp(0.0, 2.0) as usize;
    CHALDEAN_ORDER[((sign_index % 12) as usize * 3 + decan_index) % 7]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssentialDignities {
    pub domicile: ClassicalPlanet,
    pub exaltation: Option<ClassicalPlanet>,
    pub triplicity: ClassicalPlanet,
    pub term: ClassicalPlanet,
    pub face: ClassicalPlanet,
}

pub fn evaluate_essential_dignities(longitude: f64, is_day_chart: bool) -> EssentialDignities {
    let norm = longitude.rem_euclid(360.0);
    let sign_index = (norm / 30.0).floor() as u8 % 12;
    let degree_in_sign = norm - (sign_index as f64 * 30.0);
    let (day_trip, night_trip) = triplicity_rulers(sign_index);

    EssentialDignities {
        domicile: DOMICILE_RULERS[sign_index as usize],
        exaltation: EXALTATIONS.iter().find(|(s, _, _)| *s == sign_index).map(|(_, p, _)| *p),
        triplicity: if is_day_chart { day_trip } else { night_trip },
        term: ptolemaic_term(sign_index, degree_in_sign),
        face: chaldean_face(sign_index, degree_in_sign),
    }
}
