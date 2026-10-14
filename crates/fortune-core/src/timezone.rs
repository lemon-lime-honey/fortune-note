//! Timezone lookup and historical UTC resolution engine.
//!
//! Provides offline, deterministic conversion from local coordinates and calendar
//! timestamps to IANA timezones and historical UTC timestamps (including historical DST).

use chrono::{Datelike, NaiveDate, NaiveTime, Offset, TimeZone, Timelike};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimezoneError {
    #[error("Failed to initialize offline timezone finder: {0}")]
    FinderInitFailed(String),

    #[error("Coordinate ({latitude}, {longitude}) is out of range or not claimed by any timezone")]
    ZoneNotFound { latitude: f64, longitude: f64 },

    #[error("Invalid calendar date or time: {0}")]
    InvalidDateTime(String),

    #[error("Unknown IANA timezone: {0}")]
    UnknownTimezone(String),

    #[error("Time conversion failed: {0}")]
    ConversionFailed(String),
}

/// Resolved UTC timestamp and timezone metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimezoneResolution {
    pub timezone_id: String,
    pub utc_year: i32,
    pub utc_month: u32,
    pub utc_day: u32,
    pub utc_hour: u32,
    pub utc_minute: u32,
    pub utc_second: f64,
    pub utc_offset_hours: f64,
}

use std::sync::OnceLock;

static FINDER: OnceLock<Result<utz::Finder, String>> = OnceLock::new();

fn get_finder() -> Result<&'static utz::Finder, TimezoneError> {
    let res = FINDER.get_or_init(|| utz::Finder::new().map_err(|e| format!("{:?}", e)));
    match res {
        Ok(f) => Ok(f),
        Err(e) => Err(TimezoneError::FinderInitFailed(e.clone())),
    }
}

/// Look up the IANA timezone ID for the given latitude and longitude.
pub fn lookup_timezone_id(latitude: f64, longitude: f64) -> Result<String, TimezoneError> {
    let finder = get_finder()?;

    let pos = utz::Position {
        lat: latitude,
        lon: longitude,
    };

    match finder.lookup(pos) {
        Ok(Some(tz_id)) => Ok(normalize_timezone(tz_id, latitude, longitude)),
        Ok(None) => Err(TimezoneError::ZoneNotFound { latitude, longitude }),
        Err(_) => Err(TimezoneError::ZoneNotFound { latitude, longitude }),
    }
}

/// Canonicalize merged timezone identifiers to historically distinct zones.
///
/// utz compact/balanced datasets merge timezones that are identical today (e.g. Asia/Seoul into Asia/Tokyo).
/// We disambiguate South Korea coordinates so historical DST (1987-1988) resolves accurately.
fn normalize_timezone(tz_id: &str, latitude: f64, longitude: f64) -> String {
    if tz_id == "Asia/Tokyo"
        && (33.0..=38.7).contains(&latitude)
        && (124.5..=132.0).contains(&longitude)
    {
        return "Asia/Seoul".to_string();
    }
    tz_id.to_string()
}

/// Convert local date/time and coordinates into a resolved UTC timestamp.
///
/// Automatically handles historical daylight saving time (DST) and timezone transitions.
pub fn resolve_local_to_utc(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: f64,
    latitude: f64,
    longitude: f64,
) -> Result<TimezoneResolution, TimezoneError> {
    let tz_id = lookup_timezone_id(latitude, longitude)?;
    resolve_with_timezone(year, month, day, hour, minute, second, &tz_id)
}

/// Convert local date/time with an explicit IANA timezone ID into UTC.
pub fn resolve_with_timezone(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: f64,
    tz_id: &str,
) -> Result<TimezoneResolution, TimezoneError> {
    let sec_u32 = second.floor() as u32;
    let nanos = ((second - sec_u32 as f64) * 1e9).round() as u32;

    let naive_date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| TimezoneError::InvalidDateTime(format!("Invalid date: {:04}-{:02}-{:02}", year, month, day)))?;
    let naive_time = NaiveTime::from_hms_nano_opt(hour, minute, sec_u32, nanos)
        .ok_or_else(|| TimezoneError::InvalidDateTime(format!("Invalid time: {:02}:{:02}:{:02}", hour, minute, sec_u32)))?;
    let naive_dt = naive_date.and_time(naive_time);

    let tz: Tz = tz_id
        .parse()
        .map_err(|_| TimezoneError::UnknownTimezone(tz_id.to_string()))?;

    let local_dt = match tz.from_local_datetime(&naive_dt) {
        chrono::LocalResult::Single(dt) => dt,
        chrono::LocalResult::Ambiguous(dt1, _) => dt1,
        chrono::LocalResult::None => {
            // Gap during DST spring forward: advance by 1 hour to resolve
            let shifted = naive_dt + chrono::Duration::hours(1);
            tz.from_local_datetime(&shifted)
                .earliest()
                .ok_or_else(|| TimezoneError::ConversionFailed("DST gap transition could not be resolved".to_string()))?
        }
    };

    let utc_dt = local_dt.with_timezone(&chrono::Utc);
    let offset_secs = local_dt.offset().fix().local_minus_utc();
    let utc_offset_hours = offset_secs as f64 / 3600.0;

    let utc_second = utc_dt.second() as f64 + (utc_dt.nanosecond() as f64 / 1e9);

    Ok(TimezoneResolution {
        timezone_id: tz_id.to_string(),
        utc_year: utc_dt.year(),
        utc_month: utc_dt.month(),
        utc_day: utc_dt.day(),
        utc_hour: utc_dt.hour(),
        utc_minute: utc_dt.minute(),
        utc_second,
        utc_offset_hours,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_seoul_timezone() {
        let tz = lookup_timezone_id(37.5665, 126.9780).expect("Seoul lookup should succeed");
        assert_eq!(tz, "Asia/Seoul");

        // Dokdo (37.2429, 131.8670) should correctly resolve to Asia/Seoul
        let tz_dokdo = lookup_timezone_id(37.2429, 131.8670).expect("Dokdo lookup should succeed");
        assert_eq!(tz_dokdo, "Asia/Seoul");
    }

    #[test]
    fn test_lookup_new_york_timezone() {
        let tz = lookup_timezone_id(40.7128, -74.0060).expect("New York lookup should succeed");
        assert_eq!(tz, "America/New_York");
    }

    #[test]
    fn test_seoul_standard_time_resolution() {
        // 2024-02-04 17:28 KST (Standard UTC+9)
        let res = resolve_local_to_utc(2024, 2, 4, 17, 28, 0.0, 37.5665, 126.9780)
            .expect("resolution should succeed");

        assert_eq!(res.timezone_id, "Asia/Seoul");
        assert_eq!(res.utc_offset_hours, 9.0);
        assert_eq!(res.utc_year, 2024);
        assert_eq!(res.utc_month, 2);
        assert_eq!(res.utc_day, 4);
        assert_eq!(res.utc_hour, 8);
        assert_eq!(res.utc_minute, 28);
    }

    #[test]
    fn test_seoul_1988_olympic_summer_time() {
        // South Korea observed DST in 1987 and 1988 (UTC+10).
        // On 1988-08-10 at 14:30 KDT, UTC was 04:30.
        let res = resolve_local_to_utc(1988, 8, 10, 14, 30, 0.0, 37.5665, 126.9780)
            .expect("resolution should succeed");

        assert_eq!(res.timezone_id, "Asia/Seoul");
        assert_eq!(res.utc_offset_hours, 10.0, "1988 Olympic Summer Time must be UTC+10");
        assert_eq!(res.utc_year, 1988);
        assert_eq!(res.utc_month, 8);
        assert_eq!(res.utc_day, 10);
        assert_eq!(res.utc_hour, 4);
        assert_eq!(res.utc_minute, 30);
    }

    #[test]
    fn test_new_york_winter_and_summer_dst() {
        // Winter: EST (UTC-5)
        let winter = resolve_local_to_utc(2024, 1, 15, 12, 0, 0.0, 40.7128, -74.0060)
            .expect("winter resolution should succeed");
        assert_eq!(winter.utc_offset_hours, -5.0);
        assert_eq!(winter.utc_hour, 17);

        // Summer: EDT (UTC-4)
        let summer = resolve_local_to_utc(2024, 7, 15, 12, 0, 0.0, 40.7128, -74.0060)
            .expect("summer resolution should succeed");
        assert_eq!(summer.utc_offset_hours, -4.0);
        assert_eq!(summer.utc_hour, 16);
    }
}
