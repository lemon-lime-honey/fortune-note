//! Vedic astrology (Jyotish) calculation layer (Sidereal Zodiac).
//!
//! Provides deterministic computation for:
//! - Sidereal Lagna (Ascendant) and 12 Whole Sign Bhavas
//! - 19 divisional charts (Shodashavarga + D6, D8, D11) with varga lagnas and planetary houses
//! - 27 Nakshatras with 4 Padas
//! - Classical Parashari planetary dignities (Panchadha Maitri) and Combustion
//! - 7 Chara Karakas, BPHS Graha Drishti, 12 Arudha Padas, Ashtakavarga, and 5-limb Panchanga
//! - Vimshottari Dasha hierarchy (Solar 365.2425-day and Savana 360-day years)

pub mod analysis;
pub mod dignity;
pub mod vargas;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use vedaksha::astro::houses::{compute_houses, HouseSystem};
use vedaksha::math::angle::normalize_degrees;
use vedaksha::vedic::dasha::vimshottari::DashaPeriod;
use vedaksha::vedic::graha::Graha;
use vedaksha::vedic::nakshatra::Nakshatra;

use crate::ephemeris::EphemerisSnapshot;
use dignity::{compute_classical_vedic_combustion, compute_vedic_dignity, parse_graha};
pub use vargas::{calculate_varga_sign, calculate_varga_sign_and_degree, VargaDivision};

/// Year length basis for Vimshottari Dasha calculations (BPHS / Saravali vs Modern Solar).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DashaYearType {
    #[default]
    Solar365_2425,
    Savana360,
}

/// Configuration options for Vedic chart calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct VedicOptions {
    #[serde(default)]
    pub dasha_year_type: DashaYearType,
}

/// Planet placement in the sidereal framework.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicPlanet {
    pub name: String,
    pub sidereal_longitude: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub nakshatra_name: String,
    pub nakshatra_pada: u8,
    pub is_retrograde: bool,
    #[serde(default)]
    pub latitude: f64,
    #[serde(default)]
    pub house: u8,
    #[serde(default)]
    pub dignity: String,
    #[serde(default)]
    pub combustion: String,
}

/// Single planet placement within a divisional chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VargaPlacement {
    pub planet_name: String,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub house: u8,
}

/// Varga chart details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VargaChart {
    pub division: VargaDivision,
    pub lagna_sign: u8,
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

/// Jaimini Chara Karaka assignment.
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
    pub aspect_offset: u8,
    pub target_sign: u8,
    pub target_house: u8,
    pub aspected_planets: Vec<String>,
}

/// Arudha Pada for a Bhava (A1..A12).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicArudhaPada {
    pub bhava: u8,
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
    pub tithi_number: u8,
    pub paksha: String,
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

fn build_vedic_planets(
    ephem: &EphemerisSnapshot,
    lagna_sign: u8,
    sun_sidereal_lon: f64,
    planet_signs: &HashMap<Graha, u8>,
) -> Vec<VedicPlanet> {
    ephem
        .bodies
        .iter()
        .map(|b| {
            let sid_lon = normalize_degrees(b.sidereal_longitude);
            let sign_index = ((sid_lon / 30.0).floor() as u8).min(11);
            let sign_degree = (sid_lon - (sign_index as f64 * 30.0)).clamp(0.0, 30.0);
            let house = ((sign_index as i16 - lagna_sign as i16 + 12) % 12 + 1) as u8;
            let nak = Nakshatra::from_longitude(sid_lon);
            let (dignity, combustion) = parse_graha(&b.name).map_or_else(
                || ("Neutral".to_string(), "None".to_string()),
                |g| (
                    compute_vedic_dignity(g, sign_index, sign_degree, planet_signs),
                    compute_classical_vedic_combustion(g, sid_lon, sun_sidereal_lon, b.is_retrograde),
                ),
            );
            VedicPlanet {
                name: b.name.clone(),
                sidereal_longitude: sid_lon,
                sign_index,
                sign_degree,
                nakshatra_name: nak.name().to_string(),
                nakshatra_pada: Nakshatra::pada_from_longitude(sid_lon),
                is_retrograde: b.is_retrograde,
                latitude: b.latitude,
                house,
                dignity,
                combustion,
            }
        })
        .collect()
}

/// Calculate Vedic chart from an ephemeris snapshot and observer latitude (Solar 365.2425-day Dasha).
pub fn calculate_vedic(ephem: &EphemerisSnapshot, latitude: f64) -> VedicChart {
    calculate_vedic_with_options(ephem, latitude, VedicOptions::default())
}

/// Calculate Vedic chart with explicit configuration options.
pub fn calculate_vedic_with_options(
    ephem: &EphemerisSnapshot,
    latitude: f64,
    options: VedicOptions,
) -> VedicChart {
    let house_result = compute_houses(ephem.ramc, latitude, ephem.true_obliquity_deg, HouseSystem::WholeSign);
    let lagna_sidereal = normalize_degrees(house_result.asc - ephem.true_ayanamsha_deg);
    let lagna_sign = ((lagna_sidereal / 30.0).floor() as u8).min(11);
    let wholesign_bhavas = core::array::from_fn(|h| ((lagna_sign as usize + h) % 12) as f64 * 30.0);

    let mut planet_signs: HashMap<Graha, u8> = HashMap::new();
    let mut planet_lons: HashMap<Graha, f64> = HashMap::new();
    for b in &ephem.bodies {
        let sid_lon = normalize_degrees(b.sidereal_longitude);
        if let Some(g) = parse_graha(&b.name) {
            planet_signs.insert(g, ((sid_lon / 30.0).floor() as u8).min(11));
            planet_lons.insert(g, sid_lon);
        }
    }

    let sun_trop = ephem.get_body("Sun").map_or(0.0, |b| normalize_degrees(b.tropical_longitude));
    let sun_sid = planet_lons.get(&Graha::Sun).copied().unwrap_or(0.0);
    let moon_sid = planet_lons.get(&Graha::Moon).copied().unwrap_or(0.0);
    let rahu_sid = planet_lons.get(&Graha::Rahu).copied().unwrap_or(0.0);

    let planets = build_vedic_planets(ephem, lagna_sign, sun_sid, &planet_signs);
    let vargas = vargas::build_all_varga_charts(lagna_sidereal, rahu_sid, &planets);

    VedicChart {
        lagna_sidereal,
        lagna_sign,
        wholesign_bhavas,
        vargas,
        vimshottari_dashas: analysis::compute_vimshottari_dashas(moon_sid, ephem.jd_ut1, options.dasha_year_type),
        chara_karakas: analysis::compute_chara_karakas(&planets),
        drishti: analysis::compute_graha_drishti(&planets, lagna_sign),
        arudha_padas: analysis::compute_arudha_padas(lagna_sign, &planet_signs, &planet_lons),
        ashtakavarga: analysis::compute_ashtakavarga(&planet_signs, lagna_sign),
        panchanga: analysis::compute_panchanga(ephem, latitude, sun_trop, sun_sid, moon_sid),
        planets,
        ayanamsha_name: default_ayanamsha_name(),
        ayanamsha_deg: ephem.true_ayanamsha_deg,
    }
}
