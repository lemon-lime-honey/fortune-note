//! Core calculation engine for fortune-note.
//!
//! Provides a unified deterministic interface for:
//! - Pure astronomical ephemeris (`ephemeris`)
//! - Western tropical astrology (`western`)
//! - Vedic sidereal astrology (`vedic`)
//! - Saju / Four Pillars of Destiny (`saju`)

pub mod ephemeris;
pub mod saju;
pub mod timezone;
pub mod vedic;
pub mod western;

use serde::{Deserialize, Serialize};

pub use ephemeris::{compute_ephemeris, CelestialBody, EphemerisSnapshot};
pub use saju::{calculate_saju, DaeUnPeriod, Gender, Pillar, SajuChart};
pub use timezone::{lookup_timezone_id, resolve_local_to_utc, TimezoneResolution};
pub use vedic::{
    calculate_varga_sign, calculate_varga_sign_and_degree, calculate_vedic, DashaPeriodInfo,
    VargaChart, VargaDivision, VargaPlacement, VedicChart, VedicPlanet,
};
pub use western::{calculate_western, WesternAspect, WesternChart, WesternHouseSystem, WesternNode, WesternPlanet};

/// Unified input structure for fortune calculation.
///
/// Takes local calendar date, time, coordinates, and gender.
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

/// Unified output structure containing decoupled domain results and resolved timezone metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FortuneCalculationOutput {
    pub timezone_id: String,
    pub utc_offset_hours: f64,
    pub utc_year: i32,
    pub utc_month: u32,
    pub utc_day: u32,
    pub utc_hour: u32,
    pub utc_minute: u32,
    pub utc_second: f64,
    pub ephemeris: EphemerisSnapshot,
    pub western: WesternChart,
    pub vedic: VedicChart,
    pub saju: SajuChart,
}

/// Calculate all domains (ephemeris, western, vedic, saju) deterministically.
///
/// Automatically resolves timezone and historical DST from observer coordinates.
pub fn calculate_fortune(
    input: FortuneCalculationInput,
) -> Result<FortuneCalculationOutput, String> {
    let tz_res = resolve_local_to_utc(
        input.year,
        input.month,
        input.day,
        input.hour,
        input.minute,
        input.second,
        input.latitude,
        input.longitude,
    )
    .map_err(|e| format!("Timezone resolution error: {}", e))?;

    let ephem = compute_ephemeris(
        tz_res.utc_year,
        tz_res.utc_month,
        tz_res.utc_day,
        tz_res.utc_hour,
        tz_res.utc_minute,
        tz_res.utc_second,
        input.latitude,
        input.longitude,
    )
    .map_err(|e| format!("Ephemeris error: {}", e))?;

    let western = calculate_western(&ephem, input.latitude, WesternHouseSystem::Placidus);
    let vedic = calculate_vedic(&ephem, input.latitude);
    let saju = calculate_saju(&ephem, input.year, input.longitude, input.gender)
        .map_err(|e| format!("Saju error: {}", e))?;

    Ok(FortuneCalculationOutput {
        timezone_id: tz_res.timezone_id,
        utc_offset_hours: tz_res.utc_offset_hours,
        utc_year: tz_res.utc_year,
        utc_month: tz_res.utc_month,
        utc_day: tz_res.utc_day,
        utc_hour: tz_res.utc_hour,
        utc_minute: tz_res.utc_minute,
        utc_second: tz_res.utc_second,
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

        assert_eq!(output.timezone_id, "Asia/Seoul");
        assert_eq!(output.utc_offset_hours, 9.0);
        assert_eq!(output.utc_hour, 5);
        assert_eq!(output.utc_minute, 30);
        assert_eq!(output.ephemeris.bodies.len(), 9);
        assert_eq!(output.western.planets.len(), 7);
        assert_eq!(output.western.nodes.len(), 2);
        assert_eq!(output.vedic.planets.len(), 9);
        assert_eq!(output.vedic.vargas.len(), 11);
        assert_eq!(output.saju.daeun_periods.len(), 10);
    }
}
