//! House Cusp Dignities and Almuten Calculations.
//!
//! Evaluates the 5-fold dignity of all 12 house cusps and finds the Almuten (winner by score).

use serde::{Deserialize, Serialize};
use super::dignity::{ClassicalPlanet, evaluate_essential_dignities};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseCuspDignity {
    pub house_number: u8,
    pub cusp_degree: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub domicile: String,
    pub exaltation: Option<String>,
    pub triplicity: String,
    pub term: String,
    pub face: String,
    pub almuten: String,
    pub almuten_score: i32,
}

/// Compute 5-fold dignities and Almuten for each of the 12 house cusps.
pub fn calculate_house_cusp_dignities(
    cusps: &[f64; 12],
    is_day_chart: bool,
) -> Vec<HouseCuspDignity> {
    let mut results = Vec::with_capacity(12);

    for (i, &cusp) in cusps.iter().enumerate() {
        let house_number = (i + 1) as u8;
        let sign_index = (cusp / 30.0).floor() as u8 % 12;
        let sign_degree = cusp - (sign_index as f64 * 30.0);

        let d = evaluate_essential_dignities(cusp, is_day_chart);

        // Score all classical planets at this cusp
        let all_planets = [
            ClassicalPlanet::Sun,
            ClassicalPlanet::Moon,
            ClassicalPlanet::Mercury,
            ClassicalPlanet::Venus,
            ClassicalPlanet::Mars,
            ClassicalPlanet::Jupiter,
            ClassicalPlanet::Saturn,
        ];

        let mut best_planet = d.domicile;
        let mut best_score = 0;
        let mut best_highest_dignity = 0;

        for p in all_planets {
            let mut score = 0;
            if d.domicile == p { score += 5; }
            if d.exaltation == Some(p) { score += 4; }
            if d.triplicity == p { score += 3; }
            if d.term == p { score += 2; }
            if d.face == p { score += 1; }

            // In classical astrology (Ibn Ezra, Lilly), ties in Almuten score are broken
            // by highest single essential dignity (Domicile 5 > Exaltation 4 > Triplicity 3 > Term 2 > Face 1).
            let highest_dignity = if d.domicile == p {
                5
            } else if d.exaltation == Some(p) {
                4
            } else if d.triplicity == p {
                3
            } else if d.term == p {
                2
            } else if d.face == p {
                1
            } else {
                0
            };

            if score > best_score || (score == best_score && highest_dignity > best_highest_dignity) {
                best_score = score;
                best_planet = p;
                best_highest_dignity = highest_dignity;
            }
        }

        results.push(HouseCuspDignity {
            house_number,
            cusp_degree: cusp,
            sign_index,
            sign_degree,
            domicile: d.domicile.name().to_string(),
            exaltation: d.exaltation.map(|p| p.name().to_string()),
            triplicity: d.triplicity.name().to_string(),
            term: d.term.name().to_string(),
            face: d.face.name().to_string(),
            almuten: best_planet.name().to_string(),
            almuten_score: best_score,
        });
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_house_cusp_dignities_almuten() {
        let cusps = [
            15.0,  // Aries 15°
            45.0,  // Taurus 15°
            75.0,  // Gemini 15°
            105.0, // Cancer 15°
            135.0, // Leo 15°
            165.0, // Virgo 15°
            195.0, // Libra 15°
            225.0, // Scorpio 15°
            255.0, // Sagittarius 15°
            285.0, // Capricorn 15°
            315.0, // Aquarius 15°
            345.0, // Pisces 15°
        ];

        let dignities = calculate_house_cusp_dignities(&cusps, true);
        assert_eq!(dignities.len(), 12);

        // Aries 15°: Domicile Mars (+5), Exaltation Sun (+4), Triplicity Sun (+3), Term Mercury (+2), Face Sun (+1)
        // Mars score: 5. Sun score: 4 + 3 + 1 = 8. Sun wins as Almuten with 8 points.
        assert_eq!(dignities[0].domicile, "Mars");
        assert_eq!(dignities[0].almuten, "Sun");
        assert_eq!(dignities[0].almuten_score, 8);
    }
}
