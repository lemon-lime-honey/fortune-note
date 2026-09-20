//! Western astrology calculation layer (Tropical Zodiac).
//!
//! Provides deterministic computation for:
//! - 12 house cusps with Ascendant and Midheaven (default: Placidus)
//! - Planetary house placements in the tropical framework
//! - Major Ptolemaic aspects (Conjunction, Sextile, Square, Trine, Opposition)

use serde::{Deserialize, Serialize};
use vedaksha::astro::houses::{compute_houses, HouseSystem};

use crate::ephemeris::EphemerisSnapshot;

/// Supported Western house division systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WesternHouseSystem {
    Placidus,
    Koch,
    Equal,
    WholeSign,
    Campanus,
    Regiomontanus,
}

impl WesternHouseSystem {
    fn to_vedaksha_system(self) -> HouseSystem {
        match self {
            Self::Placidus => HouseSystem::Placidus,
            Self::Koch => HouseSystem::Koch,
            Self::Equal => HouseSystem::Equal,
            Self::WholeSign => HouseSystem::WholeSign,
            Self::Campanus => HouseSystem::Campanus,
            Self::Regiomontanus => HouseSystem::Regiomontanus,
        }
    }
}

/// Planet placement in the Western tropical chart (strictly 7 classical planets).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WesternPlanet {
    pub name: String,
    pub longitude: f64,       // 0.0 .. 360.0
    pub sign_index: u8,       // 0 = Aries .. 11 = Pisces
    pub sign_degree: f64,     // 0.0 .. 30.0
    pub house_number: u8,     // 1 .. 12
    pub is_retrograde: bool,
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
}

/// Determine which house a given longitude falls into based on 12 cusp degrees.
fn determine_house(lon: f64, cusps: &[f64; 12]) -> u8 {
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

    WesternChart {
        ascendant,
        midheaven,
        house_cusps: cusps,
        house_system: system,
        planets,
        nodes,
        aspects,
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

        let chart = calculate_western(&ephem, 51.5074, WesternHouseSystem::Placidus);

        assert_eq!(chart.house_cusps.len(), 12);
        assert_eq!(chart.planets.len(), 7, "Strictly 7 classical planets in western chart");
        assert_eq!(chart.nodes.len(), 2, "Strictly 2 lunar nodes (North Node & South Node)");

        assert_eq!(chart.nodes[0].name, "North Node");
        assert_eq!(chart.nodes[1].name, "South Node");
        let node_diff = ((chart.nodes[0].longitude - chart.nodes[1].longitude).abs() - 180.0).abs();
        assert!(node_diff < 0.01, "Nodes must be in 180-degree opposition");

        // House 1 cusp must match Ascendant
        let asc_diff = (chart.house_cusps[0] - chart.ascendant).abs();
        assert!(asc_diff < 1e-4 || (360.0 - asc_diff) < 1e-4, "House 1 cusp must equal Ascendant");

        // Verify all house cusps are within [0, 360)
        for cusp in &chart.house_cusps {
            assert!((0.0..360.0).contains(cusp));
        }

        // Verify all 7 planets have valid house placements
        for planet in &chart.planets {
            assert!((1..=12).contains(&planet.house_number));
            assert!((0.0..360.0).contains(&planet.longitude));
        }

        // Verify aspects conform to major aspect angles and orbs
        let valid_aspect_angles = [0.0, 60.0, 90.0, 120.0, 180.0];
        for aspect in &chart.aspects {
            assert!(
                valid_aspect_angles.contains(&aspect.exact_angle),
                "Invalid aspect angle: {}",
                aspect.exact_angle
            );
            assert!(aspect.orb <= 8.0, "Aspect orb exceeds maximum: {}", aspect.orb);
        }
    }

    #[test]
    fn test_western_high_latitude_fallback() {
        // High latitude (e.g. 70.0 N) where Placidus fails and falls back to Equal
        let ephem = compute_ephemeris(2024, 6, 21, 12, 0, 0.0, 70.0, 25.0)
            .expect("ephem should succeed");

        let chart = calculate_western(&ephem, 70.0, WesternHouseSystem::Placidus);
        assert_eq!(chart.house_cusps.len(), 12);
        assert_eq!(chart.planets.len(), 7);
        assert_eq!(chart.nodes.len(), 2);
    }
}

