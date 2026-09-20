//! Core calculation engine for fortune-note.
//!
//! Provides a unified deterministic interface for:
//! - Pure astronomical ephemeris (`ephemeris`)
//! - Western tropical astrology (`western`)
//! - Vedic sidereal astrology (`vedic`)
//! - Saju / Four Pillars of Destiny (`saju`)

pub mod ephemeris;
pub mod ffi;
pub mod saju;
pub mod vedic;
pub mod western;

uniffi::setup_scaffolding!();

use serde::{Deserialize, Serialize};

pub use ephemeris::{compute_ephemeris, CelestialBody, EphemerisSnapshot};
pub use saju::{calculate_saju, DaeUnPeriod, Gender, Pillar, SajuChart};
pub use vedic::{calculate_vedic, DashaPeriodInfo, VargaChart, VargaDivision, VargaPlacement, VedicChart, VedicPlanet};
pub use western::{calculate_western, WesternAspect, WesternChart, WesternHouseSystem, WesternPlanet};

/// Unified input structure for fortune calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FortuneCalculationInput {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub gender: Gender,
}

/// Unified output structure containing decoupled domain results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FortuneCalculationOutput {
    pub ephemeris: EphemerisSnapshot,
    pub western: WesternChart,
    pub vedic: VedicChart,
    pub saju: SajuChart,
}

/// Calculate all domains (ephemeris, western, vedic, saju) deterministically.
pub fn calculate_fortune(
    input: FortuneCalculationInput,
) -> Result<FortuneCalculationOutput, String> {
    let ephem = compute_ephemeris(
        input.year,
        input.month,
        input.day,
        input.hour,
        input.minute,
        input.second,
        input.latitude,
        input.longitude,
    )
    .map_err(|e| format!("Ephemeris error: {}", e))?;

    let western = calculate_western(&ephem, input.latitude, WesternHouseSystem::Placidus);
    let vedic = calculate_vedic(&ephem, input.latitude);
    let saju = calculate_saju(&ephem, input.year, input.longitude, input.gender)
        .map_err(|e| format!("Saju error: {}", e))?;

    Ok(FortuneCalculationOutput {
        ephemeris: ephem,
        western,
        vedic,
        saju,
    })
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_output_structure() {
        let input = FortuneCalculationInput {
            year: 1995,
            month: 10,
            day: 25,
            hour: 14,
            minute: 30,
            second: 0.0,
            latitude: 37.5665,
            longitude: 126.9780,
            gender: Gender::Male,
        };

        let output = calculate_fortune(input).expect("calculation should succeed");

        assert_eq!(output.ephemeris.bodies.len(), 9);
        assert_eq!(output.western.planets.len(), 9);
        assert_eq!(output.vedic.planets.len(), 9);
        assert_eq!(output.vedic.vargas.len(), 6);
        assert_eq!(output.saju.daeun_periods.len(), 10);
    }
}
