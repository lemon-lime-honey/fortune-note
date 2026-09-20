use crate::{calculate_fortune, FortuneCalculationInput, Gender};

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FortuneError {
    #[error("Calculation error: {message}")]
    CalculationError { message: String },
    #[error("Serialization error: {message}")]
    SerializationError { message: String },
}

#[uniffi::export]
pub fn fortune_core_version() -> String {
    crate::version().to_string()
}

#[uniffi::export]
pub fn calculate_fortune_json(input_json: String) -> Result<String, FortuneError> {
    let input: FortuneCalculationInput = serde_json::from_str(&input_json)
        .map_err(|e| FortuneError::SerializationError { message: format!("Invalid input JSON: {}", e) })?;

    let output = calculate_fortune(input)
        .map_err(|e| FortuneError::CalculationError { message: e })?;

    serde_json::to_string(&output)
        .map_err(|e| FortuneError::SerializationError { message: format!("Serialization error: {}", e) })
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
) -> Result<String, FortuneError> {
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

    let output = calculate_fortune(input)
        .map_err(|e| FortuneError::CalculationError { message: e })?;

    serde_json::to_string(&output)
        .map_err(|e| FortuneError::SerializationError { message: format!("Serialization error: {}", e) })
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

        assert!(json.contains("\"timezone_id\""));
        assert!(json.contains("\"utc_offset_hours\""));
        assert!(json.contains("\"ephemeris\""));
        assert!(json.contains("\"western\""));
        assert!(json.contains("\"vedic\""));
        assert!(json.contains("\"saju\""));
    }
}
