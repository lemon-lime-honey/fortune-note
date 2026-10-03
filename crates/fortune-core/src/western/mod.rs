//! Western astrology calculation layer (Tropical Zodiac).
//!
//! Provides deterministic computation for:
//! - 12 house cusps with Ascendant and Midheaven (Placidus, Regiomontanus, Koch, Equal, WholeSign, Campanus)
//! - Planetary house placements in the tropical framework
//! - Major Ptolemaic aspects
//! - William Lilly's Christian Astrology traditional domain engine

use serde::{Deserialize, Serialize};
pub use vedaksha::astro::houses::{compute_houses, HouseSystem};

use crate::ephemeris::EphemerisSnapshot;

pub mod almuten;
pub mod antiscia;
pub mod aspects;
pub mod dignity;
pub mod fixed_stars;
pub mod hours;
pub mod moon;
pub mod parts;
pub mod predictive;
pub mod reception;
pub mod syzygy;

/// Supported Western house division systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WesternHouseSystem {
    #[default]
    Regiomontanus,
    Placidus,
    Koch,
    Equal,
    WholeSign,
    Campanus,
}

impl WesternHouseSystem {
    pub fn to_vedaksha_system(self) -> HouseSystem {
        match self {
            Self::Placidus => HouseSystem::Placidus,
            Self::Koch => HouseSystem::Koch,
            Self::Equal => HouseSystem::Equal,
            Self::WholeSign => HouseSystem::WholeSign,
            Self::Campanus => HouseSystem::Campanus,
            Self::Regiomontanus => HouseSystem::Regiomontanus,
        }
    }

    pub fn from_str_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "placidus" => Some(Self::Placidus),
            "koch" => Some(Self::Koch),
            "equal" => Some(Self::Equal),
            "wholesign" | "whole_sign" | "whole" => Some(Self::WholeSign),
            "campanus" => Some(Self::Campanus),
            "regiomontanus" => Some(Self::Regiomontanus),
            _ => None,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Placidus => "Placidus",
            Self::Koch => "Koch",
            Self::Equal => "Equal",
            Self::WholeSign => "Whole Sign",
            Self::Campanus => "Campanus",
            Self::Regiomontanus => "Regiomontanus",
        }
    }
}

/// Planet placement in the Western tropical chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WesternPlanet {
    pub name: String,
    pub longitude: f64,       // 0.0 .. 360.0
    pub sign_index: u8,       // 0 = Aries .. 11 = Pisces
    pub sign_degree: f64,     // 0.0 .. 30.0
    pub house_number: u8,     // 1 .. 12
    pub is_retrograde: bool,
    #[serde(default)]
    pub speed: f64,
    #[serde(default)]
    pub latitude: f64,
}

/// Lunar node placement in the Western tropical chart (Dragon's Head and Tail).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WesternNode {
    pub name: String,         // "North Node", "South Node"
    pub longitude: f64,       // 0.0 .. 360.0
    pub sign_index: u8,       // 0 = Aries .. 11 = Pisces
    pub sign_degree: f64,     // 0.0 .. 30.0
    pub house_number: u8,     // 1 .. 12
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
}

/// Complete William Lilly Traditional Astrology Analysis Data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WesternTraditionalData {
    pub planet_dignities: Vec<dignity::PlanetDignityReport>,
    pub moon_condition: moon::MoonCondition,
    pub planetary_hours: hours::PlanetaryHoursReport,
    pub traditional_aspects: Vec<aspects::TraditionalAspect>,
    pub antiscia: Vec<antiscia::AntiscionPoint>,
    pub mutual_receptions: Vec<reception::MutualReception>,
    pub perfection: reception::PerfectionReport,
    pub cusp_dignities: Vec<almuten::HouseCuspDignity>,
    pub arabic_parts: Vec<parts::ArabicPart>,
    pub fixed_stars: Vec<fixed_stars::FixedStarInfo>,
    pub fixed_star_conjunctions: Vec<fixed_stars::FixedStarConjunction>,
    pub prenatal_syzygy: Option<syzygy::PrenatalSyzygy>,
}

/// Complete Western chart result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WesternChart {
    pub ascendant: f64,
    pub midheaven: f64,
    pub house_cusps: [f64; 12],
    pub house_system: WesternHouseSystem,
    pub planets: Vec<WesternPlanet>,
    pub nodes: Vec<WesternNode>,
    pub aspects: Vec<WesternAspect>,
    #[serde(default)]
    pub traditional: Option<WesternTraditionalData>,
}

/// Determine which house a given longitude falls into based on 12 cusp degrees.
pub fn determine_house(lon: f64, cusps: &[f64; 12]) -> u8 {
    for h in 0..12 {
        let cusp_curr = cusps[h];
        let cusp_next = cusps[(h + 1) % 12];

        let inside = if cusp_curr <= cusp_next {
            lon >= cusp_curr && lon < cusp_next
        } else {
            lon >= cusp_curr || lon < cusp_next
        };

        if inside {
            return (h + 1) as u8;
        }
    }
    1 // Fallback
}

/// Calculate Western astrology chart from an ephemeris snapshot and observer latitude.
pub fn calculate_western(
    ephem: &EphemerisSnapshot,
    latitude: f64,
    system: WesternHouseSystem,
) -> WesternChart {
    let house_result = compute_houses(
        ephem.ramc,
        latitude,
        ephem.true_obliquity_deg,
        system.to_vedaksha_system(),
    );

    let cusps = house_result.cusps;
    let ascendant = house_result.asc;
    let midheaven = house_result.mc;

    // 1. Separate 7 classical planets and 2 lunar nodes (Dragon's Head & Tail)
    let mut planets = Vec::with_capacity(7);
    let mut nodes = Vec::with_capacity(2);

    for b in &ephem.bodies {
        let lon = b.tropical_longitude;
        let sign_index = (lon / 30.0).floor() as u8;
        let sign_degree = lon - (sign_index as f64 * 30.0);
        let house_number = determine_house(lon, &cusps);

        match b.name.as_str() {
            "Rahu" => {
                nodes.push(WesternNode {
                    name: "North Node".to_string(),
                    longitude: lon,
                    sign_index,
                    sign_degree,
                    house_number,
                    is_retrograde: b.is_retrograde,
                });
            }
            "Ketu" => {
                nodes.push(WesternNode {
                    name: "South Node".to_string(),
                    longitude: lon,
                    sign_index,
                    sign_degree,
                    house_number,
                    is_retrograde: b.is_retrograde,
                });
            }
            _ => {
                planets.push(WesternPlanet {
                    name: b.name.clone(),
                    longitude: lon,
                    sign_index,
                    sign_degree,
                    house_number,
                    is_retrograde: b.is_retrograde,
                    speed: b.speed,
                    latitude: b.latitude,
                });
            }
        }
    }

    // 2. Compute major aspects exclusively among the 7 classical planets
    let mut aspects = Vec::new();
    let aspect_defs = [
        ("Conjunction", 0.0, 8.0),
        ("Sextile", 60.0, 6.0),
        ("Square", 90.0, 7.0),
        ("Trine", 120.0, 8.0),
        ("Opposition", 180.0, 8.0),
    ];

    for i in 0..planets.len() {
        for j in (i + 1)..planets.len() {
            let p1 = &planets[i];
            let p2 = &planets[j];

            let diff = (p1.longitude - p2.longitude).abs();
            let shortest = if diff > 180.0 { 360.0 - diff } else { diff };

            for (name, target, max_orb) in aspect_defs {
                let orb = (shortest - target).abs();
                if orb <= max_orb {
                    aspects.push(WesternAspect {
                        planet1: p1.name.clone(),
                        planet2: p2.name.clone(),
                        aspect_type: name.to_string(),
                        exact_angle: target,
                        actual_angle: shortest,
                        orb,
                    });
                }
            }
        }
    }

    // 3. William Lilly Traditional Astrology calculations
    let planet_tuples: Vec<(&str, f64, f64)> = ephem.bodies
        .iter()
        .filter(|b| b.name != "Rahu" && b.name != "Ketu")
        .map(|b| (b.name.as_str(), b.tropical_longitude, b.speed))
        .collect();

    let planet_pos_tuples: Vec<(&str, f64)> = planet_tuples
        .iter()
        .map(|&(n, lon, _)| (n, lon))
        .collect();

    let sun_opt = ephem.bodies.iter().find(|b| b.name == "Sun");
    let moon_opt = ephem.bodies.iter().find(|b| b.name == "Moon");
    let sun_lon = sun_opt.map_or(0.0, |b| b.tropical_longitude);
    let moon_lon = moon_opt.map_or(0.0, |b| b.tropical_longitude);
    let moon_speed = moon_opt.map_or(13.176, |b| b.speed);

    let sun_house = determine_house(sun_lon, &cusps);
    let is_day_chart = matches!(sun_house, 7 | 8 | 9 | 10 | 11 | 12);

    let saturn_house = ephem.bodies.iter()
        .find(|b| b.name == "Saturn")
        .map_or(1, |b| determine_house(b.tropical_longitude, &cusps));

    let north_node_lon = ephem.bodies.iter().find(|b| b.name == "Rahu").map_or(0.0, |b| b.tropical_longitude);
    let south_node_lon = ephem.bodies.iter().find(|b| b.name == "Ketu").map_or(180.0, |b| b.tropical_longitude);

    let mercury_lon = ephem.bodies.iter().find(|b| b.name == "Mercury").map_or(0.0, |b| b.tropical_longitude);
    let venus_lon = ephem.bodies.iter().find(|b| b.name == "Venus").map_or(0.0, |b| b.tropical_longitude);
    let mars_lon = ephem.bodies.iter().find(|b| b.name == "Mars").map_or(0.0, |b| b.tropical_longitude);
    let jupiter_lon = ephem.bodies.iter().find(|b| b.name == "Jupiter").map_or(0.0, |b| b.tropical_longitude);
    let saturn_lon = ephem.bodies.iter().find(|b| b.name == "Saturn").map_or(0.0, |b| b.tropical_longitude);

    // 3.1 Dignities
    let mut planet_dignities = Vec::new();
    for p in &planets {
        if let Some(cp) = dignity::ClassicalPlanet::from_name(&p.name) {
            planet_dignities.push(dignity::calculate_planet_dignity(
                cp,
                p.longitude,
                p.speed,
                p.house_number,
                sun_lon,
                is_day_chart,
                north_node_lon,
                south_node_lon,
            ));
        }
    }

    // 3.2 Moon Condition
    let moon_condition = moon::evaluate_moon_condition(
        moon_lon,
        moon_speed,
        sun_lon,
        &planet_tuples,
    );

    // 3.3 Planetary Hours & Radicality (Astronomical Hour Angle & True Sunrise/Sunset)
    let planetary_hours = hours::calculate_astronomical_planetary_hours(
        ephem.jd_ut1,
        latitude,
        ephem.ramc,
        sun_lon,
        ephem.true_obliquity_deg,
        ascendant,
        saturn_house,
        moon_condition.is_void_of_course,
        moon_condition.is_via_combusta,
    );

    // 3.4 Traditional Dynamic Aspects
    let traditional_aspects = aspects::calculate_traditional_aspects(&planet_tuples);

    // 3.5 Antiscia Table
    let mut antiscia_input = planet_pos_tuples.clone();
    antiscia_input.push(("Ascendant", ascendant));
    antiscia_input.push(("Midheaven", midheaven));
    let antiscia = antiscia::calculate_antiscia_table(&antiscia_input);

    // 3.6 Mutual Receptions & Perfection
    let mutual_receptions = reception::find_mutual_receptions(&planet_pos_tuples, is_day_chart);
    let perfection = reception::analyze_perfection_mechanisms(&planet_tuples);

    // 3.7 House Cusp Dignities
    let cusp_dignities = almuten::calculate_house_cusp_dignities(&cusps, is_day_chart);

    // 3.8 Arabic Parts
    let arabic_parts = parts::calculate_arabic_parts(
        ascendant,
        sun_lon,
        moon_lon,
        mercury_lon,
        venus_lon,
        mars_lon,
        jupiter_lon,
        saturn_lon,
        &cusps,
        is_day_chart,
        true,
    );

    // 3.9 Fixed Stars
    let fixed_stars = fixed_stars::calculate_traditional_fixed_stars(ephem.jd_tt);
    let mut star_check_bodies = planet_pos_tuples.clone();
    star_check_bodies.push(("Ascendant", ascendant));
    star_check_bodies.push(("Midheaven", midheaven));
    let fixed_star_conjunctions = fixed_stars::detect_fixed_star_conjunctions(&fixed_stars, &star_check_bodies, 1.5);
    // 3.10 Prenatal Syzygy (SAN)
    let prenatal_syzygy = syzygy::calculate_prenatal_syzygy_with_coords(ephem.jd_ut1, sun_lon, moon_lon).ok();

    let traditional = Some(WesternTraditionalData {
        planet_dignities,
        moon_condition,
        planetary_hours,
        traditional_aspects,
        antiscia,
        mutual_receptions,
        perfection,
        cusp_dignities,
        arabic_parts,
        fixed_stars,
        fixed_star_conjunctions,
        prenatal_syzygy,
    });

    WesternChart {
        ascendant,
        midheaven,
        house_cusps: cusps,
        house_system: system,
        planets,
        nodes,
        aspects,
        traditional,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ephemeris::compute_ephemeris;

    #[test]
    fn test_western_chart_generation() {
        let ephem = compute_ephemeris(2024, 3, 20, 12, 0, 0.0, 51.5074, -0.1278) // London
            .expect("ephem should succeed");

        let chart = calculate_western(&ephem, 51.5074, WesternHouseSystem::Regiomontanus);

        assert_eq!(chart.house_cusps.len(), 12);
        assert_eq!(chart.planets.len(), 7, "Strictly 7 classical planets in western chart");
        assert_eq!(chart.nodes.len(), 2, "Strictly 2 lunar nodes (North Node & South Node)");
        assert!(chart.traditional.is_some(), "Traditional data must be calculated");

        let trad = chart.traditional.as_ref().unwrap();
        assert_eq!(trad.planet_dignities.len(), 7, "7 classical planets have dignities");
        assert_eq!(trad.cusp_dignities.len(), 12, "12 cusps have dignities");
        assert!(!trad.fixed_stars.is_empty(), "Fixed stars must be populated");
        assert!(!trad.arabic_parts.is_empty(), "Arabic parts must be calculated");
    }

    #[test]
    fn test_western_high_latitude_fallback() {
        let ephem = compute_ephemeris(2024, 6, 21, 12, 0, 0.0, 70.0, 25.0)
            .expect("ephem should succeed");

        let chart = calculate_western(&ephem, 70.0, WesternHouseSystem::Placidus);
        assert_eq!(chart.house_cusps.len(), 12);
        assert_eq!(chart.planets.len(), 7);
        assert_eq!(chart.nodes.len(), 2);
    }

    #[test]
    fn test_phase5_precision_corrections() {
        // 2000-01-01 03:00 UTC (12:00 KST, Saturday) Seoul (37.5665, 126.9780)
        let ephem = compute_ephemeris(2000, 1, 1, 3, 0, 0.0, 37.5665, 126.9780)
            .expect("ephem should succeed");

        let chart = calculate_western(&ephem, 37.5665, WesternHouseSystem::Placidus);
        let trad = chart.traditional.expect("traditional data should exist");

        // 1. Planetary Hours: Daytime on Saturday at noon
        assert_eq!(trad.planetary_hours.day_ruler, "Saturn");
        assert!(trad.planetary_hours.is_day_hour);
        // Hour in Saturday noon is either Mars (Hour 3) or Sun (Hour 4) or Venus depending on exact sunrise
        assert!(!trad.planetary_hours.hour_ruler.is_empty());

        // 2. Moon Condition: Lunar day 25/Waning Crescent (elongation ~298.8 deg)
        assert_eq!(trad.moon_condition.lunar_phase_name, "Waning Crescent");

        // 3. Prenatal Syzygy: December 1999 Full Moon
        assert!(trad.prenatal_syzygy.is_some());
        let san = trad.prenatal_syzygy.as_ref().unwrap();
        assert_eq!(san.syzygy_type, "Full Moon");
        assert!(san.exact_datetime_utc.starts_with("1999-12-22"));

        // 4. Arabic Parts: 10 parts including 7 Hermetic lots
        assert!(trad.arabic_parts.len() >= 10);
        let part_names: Vec<&str> = trad.arabic_parts.iter().map(|p| p.name.as_str()).collect();
        assert!(part_names.contains(&"Part of Fortune"));
        assert!(part_names.contains(&"Part of Spirit"));
        assert!(part_names.contains(&"Lot of Eros"));
        assert!(part_names.contains(&"Lot of Victory"));
        assert!(part_names.contains(&"Lot of Necessity"));
        assert!(part_names.contains(&"Lot of Courage"));
        assert!(part_names.contains(&"Lot of Nemesis"));

        // 5. Fixed Stars: 45 major stars populated
        assert_eq!(trad.fixed_stars.len(), 45);
        let star_names: Vec<&str> = trad.fixed_stars.iter().map(|s| s.name.as_str()).collect();
        assert!(star_names.contains(&"Arcturus"));
        assert!(star_names.contains(&"Achernar"));
        assert!(star_names.contains(&"Scheat"));
        assert!(star_names.contains(&"Markab"));
        assert!(star_names.contains(&"Deneb Algedi"));
    }
}
