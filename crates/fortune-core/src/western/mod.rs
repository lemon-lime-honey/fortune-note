//! Western astrology calculation layer (Tropical Zodiac).
//!
//! Provides deterministic computation for:
//! - 12 house cusps with Ascendant and Midheaven (WholeSign for Hellenistic, Regiomontanus for Medieval)
//! - Planetary house placements in the tropical framework
//! - Major Ptolemaic aspects
//! - Strictly separated Hellenistic (Whole Sign) and Medieval (Regiomontanus) traditional engines

use serde::{Deserialize, Serialize};
pub use vedaksha::astro::houses::{compute_houses, HouseSystem};

use crate::ephemeris::EphemerisSnapshot;

pub mod almuten;
pub mod antiscia;
pub mod aspects;
pub mod dignity;
pub mod fixed_stars;
pub mod hellenistic;
pub mod hours;
pub mod moon;
pub mod parts;
pub mod predictive;
pub mod reception;
pub mod syzygy;
pub mod traditional;

pub use hellenistic::{HellenisticAnglePoints, HellenisticData, HellenisticInput, HellenisticPlanetCondition};
pub use traditional::WesternTraditionalData;

/// Supported Western house division systems (strictly Hellenistic Whole Sign vs Medieval Regiomontanus).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WesternHouseSystem {
    #[default]
    Regiomontanus,
    WholeSign,
}

impl WesternHouseSystem {
    pub fn to_vedaksha_system(self) -> HouseSystem {
        match self {
            Self::WholeSign => HouseSystem::WholeSign,
            Self::Regiomontanus => HouseSystem::Regiomontanus,
        }
    }

    pub fn from_str_name(name: &str) -> Option<Self> {
        match name.trim().to_lowercase().as_str() {
            "wholesign" | "whole_sign" | "whole" | "hellenistic" | "헬레니즘" => Some(Self::WholeSign),
            "regiomontanus" | "regio" | "medieval" | "중세" => Some(Self::Regiomontanus),
            _ => None,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::WholeSign => "Whole Sign",
            Self::Regiomontanus => "Regiomontanus",
        }
    }
}

/// Planet placement in the Western tropical chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WesternPlanet {
    pub name: String,
    pub longitude: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub house_number: u8,
    pub is_retrograde: bool,
    #[serde(default)]
    pub speed: f64,
    #[serde(default)]
    pub latitude: f64,
}

/// Lunar node placement in the Western tropical chart (Dragon's Head and Tail).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WesternNode {
    pub name: String,
    pub longitude: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub house_number: u8,
    pub is_retrograde: bool,
}

/// Planetary aspect relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WesternAspect {
    pub planet1: String,
    pub planet2: String,
    pub aspect_type: String,
    pub exact_angle: f64,
    pub actual_angle: f64,
    pub orb: f64,
    #[serde(default)]
    pub is_applying: bool,
}

/// Complete Western astrology chart output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WesternChart {
    pub ascendant: f64,
    pub midheaven: f64,
    pub house_cusps: [f64; 12],
    pub house_system: WesternHouseSystem,
    pub planets: Vec<WesternPlanet>,
    pub nodes: Vec<WesternNode>,
    pub aspects: Vec<WesternAspect>,
    pub traditional: Option<WesternTraditionalData>,
    pub hellenistic: Option<HellenisticData>,
}

const DEFAULT_MAJOR_ASPECTS: [(&str, f64, f64); 5] = [
    ("Conjunction", 0.0, 8.0),
    ("Sextile", 60.0, 6.0),
    ("Square", 90.0, 7.0),
    ("Trine", 120.0, 8.0),
    ("Opposition", 180.0, 8.0),
];

pub(crate) const DEFAULT_FIXED_STAR_MAX_ORB: f64 = 2.0;

/// Determine house number (1..=12) for a given tropical longitude based on house cusps.
pub fn determine_house(longitude: f64, cusps: &[f64; 12]) -> u8 {
    let lon = longitude.rem_euclid(360.0);
    for i in 0..12 {
        let start = cusps[i].rem_euclid(360.0);
        let end = cusps[(i + 1) % 12].rem_euclid(360.0);
        let in_house = if start < end {
            lon >= start && lon < end
        } else {
            lon >= start || lon < end
        };
        if in_house {
            return (i + 1) as u8;
        }
    }
    1
}

/// Determine traditional house number with William Lilly's 5-degree rule for quadrant systems.
pub fn determine_house_traditional(
    longitude: f64,
    cusps: &[f64; 12],
    system: WesternHouseSystem,
) -> u8 {
    let base_house = determine_house(longitude, cusps);
    if matches!(system, WesternHouseSystem::WholeSign) {
        return base_house;
    }

    let next_house_idx = (base_house as usize) % 12;
    let next_cusp = cusps[next_house_idx].rem_euclid(360.0);
    let lon = longitude.rem_euclid(360.0);
    let dist_to_next = (next_cusp - lon).rem_euclid(360.0);

    if dist_to_next > 0.0 && dist_to_next <= 5.0 {
        (next_house_idx + 1) as u8
    } else {
        base_house
    }
}

fn collect_planets_and_nodes(
    ephem: &EphemerisSnapshot,
    cusps: &[f64; 12],
) -> (Vec<WesternPlanet>, Vec<WesternNode>) {
    let mut planets = Vec::with_capacity(7);
    let mut nodes = Vec::with_capacity(2);

    for b in &ephem.bodies {
        let lon = b.tropical_longitude;
        let sign_index = (lon / 30.0).floor() as u8 % 12;
        let sign_degree = lon - (sign_index as f64 * 30.0);
        let house_number = determine_house(lon, cusps);

        match b.name.as_str() {
            "Rahu" | "Ketu" => nodes.push(WesternNode {
                name: crate::celestial::normalize_western_body_name(&b.name),
                longitude: lon,
                sign_index,
                sign_degree,
                house_number,
                is_retrograde: b.is_retrograde,
            }),
            _ => planets.push(WesternPlanet {
                name: b.name.clone(),
                longitude: lon,
                sign_index,
                sign_degree,
                house_number,
                is_retrograde: b.is_retrograde,
                speed: b.speed,
                latitude: b.latitude,
            }),
        }
    }

    (planets, nodes)
}

fn compute_major_aspects(planets: &[WesternPlanet]) -> Vec<WesternAspect> {
    let mut aspects = Vec::new();
    for i in 0..planets.len() {
        for j in (i + 1)..planets.len() {
            let (p1, p2) = (&planets[i], &planets[j]);
            let diff = (p1.longitude - p2.longitude).abs().rem_euclid(360.0);
            let shortest = if diff > 180.0 { 360.0 - diff } else { diff };

            for (name, target, max_orb) in DEFAULT_MAJOR_ASPECTS {
                let orb = (shortest - target).abs();
                if orb <= max_orb {
                    aspects.push(WesternAspect {
                        planet1: p1.name.clone(),
                        planet2: p2.name.clone(),
                        aspect_type: name.to_string(),
                        exact_angle: target,
                        actual_angle: shortest,
                        orb,
                        is_applying: aspects::is_aspect_applying(
                            p1.longitude,
                            p1.speed,
                            p2.longitude,
                            p2.speed,
                            target,
                        ),
                    });
                }
            }
        }
    }
    aspects
}

fn build_domain_analysis(
    ctx: &traditional::TraditionalBuildContext<'_>,
) -> (Option<WesternTraditionalData>, Option<HellenisticData>) {
    if ctx.system == WesternHouseSystem::WholeSign {
        let h_input = HellenisticInput {
            planet_tuples: ctx.planet_tuples,
            planet_pos_tuples: ctx.planet_pos_tuples,
            ascendant: ctx.ascendant,
            midheaven: ctx.midheaven,
            cusps: ctx.cusps,
            is_day_chart: ctx.is_day_chart,
            is_male: ctx.is_male,
            jd_ut1: ctx.ephem.jd_ut1,
            jd_tt: ctx.ephem.jd_tt,
            observer_lat: ctx.latitude,
            obliquity_deg: ctx.ephem.true_obliquity_deg,
            fixed_star_max_orb: DEFAULT_FIXED_STAR_MAX_ORB,
        };
        (None, Some(hellenistic::calculate_hellenistic_data(&h_input)))
    } else {
        (Some(traditional::build_traditional_data(ctx)), None)
    }
}

/// Calculate Western astrology chart (Natal default mode: `is_horary = false`).
pub fn calculate_western(
    ephem: &EphemerisSnapshot,
    latitude: f64,
    system: WesternHouseSystem,
    is_male: bool,
) -> WesternChart {
    calculate_western_with_mode(ephem, latitude, system, is_male, false)
}

/// Calculate Western astrology chart with explicit Natal (`is_horary = false`) or Horary (`is_horary = true`) mode.
pub fn calculate_western_with_mode(
    ephem: &EphemerisSnapshot,
    latitude: f64,
    system: WesternHouseSystem,
    is_male: bool,
    is_horary: bool,
) -> WesternChart {
    let house_result = compute_houses(
        ephem.ramc,
        latitude,
        ephem.true_obliquity_deg,
        system.to_vedaksha_system(),
    );
    let (cusps, ascendant, midheaven) = (house_result.cusps, house_result.asc, house_result.mc);
    let (planets, nodes) = collect_planets_and_nodes(ephem, &cusps);
    let aspects = compute_major_aspects(&planets);

    let planet_tuples: Vec<(&str, f64, f64)> = ephem
        .bodies
        .iter()
        .filter(|b| b.name != "Rahu" && b.name != "Ketu")
        .map(|b| (b.name.as_str(), b.tropical_longitude, b.speed))
        .collect();
    let planet_pos_tuples: Vec<(&str, f64)> = planet_tuples.iter().map(|&(n, lon, _)| (n, lon)).collect();

    let sun_lon = ephem.get_body("Sun").map_or(0.0, |b| b.tropical_longitude);
    let is_day_chart = (sun_lon - ascendant).rem_euclid(360.0) >= 180.0;

    let ctx = traditional::TraditionalBuildContext {
        ephem,
        planets: &planets,
        planet_tuples: &planet_tuples,
        planet_pos_tuples: &planet_pos_tuples,
        cusps: &cusps,
        ascendant,
        midheaven,
        latitude,
        system,
        is_day_chart,
        is_male,
        is_horary,
    };
    let (traditional, hellenistic) = build_domain_analysis(&ctx);

    WesternChart {
        ascendant,
        midheaven,
        house_cusps: cusps,
        house_system: system,
        planets,
        nodes,
        aspects,
        traditional,
        hellenistic,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::celestial::{CalendarDateTime, GeoCoordinate};
    use crate::ephemeris::compute_ephemeris;

    const SEOUL: GeoCoordinate = GeoCoordinate::new(37.5665, 126.9780);

    fn chart_for(
        dt: CalendarDateTime,
        loc: GeoCoordinate,
        system: WesternHouseSystem,
        is_male: bool,
    ) -> WesternChart {
        let ephem = compute_ephemeris(dt, loc).expect("ephem should succeed");
        calculate_western(&ephem, loc.latitude, system, is_male)
    }

    fn assert_basic_western_shape(chart: &WesternChart) {
        assert_eq!(chart.house_cusps.len(), 12);
        assert_eq!(chart.planets.len(), 7);
        assert_eq!(chart.nodes.len(), 2);
    }

    #[test]
    fn test_western_chart_generation_and_high_latitude() {
        let chart = chart_for(
            CalendarDateTime::new(2024, 3, 20, 12, 0, 0.0),
            GeoCoordinate::new(51.5074, -0.1278),
            WesternHouseSystem::Regiomontanus,
            true,
        );
        assert_basic_western_shape(&chart);

        let trad = chart.traditional.as_ref().expect("Traditional data must exist");
        assert_eq!(trad.planet_dignities.len(), 7);
        assert_eq!(trad.cusp_dignities.len(), 12);
        assert!(!trad.fixed_stars.is_empty());
        assert!(!trad.arabic_parts.is_empty());
        assert!(!trad.fixed_star_conjunctions.iter().any(|c| c.body_name == "Immum Coeli"));

        let polar = chart_for(
            CalendarDateTime::new(2024, 6, 21, 12, 0, 0.0),
            GeoCoordinate::new(70.0, 25.0),
            WesternHouseSystem::Regiomontanus,
            true,
        );
        assert_basic_western_shape(&polar);
    }

    #[test]
    fn test_medieval_precision_and_marriage_gender_branch() {
        let j2000 = CalendarDateTime::new(2000, 1, 1, 3, 0, 0.0);
        let male = chart_for(j2000, SEOUL, WesternHouseSystem::Regiomontanus, true);
        let female = chart_for(j2000, SEOUL, WesternHouseSystem::Regiomontanus, false);

        assert!(male.hellenistic.is_none());
        let trad = male.traditional.as_ref().expect("traditional data should exist");
        assert_eq!(trad.planetary_hours.day_ruler, "Saturn");
        assert!(trad.planetary_hours.is_day_hour);
        assert_eq!(trad.moon_condition.lunar_phase_name, "Waning Crescent");
        let san = trad.prenatal_syzygy.as_ref().expect("SAN should exist");
        assert_eq!(san.syzygy_type, "Full Moon");
        assert!(san.exact_datetime_utc.starts_with("1999-12-22"));
        assert!(trad.arabic_parts.len() >= 10);
        assert_eq!(trad.fixed_stars.len(), 31);

        let male_m = trad.arabic_parts.iter().find(|p| p.name == "Part of Marriage").unwrap();
        let female_m = female.traditional.as_ref().unwrap().arabic_parts
            .iter().find(|p| p.name == "Part of Marriage").unwrap();
        assert_ne!(male_m.longitude, female_m.longitude);
        let sym = (male_m.longitude + female_m.longitude - 2.0 * male.ascendant).rem_euclid(360.0);
        assert!(sym.abs() < 1e-6 || (360.0 - sym).abs() < 1e-6);
    }

    #[test]
    fn test_hellenistic_whole_sign_and_day_sect() {
        let oslo = chart_for(
            CalendarDateTime::new(1990, 1, 1, 5, 0, 0.0),
            GeoCoordinate::new(60.0, 10.0),
            WesternHouseSystem::WholeSign,
            true,
        );
        assert!(oslo.traditional.is_none());
        let h = oslo.hellenistic.as_ref().expect("Hellenistic data must exist");
        assert_eq!((h.planet_conditions.len(), h.angles.ascendant_sign, h.angles.midheaven_house), (7, 7, 12));
        assert!(!h.angles.is_mc_in_10th);

        let solstice = chart_for(
            CalendarDateTime::new(2024, 6, 20, 21, 0, 0.0),
            SEOUL,
            WesternHouseSystem::WholeSign,
            true,
        );
        let sh = solstice.hellenistic.expect("Hellenistic data must exist");
        let sun = solstice.planets.iter().find(|p| p.name == "Sun").unwrap();
        let moon = solstice.planets.iter().find(|p| p.name == "Moon").unwrap();
        assert_eq!(sun.house_number, 1);

        let p_fortune = sh.lots.iter().find(|l| l.name == "Part of Fortune").unwrap();
        let p_spirit = sh.lots.iter().find(|l| l.name == "Part of Spirit").unwrap();
        assert!((p_fortune.longitude - (solstice.ascendant + moon.longitude - sun.longitude).rem_euclid(360.0)).abs() < 1e-6);
        assert!((p_spirit.longitude - (solstice.ascendant + sun.longitude - moon.longitude).rem_euclid(360.0)).abs() < 1e-6);
    }
}
