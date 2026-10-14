//! Mutual Reception and Horary Perfection Mechanisms.
//!
//! Based on Christian Astrology (1647), Chapter XIX (p. 110-112) & Chapter XXIII.
//!
//! Implements:
//! - Mutual Reception (Domicile, Exaltation, Mixed Major/Minor, Minor by Triplicity/Term/Face)
//! - Translation of Light (Lilly CA p. 110)
//! - Collection of Light (Lilly CA p. 110)
//! - Besiegement between the infortunes Mars and Saturn (Lilly CA p. 113)

use serde::{Deserialize, Serialize};
use super::aspects::calculate_traditional_aspects;
use super::dignity::{ClassicalPlanet, EssentialDignities, evaluate_essential_dignities};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutualReception {
    pub planet1: String,
    pub planet2: String,
    pub reception_type: String, // "Domicile", "Exaltation", "Mixed (Domicile & Exaltation)", "Mixed (Major & Minor)", "Minor"
    pub description: String,
}

/// Helper to gather all essential dignities of a ruler planet at a given zodiac point.
fn get_dignities_of_ruler(d: &EssentialDignities, ruler: ClassicalPlanet) -> Vec<&'static str> {
    let mut list = Vec::new();
    if d.domicile == ruler {
        list.push("Domicile");
    }
    if d.exaltation == Some(ruler) {
        list.push("Exaltation");
    }
    if d.triplicity == ruler {
        list.push("Triplicity");
    }
    if d.term == ruler {
        list.push("Term");
    }
    if d.face == ruler {
        list.push("Face");
    }
    list
}

/// Detect mutual receptions between all pairs of classical planets.
pub fn find_mutual_receptions(
    planets: &[(&str, f64)],
    is_day_chart: bool,
) -> Vec<MutualReception> {
    let mut results = Vec::new();

    for i in 0..planets.len() {
        for j in (i + 1)..planets.len() {
            let (name1, lon1) = planets[i];
            let (name2, lon2) = planets[j];

            let p1_opt = ClassicalPlanet::from_name(name1);
            let p2_opt = ClassicalPlanet::from_name(name2);
            if p1_opt.is_none() || p2_opt.is_none() {
                continue;
            }
            let p1 = p1_opt.unwrap();
            let p2 = p2_opt.unwrap();

            let d1 = evaluate_essential_dignities(lon1, is_day_chart);
            let d2 = evaluate_essential_dignities(lon2, is_day_chart);

            let p1_in_p2 = get_dignities_of_ruler(&d1, p2);
            let p2_in_p1 = get_dignities_of_ruler(&d2, p1);

            if p1_in_p2.is_empty() || p2_in_p1.is_empty() {
                continue;
            }

            let p1_has_dom = p1_in_p2.contains(&"Domicile");
            let p2_has_dom = p2_in_p1.contains(&"Domicile");
            let p1_has_exalt = p1_in_p2.contains(&"Exaltation");
            let p2_has_exalt = p2_in_p1.contains(&"Exaltation");

            let p1_has_major = p1_has_dom || p1_has_exalt;
            let p2_has_major = p2_has_dom || p2_has_exalt;

            let reception_type = if p1_has_dom && p2_has_dom {
                "Domicile".to_string()
            } else if p1_has_exalt && p2_has_exalt {
                "Exaltation".to_string()
            } else if (p1_has_dom && p2_has_exalt) || (p1_has_exalt && p2_has_dom) {
                "Mixed (Domicile & Exaltation)".to_string()
            } else if p1_has_major || p2_has_major {
                "Mixed (Major & Minor)".to_string()
            } else {
                "Minor".to_string()
            };

            let desc = format!(
                "{} in {} of {}, and {} in {} of {}",
                name1,
                p1_in_p2.join("/"),
                name2,
                name2,
                p2_in_p1.join("/"),
                name1
            );

            results.push(MutualReception {
                planet1: name1.to_string(),
                planet2: name2.to_string(),
                reception_type,
                description: desc,
            });
        }
    }

    results
}

/// Perfection and Impediment report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfectionReport {
    pub translation_of_light: Vec<String>,
    pub collection_of_light: Vec<String>,
    pub besiegement: Vec<String>,
}

/// Check if two angles are within a directed short arc between a and b.
fn is_between_angles(test: f64, a: f64, b: f64) -> bool {
    let diff = (b - a).rem_euclid(360.0);
    if diff <= 180.0 {
        // Short arc is from a to b
        let dist = (test - a).rem_euclid(360.0);
        dist > 0.001 && dist < diff - 0.001
    } else {
        // Short arc is from b to a
        let rev_diff = (a - b).rem_euclid(360.0);
        let dist = (test - b).rem_euclid(360.0);
        dist > 0.001 && dist < rev_diff - 0.001
    }
}

/// Angular distance on a circle (0.0 .. 180.0).
fn angular_distance(a: f64, b: f64) -> f64 {
    let diff = (a - b).abs() % 360.0;
    if diff > 180.0 {
        360.0 - diff
    } else {
        diff
    }
}

/// Analyze traditional perfection mechanisms (Translation, Collection, Besiegement).
pub fn analyze_perfection_mechanisms(
    planets: &[(&str, f64, f64)], // (name, longitude, speed)
) -> PerfectionReport {
    let aspects = calculate_traditional_aspects(planets);

    let mut translation = Vec::new();
    let mut collection = Vec::new();
    let mut besiegement = Vec::new();

    let n = planets.len();

    // 1. Translation of Light (Lilly CA p. 110)
    // Fast planet T separates from P1 and immediately applies to P2.
    // P1 and P2 do not have an active applying aspect between them.
    for i in 0..n {
        let (t_name, _, t_speed) = planets[i];

        for j in 0..n {
            if i == j { continue; }
            let (p1_name, _, p1_speed) = planets[j];
            if t_speed.abs() <= p1_speed.abs() { continue; }

            // Find aspect between T and P1 where T is separating from P1
            let asp_t_p1 = aspects.iter().find(|a| {
                ((a.planet1 == t_name && a.planet2 == p1_name) || (a.planet1 == p1_name && a.planet2 == t_name))
                    && !a.is_applying
            });
            let asp_t_p1 = match asp_t_p1 {
                Some(a) => a,
                None => continue,
            };

            for (k, &(p2_name, _, p2_speed)) in planets.iter().enumerate() {
                if k == i || k == j { continue; }
                if t_speed.abs() <= p2_speed.abs() { continue; }

                // Find aspect between T and P2 where T is applying to P2
                let asp_t_p2 = aspects.iter().find(|a| {
                    ((a.planet1 == t_name && a.planet2 == p2_name) || (a.planet1 == p2_name && a.planet2 == t_name))
                        && a.is_applying
                });
                let asp_t_p2 = match asp_t_p2 {
                    Some(a) => a,
                    None => continue,
                };

                // Ensure P1 and P2 do not have an active applying aspect directly
                let p1_p2_applying = aspects.iter().any(|a| {
                    ((a.planet1 == p1_name && a.planet2 == p2_name) || (a.planet1 == p2_name && a.planet2 == p1_name))
                        && a.is_applying
                });

                if !p1_p2_applying {
                    let desc = format!(
                        "{} (speed {:+.2}°/d) translates light from {} (separating {}) to {} (applying {})",
                        t_name, t_speed, p1_name, asp_t_p1.aspect_type, p2_name, asp_t_p2.aspect_type
                    );
                    if !translation.contains(&desc) {
                        translation.push(desc);
                    }
                }
            }
        }
    }

    // 2. Collection of Light (Lilly CA p. 110)
    // Slower heavier planet C collects light from both P1 and P2 (both P1 and P2 apply to C).
    // P1 and P2 do not apply directly to each other.
    for i in 0..n {
        let (c_name, _, c_speed) = planets[i];

        for j in 0..n {
            if i == j { continue; }
            let (p1_name, _, p1_speed) = planets[j];
            if c_speed.abs() >= p1_speed.abs() { continue; }

            // P1 must apply to C
            let asp_p1_c = aspects.iter().find(|a| {
                ((a.planet1 == p1_name && a.planet2 == c_name) || (a.planet1 == c_name && a.planet2 == p1_name))
                    && a.is_applying
            });
            let asp_p1_c = match asp_p1_c {
                Some(a) => a,
                None => continue,
            };

            for (k, &(p2_name, _, p2_speed)) in planets.iter().enumerate().skip(j + 1) {
                if k == i { continue; }
                if c_speed.abs() >= p2_speed.abs() { continue; }

                // P2 must apply to C
                let asp_p2_c = aspects.iter().find(|a| {
                    ((a.planet1 == p2_name && a.planet2 == c_name) || (a.planet1 == c_name && a.planet2 == p2_name))
                        && a.is_applying
                });
                let asp_p2_c = match asp_p2_c {
                    Some(a) => a,
                    None => continue,
                };

                // P1 and P2 do not apply directly to each other
                let p1_p2_applying = aspects.iter().any(|a| {
                    ((a.planet1 == p1_name && a.planet2 == p2_name) || (a.planet1 == p2_name && a.planet2 == p1_name))
                        && a.is_applying
                });

                if !p1_p2_applying {
                    let desc = format!(
                        "{} (speed {:+.2}°/d) collects light from {} (applying {}) and {} (applying {})",
                        c_name, c_speed, p1_name, asp_p1_c.aspect_type, p2_name, asp_p2_c.aspect_type
                    );
                    if !collection.contains(&desc) {
                        collection.push(desc);
                    }
                }
            }
        }
    }

    // 3. Bodily Besiegement (Lilly CA p. 113)
    // Target planet is between Mars and Saturn within combined moiety orbs.
    let mars = planets.iter().find(|(n, _, _)| *n == "Mars");
    let saturn = planets.iter().find(|(n, _, _)| *n == "Saturn");

    if let (Some(&(_, m_lon, _)), Some(&(_, s_lon, _))) = (mars, saturn) {
        let mars_saturn_dist = angular_distance(m_lon, s_lon);
        // Bodily besiegement requires Mars and Saturn to be reasonably close (within ~17°)
        let max_besiegement_span = ClassicalPlanet::Mars.moiety() + ClassicalPlanet::Saturn.moiety(); // 4.0 + 4.5 = 8.5° orbs each, span <= 17°

        if mars_saturn_dist <= max_besiegement_span * 2.0 {
            for &(p_name, p_lon, _) in planets {
                if p_name == "Mars" || p_name == "Saturn" {
                    continue;
                }
                if is_between_angles(p_lon, m_lon, s_lon) {
                    let dist_m = angular_distance(p_lon, m_lon);
                    let dist_s = angular_distance(p_lon, s_lon);

                    let p_moiety = ClassicalPlanet::from_name(p_name).map_or(4.0, |cp| cp.moiety());
                    if dist_m <= ClassicalPlanet::Mars.moiety() + p_moiety
                        && dist_s <= ClassicalPlanet::Saturn.moiety() + p_moiety
                    {
                        besiegement.push(format!(
                            "{} is besieged between Mars ({:.1}°) and Saturn ({:.1}°) [Dist to Mars: {:.1}°, Dist to Saturn: {:.1}°]",
                            p_name, m_lon, s_lon, dist_m, dist_s
                        ));
                    }
                }
            }
        }
    }

    PerfectionReport {
        translation_of_light: translation,
        collection_of_light: collection,
        besiegement,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutual_receptions_domicile_and_mixed() {
        // Mars in Capricorn (Saturn domicile), Saturn in Scorpio (Mars domicile) -> Domicile
        let planets = [
            ("Mars", 275.0),   // Capricorn 5° (Saturn domicile, Mars exaltation)
            ("Saturn", 220.0), // Scorpio 10° (Mars domicile)
        ];
        let receptions = find_mutual_receptions(&planets, true);
        assert_eq!(receptions.len(), 1);
        assert_eq!(receptions[0].reception_type, "Domicile");

        // Sun in Scorpio (Mars domicile), Mars in Sagittarius (Sun triplicity in day)
        // -> Mixed (Major & Minor)
        let planets2 = [
            ("Sun", 220.0),    // Scorpio 10° (Mars domicile)
            ("Mars", 250.0),   // Sagittarius 10° (Sun day triplicity)
        ];
        let receptions2 = find_mutual_receptions(&planets2, true);
        assert_eq!(receptions2.len(), 1);
        assert_eq!(receptions2[0].reception_type, "Mixed (Major & Minor)");
    }

    #[test]
    fn test_translation_of_light() {
        // Moon (fast: 13°/d at 15° Leo)
        // Mars (slow: 0.6°/d at 10° Leo) -> Moon is separating from Mars (Conjunction, 5° past)
        // Venus (medium: 1.1°/d at 20° Leo) -> Moon is applying to Venus (Conjunction, 5° ahead)
        // Mars and Venus are 10° apart (outside direct conjunction orb of 7.5°)
        let planets = [
            ("Mars", 130.0, 0.6),   // 10° Leo
            ("Moon", 135.0, 13.0),  // 15° Leo (translates light)
            ("Venus", 140.0, 1.1),  // 20° Leo
        ];

        let report = analyze_perfection_mechanisms(&planets);
        assert!(!report.translation_of_light.is_empty(), "Translation of light must be detected");
        assert!(report.translation_of_light[0].contains("Moon"));
        assert!(report.translation_of_light[0].contains("translates light from Mars"));
        assert!(report.translation_of_light[0].contains("to Venus"));
    }

    #[test]
    fn test_collection_of_light() {
        // Saturn (slowest: 0.05°/d at 100° Cancer)
        // Mercury (fast: 1.5°/d at 95° Cancer) applying to Saturn
        // Venus (fast: 1.2°/d at 93° Cancer) applying to Saturn
        // Mercury and Venus are 2° apart but both applying to Saturn
        let planets = [
            ("Mercury", 95.0, 1.5),
            ("Venus", 93.0, 1.2),
            ("Saturn", 100.0, 0.05),
        ];

        let report = analyze_perfection_mechanisms(&planets);
        assert!(!report.collection_of_light.is_empty(), "Collection of light must be detected");
        assert!(report.collection_of_light[0].contains("Saturn"));
        assert!(report.collection_of_light[0].contains("collects light"));
    }

    #[test]
    fn test_bodily_besiegement() {
        // Mars at 100°, Saturn at 112° (distance 12°)
        // Venus at 106° (right in the middle of Mars and Saturn)
        let planets = [
            ("Mars", 100.0, 0.6),
            ("Venus", 106.0, 1.2),
            ("Saturn", 112.0, 0.05),
        ];

        let report = analyze_perfection_mechanisms(&planets);
        assert_eq!(report.besiegement.len(), 1, "Venus should be besieged between Mars and Saturn");
        assert!(report.besiegement[0].contains("Venus is besieged between Mars"));
    }
}
