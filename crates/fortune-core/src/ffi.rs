//! UniFFI binding boundary for mobile and cross-platform clients.

use crate::{calculate_fortune, FortuneCalculationInput, Gender};

#[uniffi::export]
pub fn fortune_core_version() -> String {
    crate::version().to_string()
}

#[uniffi::export]
pub fn calculate_fortune_json(input_json: String) -> Result<String, String> {
    let input: FortuneCalculationInput = serde_json::from_str(&input_json)
        .map_err(|e| format!("Invalid input JSON: {}", e))?;

    let output = calculate_fortune(input)?;

    serde_json::to_string(&output)
        .map_err(|e| format!("Serialization error: {}", e))
}

#[uniffi::export]
pub fn calculate_fortune_direct(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: f64,
    latitude: f64,
    longitude: f64,
    is_male: bool,
) -> Result<String, String> {
    let gender = if is_male { Gender::Male } else { Gender::Female };
    let input = FortuneCalculationInput {
        year,
        month,
        day,
        hour,
        minute,
        second,
        latitude,
        longitude,
        gender,
    };

    let output = calculate_fortune(input)?;
    serde_json::to_string(&output)
        .map_err(|e| format!("Serialization error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_exports() {
        assert_eq!(fortune_core_version(), "0.1.0");

        let json = calculate_fortune_direct(
            2024, 3, 20, 12, 0, 0.0, 37.5665, 126.9780, true,
        ).expect("FFI call should succeed");

        assert!(json.contains("\"ephemeris\""));
        assert!(json.contains("\"western\""));
        assert!(json.contains("\"vedic\""));
        assert!(json.contains("\"saju\""));
    }
}
