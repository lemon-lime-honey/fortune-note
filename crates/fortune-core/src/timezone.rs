//! Offline timezone lookup and historical DST conversion layer.
//!
//! Resolves geographic coordinates (latitude, longitude) into IANA timezone IDs
//! offline via `utz` (embedded compressed polygon dataset), and converts local
//! calendar date/time into exact UTC timestamps via `chrono-tz`.

use std::sync::OnceLock;

use chrono::{Datelike, NaiveDate, NaiveTime, Offset, TimeZone, Timelike};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::celestial::{CalendarDateTime, GeoCoordinate};

#[derive(Error, Debug)]
pub enum TimezoneError {
    #[error("Failed to initialize offline timezone finder: {0}")]
    FinderInitFailed(String),
    #[error("No timezone found for coordinates ({latitude}, {longitude})")]
    ZoneNotFound { latitude: f64, longitude: f64 },
    #[error("Unknown IANA timezone identifier: {0}")]
    UnknownTimezone(String),
    #[error("Invalid local date or time: {0}")]
    InvalidDateTime(String),
    #[error("Timezone conversion failed: {0}")]
    ConversionFailed(String),
}

/// Result of resolving a local time at coordinates to UTC.
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

impl TimezoneResolution {
    pub fn utc_datetime(&self) -> CalendarDateTime {
        CalendarDateTime::new(
            self.utc_year,
            self.utc_month,
            self.utc_day,
            self.utc_hour,
            self.utc_minute,
            self.utc_second,
        )
    }
}

static TZ_FINDER: OnceLock<Result<utz::Finder, String>> = OnceLock::new();

fn get_finder() -> Result<&'static utz::Finder, TimezoneError> {
    let res = TZ_FINDER.get_or_init(|| utz::Finder::new().map_err(|e| format!("{:?}", e)));
    match res {
        Ok(finder) => Ok(finder),
        Err(msg) => Err(TimezoneError::FinderInitFailed(msg.clone())),
    }
}

/// Lookup IANA timezone identifier (e.g., "Asia/Seoul", "America/New_York") from coordinates.
pub fn lookup_timezone_id(latitude: f64, longitude: f64) -> Result<String, TimezoneError> {
    let finder = get_finder()?;
    let pos = utz::Position {
        lat: latitude,
        lon: longitude,
    };

    match finder.lookup(pos) {
        Ok(Some(tz_id)) => Ok(normalize_timezone(tz_id, latitude, longitude)),
        Ok(None) | Err(_) => Err(TimezoneError::ZoneNotFound { latitude, longitude }),
    }
}

/// Determine whether coordinates lie inside South Korea (mainland, Jeju, Ulleungdo, Dokdo)
/// while excluding Tsushima, Iki, Kyushu, and western Honshu across the Korea Strait.
fn is_south_korea_coordinate(latitude: f64, longitude: f64) -> bool {
    if !(33.0..=38.7).contains(&latitude) || !(124.5..=132.0).contains(&longitude) {
        return false;
    }
    if longitude <= 128.5 {
        return true;
    }
    if longitude <= 129.65 {
        let strait_min_lat = 34.35 + (longitude - 128.5) * 0.74;
        return latitude >= strait_min_lat;
    }
    // East of 129.65E, only Ulleungdo/Dokdo/Gyeongsangbuk-do waters (>= 36.4N) belong to South Korea;
    // western Honshu (Shimonoseki/Hagi/Hamada) lies south of 35.0N.
    latitude >= 36.4
}

/// Canonicalize merged timezone identifiers to historically distinct zones.
///
/// utz compact/balanced datasets merge timezones that are identical today (e.g. Asia/Seoul into Asia/Tokyo).
/// We disambiguate South Korea coordinates so historical DST (1987-1988) resolves accurately
/// without misclassifying Kyushu/Chugoku or Tsushima coordinates in Japan.
fn normalize_timezone(tz_id: &str, latitude: f64, longitude: f64) -> String {
    if tz_id == "Asia/Tokyo" && is_south_korea_coordinate(latitude, longitude) {
        return "Asia/Seoul".to_string();
    }
    tz_id.to_string()
}

/// Convert local date/time and coordinates into a resolved UTC timestamp.
///
/// Automatically handles historical daylight saving time (DST) and timezone transitions.
pub fn resolve_local_to_utc(
    dt: CalendarDateTime,
    loc: GeoCoordinate,
) -> Result<TimezoneResolution, TimezoneError> {
    let tz_id = lookup_timezone_id(loc.latitude, loc.longitude)?;
    resolve_with_timezone(dt, &tz_id)
}

/// Convert local date/time into UTC using an optional pre-resolved timezone ID,
/// falling back to coordinate lookup when `timezone_id` is `None`.
pub fn resolve_local_to_utc_with_tz(
    dt: CalendarDateTime,
    loc: GeoCoordinate,
    timezone_id: Option<&str>,
) -> Result<TimezoneResolution, TimezoneError> {
    if let Some(tz_id) = timezone_id.filter(|s| !s.is_empty()) {
        return resolve_with_timezone(dt, tz_id);
    }
    resolve_local_to_utc(dt, loc)
}

/// Convert local date/time with an explicit IANA timezone ID into UTC.
pub fn resolve_with_timezone(
    dt: CalendarDateTime,
    tz_id: &str,
) -> Result<TimezoneResolution, TimezoneError> {
    let sec_u32 = dt.second.floor() as u32;
    let nanos = ((dt.second - sec_u32 as f64) * 1e9).round() as u32;

    let naive_date = NaiveDate::from_ymd_opt(dt.year, dt.month, dt.day).ok_or_else(|| {
        TimezoneError::InvalidDateTime(format!("Invalid date: {:04}-{:02}-{:02}", dt.year, dt.month, dt.day))
    })?;
    let naive_time = NaiveTime::from_hms_nano_opt(dt.hour, dt.minute, sec_u32, nanos).ok_or_else(|| {
        TimezoneError::InvalidDateTime(format!("Invalid time: {:02}:{:02}:{:02}", dt.hour, dt.minute, sec_u32))
    })?;
    let naive_dt = naive_date.and_time(naive_time);

    let tz: Tz = tz_id
        .parse()
        .map_err(|_| TimezoneError::UnknownTimezone(tz_id.to_string()))?;

    let local_dt = match tz.from_local_datetime(&naive_dt) {
        chrono::LocalResult::Single(resolved) => resolved,
        chrono::LocalResult::Ambiguous(first, _) => first,
        chrono::LocalResult::None => {
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

    fn assert_utc_resolution(
        dt: CalendarDateTime,
        loc: GeoCoordinate,
        expected_tz: &str,
        expected_offset: f64,
        expected_utc_hour: u32,
        expected_utc_min: u32,
    ) {
        let res = resolve_local_to_utc(dt, loc).expect("resolution should succeed");
        assert_eq!(res.timezone_id, expected_tz);
        assert_eq!(res.utc_offset_hours, expected_offset);
        assert_eq!(res.utc_hour, expected_utc_hour);
        assert_eq!(res.utc_minute, expected_utc_min);
    }

    #[test]
    fn test_lookup_seoul_and_new_york_timezones() {
        assert_eq!(lookup_timezone_id(37.5665, 126.9780).unwrap(), "Asia/Seoul");
        assert_eq!(lookup_timezone_id(37.2429, 131.8670).unwrap(), "Asia/Seoul");
        assert_eq!(lookup_timezone_id(40.7128, -74.0060).unwrap(), "America/New_York");
    }

    #[test]
    fn test_seoul_standard_and_1988_olympic_dst() {
        let seoul = GeoCoordinate::new(37.5665, 126.9780);
        assert_utc_resolution(
            CalendarDateTime::new(2024, 2, 4, 17, 28, 0.0),
            seoul,
            "Asia/Seoul",
            9.0,
            8,
            28,
        );
        assert_utc_resolution(
            CalendarDateTime::new(1988, 8, 10, 14, 30, 0.0),
            seoul,
            "Asia/Seoul",
            10.0,
            4,
            30,
        );
    }

    #[test]
    fn test_western_japan_not_misclassified_as_seoul() {
        for (city, lat, lon) in [
            ("Fukuoka", 33.5902, 130.4017),
            ("Kitakyushu", 33.8834, 130.8752),
            ("Shimonoseki", 33.9578, 130.9415),
            ("Tsushima", 34.2027, 129.2878),
            ("Saga", 33.2635, 130.3009),
        ] {
            assert_eq!(
                lookup_timezone_id(lat, lon).unwrap(),
                "Asia/Tokyo",
                "{} must resolve to Asia/Tokyo",
                city
            );
        }

        // Japan did not observe South Korea's 1988 Olympic DST (must remain UTC+9).
        assert_utc_resolution(
            CalendarDateTime::new(1988, 8, 10, 14, 30, 0.0),
            GeoCoordinate::new(33.5902, 130.4017),
            "Asia/Tokyo",
            9.0,
            5,
            30,
        );

        for (city, lat, lon) in [
            ("Busan", 35.1796, 129.0756),
            ("Geoje", 34.8806, 128.6211),
            ("Jeju", 33.4996, 126.5312),
            ("Ulleungdo", 37.4844, 130.9057),
            ("Dokdo", 37.2429, 131.8670),
        ] {
            assert_eq!(
                lookup_timezone_id(lat, lon).unwrap(),
                "Asia/Seoul",
                "{} must resolve to Asia/Seoul",
                city
            );
        }
    }

    #[test]
    fn test_new_york_winter_and_summer_dst() {
        let nyc = GeoCoordinate::new(40.7128, -74.0060);
        assert_utc_resolution(
            CalendarDateTime::new(2024, 1, 15, 12, 0, 0.0),
            nyc,
            "America/New_York",
            -5.0,
            17,
            0,
        );
        assert_utc_resolution(
            CalendarDateTime::new(2024, 7, 15, 12, 0, 0.0),
            nyc,
            "America/New_York",
            -4.0,
            16,
            0,
        );
    }
}
