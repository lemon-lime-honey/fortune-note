//! Mutual Reception and Horary Perfection Mechanisms.
//!
//! Based on Christian Astrology (1647), Chapter XIX (p. 112) & Chapter XXIII.

use serde::{Deserialize, Serialize};
use super::dignity::{ClassicalPlanet, evaluate_essential_dignities};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualReception {
    pub planet1: String,
    pub planet2: String,
    pub reception_type: String, // "Domicile", "Exaltation", "Mixed (Domicile & Exaltation)", "Term/Triplicity"
    pub description: String,
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
            if p1_opt.is_none() || p2_opt.is_none() { continue; }
            let p1 = p1_opt.unwrap();
            let p2 = p2_opt.unwrap();

            let d1 = evaluate_essential_dignities(lon1, is_day_chart);
            let d2 = evaluate_essential_dignities(lon2, is_day_chart);

            // Domicile reception: P1 in P2's domicile AND P2 in P1's domicile
            if d1.domicile == p2 && d2.domicile == p1 {
                results.push(MutualReception {
                    planet1: name1.to_string(),
                    planet2: name2.to_string(),
                    reception_type: "Domicile".into(),
                    description: format!("{} and {} are in mutual reception by Domicile", name1, name2),
                });
                continue;
            }

            // Exaltation reception
            if d1.exaltation == Some(p2) && d2.exaltation == Some(p1) {
                results.push(MutualReception {
                    planet1: name1.to_string(),
                    planet2: name2.to_string(),
                    reception_type: "Exaltation".into(),
                    description: format!("{} and {} are in mutual reception by Exaltation", name1, name2),
                });
                continue;
            }

            // Mixed: Domicile and Exaltation
            if (d1.domicile == p2 && d2.exaltation == Some(p1)) || (d1.exaltation == Some(p2) && d2.domicile == p1) {
                results.push(MutualReception {
                    planet1: name1.to_string(),
                    planet2: name2.to_string(),
                    reception_type: "Mixed (Domicile & Exaltation)".into(),
                    description: format!("{} and {} are in mutual reception by Domicile and Exaltation", name1, name2),
                });
                continue;
            }

            // Minor reception: Triplicity or Term
            let p1_in_p2_minor = d1.triplicity == p2 || d1.term == p2;
            let p2_in_p1_minor = d2.triplicity == p1 || d2.term == p1;
            if p1_in_p2_minor && p2_in_p1_minor {
                results.push(MutualReception {
                    planet1: name1.to_string(),
                    planet2: name2.to_string(),
                    reception_type: "Minor (Triplicity/Term)".into(),
                    description: format!("{} and {} are in minor mutual reception by Triplicity/Term", name1, name2),
                });
            }
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

pub fn analyze_perfection_mechanisms(
    planets: &[(&str, f64, f64)], // (name, lon, speed)
) -> PerfectionReport {
    let translation = Vec::new();
    let collection = Vec::new();
    let mut besiegement = Vec::new();

    // Check Besiegement (Besieged between Saturn and Mars)
    let mars = planets.iter().find(|(n, _, _)| *n == "Mars");
    let saturn = planets.iter().find(|(n, _, _)| *n == "Saturn");

    if let (Some(&(_, m_lon, _)), Some(&(_, s_lon, _))) = (mars, saturn) {
        for &(p_name, p_lon, _) in planets {
            if p_name == "Mars" || p_name == "Saturn" { continue; }
            let in_between = if m_lon < s_lon {
                p_lon > m_lon && p_lon < s_lon
            } else {
                p_lon > m_lon || p_lon < s_lon
            };
            if in_between {
                let dist_m = (p_lon - m_lon).abs().min(360.0 - (p_lon - m_lon).abs());
                let dist_s = (p_lon - s_lon).abs().min(360.0 - (p_lon - s_lon).abs());
                if dist_m <= 8.0 && dist_s <= 9.0 {
                    besiegement.push(format!("{} is besieged between Mars and Saturn", p_name));
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
