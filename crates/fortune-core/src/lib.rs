#![allow(clippy::too_many_arguments)]

//! Core calculation engine for fortune-note.
//!
//! Provides a unified deterministic interface for:
//! - Shared astronomical value objects & spherical/calendar math (`celestial`)
//! - Pure astronomical ephemeris (`ephemeris`)
//! - Western tropical astrology (`western`)
//! - Vedic sidereal astrology (`vedic`)
//! - Saju / Four Pillars of Destiny (`saju`)

pub mod celestial;
pub mod ephemeris;
pub mod saju;
pub mod timezone;
pub mod vedic;
pub mod western;

use serde::{Deserialize, Serialize};

pub use celestial::{CalendarDate, CalendarDateTime, GeoCoordinate};
pub use ephemeris::{compute_ephemeris, CelestialBody, EphemerisSnapshot};
pub use saju::{
    calculate_saju, calculate_saju_with_options, DaeUnPeriod, Gender, Pillar, SajuChart,
    SajuOptions, ZiHourMode,
};
pub use timezone::{
    lookup_timezone_id, resolve_local_to_utc, resolve_local_to_utc_with_tz, resolve_with_timezone,
    TimezoneResolution,
};
pub use vedic::{
    calculate_varga_sign, calculate_varga_sign_and_degree, calculate_vedic,
    calculate_vedic_with_options, BhinnaAshtakavargaRow, DashaPeriodInfo, DashaYearType,
    VargaChart, VargaDivision, VargaPlacement, VedicArudhaPada, VedicAshtakavarga, VedicChart,
    VedicDrishtiInfo, VedicKarakaInfo, VedicOptions, VedicPanchanga, VedicPlanet,
};
pub use western::{
    calculate_western, calculate_western_with_mode, WesternAspect, WesternChart,
    WesternHouseSystem, WesternNode, WesternPlanet, WesternTraditionalData,
};

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
    #[serde(default)]
    pub western_house_system: Option<WesternHouseSystem>,
    #[serde(default)]
    pub timezone_id: Option<String>,
    #[serde(default)]
    pub is_horary: bool,
    #[serde(default)]
    pub saju_options: SajuOptions,
}

impl FortuneCalculationInput {
    pub fn local_datetime(&self) -> CalendarDateTime {
        CalendarDateTime::new(self.year, self.month, self.day, self.hour, self.minute, self.second)
    }

    pub fn location(&self) -> GeoCoordinate {
        GeoCoordinate::new(self.latitude, self.longitude)
    }
}

/// Decoupled Saju calculation output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SajuCalculationOutput {
    pub timezone_id: String,
    pub utc_offset_hours: f64,
    pub saju: SajuChart,
}

/// Decoupled Western calculation output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WesternCalculationOutput {
    pub timezone_id: String,
    pub utc_offset_hours: f64,
    pub ephemeris: EphemerisSnapshot,
    pub western: WesternChart,
}

/// Decoupled Vedic calculation output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicCalculationOutput {
    pub timezone_id: String,
    pub utc_offset_hours: f64,
    pub vedic: VedicChart,
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

/// Resolve timezone and compute ephemeris snapshot for a `FortuneCalculationInput`.
pub fn resolve_fortune_context(
    input: &FortuneCalculationInput,
) -> Result<(TimezoneResolution, EphemerisSnapshot), String> {
    let dt = input.local_datetime();
    let loc = input.location();

    let tz_res = resolve_local_to_utc_with_tz(dt, loc, input.timezone_id.as_deref())
        .map_err(|e| format!("Timezone resolution error: {}", e))?;

    let ephem = compute_ephemeris(tz_res.utc_datetime(), loc)
        .map_err(|e| format!("Ephemeris error: {}", e))?;

    Ok((tz_res, ephem))
}

fn compute_saju_for_input(
    ephem: &EphemerisSnapshot,
    input: &FortuneCalculationInput,
) -> Result<SajuChart, String> {
    calculate_saju_with_options(ephem, input.year, input.longitude, input.gender, input.saju_options)
        .map_err(|e| format!("Saju error: {}", e))
}

fn compute_western_for_input(
    ephem: &EphemerisSnapshot,
    input: &FortuneCalculationInput,
) -> WesternChart {
    calculate_western_with_mode(
        ephem,
        input.latitude,
        input.western_house_system.unwrap_or_default(),
        input.gender == Gender::Male,
        input.is_horary,
    )
}

/// Calculate Saju independently without western or vedic astrological overhead.
pub fn calculate_fortune_saju(
    input: FortuneCalculationInput,
) -> Result<SajuCalculationOutput, String> {
    let (tz_res, ephem) = resolve_fortune_context(&input)?;
    let saju = compute_saju_for_input(&ephem, &input)?;
    Ok(SajuCalculationOutput {
        timezone_id: tz_res.timezone_id,
        utc_offset_hours: tz_res.utc_offset_hours,
        saju,
    })
}

/// Calculate Western astrology chart independently without saju or vedic overhead.
pub fn calculate_fortune_western(
    input: FortuneCalculationInput,
) -> Result<WesternCalculationOutput, String> {
    let (tz_res, ephem) = resolve_fortune_context(&input)?;
    let western = compute_western_for_input(&ephem, &input);
    Ok(WesternCalculationOutput {
        timezone_id: tz_res.timezone_id,
        utc_offset_hours: tz_res.utc_offset_hours,
        ephemeris: ephem,
        western,
    })
}

/// Calculate Vedic astrology chart independently without saju or western overhead.
pub fn calculate_fortune_vedic(
    input: FortuneCalculationInput,
) -> Result<VedicCalculationOutput, String> {
    let (tz_res, ephem) = resolve_fortune_context(&input)?;
    let vedic = calculate_vedic(&ephem, input.latitude);
    Ok(VedicCalculationOutput {
        timezone_id: tz_res.timezone_id,
        utc_offset_hours: tz_res.utc_offset_hours,
        vedic,
    })
}

/// Calculate all domains (ephemeris, western, vedic, saju) deterministically.
pub fn calculate_fortune(
    input: FortuneCalculationInput,
) -> Result<FortuneCalculationOutput, String> {
    let (tz_res, ephem) = resolve_fortune_context(&input)?;
    let western = compute_western_for_input(&ephem, &input);
    let vedic = calculate_vedic(&ephem, input.latitude);
    let saju = compute_saju_for_input(&ephem, &input)?;

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

    fn sample_seoul_input(timezone_id: Option<&str>) -> FortuneCalculationInput {
        FortuneCalculationInput {
            year: 1995,
            month: 10,
            day: 25,
            hour: 14,
            minute: 30,
            second: 0.0,
            latitude: 37.5665,
            longitude: 126.9780,
            gender: Gender::Male,
            western_house_system: None,
            timezone_id: timezone_id.map(str::to_string),
            is_horary: false,
            saju_options: Default::default(),
        }
    }

    #[test]
    fn test_unified_output_structure() {
        let output = calculate_fortune(sample_seoul_input(None)).expect("calculation should succeed");
        assert_eq!(output.timezone_id, "Asia/Seoul");
        assert_eq!(output.utc_offset_hours, 9.0);
        assert_eq!(output.utc_hour, 5);
        assert_eq!(output.utc_minute, 30);
        assert_eq!(output.ephemeris.bodies.len(), 9);
        assert_eq!(output.western.planets.len(), 7);
        assert_eq!(output.western.nodes.len(), 2);
        assert_eq!(output.vedic.planets.len(), 9);
        assert_eq!(output.vedic.vargas.len(), 19);
        assert_eq!(output.saju.daeun_periods.len(), 10);
    }

    #[test]
    fn test_decoupled_domains() {
        let input = sample_seoul_input(Some("Asia/Seoul"));
        let saju_out = calculate_fortune_saju(input.clone()).expect("saju should succeed");
        assert_eq!(saju_out.saju.daeun_periods.len(), 10);

        let western_out = calculate_fortune_western(input.clone()).expect("western should succeed");
        assert_eq!(western_out.western.planets.len(), 7);

        let vedic_out = calculate_fortune_vedic(input).expect("vedic should succeed");
        assert_eq!(vedic_out.vedic.vargas.len(), 19);
    }

    #[test]
    fn test_calculate_fortune_western_gender_part_of_marriage() {
        let mut input = sample_seoul_input(Some("Asia/Seoul"));
        let male_out = calculate_fortune_western(input.clone()).expect("male western should succeed");
        input.gender = Gender::Female;
        let female_out = calculate_fortune_western(input).expect("female western should succeed");

        let male_m = male_out.western.traditional.unwrap().arabic_parts
            .into_iter().find(|p| p.name == "Part of Marriage").unwrap();
        let female_m = female_out.western.traditional.unwrap().arabic_parts
            .into_iter().find(|p| p.name == "Part of Marriage").unwrap();
        assert_ne!(male_m.longitude, female_m.longitude);
    }

    #[test]
    fn test_natal_excludes_horary_fields() {
        let mut input = sample_seoul_input(Some("Asia/Seoul"));
        let natal_trad = calculate_fortune_western(input.clone())
            .expect("natal western should succeed")
            .western.traditional.expect("traditional data should be present");
        assert!(!natal_trad.moon_condition.is_void_of_course);
        assert!(!natal_trad.moon_condition.is_via_combusta);
        assert!(natal_trad.moon_condition.next_applying_aspect.is_none());
        assert!(natal_trad.planetary_hours.radicality_reason.is_empty());
        assert!(natal_trad.planetary_hours.considerations.is_empty());
        assert!(natal_trad.perfection.translation_of_light.is_empty());

        input.is_horary = true;
        let horary_trad = calculate_fortune_western(input)
            .expect("horary western should succeed")
            .western.traditional.expect("traditional data should be present");
        assert!(!horary_trad.planetary_hours.radicality_reason.is_empty());
        assert!(!horary_trad.planetary_hours.considerations.is_empty());
    }
}
