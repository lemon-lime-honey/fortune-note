//! Vedic astrology (Jyotish) calculation layer (Sidereal Zodiac).
//!
//! Provides deterministic computation for:
//! - Sidereal Lagna (Ascendant) and 12 Whole Sign Bhavas
//! - 19 divisional charts (Shodashavarga + D6, D8, D11) with varga lagnas and planetary houses
//! - 27 Nakshatras with 4 Padas
//! - Classical Parashari planetary dignities (Panchadha Maitri, Exaltation, Moolatrikona, Own Sign, Debilitation) and Combustion
//! - Jaimini 8 Chara Karakas, BPHS Graha Drishti, 12 Arudha Padas, Ashtakavarga (Bhinna & Sarva), and 5-limb Panchanga
//! - Vimshottari Dasha hierarchy (Maha and Antar periods)

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use vedaksha::astro::houses::{compute_houses, HouseSystem};
use vedaksha::math::angle::normalize_degrees;
use vedaksha::vedic::arudha::{arudha_pada, ArudhaException};
use vedaksha::vedic::ashtakavarga::{
    bhinna_ashtakavarga, sarvashtakavarga, BhinnaAshtakavargaInput,
};
use vedaksha::vedic::combustion::{combustion_state, CombustionState};
use vedaksha::vedic::dasha::vimshottari::{compute_vimshottari, DashaPeriod};
use vedaksha::vedic::graha::Graha;
use vedaksha::vedic::nakshatra::Nakshatra;
use vedaksha::vedic::panchanga::{compute_karana, compute_panchanga_yoga};

use crate::ephemeris::EphemerisSnapshot;

const RASHI_NAMES: [&str; 12] = [
    "Mesha (Aries)",
    "Vrishabha (Taurus)",
    "Mithuna (Gemini)",
    "Karka (Cancer)",
    "Simha (Leo)",
    "Kanya (Virgo)",
    "Tula (Libra)",
    "Vrishchika (Scorpio)",
    "Dhanu (Sagittarius)",
    "Makara (Capricorn)",
    "Kumbha (Aquarius)",
    "Meena (Pisces)",
];

const TITHI_NAMES: [&str; 15] = [
    "Pratipada",
    "Dwitiya",
    "Tritiya",
    "Chaturthi",
    "Panchami",
    "Shashthi",
    "Saptami",
    "Ashtami",
    "Navami",
    "Dashami",
    "Ekadashi",
    "Dwadashi",
    "Trayodashi",
    "Chaturdashi",
    "Purnima / Amavasya",
];

/// Supported divisional charts in Fortune Note (16 BPHS Shodashavarga + D6, D8, D11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VargaDivision {
    D1,
    D2,
    D3,
    D4,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D16,
    D20,
    D24,
    D27,
    D30,
    D40,
    D45,
    D60,
}

impl VargaDivision {
    pub const ALL: [VargaDivision; 19] = [
        Self::D1,
        Self::D2,
        Self::D3,
        Self::D4,
        Self::D6,
        Self::D7,
        Self::D8,
        Self::D9,
        Self::D10,
        Self::D11,
        Self::D12,
        Self::D16,
        Self::D20,
        Self::D24,
        Self::D27,
        Self::D30,
        Self::D40,
        Self::D45,
        Self::D60,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Self::D1 => "Rasi (D1)",
            Self::D2 => "Hora (D2)",
            Self::D3 => "Drekkana (D3)",
            Self::D4 => "Chaturthamsha (D4)",
            Self::D6 => "Shashthamsha (D6)",
            Self::D7 => "Saptamsha (D7)",
            Self::D8 => "Ashtamamsha (D8)",
            Self::D9 => "Navamsha (D9)",
            Self::D10 => "Dashamsha (D10)",
            Self::D11 => "Rudramsha (D11)",
            Self::D12 => "Dwadashamsha (D12)",
            Self::D16 => "Shodashamsha (D16)",
            Self::D20 => "Vimshamsha (D20)",
            Self::D24 => "Chaturvimshamsha (D24)",
            Self::D27 => "Saptavimshamsha (D27)",
            Self::D30 => "Trimshamsha (D30)",
            Self::D40 => "Khavedamsha (D40)",
            Self::D45 => "Akshavedamsha (D45)",
            Self::D60 => "Shashtiamsha (D60)",
        }
    }
}

/// Computes the 0-based part index (`0..n-1`) and normalized degree (`0.0..30.0`)
/// for an equal `n`-fold division of a 30-degree sign without double-division float underflow.
#[inline]
fn uniform_varga_part_and_degree(pos_in_sign: f64, n: u8) -> (u8, f64) {
    let clamped = pos_in_sign.clamp(0.0, 30.0);
    let scaled = clamped * (n as f64);
    let raw_part = scaled / 30.0;
    let rounded = raw_part.round();
    let snapped_part = if (raw_part - rounded).abs() < 1e-9 {
        rounded
    } else {
        raw_part.floor()
    };
    let part = (snapped_part.max(0.0) as u8).min(n.saturating_sub(1));
    let mut degree = scaled - (part as f64 * 30.0);
    if degree.abs() < 1e-9 {
        degree = 0.0;
    }
    (part, degree.clamp(0.0, 30.0))
}

/// Computes the divisional sign index (0..11) and normalized degree in that sign (0.0..30.0)
/// for any supported varga division.
pub fn calculate_varga_sign_and_degree(sidereal_lon: f64, div: VargaDivision) -> (u8, f64) {
    let norm = normalize_degrees(sidereal_lon);
    let raw_sign = norm / 30.0;
    let rounded_sign = raw_sign.round();
    let snapped_sign = if (raw_sign - rounded_sign).abs() < 1e-11 && rounded_sign < 12.0 {
        rounded_sign
    } else {
        raw_sign.floor()
    };
    let sign_idx = (snapped_sign.max(0.0) as u8).min(11);
    let mut pos_in_sign = (norm - (sign_idx as f64 * 30.0)).clamp(0.0, 30.0);
    if pos_in_sign.abs() < 1e-11 {
        pos_in_sign = 0.0;
    }

    match div {
        VargaDivision::D1 => (sign_idx, pos_in_sign),
        VargaDivision::D2 => {
            // Parashari Hora (BPHS Ch. 6):
            // In odd signs (sign_idx % 2 == 0): 1st half -> Leo (4), 2nd half -> Cancer (3)
            // In even signs (sign_idx % 2 != 0): 1st half -> Cancer (3), 2nd half -> Leo (4)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 2);
            let is_first_half = part == 0;
            let is_odd_sign = sign_idx.is_multiple_of(2);
            let sign = match (is_odd_sign, is_first_half) {
                (true, true) => 4,   // Odd 1st half: Leo
                (true, false) => 3,  // Odd 2nd half: Cancer
                (false, true) => 3,  // Even 1st half: Cancer
                (false, false) => 4, // Even 2nd half: Leo
            };
            (sign, degree)
        }
        VargaDivision::D3 => {
            // Parashari Drekkana (BPHS Ch. 6): 10-degree divisions (0..2)
            // 1st part: same sign; 2nd part: 5th sign (+4); 3rd part: 9th sign (+8)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 3);
            let offset = match part {
                0 => 0,
                1 => 4,
                _ => 8,
            };
            ((sign_idx + offset) % 12, degree)
        }
        VargaDivision::D4 => {
            // Parashari Chaturthamsha (BPHS Ch. 6): 7.5-degree divisions (0..3)
            // 1st part: same sign; 2nd part: 4th sign (+3); 3rd part: 7th sign (+6); 4th part: 10th sign (+9)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 4);
            let offset = part * 3;
            ((sign_idx + offset) % 12, degree)
        }
        VargaDivision::D6 => {
            // Parashari Shashthamsha: 5-degree divisions (0..5)
            // Odd signs start from Aries (0); Even signs start from Libra (6)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 6);
            let start = if sign_idx.is_multiple_of(2) { 0 } else { 6 };
            ((start + part) % 12, degree)
        }
        VargaDivision::D7 => {
            // Parashari Saptamsha (BPHS Ch. 6): (30/7)-degree divisions (0..6)
            // Odd signs start from sign itself; Even signs start from 7th sign (+6)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 7);
            let start = if sign_idx.is_multiple_of(2) {
                sign_idx
            } else {
                (sign_idx + 6) % 12
            };
            ((start + part) % 12, degree)
        }
        VargaDivision::D8 => {
            // Jaimini Ashtamamsha: 3.75-degree divisions (0..7)
            // Movable start Aries (0), Fixed start Sagittarius (8), Dual start Leo (4)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 8);
            let start = match sign_idx % 3 {
                0 => 0, // Movable: Aries
                1 => 8, // Fixed: Sagittarius
                _ => 4, // Dual: Leo
            };
            ((start + part) % 12, degree)
        }
        VargaDivision::D9 => {
            // Parashari Navamsha (BPHS Ch. 6): (30/9 = 3°20') divisions (0..8)
            // Movable start from sign itself, Fixed from 9th (+8), Dual from 5th (+4)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 9);
            let start = match sign_idx % 3 {
                0 => sign_idx,
                1 => (sign_idx + 8) % 12,
                _ => (sign_idx + 4) % 12,
            };
            ((start + part) % 12, degree)
        }
        VargaDivision::D10 => {
            // Parashari Dashamsha (BPHS Ch. 6): 3-degree divisions (0..9)
            // Odd signs start from sign itself; Even signs start from 9th sign (+8)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 10);
            let start = if sign_idx.is_multiple_of(2) {
                sign_idx
            } else {
                (sign_idx + 8) % 12
            };
            ((start + part) % 12, degree)
        }
        VargaDivision::D11 => {
            // Jaimini Rudramsha: (30/11)-degree divisions (0..10)
            // Start sign = count backward from Aries by sign position, then count forward by part
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 11);
            let start = (12 - sign_idx) % 12;
            ((start + part) % 12, degree)
        }
        VargaDivision::D12 => {
            // Parashari Dwadashamsha (BPHS Ch. 6): 2.5-degree divisions (0..11)
            // Count from sign itself through 12 signs
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 12);
            ((sign_idx + part) % 12, degree)
        }
        VargaDivision::D16 => {
            // Parashari Shodashamsha / Kalamsa (BPHS Ch. 6): 1°52'30" divisions (0..15)
            // Movable start Aries (0), Fixed start Leo (4), Dual start Sagittarius (8)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 16);
            let start = match sign_idx % 3 {
                0 => 0, // Movable: Aries
                1 => 4, // Fixed: Leo
                _ => 8, // Dual: Sagittarius
            };
            ((start + part) % 12, degree)
        }
        VargaDivision::D20 => {
            // Parashari Vimshamsha (BPHS Ch. 6): 1°30' divisions (0..19)
            // Movable start Aries (0), Fixed start Sagittarius (8), Dual start Leo (4)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 20);
            let start = match sign_idx % 3 {
                0 => 0, // Movable: Aries
                1 => 8, // Fixed: Sagittarius
                _ => 4, // Dual: Leo
            };
            ((start + part) % 12, degree)
        }
        VargaDivision::D24 => {
            // Parashari Chaturvimshamsha / Siddhamsha (BPHS Ch. 6): 1°15' divisions (0..23)
            // Odd signs start from Leo (4); Even signs start from Cancer (3), both counting forward
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 24);
            let start = if sign_idx.is_multiple_of(2) { 4 } else { 3 };
            ((start + part) % 12, degree)
        }
        VargaDivision::D27 => {
            // Parashari Saptavimshamsha / Bhamsha (BPHS Ch. 6): 1°06'40" divisions (0..26)
            // Fire signs start Aries (0), Earth start Cancer (3), Air start Libra (6), Water start Capricorn (9)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 27);
            let start = match sign_idx % 4 {
                0 => 0, // Fire: Aries
                1 => 3, // Earth: Cancer
                2 => 6, // Air: Libra
                _ => 9, // Water: Capricorn
            };
            ((start + part) % 12, degree)
        }
        VargaDivision::D30 => {
            // Parashari Trimshamsha (BPHS Ch. 6): unequal 5-part planetary lordship segments
            // Odd signs: 0..5 Mars (Aries=0), 5..10 Saturn (Aquarius=10), 10..18 Jupiter (Sagittarius=8),
            //            18..25 Mercury (Gemini=2), 25..30 Venus (Taurus=1)
            // Even signs: 0..5 Venus (Libra=6), 5..12 Mercury (Virgo=5), 12..20 Jupiter (Pisces=11),
            //             20..25 Saturn (Capricorn=9), 25..30 Mars (Scorpio=7)
            let (sign, seg_start, seg_span) = if sign_idx.is_multiple_of(2) {
                if pos_in_sign < 5.0 - 1e-9 {
                    (0, 0.0, 5.0)
                } else if pos_in_sign < 10.0 - 1e-9 {
                    (10, 5.0, 5.0)
                } else if pos_in_sign < 18.0 - 1e-9 {
                    (8, 10.0, 8.0)
                } else if pos_in_sign < 25.0 - 1e-9 {
                    (2, 18.0, 7.0)
                } else {
                    (1, 25.0, 5.0)
                }
            } else if pos_in_sign < 5.0 - 1e-9 {
                (6, 0.0, 5.0)
            } else if pos_in_sign < 12.0 - 1e-9 {
                (5, 5.0, 7.0)
            } else if pos_in_sign < 20.0 - 1e-9 {
                (11, 12.0, 8.0)
            } else if pos_in_sign < 25.0 - 1e-9 {
                (9, 20.0, 5.0)
            } else {
                (7, 25.0, 5.0)
            };
            let mut degree = ((pos_in_sign - seg_start).max(0.0) / seg_span) * 30.0;
            if degree.abs() < 1e-9 {
                degree = 0.0;
            }
            (sign, degree.clamp(0.0, 30.0))
        }
        VargaDivision::D40 => {
            // Parashari Khavedamsha (BPHS Ch. 6): 0°45' divisions (0..39)
            // Odd signs start from Aries (0); Even signs start from Libra (6)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 40);
            let start = if sign_idx.is_multiple_of(2) { 0 } else { 6 };
            ((start + part) % 12, degree)
        }
        VargaDivision::D45 => {
            // Parashari Akshavedamsha (BPHS Ch. 6): 0°40' divisions (0..44)
            // Movable start Aries (0), Fixed start Leo (4), Dual start Sagittarius (8)
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 45);
            let start = match sign_idx % 3 {
                0 => 0, // Movable: Aries
                1 => 4, // Fixed: Leo
                _ => 8, // Dual: Sagittarius
            };
            ((start + part) % 12, degree)
        }
        VargaDivision::D60 => {
            // Parashari Shashtiamsha (BPHS Ch. 6): 0°30' divisions (0..59)
            // Count from the sign itself for both odd and even signs
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, 60);
            ((sign_idx + part) % 12, degree)
        }
    }
}

/// Computes the divisional sign index (0..11) for any supported varga division.
pub fn calculate_varga_sign(sidereal_lon: f64, div: VargaDivision) -> u8 {
    calculate_varga_sign_and_degree(sidereal_lon, div).0
}

/// Planet placement in the sidereal framework.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicPlanet {
    pub name: String,
    pub sidereal_longitude: f64,
    pub sign_index: u8,     // 0 = Mesha (Aries) .. 11 = Meena (Pisces)
    pub sign_degree: f64,   // 0.0 .. 30.0
    pub nakshatra_name: String,
    pub nakshatra_pada: u8, // 1 .. 4
    pub is_retrograde: bool,
    #[serde(default)]
    pub latitude: f64,
    #[serde(default)]
    pub house: u8,          // 1 .. 12 in D1 Whole Sign from Lagna
    #[serde(default)]
    pub dignity: String,    // Exalted, Moolatrikona, Own Sign, Great Friend, Friend, Neutral, Enemy, Great Enemy, Debilitated
    #[serde(default)]
    pub combustion: String, // None, Combust, DeeplyCombust
}

/// Single planet placement within a divisional chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VargaPlacement {
    pub planet_name: String,
    pub sign_index: u8,   // 0 .. 11
    pub sign_degree: f64, // 0.0 .. 30.0 relative degree within the varga sign
    pub house: u8,        // 1 .. 12 counted from varga lagna
}

/// Varga chart details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VargaChart {
    pub division: VargaDivision,
    pub lagna_sign: u8, // 0 .. 11
    pub placements: Vec<VargaPlacement>,
}

/// Simplified dasha timeline period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashaPeriodInfo {
    pub lord: String,
    pub start_jd: f64,
    pub end_jd: f64,
    pub sub_periods: Vec<DashaPeriodInfo>,
}

impl From<&DashaPeriod> for DashaPeriodInfo {
    fn from(p: &DashaPeriod) -> Self {
        Self {
            lord: format!("{:?}", p.lord),
            start_jd: p.start_jd,
            end_jd: p.end_jd,
            sub_periods: p.sub_periods.iter().map(DashaPeriodInfo::from).collect(),
        }
    }
}

/// Jaimini Chara Karaka assignment (8-Karaka scheme).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicKarakaInfo {
    pub karaka: String,
    pub planet: String,
    pub effective_degree: f64,
}

/// Parashari full Graha Drishti (planetary aspect) entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicDrishtiInfo {
    pub from_planet: String,
    pub from_sign: u8,
    pub from_house: u8,
    pub aspect_offset: u8, // 3, 4, 5, 7, 8, 9, 10
    pub target_sign: u8,
    pub target_house: u8,
    pub aspected_planets: Vec<String>,
}

/// Arudha Pada for a Bhava (A1..A12).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicArudhaPada {
    pub bhava: u8, // 1..12
    pub code: String,
    pub name: String,
    pub sign_index: u8,
    pub sign_name: String,
    pub house_from_lagna: u8,
}

/// Bhinna Ashtakavarga table row for a single planet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BhinnaAshtakavargaRow {
    pub planet: String,
    pub bindus: [u8; 12],
    pub total: u16,
}

/// Complete Ashtakavarga summary (7 Bhinna tables + 12-sign Sarvashtakavarga).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VedicAshtakavarga {
    pub bhinna: Vec<BhinnaAshtakavargaRow>,
    pub sarva: [u8; 12],
    pub total_bindus: u16,
}

/// Classical 5-limb Panchanga at the chart epoch.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VedicPanchanga {
    pub tithi_name: String,
    pub tithi_number: u8, // 1..30
    pub paksha: String,   // Shukla or Krishna
    pub vara_name: String,
    pub vara_lord: String,
    pub nakshatra_name: String,
    pub nakshatra_pada: u8,
    pub nakshatra_lord: String,
    pub yoga_name: String,
    pub karana_name: String,
}

/// Complete Vedic astrology chart result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicChart {
    pub lagna_sidereal: f64,
    pub lagna_sign: u8,
    pub wholesign_bhavas: [f64; 12],
    pub planets: Vec<VedicPlanet>,
    pub vargas: BTreeMap<VargaDivision, VargaChart>,
    pub vimshottari_dashas: Vec<DashaPeriodInfo>,
    #[serde(default)]
    pub chara_karakas: Vec<VedicKarakaInfo>,
    #[serde(default)]
    pub drishti: Vec<VedicDrishtiInfo>,
    #[serde(default)]
    pub arudha_padas: Vec<VedicArudhaPada>,
    #[serde(default)]
    pub ashtakavarga: VedicAshtakavarga,
    #[serde(default)]
    pub panchanga: VedicPanchanga,
    #[serde(default = "default_ayanamsha_name")]
    pub ayanamsha_name: String,
    #[serde(default)]
    pub ayanamsha_deg: f64,
}

fn default_ayanamsha_name() -> String {
    "True Lahiri (Chitra Paksha)".to_string()
}

fn parse_graha(name: &str) -> Option<Graha> {
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

fn sign_lord(sign_index: u8) -> Graha {
    match sign_index % 12 {
        0 | 7 => Graha::Mars,     // Aries, Scorpio
        1 | 6 => Graha::Venus,    // Taurus, Libra
        2 | 5 => Graha::Mercury,  // Gemini, Virgo
        3 => Graha::Moon,         // Cancer
        4 => Graha::Sun,          // Leo
        8 | 11 => Graha::Jupiter, // Sagittarius, Pisces
        _ => Graha::Saturn,       // Capricorn, Aquarius
    }
}

/// Natural relationship (Naisargika Maitri) per BPHS Ch. 3: +1 Friend, 0 Neutral, -1 Enemy.
fn natural_relationship(planet: Graha, other: Graha) -> i8 {
    match planet {
        Graha::Sun => match other {
            Graha::Moon | Graha::Mars | Graha::Jupiter => 1,
            Graha::Mercury => 0,
            _ => -1,
        },
        Graha::Moon => match other {
            Graha::Sun | Graha::Mercury => 1,
            _ => 0,
        },
        Graha::Mars => match other {
            Graha::Sun | Graha::Moon | Graha::Jupiter => 1,
            Graha::Venus | Graha::Saturn => 0,
            _ => -1,
        },
        Graha::Mercury => match other {
            Graha::Sun | Graha::Venus => 1,
            Graha::Mars | Graha::Jupiter | Graha::Saturn => 0,
            _ => -1,
        },
        Graha::Jupiter => match other {
            Graha::Sun | Graha::Moon | Graha::Mars => 1,
            Graha::Saturn => 0,
            _ => -1,
        },
        Graha::Venus => match other {
            Graha::Mercury | Graha::Saturn => 1,
            Graha::Mars | Graha::Jupiter => 0,
            _ => -1,
        },
        Graha::Saturn => match other {
            Graha::Mercury | Graha::Venus => 1,
            Graha::Jupiter => 0,
            _ => -1,
        },
        Graha::Rahu => match other {
            Graha::Mercury | Graha::Venus | Graha::Saturn => 1,
            Graha::Jupiter => 0,
            _ => -1,
        },
        Graha::Ketu => match other {
            Graha::Mars | Graha::Venus | Graha::Saturn => 1,
            Graha::Mercury | Graha::Jupiter => 0,
            _ => -1,
        },
    }
}

/// Computes classical Vedic dignity (BPHS Ch. 3):
/// Exalted, Moolatrikona, Own Sign, Debilitated, or 5-fold Panchadha Maitri
/// (Great Friend, Friend, Neutral, Enemy, Great Enemy).
fn compute_vedic_dignity(
    graha: Graha,
    sign_index: u8,
    sign_degree: f64,
    planet_signs: &HashMap<Graha, u8>,
) -> String {
    // 1. Check Moolatrikona and Exaltation / Own Sign per BPHS Ch. 3 degree ranges
    match graha {
        Graha::Sun => {
            if sign_index == 0 {
                return "Exalted".to_string();
            }
            if sign_index == 6 {
                return "Debilitated".to_string();
            }
            if sign_index == 4 {
                if sign_degree < 20.0 {
                    return "Moolatrikona".to_string();
                }
                return "Own Sign".to_string();
            }
        }
        Graha::Moon => {
            if sign_index == 1 {
                if sign_degree <= 3.0 {
                    return "Exalted".to_string();
                }
                return "Moolatrikona".to_string();
            }
            if sign_index == 7 {
                return "Debilitated".to_string();
            }
            if sign_index == 3 {
                return "Own Sign".to_string();
            }
        }
        Graha::Mars => {
            if sign_index == 9 {
                return "Exalted".to_string();
            }
            if sign_index == 3 {
                return "Debilitated".to_string();
            }
            if sign_index == 0 {
                if sign_degree < 12.0 {
                    return "Moolatrikona".to_string();
                }
                return "Own Sign".to_string();
            }
            if sign_index == 7 {
                return "Own Sign".to_string();
            }
        }
        Graha::Mercury => {
            if sign_index == 5 {
                if sign_degree < 15.0 {
                    return "Exalted".to_string();
                } else if sign_degree < 20.0 {
                    return "Moolatrikona".to_string();
                }
                return "Own Sign".to_string();
            }
            if sign_index == 11 {
                return "Debilitated".to_string();
            }
            if sign_index == 2 {
                return "Own Sign".to_string();
            }
        }
        Graha::Jupiter => {
            if sign_index == 3 {
                return "Exalted".to_string();
            }
            if sign_index == 9 {
                return "Debilitated".to_string();
            }
            if sign_index == 8 {
                if sign_degree < 10.0 {
                    return "Moolatrikona".to_string();
                }
                return "Own Sign".to_string();
            }
            if sign_index == 11 {
                return "Own Sign".to_string();
            }
        }
        Graha::Venus => {
            if sign_index == 11 {
                return "Exalted".to_string();
            }
            if sign_index == 5 {
                return "Debilitated".to_string();
            }
            if sign_index == 6 {
                if sign_degree < 15.0 {
                    return "Moolatrikona".to_string();
                }
                return "Own Sign".to_string();
            }
            if sign_index == 1 {
                return "Own Sign".to_string();
            }
        }
        Graha::Saturn => {
            if sign_index == 6 {
                return "Exalted".to_string();
            }
            if sign_index == 0 {
                return "Debilitated".to_string();
            }
            if sign_index == 10 {
                if sign_degree < 20.0 {
                    return "Moolatrikona".to_string();
                }
                return "Own Sign".to_string();
            }
            if sign_index == 9 {
                return "Own Sign".to_string();
            }
        }
        Graha::Rahu => {
            if sign_index == 1 {
                return "Exalted".to_string();
            }
            if sign_index == 7 {
                return "Debilitated".to_string();
            }
            if sign_index == 2 {
                return "Moolatrikona".to_string();
            }
            if sign_index == 10 {
                return "Own Sign".to_string();
            }
        }
        Graha::Ketu => {
            if sign_index == 7 {
                return "Exalted".to_string();
            }
            if sign_index == 1 {
                return "Debilitated".to_string();
            }
            if sign_index == 8 {
                return "Moolatrikona".to_string();
            }
            if sign_index == 7 {
                return "Own Sign".to_string();
            }
        }
    }

    // 2. Panchadha Maitri (5-fold compound relationship: Naisargika + Tatkalika)
    let disp = sign_lord(sign_index);
    let naisargika = natural_relationship(graha, disp);
    let disp_sign = planet_signs.get(&disp).copied().unwrap_or(sign_index);
    let house_dist = ((disp_sign as i16 - sign_index as i16 + 12) % 12 + 1) as u8;
    // Tatkalika Maitri (BPHS Ch. 3 v. 56): planets in 2, 3, 4, 10, 11, 12 from a planet are temporary friends (+1), others enemies (-1)
    let tatkalika: i8 = if matches!(house_dist, 2 | 3 | 4 | 10 | 11 | 12) {
        1
    } else {
        -1
    };

    match naisargika + tatkalika {
        2 => "Great Friend".to_string(),
        1 => "Friend".to_string(),
        0 => "Neutral".to_string(),
        -1 => "Enemy".to_string(),
        _ => "Great Enemy".to_string(),
    }
}

const SEVEN_KARAKA_LABELS: [&str; 7] = [
    "Atmakaraka (AK)",
    "Amatyakaraka (AmK)",
    "Bhratrikaraka (BK)",
    "Matrikaraka (MK)",
    "Putrakaraka (PK)",
    "Gnatikaraka (GK)",
    "Darakaraka (DK)",
];

/// Resolves the effective Arudha lord of a Rasi (0..11), applying Jaimini co-lordship
/// rules for Scorpio (`sign_index == 7`: Mars vs Ketu) and Aquarius (`sign_index == 10`: Saturn vs Rahu).
fn arudha_sign_lord(
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

    // 1. If one lord is in the sign itself and the other is in a different sign,
    // Jaimini Sutras (1.1.30-31) prefer the lord NOT in the sign itself so a non-zero step is taken.
    if sign_a == rashi && sign_b != rashi {
        return lord_b;
    }
    if sign_b == rashi && sign_a != rashi {
        return lord_a;
    }

    // 2. Compare number of conjoined planets in each lord's sign (more conjoined grahas = stronger).
    let count_a = planet_signs.values().filter(|&&s| s == sign_a).count();
    let count_b = planet_signs.values().filter(|&&s| s == sign_b).count();
    if count_a > count_b {
        return lord_a;
    }
    if count_b > count_a {
        return lord_b;
    }

    // 3. Tie-breaker: higher effective degree in sign (for Rahu/Ketu, 30.0 - degree_in_sign).
    let eff_deg = |g: Graha| -> f64 {
        let lon = planet_lons.get(&g).copied().unwrap_or(0.0);
        let deg = normalize_degrees(lon) % 30.0;
        if matches!(g, Graha::Rahu | Graha::Ketu) {
            30.0 - deg
        } else {
            deg
        }
    };
    if eff_deg(lord_a) >= eff_deg(lord_b) {
        lord_a
    } else {
        lord_b
    }
}

fn arudha_name(bhava: u8) -> (&'static str, &'static str) {
    match bhava {
        1 => ("AL", "Arudha Lagna"),
        2 => ("A2", "Dhana Pada"),
        3 => ("A3", "Vikrama Pada"),
        4 => ("A4", "Matri Pada"),
        5 => ("A5", "Mantra Pada"),
        6 => ("A6", "Roga Pada"),
        7 => ("A7", "Dara Pada"),
        8 => ("A8", "Mrityu Pada"),
        9 => ("A9", "Bhagya Pada"),
        10 => ("A10", "Rajya Pada"),
        11 => ("A11", "Labha Pada"),
        _ => ("UL", "Upapada Lagna"),
    }
}

/// Calculate Vedic chart from an ephemeris snapshot and observer latitude.
pub fn calculate_vedic(ephem: &EphemerisSnapshot, latitude: f64) -> VedicChart {
    // 1. Calculate Sidereal Lagna using WholeSign framework
    let house_result = compute_houses(
        ephem.ramc,
        latitude,
        ephem.true_obliquity_deg,
        HouseSystem::WholeSign,
    );
    let lagna_sidereal = normalize_degrees(house_result.asc - ephem.true_ayanamsha_deg);
    let lagna_sign = ((lagna_sidereal / 30.0).floor() as u8).min(11);

    let wholesign_bhavas = core::array::from_fn(|h| {
        let sign = (lagna_sign as usize + h) % 12;
        sign as f64 * 30.0
    });

    // Pre-collect planet longitudes, signs, and retrograde states for dignity & combustion
    let mut sun_sidereal_lon = 0.0;
    let mut moon_sidereal_lon = 0.0;
    let mut rahu_sidereal_lon = 0.0;
    let mut planet_signs: HashMap<Graha, u8> = HashMap::new();
    let mut planet_lons: HashMap<Graha, f64> = HashMap::new();

    for b in &ephem.bodies {
        let sid_lon = normalize_degrees(b.sidereal_longitude);
        let sign_idx = ((sid_lon / 30.0).floor() as u8).min(11);
        if b.name == "Sun" {
            sun_sidereal_lon = sid_lon;
        } else if b.name == "Moon" {
            moon_sidereal_lon = sid_lon;
        } else if b.name == "Rahu" {
            rahu_sidereal_lon = sid_lon;
        }
        if let Some(g) = parse_graha(&b.name) {
            planet_signs.insert(g, sign_idx);
            planet_lons.insert(g, sid_lon);
        }
    }

    // 2. Map planets to sidereal signs, nakshatras, houses, dignities, and combustion
    let mut planets = Vec::with_capacity(ephem.bodies.len());

    for b in &ephem.bodies {
        let sid_lon = normalize_degrees(b.sidereal_longitude);
        let sign_index = ((sid_lon / 30.0).floor() as u8).min(11);
        let sign_degree = (sid_lon - (sign_index as f64 * 30.0)).clamp(0.0, 30.0);
        let house = ((sign_index as i16 - lagna_sign as i16 + 12) % 12 + 1) as u8;

        let nak = Nakshatra::from_longitude(sid_lon);
        let pada = Nakshatra::pada_from_longitude(sid_lon);

        let (dignity, combustion) = if let Some(g) = parse_graha(&b.name) {
            let dig = compute_vedic_dignity(g, sign_index, sign_degree, &planet_signs);
            let comb = match combustion_state(g, sid_lon, sun_sidereal_lon, b.is_retrograde) {
                CombustionState::None => "None".to_string(),
                CombustionState::Combust => "Combust".to_string(),
                CombustionState::DeeplyCombust => "DeeplyCombust".to_string(),
            };
            (dig, comb)
        } else {
            ("Neutral".to_string(), "None".to_string())
        };

        planets.push(VedicPlanet {
            name: b.name.clone(),
            sidereal_longitude: sid_lon,
            sign_index,
            sign_degree,
            nakshatra_name: nak.name().to_string(),
            nakshatra_pada: pada,
            is_retrograde: b.is_retrograde,
            latitude: b.latitude,
            house,
            dignity,
            combustion,
        });
    }

    // 3. Compute the 19 supported divisional charts (with Rahu-Ketu 1-7 opposition enforced)
    let mut vargas = BTreeMap::new();
    for div in VargaDivision::ALL {
        let (v_lagna, _) = calculate_varga_sign_and_degree(lagna_sidereal, div);
        let (rahu_v_sign, rahu_v_deg) = calculate_varga_sign_and_degree(rahu_sidereal_lon, div);

        let mut placements = Vec::with_capacity(planets.len());
        for p in &planets {
            let (p_sign, p_degree) = if p.name == "Ketu" {
                let opp_sign = match div {
                    // In D2 Hora, only Cancer (3) and Leo (4) exist; Ketu occupies the opposite Hora from Rahu
                    VargaDivision::D2 => {
                        if rahu_v_sign == 4 {
                            3
                        } else {
                            4
                        }
                    }
                    // In all 12-sign divisional charts, Ketu is strictly 7th (opposite) from Rahu
                    _ => (rahu_v_sign + 6) % 12,
                };
                (opp_sign, rahu_v_deg)
            } else {
                calculate_varga_sign_and_degree(p.sidereal_longitude, div)
            };
            let house = ((p_sign as i16 - v_lagna as i16 + 12) % 12 + 1) as u8;

            placements.push(VargaPlacement {
                planet_name: p.name.clone(),
                sign_index: p_sign,
                sign_degree: p_degree,
                house,
            });
        }

        vargas.insert(
            div,
            VargaChart {
                division: div,
                lagna_sign: v_lagna,
                placements,
            },
        );
    }

    // 4. Compute Vimshottari Dasha (Maha and Antar levels)
    let dasha_tree = compute_vimshottari(moon_sidereal_lon, ephem.jd_ut1, 2);
    let vimshottari_dashas = dasha_tree
        .maha_dashas
        .iter()
        .map(DashaPeriodInfo::from)
        .collect();

    // 5. Compute Classical 7 Chara Karakas (Sun..Saturn sorted descending by sign_degree)
    let mut seven_candidates: Vec<(&str, f64)> = planets
        .iter()
        .filter(|p| p.name != "Rahu" && p.name != "Ketu")
        .map(|p| (p.name.as_str(), p.sign_degree))
        .collect();
    seven_candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let chara_karakas = seven_candidates
        .into_iter()
        .zip(SEVEN_KARAKA_LABELS.iter())
        .map(|((p_name, deg), &label)| VedicKarakaInfo {
            karaka: label.to_string(),
            planet: p_name.to_string(),
            effective_degree: deg,
        })
        .collect();

    // 6. Compute BPHS Full Graha Drishti (Ch. 26: 7 Grahas + Rahu; headless Ketu excluded)
    let mut drishti = Vec::new();
    for p in &planets {
        if p.name == "Ketu" {
            continue;
        }
        let offsets: &[u8] = match p.name.as_str() {
            "Mars" => &[4, 7, 8],
            "Jupiter" => &[5, 7, 9],
            "Saturn" => &[3, 7, 10],
            "Rahu" => &[5, 7, 9],
            _ => &[7],
        };
        for &offset in offsets {
            let target_sign = (p.sign_index + offset - 1) % 12;
            let target_house = ((target_sign as i16 - lagna_sign as i16 + 12) % 12 + 1) as u8;
            let aspected_planets: Vec<String> = planets
                .iter()
                .filter(|other| other.name != p.name && other.sign_index == target_sign)
                .map(|other| other.name.clone())
                .collect();

            drishti.push(VedicDrishtiInfo {
                from_planet: p.name.clone(),
                from_sign: p.sign_index,
                from_house: p.house,
                aspect_offset: offset,
                target_sign,
                target_house,
                aspected_planets,
            });
        }
    }

    // 7. Compute 12 Arudha Padas (BPHS Ch. 29 with Scorpio/Aquarius co-lordship and 10th-from-landed exception)
    let mut arudha_padas = Vec::with_capacity(12);
    for bhava in 1..=12u8 {
        let bhava_rashi = (lagna_sign + bhava - 1) % 12;
        let lord = arudha_sign_lord(bhava_rashi, &planet_signs, &planet_lons);
        let lord_rashi = planet_signs.get(&lord).copied().unwrap_or(bhava_rashi);
        let pada_sign = arudha_pada(bhava_rashi, lord_rashi, ArudhaException::TenthFromLanded);
        let house_from_lagna = ((pada_sign as i16 - lagna_sign as i16 + 12) % 12 + 1) as u8;
        let (code, name) = arudha_name(bhava);

        arudha_padas.push(VedicArudhaPada {
            bhava,
            code: code.to_string(),
            name: name.to_string(),
            sign_index: pada_sign,
            sign_name: RASHI_NAMES[pada_sign as usize].to_string(),
            house_from_lagna,
        });
    }

    // 8. Compute Ashtakavarga (Bhinna & Sarva, BPHS Ch. 66)
    let av_input = BhinnaAshtakavargaInput {
        sun: *planet_signs.get(&Graha::Sun).unwrap_or(&0),
        moon: *planet_signs.get(&Graha::Moon).unwrap_or(&0),
        mars: *planet_signs.get(&Graha::Mars).unwrap_or(&0),
        mercury: *planet_signs.get(&Graha::Mercury).unwrap_or(&0),
        jupiter: *planet_signs.get(&Graha::Jupiter).unwrap_or(&0),
        venus: *planet_signs.get(&Graha::Venus).unwrap_or(&0),
        saturn: *planet_signs.get(&Graha::Saturn).unwrap_or(&0),
        lagna: lagna_sign,
    };
    let bhinna_tables = bhinna_ashtakavarga(&av_input);
    let sarva = sarvashtakavarga(&bhinna_tables);
    let total_bindus: u16 = sarva.iter().map(|&b| b as u16).sum();
    let bhinna = bhinna_tables
        .iter()
        .map(|t| BhinnaAshtakavargaRow {
            planet: format!("{:?}", t.planet),
            bindus: t.bindus,
            total: t.total as u16,
        })
        .collect();
    let ashtakavarga = VedicAshtakavarga {
        bhinna,
        sarva,
        total_bindus,
    };

    // 9. Compute 5-limb Panchanga (Tithi, Vara, Nakshatra, Yoga, Karana)
    let elongation = normalize_degrees(moon_sidereal_lon - sun_sidereal_lon);
    let tithi_number = ((elongation / 12.0).floor() as u8).min(29) + 1;
    let paksha = if tithi_number <= 15 {
        "Shukla Paksha".to_string()
    } else {
        "Krishna Paksha".to_string()
    };
    let tithi_idx = ((tithi_number - 1) % 15) as usize;
    let tithi_name = if tithi_number == 15 {
        "Purnima".to_string()
    } else if tithi_number == 30 {
        "Amavasya".to_string()
    } else {
        TITHI_NAMES[tithi_idx].to_string()
    };

    // Weekday from Julian Day (0 = Monday .. 6 = Sunday for (jd + 0.5).floor() % 7)
    let weekday_idx = ((ephem.jd_ut1 + 0.5).floor() as i64).rem_euclid(7) as u8;
    let (vara_name, vara_lord) = match weekday_idx {
        0 => ("Somavara (Monday)", "Moon"),
        1 => ("Mangalavara (Tuesday)", "Mars"),
        2 => ("Budhavara (Wednesday)", "Mercury"),
        3 => ("Guruvara (Thursday)", "Jupiter"),
        4 => ("Shukravara (Friday)", "Venus"),
        5 => ("Shanivara (Saturday)", "Saturn"),
        _ => ("Ravivara (Sunday)", "Sun"),
    };

    let moon_nak = Nakshatra::from_longitude(moon_sidereal_lon);
    let moon_pada = Nakshatra::pada_from_longitude(moon_sidereal_lon);
    let yoga = compute_panchanga_yoga(sun_sidereal_lon, moon_sidereal_lon);
    let karana = compute_karana(moon_sidereal_lon, sun_sidereal_lon);

    let panchanga = VedicPanchanga {
        tithi_name,
        tithi_number,
        paksha,
        vara_name: vara_name.to_string(),
        vara_lord: vara_lord.to_string(),
        nakshatra_name: moon_nak.name().to_string(),
        nakshatra_pada: moon_pada,
        nakshatra_lord: format!("{:?}", moon_nak.dasha_lord()),
        yoga_name: yoga.name.to_string(),
        karana_name: karana.name.to_string(),
    };

    VedicChart {
        lagna_sidereal,
        lagna_sign,
        wholesign_bhavas,
        planets,
        vargas,
        vimshottari_dashas,
        chara_karakas,
        drishti,
        arudha_padas,
        ashtakavarga,
        panchanga,
        ayanamsha_name: default_ayanamsha_name(),
        ayanamsha_deg: ephem.true_ayanamsha_deg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ephemeris::compute_ephemeris;

    #[test]
    fn test_vedic_chart_generation() {
        let ephem = compute_ephemeris(2024, 3, 20, 12, 0, 0.0, 28.6139, 77.2090)
            .expect("ephem should succeed");

        let chart = calculate_vedic(&ephem, 28.6139);

        // Verify all 19 supported Varga divisions and Rahu-Ketu opposition in every varga
        assert_eq!(chart.vargas.len(), 19);
        for div in VargaDivision::ALL {
            let v_chart = chart.vargas.get(&div).expect("varga must exist");
            assert_eq!(v_chart.placements.len(), 9);
            assert!(v_chart.lagna_sign <= 11);

            let rahu_p = v_chart.placements.iter().find(|p| p.planet_name == "Rahu").unwrap();
            let ketu_p = v_chart.placements.iter().find(|p| p.planet_name == "Ketu").unwrap();
            if div == VargaDivision::D2 {
                assert_ne!(rahu_p.sign_index, ketu_p.sign_index, "D2 Rahu and Ketu must occupy opposite Horas");
            } else {
                assert_eq!(
                    (rahu_p.sign_index + 6) % 12,
                    ketu_p.sign_index,
                    "Rahu and Ketu must be 180 deg (7th sign) apart in {:?}",
                    div
                );
            }
        }

        // Verify Whole-sign Bhavas
        assert_eq!(chart.wholesign_bhavas.len(), 12);

        // Verify Planets, Nakshatras, Houses, Dignities, Combustion
        assert_eq!(chart.planets.len(), 9);
        for p in &chart.planets {
            assert!((1..=12).contains(&p.house));
            assert!(!p.dignity.is_empty());
            assert!(!p.combustion.is_empty());
            assert!(!p.nakshatra_name.is_empty());
        }

        // Verify Classical Vedic indicators (7-Karaka scheme, no Ketu in Drishti)
        assert_eq!(chart.chara_karakas.len(), 7);
        assert!(chart.chara_karakas.iter().all(|ck| ck.planet != "Rahu" && ck.planet != "Ketu"));
        assert!(chart.drishti.iter().all(|d| d.from_planet != "Ketu"));
        assert_eq!(chart.arudha_padas.len(), 12);
        assert_eq!(chart.ashtakavarga.bhinna.len(), 7);
        assert_eq!(chart.ashtakavarga.total_bindus, 337);
        assert!(!chart.drishti.is_empty());
        assert!((1..=30).contains(&chart.panchanga.tithi_number));
        assert!(!chart.panchanga.yoga_name.is_empty());
        assert!(!chart.panchanga.karana_name.is_empty());
        assert_eq!(chart.ayanamsha_name, "True Lahiri (Chitra Paksha)");
        assert!(chart.ayanamsha_deg > 20.0);

        // Verify Vimshottari Dasha periods continuity and Antardashas
        assert_eq!(chart.vimshottari_dashas.len(), 9);
        for dasha in &chart.vimshottari_dashas {
            assert_eq!(
                dasha.sub_periods.len(),
                9,
                "Each Mahadasha must contain 9 Antardashas"
            );
        }
    }

    #[test]
    fn test_varga_calculations_accuracy() {
        // D1 Rasi
        let (sign, deg) = calculate_varga_sign_and_degree(15.5, VargaDivision::D1);
        assert_eq!(sign, 0); // Aries
        assert!((deg - 15.5).abs() < 1e-6);

        // D2 Hora (Parashari BPHS: Sun/Moon dual sign distribution)
        let (sign, deg) = calculate_varga_sign_and_degree(7.5, VargaDivision::D2);
        assert_eq!(sign, 4); // Leo
        assert!((deg - 15.0).abs() < 1e-6);

        let (sign, deg) = calculate_varga_sign_and_degree(20.0, VargaDivision::D2);
        assert_eq!(sign, 3); // Cancer
        assert!((deg - 10.0).abs() < 1e-6);

        let (sign, deg) = calculate_varga_sign_and_degree(35.0, VargaDivision::D2);
        assert_eq!(sign, 3); // Cancer
        assert!((deg - 10.0).abs() < 1e-6);

        let (sign, deg) = calculate_varga_sign_and_degree(50.0, VargaDivision::D2);
        assert_eq!(sign, 4); // Leo
        assert!((deg - 10.0).abs() < 1e-6);

        // D3 Drekkana (0-10 same, 10-20 +4, 20-30 +8)
        assert_eq!(calculate_varga_sign(5.0, VargaDivision::D3), 0); // Aries 5 -> Aries
        assert_eq!(calculate_varga_sign(15.0, VargaDivision::D3), 4); // Aries 15 -> Leo
        assert_eq!(calculate_varga_sign(25.0, VargaDivision::D3), 8); // Aries 25 -> Sagittarius

        // D4 Chaturthamsha (7.5-deg Kendra offsets: 0, 3, 6, 9)
        assert_eq!(calculate_varga_sign(2.0, VargaDivision::D4), 0); // Aries (0 + 0)
        assert_eq!(calculate_varga_sign(8.0, VargaDivision::D4), 3); // Cancer (0 + 3)
        assert_eq!(calculate_varga_sign(16.0, VargaDivision::D4), 6); // Libra (0 + 6)
        assert_eq!(calculate_varga_sign(23.0, VargaDivision::D4), 9); // Capricorn (0 + 9)
        assert_eq!(calculate_varga_sign(38.0, VargaDivision::D4), 4); // Taurus 8 deg -> Leo (1 + 3)

        // D6 Shashthamsha (Odd signs start from Aries = 0; Even signs start from Libra = 6)
        assert_eq!(calculate_varga_sign(2.0, VargaDivision::D6), 0); // Aries (odd) part 0 -> Aries (0)
        assert_eq!(calculate_varga_sign(6.0, VargaDivision::D6), 1); // Aries (odd) part 1 -> Taurus (1)
        assert_eq!(calculate_varga_sign(32.0, VargaDivision::D6), 6); // Taurus (even) part 0 -> Libra (6)
        assert_eq!(calculate_varga_sign(36.0, VargaDivision::D6), 7); // Taurus (even) part 1 -> Scorpio (7)
        assert_eq!(calculate_varga_sign(62.0, VargaDivision::D6), 0); // Gemini (odd) part 0 -> Aries (0)

        // D7 Saptamsha (Odd: same; Even: 7th)
        assert_eq!(calculate_varga_sign(2.0, VargaDivision::D7), 0); // Aries
        assert_eq!(calculate_varga_sign(32.0, VargaDivision::D7), 7); // Taurus -> Scorpio

        // D8 Ashtamamsha (Movable: Aries, Fixed: Sag, Dual: Leo)
        assert_eq!(calculate_varga_sign(1.0, VargaDivision::D8), 0); // Aries (Movable) -> Aries
        assert_eq!(calculate_varga_sign(31.0, VargaDivision::D8), 8); // Taurus (Fixed) -> Sagittarius
        assert_eq!(calculate_varga_sign(61.0, VargaDivision::D8), 4); // Dual (Gemini) -> Leo

        // D9 Navamsha (3°20' divisions) + Exact boundary precision test (10.0° and 20.0°)
        assert_eq!(calculate_varga_sign(1.0, VargaDivision::D9), 0); // Aries
        assert_eq!(calculate_varga_sign(4.0, VargaDivision::D9), 1); // Taurus
        assert_eq!(calculate_varga_sign(31.0, VargaDivision::D9), 9); // Taurus 1 deg -> Capricorn
        let (d9_sign_10, d9_deg_10) = calculate_varga_sign_and_degree(10.0, VargaDivision::D9);
        assert_eq!(d9_sign_10, 3); // Part 3 -> Cancer (3), NOT Gemini (2) with 30.0 deg
        assert!(d9_deg_10.abs() < 1e-6);
        let (d9_sign_20, d9_deg_20) = calculate_varga_sign_and_degree(20.0, VargaDivision::D9);
        assert_eq!(d9_sign_20, 6); // Part 6 -> Libra (6)
        assert!(d9_deg_20.abs() < 1e-6);

        // D10 Dashamsha (Odd: same, Even: 9th)
        assert_eq!(calculate_varga_sign(1.0, VargaDivision::D10), 0); // Aries
        assert_eq!(calculate_varga_sign(31.0, VargaDivision::D10), 9); // Taurus -> Capricorn (1 + 8)

        // D11 Rudramsha (Gemini: start Aquarius, count forward)
        assert_eq!(calculate_varga_sign(61.0, VargaDivision::D11), 10);
        assert_eq!(calculate_varga_sign(63.0, VargaDivision::D11), 11);

        // D12 Dwadashamsha (Count from same sign)
        assert_eq!(calculate_varga_sign(1.0, VargaDivision::D12), 0); // Aries
        assert_eq!(calculate_varga_sign(3.0, VargaDivision::D12), 1); // Taurus
        assert_eq!(calculate_varga_sign(31.0, VargaDivision::D12), 1); // Taurus 1 deg -> Taurus

        // D16 Shodashamsha (Movable: Aries 0, Fixed: Leo 4, Dual: Sag 8)
        assert_eq!(calculate_varga_sign(1.0, VargaDivision::D16), 0); // Aries -> Aries
        assert_eq!(calculate_varga_sign(31.0, VargaDivision::D16), 4); // Taurus -> Leo
        assert_eq!(calculate_varga_sign(61.0, VargaDivision::D16), 8); // Gemini -> Sagittarius

        // D20 Vimshamsha (Movable: Aries 0, Fixed: Sag 8, Dual: Leo 4)
        assert_eq!(calculate_varga_sign(1.0, VargaDivision::D20), 0); // Aries -> Aries
        assert_eq!(calculate_varga_sign(31.0, VargaDivision::D20), 8); // Taurus -> Sagittarius
        assert_eq!(calculate_varga_sign(61.0, VargaDivision::D20), 4); // Gemini -> Leo

        // D24 Chaturvimshamsha (Odd: Leo 4, Even: Cancer 3, forward)
        assert_eq!(calculate_varga_sign(0.5, VargaDivision::D24), 4); // Aries part 0 -> Leo
        assert_eq!(calculate_varga_sign(2.0, VargaDivision::D24), 5); // Aries part 1 -> Virgo
        assert_eq!(calculate_varga_sign(30.5, VargaDivision::D24), 3); // Taurus part 0 -> Cancer
        assert_eq!(calculate_varga_sign(32.0, VargaDivision::D24), 4); // Taurus part 1 -> Leo

        // D27 Saptavimshamsha (Fire: Aries 0, Earth: Cancer 3, Air: Libra 6, Water: Cap 9)
        assert_eq!(calculate_varga_sign(0.5, VargaDivision::D27), 0); // Aries -> Aries
        assert_eq!(calculate_varga_sign(30.5, VargaDivision::D27), 3); // Taurus -> Cancer
        assert_eq!(calculate_varga_sign(60.5, VargaDivision::D27), 6); // Gemini -> Libra
        assert_eq!(calculate_varga_sign(90.5, VargaDivision::D27), 9); // Cancer -> Capricorn

        // D30 Trimshamsha (Unequal 5 segments)
        // Odd sign (Aries): 0..5->0, 5..10->10, 10..18->8, 18..25->2, 25..30->1
        assert_eq!(calculate_varga_sign(2.5, VargaDivision::D30), 0);
        assert_eq!(calculate_varga_sign(7.5, VargaDivision::D30), 10);
        assert_eq!(calculate_varga_sign(14.0, VargaDivision::D30), 8);
        assert_eq!(calculate_varga_sign(21.0, VargaDivision::D30), 2);
        assert_eq!(calculate_varga_sign(27.0, VargaDivision::D30), 1);
        // Even sign (Taurus): 0..5->6, 5..12->5, 12..20->11, 20..25->9, 25..30->7
        assert_eq!(calculate_varga_sign(32.5, VargaDivision::D30), 6);
        assert_eq!(calculate_varga_sign(38.0, VargaDivision::D30), 5);
        assert_eq!(calculate_varga_sign(46.0, VargaDivision::D30), 11);
        assert_eq!(calculate_varga_sign(52.0, VargaDivision::D30), 9);
        assert_eq!(calculate_varga_sign(57.0, VargaDivision::D30), 7);

        // D40 Khavedamsha (Odd: Aries 0, Even: Libra 6)
        assert_eq!(calculate_varga_sign(0.2, VargaDivision::D40), 0);
        assert_eq!(calculate_varga_sign(30.2, VargaDivision::D40), 6);

        // D45 Akshavedamsha (Movable: Aries 0, Fixed: Leo 4, Dual: Sag 8)
        assert_eq!(calculate_varga_sign(0.2, VargaDivision::D45), 0);
        assert_eq!(calculate_varga_sign(30.2, VargaDivision::D45), 4);
        assert_eq!(calculate_varga_sign(60.2, VargaDivision::D45), 8);

        // D60 Shashtiamsha (Count from sign itself)
        assert_eq!(calculate_varga_sign(0.2, VargaDivision::D60), 0); // Aries part 0 -> Aries
        assert_eq!(calculate_varga_sign(0.7, VargaDivision::D60), 1); // Aries part 1 -> Taurus
        assert_eq!(calculate_varga_sign(30.2, VargaDivision::D60), 1); // Taurus part 0 -> Taurus
    }
}
