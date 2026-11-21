//! Shared astronomical constants, value objects, and spherical/calendar math.
//!
//! Single source of truth for Julian Day calendar conversions, equatorial transforms,
//! sunrise horizon geometry, and Western body name normalization across all engines.

use serde::{Deserialize, Serialize};
use vedaksha::ephem::julian::{calendar_to_jd, jd_to_calendar as vedaksha_jd_to_calendar};

/// Mean obliquity of the ecliptic at J2000.0 (degrees).
pub const MEAN_OBLIQUITY_J2000_DEG: f64 = 23.439_291_1;

/// Classical Naibod mean solar diurnal arc key (59' 08" per tropical year, degrees).
pub const NAIBOD_DAILY_RATE_DEG: f64 = 59.0 / 60.0 + 8.0 / 3600.0;

/// Mean tropical year length in ephemeris days.
pub const TROPICAL_YEAR_DAYS: f64 = 365.242_198_78;

/// Mean sidereal diurnal rotation of the horizon/meridian (degrees per day).
pub const DIURNAL_ROTATION_DEG_PER_DAY: f64 = 360.9856;

/// Standard solar upper-limb altitude at astronomical sunrise/sunset (-50 arcminutes).
pub const SUNRISE_ALTITUDE_DEG: f64 = -0.833_333_3;

/// Canonical 12 tropical zodiac sign names (0 = Aries .. 11 = Pisces).
pub const ZODIAC_SIGNS: [&str; 12] = [
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces",
];

/// Calendar date (year, month, day).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl CalendarDate {
    pub const fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    pub fn to_julian_day(self) -> f64 {
        calendar_to_jd(self.year, self.month, self.day as f64)
    }
}

/// Calendar date and clock time with fractional seconds.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CalendarDateTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: f64,
}

impl CalendarDateTime {
    pub const fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: f64) -> Self {
        Self { year, month, day, hour, minute, second }
    }

    pub const fn from_date(date: CalendarDate) -> Self {
        Self {
            year: date.year,
            month: date.month,
            day: date.day,
            hour: 0,
            minute: 0,
            second: 0.0,
        }
    }

    pub const fn date(self) -> CalendarDate {
        CalendarDate { year: self.year, month: self.month, day: self.day }
    }

    pub fn to_julian_day(self) -> f64 {
        let day_fraction = self.day as f64
            + (self.hour as f64 + (self.minute as f64 + self.second / 60.0) / 60.0) / 24.0;
        calendar_to_jd(self.year, self.month, day_fraction)
    }
}

/// Geographic observer coordinates in decimal degrees.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GeoCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoCoordinate {
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }
}

/// Solar horizon geometry for sunrise/sunset, planetary hours, and Vedic Suryodaya Vara.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolarHorizonState {
    pub hour_angle_deg: f64,
    pub semi_diurnal_deg: f64,
    pub is_polar_day: bool,
    pub is_polar_night: bool,
    pub local_noon_jd: f64,
}

/// Convert ecliptic longitude (at ecliptic latitude = 0) to equatorial coordinates (RA, Dec) in degrees.
pub fn ecliptic_to_equatorial(longitude_deg: f64, obliquity_deg: f64) -> (f64, f64) {
    let l_rad = longitude_deg.to_radians();
    let eps_rad = obliquity_deg.to_radians();

    let sin_delta = eps_rad.sin() * l_rad.sin();
    let delta_deg = sin_delta.clamp(-1.0, 1.0).asin().to_degrees();

    let alpha_rad = (eps_rad.cos() * l_rad.sin()).atan2(l_rad.cos());
    let alpha_deg = alpha_rad.to_degrees().rem_euclid(360.0);

    (alpha_deg, delta_deg)
}

/// Compute local solar hour angle, semi-diurnal arc (with -50' refraction), polar flags, and local solar noon JD.
pub fn compute_solar_horizon_state(
    sun_tropical_lon: f64,
    obliquity_deg: f64,
    latitude_deg: f64,
    ramc_deg: f64,
    jd_ut1: f64,
) -> SolarHorizonState {
    let (alpha_deg, delta_deg) = ecliptic_to_equatorial(sun_tropical_lon, obliquity_deg);
    let lat_rad = latitude_deg.to_radians();
    let dec_rad = delta_deg.to_radians();
    let h0_rad = SUNRISE_ALTITUDE_DEG.to_radians();

    let denom = lat_rad.cos() * dec_rad.cos();
    let cos_h0 = if denom > 1e-7 {
        (h0_rad.sin() - lat_rad.sin() * dec_rad.sin()) / denom
    } else {
        0.0
    };

    let (semi_diurnal_deg, is_polar_day, is_polar_night) = if cos_h0 < -1.0 {
        (180.0, true, false)
    } else if cos_h0 > 1.0 {
        (0.0, false, true)
    } else {
        (cos_h0.clamp(-1.0, 1.0).acos().to_degrees(), false, false)
    };

    let mut hour_angle_deg = (ramc_deg - alpha_deg).rem_euclid(360.0);
    if hour_angle_deg > 180.0 {
        hour_angle_deg -= 360.0;
    }

    let local_noon_jd = jd_ut1 - (hour_angle_deg / 360.0);

    SolarHorizonState {
        hour_angle_deg,
        semi_diurnal_deg,
        is_polar_day,
        is_polar_night,
        local_noon_jd,
    }
}

/// Convert Julian Day to Gregorian/Julian calendar date and time (UTC) via `vedaksha`.
pub fn jd_to_calendar(jd: f64) -> CalendarDateTime {
    let (year, month, day_frac) = vedaksha_jd_to_calendar(jd);
    let day_floor = day_frac.floor();
    let day = day_floor.max(1.0) as u32;
    let day_sec = ((day_frac - day_floor) * 86_400.0).clamp(0.0, 86_399.999_999);
    let hour = (day_sec / 3600.0).floor() as u32 % 24;
    let rem_sec = day_sec - (hour as f64 * 3600.0);
    let minute = (rem_sec / 60.0).floor() as u32 % 60;
    let second = (rem_sec - (minute as f64 * 60.0)).max(0.0);

    CalendarDateTime::new(year, month, day, hour, minute, second)
}

/// Format Julian Day as `YYYY-MM-DD`.
pub fn format_jd_date(jd: f64) -> String {
    let dt = jd_to_calendar(jd);
    format!("{:04}-{:02}-{:02}", dt.year, dt.month, dt.day)
}

/// Format Julian Day as `YYYY-MM-DD HH:MM:SS UTC` (rounded to nearest second).
pub fn format_jd_utc_seconds(jd: f64) -> String {
    let dt = jd_to_calendar(jd + 0.5 / 86_400.0);
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
        dt.year,
        dt.month,
        dt.day,
        dt.hour,
        dt.minute,
        dt.second.floor() as u32
    )
}

/// Compute completed age and the year of the most recent birthday on or before `target`.
pub fn completed_age(birth: CalendarDate, target: CalendarDate) -> (u32, i32) {
    if (target.month, target.day) >= (birth.month, birth.day) {
        ((target.year - birth.year).max(0) as u32, target.year)
    } else {
        ((target.year - birth.year - 1).max(0) as u32, target.year - 1)
    }
}

/// Normalize Vedic lunar node names (`Rahu`/`Ketu`) to canonical Western names.
pub fn normalize_western_body_name(name: &str) -> String {
    match name {
        "Rahu" => "North Node".to_string(),
        "Ketu" => "South Node".to_string(),
        other => other.to_string(),
    }
}

/// Match an ephemeris body name against a user/API body name (including node aliases).
pub fn matches_western_body(body_name: &str, query: &str) -> bool {
    body_name.eq_ignore_ascii_case(query)
        || (body_name == "Rahu" && query.eq_ignore_ascii_case("North Node"))
        || (body_name == "Ketu" && query.eq_ignore_ascii_case("South Node"))
        || (body_name == "North Node" && query.eq_ignore_ascii_case("Rahu"))
        || (body_name == "South Node" && query.eq_ignore_ascii_case("Ketu"))
}
