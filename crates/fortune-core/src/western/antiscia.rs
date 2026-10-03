//! Antiscia and Contra-Antiscia (Solstice Points).
//!
//! Based on Christian Astrology (1647), Chapter XVI (p. 90-93).
//! Antiscion reflects across the Cancer-Capricorn Solstice axis (90° - 270°).
//! Contra-Antiscion is directly opposite the Antiscion (reflecting across Aries-Libra).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiscionPoint {
    pub body_name: String,
    pub true_longitude: f64,
    pub antiscion_longitude: f64,
    pub antiscion_sign_index: u8,
    pub antiscion_degree: f64,
    pub contra_antiscion_longitude: f64,
}

/// Compute antiscion and contra-antiscion for a zodiac longitude.
pub fn calculate_antiscion(longitude: f64) -> (f64, f64) {
    // Solstice axis reflection: 0° Cancer (90°) <-> 0° Gemini (60°) reflection
    // Formula: (180.0 - longitude) mod 360.0
    let antiscion = (180.0 - longitude).rem_euclid(360.0);
    let contra_antiscion = (antiscion + 180.0).rem_euclid(360.0);
    (antiscion, contra_antiscion)
}

/// Calculate antiscion table for all bodies in the chart.
pub fn calculate_antiscia_table(
    bodies: &[(&str, f64)],
) -> Vec<AntiscionPoint> {
    bodies.iter().map(|&(name, lon)| {
        let (antiscion, contra) = calculate_antiscion(lon);
        let sign_idx = (antiscion / 30.0).floor() as u8 % 12;
        let deg = antiscion - (sign_idx as f64 * 30.0);

        AntiscionPoint {
            body_name: name.to_string(),
            true_longitude: lon,
            antiscion_longitude: antiscion,
            antiscion_sign_index: sign_idx,
            antiscion_degree: deg,
            contra_antiscion_longitude: contra,
        }
    }).collect()
}
