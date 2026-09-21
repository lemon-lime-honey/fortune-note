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
use vedaksha::vedic::varga::{varga_sign, VargaType};

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

/// Computes the divisional sign index (0..11) for any supported varga division.
pub fn calculate_varga_sign(sidereal_lon: f64, div: VargaDivision) -> u8 {
    let norm = normalize_degrees(sidereal_lon);
    let sign_idx = (norm / 30.0).floor() as u8;
    let pos_in_sign = norm - (sign_idx as f64 * 30.0);

    match div {
        VargaDivision::D1 => sign_idx,
        VargaDivision::D2 => varga_sign(norm, VargaType::Hora),
        VargaDivision::D3 => varga_sign(norm, VargaType::Drekkana),
        VargaDivision::D4 => varga_sign(norm, VargaType::Chaturthamsha),
        VargaDivision::D6 => {
            // Jaimini Shashthamsha: 5-degree segments (0..5)
            // Odd signs start from Aries (0), even signs start from Libra (6)
            let part = (pos_in_sign / 5.0).floor().min(5.0) as u8;
            let start = if sign_idx % 2 == 0 { 0 } else { 6 };
            (start + part) % 12
        }
        VargaDivision::D7 => varga_sign(norm, VargaType::Saptamsha),
        VargaDivision::D8 => {
            // Jaimini Ashtamamsha: 3.75-degree segments (0..7)
            // Movable signs start Aries (0), Fixed start Sagittarius (8), Dual start Leo (4)
            let part = (pos_in_sign / 3.75).floor().min(7.0) as u8;
            let start = match sign_idx % 3 {
                0 => 0, // Movable: Aries
                1 => 8, // Fixed: Sagittarius
                _ => 4, // Dual: Leo
            };
            (start + part) % 12
        }
        VargaDivision::D9 => varga_sign(norm, VargaType::Navamsha),
        VargaDivision::D10 => varga_sign(norm, VargaType::Dashamsha),
        VargaDivision::D11 => {
            // Jaimini Rudramsha: 30/11 degree segments (0..10)
            let part = (pos_in_sign / (30.0 / 11.0)).floor().min(10.0) as u8;
            let start = (12 - sign_idx) % 12;
            (start + 12 - (part % 12)) % 12
        }
        VargaDivision::D12 => varga_sign(norm, VargaType::Dwadashamsha),
    }
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
}

/// Single planet placement within a divisional chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VargaPlacement {
    pub planet_name: String,
    pub sign_index: u8,       // 0 .. 11
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

    let mut wholesign_bhavas = [0.0; 12];
    for h in 0..12 {
        let sign = (lagna_sign as usize + h) % 12;
        wholesign_bhavas[h] = sign as f64 * 30.0;
    }

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
        });
    }

    // 3. Compute the 11 supported divisional charts
    let mut vargas = BTreeMap::new();
    for div in VargaDivision::ALL {
        let v_lagna = calculate_varga_sign(lagna_sidereal, div);

        let mut placements = Vec::with_capacity(planets.len());
        for p in &planets {
            let p_sign = calculate_varga_sign(p.sidereal_longitude, div);
            let house = ((p_sign as i16 - v_lagna as i16 + 12) % 12 + 1) as u8;

            placements.push(VargaPlacement {
                planet_name: p.name.clone(),
                sign_index: p_sign,
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
}
