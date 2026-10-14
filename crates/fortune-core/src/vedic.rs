//! Vedic astrology (Jyotish) calculation layer (Sidereal Zodiac).
//!
//! Provides deterministic computation for:
//! - Sidereal Lagna (Ascendant) and 12 Whole Sign Bhavas
//! - 11 divisional charts (D1, D2, D3, D4, D6, D7, D8, D9, D10, D11, D12) with varga lagnas and planetary houses
//! - 27 Nakshatras with 4 Padas
//! - Vimshottari Dasha hierarchy (Maha and Antar periods)

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use vedaksha::astro::houses::{compute_houses, HouseSystem};
use vedaksha::math::angle::normalize_degrees;
use vedaksha::vedic::dasha::vimshottari::{compute_vimshottari, DashaPeriod};
use vedaksha::vedic::nakshatra::Nakshatra;

use crate::ephemeris::EphemerisSnapshot;

/// Supported divisional charts in Fortune Note (robust for minute-level birth data).
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
}

impl VargaDivision {
    pub const ALL: [VargaDivision; 11] = [
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
        }
    }
}

/// Computes the divisional sign index (0..11) and normalized degree in that sign (0.0..30.0)
/// for any supported varga division.
pub fn calculate_varga_sign_and_degree(sidereal_lon: f64, div: VargaDivision) -> (u8, f64) {
    let norm = normalize_degrees(sidereal_lon);
    let sign_idx = ((norm / 30.0).floor() as u8).min(11);
    let pos_in_sign = (norm - (sign_idx as f64 * 30.0)).clamp(0.0, 30.0);

    match div {
        VargaDivision::D1 => (sign_idx, pos_in_sign),
        VargaDivision::D2 => {
            // Parashari Hora (BPHS):
            // In odd signs (sign_idx % 2 == 0):
            //   1st half (0..15 deg): Leo (4, Sun)
            //   2nd half (15..30 deg): Cancer (3, Moon)
            // In even signs (sign_idx % 2 != 0):
            //   1st half (0..15 deg): Cancer (3, Moon)
            //   2nd half (15..30 deg): Leo (4, Sun)
            let is_first_half = pos_in_sign < 15.0;
            let is_odd_sign = sign_idx.is_multiple_of(2);
            let sign = match (is_odd_sign, is_first_half) {
                (true, true) => 4,   // Odd 1st half: Leo
                (true, false) => 3,  // Odd 2nd half: Cancer
                (false, true) => 3,  // Even 1st half: Cancer
                (false, false) => 4, // Even 2nd half: Leo
            };
            let pos_in_part = if is_first_half {
                pos_in_sign
            } else {
                pos_in_sign - 15.0
            };
            let degree = (pos_in_part / 15.0) * 30.0;
            (sign, degree.clamp(0.0, 30.0))
        }
        VargaDivision::D3 => {
            // Parashari Drekkana: 10-degree divisions (0..2)
            // 1st part: same sign; 2nd part: 5th sign (+4); 3rd part: 9th sign (+8)
            let part = ((pos_in_sign / 10.0).floor() as u8).min(2);
            let offset = match part {
                0 => 0,
                1 => 4,
                _ => 8,
            };
            let sign = (sign_idx + offset) % 12;
            let pos_in_part = pos_in_sign - (part as f64 * 10.0);
            let degree = (pos_in_part / 10.0) * 30.0;
            (sign, degree.clamp(0.0, 30.0))
        }
        VargaDivision::D4 => {
            // Parashari Chaturthamsha: 7.5-degree divisions (0..3)
            // 1st part: same sign; 2nd part: 4th sign (+3); 3rd part: 7th sign (+6); 4th part: 10th sign (+9)
            let part = ((pos_in_sign / 7.5).floor() as u8).min(3);
            let offset = part * 3;
            let sign = (sign_idx + offset) % 12;
            let pos_in_part = pos_in_sign - (part as f64 * 7.5);
            let degree = (pos_in_part / 7.5) * 30.0;
            (sign, degree.clamp(0.0, 30.0))
        }
        VargaDivision::D6 => {
            // Parashari Shashthamsha: 5-degree divisions (0..5)
            // Odd signs start from the sign itself; Even signs start from 7th sign (+6)
            let part = ((pos_in_sign / 5.0).floor() as u8).min(5);
            let start = if sign_idx.is_multiple_of(2) {
                sign_idx
            } else {
                (sign_idx + 6) % 12
            };
            let sign = (start + part) % 12;
            let pos_in_part = pos_in_sign - (part as f64 * 5.0);
            let degree = (pos_in_part / 5.0) * 30.0;
            (sign, degree.clamp(0.0, 30.0))
        }
        VargaDivision::D7 => {
            // Parashari Saptamsha: (30/7)-degree divisions (0..6)
            // Odd signs start from sign itself; Even signs start from 7th sign (+6)
            let span = 30.0 / 7.0;
            let part = ((pos_in_sign / span).floor() as u8).min(6);
            let start = if sign_idx.is_multiple_of(2) {
                sign_idx
            } else {
                (sign_idx + 6) % 12
            };
            let sign = (start + part) % 12;
            let pos_in_part = pos_in_sign - (part as f64 * span);
            let degree = (pos_in_part / span) * 30.0;
            (sign, degree.clamp(0.0, 30.0))
        }
        VargaDivision::D8 => {
            // Jaimini Ashtamamsha: 3.75-degree divisions (0..7)
            // Movable start Aries (0), Fixed start Sagittarius (8), Dual start Leo (4)
            let part = ((pos_in_sign / 3.75).floor() as u8).min(7);
            let start = match sign_idx % 3 {
                0 => 0, // Movable: Aries
                1 => 8, // Fixed: Sagittarius
                _ => 4, // Dual: Leo
            };
            let sign = (start + part) % 12;
            let pos_in_part = pos_in_sign - (part as f64 * 3.75);
            let degree = (pos_in_part / 3.75) * 30.0;
            (sign, degree.clamp(0.0, 30.0))
        }
        VargaDivision::D9 => {
            // Parashari Navamsha: (30/9 = 3°20') divisions (0..8)
            // Movable start from sign itself, Fixed from 9th (+8), Dual from 5th (+4)
            let span = 30.0 / 9.0;
            let part = ((pos_in_sign / span).floor() as u8).min(8);
            let start = match sign_idx % 3 {
                0 => sign_idx,
                1 => (sign_idx + 8) % 12,
                _ => (sign_idx + 4) % 12,
            };
            let sign = (start + part) % 12;
            let pos_in_part = pos_in_sign - (part as f64 * span);
            let degree = (pos_in_part / span) * 30.0;
            (sign, degree.clamp(0.0, 30.0))
        }
        VargaDivision::D10 => {
            // Parashari Dashamsha: 3-degree divisions (0..9)
            // Odd signs start from sign itself; Even signs start from 9th sign (+8)
            let part = ((pos_in_sign / 3.0).floor() as u8).min(9);
            let start = if sign_idx.is_multiple_of(2) {
                sign_idx
            } else {
                (sign_idx + 8) % 12
            };
            let sign = (start + part) % 12;
            let pos_in_part = pos_in_sign - (part as f64 * 3.0);
            let degree = (pos_in_part / 3.0) * 30.0;
            (sign, degree.clamp(0.0, 30.0))
        }
        VargaDivision::D11 => {
            // Jaimini Rudramsha: (30/11)-degree divisions (0..10)
            // Start sign = count backward from Aries by sign position
            // Then count forward by part
            let span = 30.0 / 11.0;
            let part = ((pos_in_sign / span).floor() as u8).min(10);
            let start = (12 - sign_idx) % 12;
            let sign = (start + part) % 12;
            let pos_in_part = pos_in_sign - (part as f64 * span);
            let degree = (pos_in_part / span) * 30.0;
            (sign, degree.clamp(0.0, 30.0))
        }
        VargaDivision::D12 => {
            // Parashari Dwadashamsha: 2.5-degree divisions (0..11)
            // Count from sign itself through 12 signs
            let part = ((pos_in_sign / 2.5).floor() as u8).min(11);
            let sign = (sign_idx + part) % 12;
            let pos_in_part = pos_in_sign - (part as f64 * 2.5);
            let degree = (pos_in_part / 2.5) * 30.0;
            (sign, degree.clamp(0.0, 30.0))
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
    pub sign_index: u8,       // 0 = Mesha (Aries) .. 11 = Meena (Pisces)
    pub sign_degree: f64,     // 0.0 .. 30.0
    pub nakshatra_name: String,
    pub nakshatra_pada: u8,   // 1 .. 4
    pub is_retrograde: bool,
    #[serde(default)]
    pub latitude: f64,
}

/// Single planet placement within a divisional chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VargaPlacement {
    pub planet_name: String,
    pub sign_index: u8,       // 0 .. 11
    pub sign_degree: f64,     // 0.0 .. 30.0 relative degree within the varga sign
    pub house: u8,            // 1 .. 12 counted from varga lagna
}

/// Varga chart details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VargaChart {
    pub division: VargaDivision,
    pub lagna_sign: u8,       // 0 .. 11
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

/// Complete Vedic astrology chart result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicChart {
    pub lagna_sidereal: f64,
    pub lagna_sign: u8,
    pub wholesign_bhavas: [f64; 12],
    pub planets: Vec<VedicPlanet>,
    pub vargas: BTreeMap<VargaDivision, VargaChart>,
    pub vimshottari_dashas: Vec<DashaPeriodInfo>,
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
    let lagna_sign = (lagna_sidereal / 30.0).floor() as u8;

    let wholesign_bhavas = core::array::from_fn(|h| {
        let sign = (lagna_sign as usize + h) % 12;
        sign as f64 * 30.0
    });

    // 2. Map planets to sidereal signs and nakshatras
    let mut planets = Vec::with_capacity(ephem.bodies.len());
    let mut moon_sidereal_lon = 0.0;

    for b in &ephem.bodies {
        let sid_lon = b.sidereal_longitude;
        let sign_index = (sid_lon / 30.0).floor() as u8;
        let sign_degree = sid_lon - (sign_index as f64 * 30.0);

        let nak = Nakshatra::from_longitude(sid_lon);
        let pada = Nakshatra::pada_from_longitude(sid_lon);

        if b.name == "Moon" {
            moon_sidereal_lon = sid_lon;
        }

        planets.push(VedicPlanet {
            name: b.name.clone(),
            sidereal_longitude: sid_lon,
            sign_index,
            sign_degree,
            nakshatra_name: format!("{:?}", nak),
            nakshatra_pada: pada,
            is_retrograde: b.is_retrograde,
            latitude: b.latitude,
        });
    }

    // 3. Compute the 11 supported divisional charts
    let mut vargas = BTreeMap::new();
    for div in VargaDivision::ALL {
        let (v_lagna, _) = calculate_varga_sign_and_degree(lagna_sidereal, div);

        let mut placements = Vec::with_capacity(planets.len());
        for p in &planets {
            let (p_sign, p_degree) = calculate_varga_sign_and_degree(p.sidereal_longitude, div);
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

    VedicChart {
        lagna_sidereal,
        lagna_sign,
        wholesign_bhavas,
        planets,
        vargas,
        vimshottari_dashas,
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

        // Verify all 11 supported Varga divisions
        assert_eq!(chart.vargas.len(), 11);
        for div in VargaDivision::ALL {
            let v_chart = chart.vargas.get(&div).expect("varga must exist");
            assert_eq!(v_chart.placements.len(), 9);
            assert!(v_chart.lagna_sign <= 11);
        }

        // Verify Whole-sign Bhavas
        assert_eq!(chart.wholesign_bhavas.len(), 12);

        // Verify Planets and Nakshatras
        assert_eq!(chart.planets.len(), 9);

        // Verify Vimshottari Dasha periods continuity and Antardashas
        assert_eq!(chart.vimshottari_dashas.len(), 9);
        for dasha in &chart.vimshottari_dashas {
            assert_eq!(dasha.sub_periods.len(), 9, "Each Mahadasha must contain 9 Antardashas");
        }
    }

    #[test]
    fn test_varga_calculations_accuracy() {
        // D1 Rasi
        let (sign, deg) = calculate_varga_sign_and_degree(15.5, VargaDivision::D1);
        assert_eq!(sign, 0); // Aries
        assert!((deg - 15.5).abs() < 1e-6);

        // D2 Hora (Parashari BPHS: Sun/Moon dual sign distribution)
        // Aries (odd sign, sign_idx 0):
        // 7.5 deg -> 1st half -> Leo (4), degree = 15.0
        let (sign, deg) = calculate_varga_sign_and_degree(7.5, VargaDivision::D2);
        assert_eq!(sign, 4); // Leo
        assert!((deg - 15.0).abs() < 1e-6);

        // 20.0 deg -> 2nd half -> Cancer (3), degree = ((20 - 15) / 15) * 30 = 10.0
        let (sign, deg) = calculate_varga_sign_and_degree(20.0, VargaDivision::D2);
        assert_eq!(sign, 3); // Cancer
        assert!((deg - 10.0).abs() < 1e-6);

        // Taurus (even sign, sign_idx 1):
        // 35.0 deg (Taurus 5 deg) -> 1st half -> Cancer (3), degree = 10.0
        let (sign, deg) = calculate_varga_sign_and_degree(35.0, VargaDivision::D2);
        assert_eq!(sign, 3); // Cancer
        assert!((deg - 10.0).abs() < 1e-6);

        // 50.0 deg (Taurus 20 deg) -> 2nd half -> Leo (4), degree = 10.0
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

        // D6 Shashthamsha (Odd: same sign; Even: 7th sign)
        assert_eq!(calculate_varga_sign(2.0, VargaDivision::D6), 0); // Aries -> Aries
        assert_eq!(calculate_varga_sign(6.0, VargaDivision::D6), 1); // Aries -> Taurus
        assert_eq!(calculate_varga_sign(32.0, VargaDivision::D6), 7); // Taurus (even) -> Scorpio (1 + 6)
        assert_eq!(calculate_varga_sign(62.0, VargaDivision::D6), 2); // Gemini (odd) -> Gemini (2)

        // D7 Saptamsha (Odd: same; Even: 7th)
        assert_eq!(calculate_varga_sign(2.0, VargaDivision::D7), 0); // Aries
        assert_eq!(calculate_varga_sign(32.0, VargaDivision::D7), 7); // Taurus -> Scorpio

        // D8 Ashtamamsha (Movable: Aries, Fixed: Sag, Dual: Leo)
        assert_eq!(calculate_varga_sign(1.0, VargaDivision::D8), 0); // Aries (Movable) -> Aries
        assert_eq!(calculate_varga_sign(31.0, VargaDivision::D8), 8); // Taurus (Fixed) -> Sagittarius
        assert_eq!(calculate_varga_sign(61.0, VargaDivision::D8), 4); // Dual (Gemini) -> Leo

        // D9 Navamsha (3°20' divisions)
        assert_eq!(calculate_varga_sign(1.0, VargaDivision::D9), 0); // Aries
        assert_eq!(calculate_varga_sign(4.0, VargaDivision::D9), 1); // Taurus
        assert_eq!(calculate_varga_sign(31.0, VargaDivision::D9), 9); // Taurus 1 deg -> Capricorn

        // D10 Dashamsha (Odd: same, Even: 9th)
        assert_eq!(calculate_varga_sign(1.0, VargaDivision::D10), 0); // Aries
        assert_eq!(calculate_varga_sign(31.0, VargaDivision::D10), 9); // Taurus -> Capricorn (1 + 8)

        // D11 Rudramsha (Gemini: start Aquarius, count forward)
        // Gemini = sign_idx 2. start = (12 - 2) % 12 = 10 (Aquarius).
        // Part 0 (0..2.727 deg) -> Aquarius (10)
        assert_eq!(calculate_varga_sign(61.0, VargaDivision::D11), 10);
        // Part 1 (2.727..5.454 deg) -> Pisces (11)
        assert_eq!(calculate_varga_sign(63.0, VargaDivision::D11), 11);

        // D12 Dwadashamsha (Count from same sign)
        assert_eq!(calculate_varga_sign(1.0, VargaDivision::D12), 0); // Aries
        assert_eq!(calculate_varga_sign(3.0, VargaDivision::D12), 1); // Taurus
        assert_eq!(calculate_varga_sign(31.0, VargaDivision::D12), 1); // Taurus 1 deg -> Taurus
    }
}
