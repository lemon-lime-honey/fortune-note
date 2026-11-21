//! Saju (Four Pillars and DaeUn) pure character calculation layer.
//!
//! Provides lightweight computation strictly focused on:
//! - Four Pillars (Year, Month, Day, Hour) GanZhi characters
//! - DaeUn (10-year major luck) start age and pillar sequence

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use vedaksha::ephem::julian::calendar_to_jd;

use crate::celestial::{ecliptic_to_equatorial, NAIBOD_DAILY_RATE_DEG};
use crate::ephemeris::{find_solar_longitude_moment, EphemerisError, EphemerisSnapshot};

#[derive(Error, Debug)]
pub enum SajuError {
    #[error("Ephemeris calculation failed: {0}")]
    Ephemeris(#[from] EphemerisError),
}

pub const HEAVENLY_STEMS: [&str; 10] = ["갑", "을", "병", "정", "무", "기", "경", "신", "임", "계"];
pub const HEAVENLY_STEMS_HANJA: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub const EARTHLY_BRANCHES: [&str; 12] = ["자", "축", "인", "묘", "진", "사", "오", "미", "신", "유", "술", "해"];
pub const EARTHLY_BRANCHES_HANJA: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pillar {
    pub stem_index: u8,
    pub branch_index: u8,
}

impl Pillar {
    pub fn new(stem: u8, branch: u8) -> Self {
        Self { stem_index: stem % 10, branch_index: branch % 12 }
    }

    pub fn korean(&self) -> String {
        format!("{}{}", HEAVENLY_STEMS[self.stem_index as usize], EARTHLY_BRANCHES[self.branch_index as usize])
    }

    pub fn hanja(&self) -> String {
        format!("{}{}", HEAVENLY_STEMS_HANJA[self.stem_index as usize], EARTHLY_BRANCHES_HANJA[self.branch_index as usize])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaeUnPeriod {
    pub order: u8,
    pub start_age: u32,
    pub start_jd: f64,
    pub pillar: Pillar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SajuChart {
    pub year_pillar: Pillar,
    pub month_pillar: Pillar,
    pub day_pillar: Pillar,
    pub hour_pillar: Pillar,
    pub daeun_number: u32,
    pub is_forward_daeun: bool,
    pub daeun_periods: Vec<DaeUnPeriod>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZiHourMode {
    NextDayAt23,
    MidnightAt00,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SajuOptions {
    pub zi_hour_mode: ZiHourMode,
    pub apply_equation_of_time: bool,
}

impl Default for SajuOptions {
    fn default() -> Self {
        Self { zi_hour_mode: ZiHourMode::NextDayAt23, apply_equation_of_time: false }
    }
}

const MAJOR_TERMS: [(f64, u8); 12] = [
    (315.0, 2), (345.0, 3), (15.0, 4), (45.0, 5), (75.0, 6), (105.0, 7),
    (135.0, 8), (165.0, 9), (195.0, 10), (225.0, 11), (255.0, 0), (285.0, 1),
];

fn find_surrounding_major_terms(sun_lon: f64, jd: f64) -> Result<(u8, f64, f64), SajuError> {
    let mut prev_idx = 0;
    for i in 0..12 {
        let (l_curr, _) = MAJOR_TERMS[i];
        let (l_next, _) = MAJOR_TERMS[(i + 1) % 12];
        let inside = if l_curr > l_next {
            sun_lon >= l_curr || sun_lon < l_next
        } else {
            sun_lon >= l_curr && sun_lon < l_next
        };
        if inside {
            prev_idx = i;
            break;
        }
    }

    let (prev_lon, month_branch) = MAJOR_TERMS[prev_idx];
    let (next_lon, _) = MAJOR_TERMS[(prev_idx + 1) % 12];
    let prev_diff = (sun_lon - prev_lon).rem_euclid(360.0);
    let next_diff = (next_lon - sun_lon).rem_euclid(360.0);

    let prev_jd = find_solar_longitude_moment(jd - (prev_diff / NAIBOD_DAILY_RATE_DEG), prev_lon)?;
    let next_jd = find_solar_longitude_moment(jd + (next_diff / NAIBOD_DAILY_RATE_DEG), next_lon)?;
    Ok((month_branch, prev_jd, next_jd))
}

fn build_daeun_periods(
    month_stem: u8,
    month_branch: u8,
    jd: f64,
    term_diff_days: f64,
    is_forward: bool,
) -> (u32, Vec<DaeUnPeriod>) {
    let daeun_number = (term_diff_days / 3.0).round().clamp(1.0, 10.0) as u32;
    let periods = (1..=10u8)
        .map(|i| DaeUnPeriod {
            order: i,
            start_age: daeun_number + ((i - 1) as u32 * 10),
            start_jd: jd + term_diff_days + ((i - 1) as f64 * 10.0 * 365.25),
            pillar: calculate_daeun_pillar(month_stem, month_branch, i, is_forward),
        })
        .collect();
    (daeun_number, periods)
}

/// Calculate Four Pillars and DaeUn with configurable Saju options.
pub fn calculate_saju_with_options(
    ephem: &EphemerisSnapshot,
    year: i32,
    longitude: f64,
    gender: Gender,
    options: SajuOptions,
) -> Result<SajuChart, SajuError> {
    let sun_lon = ephem.get_body("Sun").expect("Sun required").tropical_longitude;
    let jd = ephem.jd_ut1;
    let (month_branch, prev_jd, next_jd) = find_surrounding_major_terms(sun_lon, jd)?;

    let lichun_jd = find_solar_longitude_moment(calendar_to_jd(year, 2, 4.0), 315.0)?;
    let saju_year = if jd < lichun_jd { year - 1 } else { year };
    let year_stem = (saju_year - 4).rem_euclid(10) as u8;
    let year_branch = (saju_year - 4).rem_euclid(12) as u8;
    let year_pillar = Pillar::new(year_stem, year_branch);

    let month_stem = ((year_stem % 5) * 2 + 2 + (month_branch + 10) % 12) % 10;
    let month_pillar = Pillar::new(month_stem, month_branch);

    let local_jd = jd + (longitude / 15.0 / 24.0);
    let effective_solar_hours = if options.apply_equation_of_time {
        let (sun_ra, _) = ecliptic_to_equatorial(sun_lon, ephem.true_obliquity_deg);
        ((ephem.ramc - sun_ra + 180.0).rem_euclid(360.0)) / 15.0
    } else {
        ((local_jd + 0.5).rem_euclid(1.0)) * 24.0
    };

    let (day_pillar, hour_pillar) = calculate_day_and_hour_pillars(local_jd, effective_solar_hours, options.zi_hour_mode);

    let is_year_yang = year_stem.is_multiple_of(2);
    let is_forward_daeun = match gender {
        Gender::Male => is_year_yang,
        Gender::Female => !is_year_yang,
    };
    let term_diff_days = if is_forward_daeun { next_jd - jd } else { jd - prev_jd };
    let (daeun_number, daeun_periods) =
        build_daeun_periods(month_stem, month_branch, jd, term_diff_days, is_forward_daeun);

    Ok(SajuChart {
        year_pillar,
        month_pillar,
        day_pillar,
        hour_pillar,
        daeun_number,
        is_forward_daeun,
        daeun_periods,
    })
}

/// Calculate Four Pillars and DaeUn with standard default options.
pub fn calculate_saju(
    ephem: &EphemerisSnapshot,
    year: i32,
    longitude: f64,
    gender: Gender,
) -> Result<SajuChart, SajuError> {
    calculate_saju_with_options(ephem, year, longitude, gender, SajuOptions::default())
}

/// Calculate Day Pillar from local Julian Date (Epoch: 2000-01-01 12:00 UT = Wu-Wu 戊午).
pub fn calculate_day_pillar(local_jd: f64) -> (Pillar, f64) {
    let day_number = (local_jd + 0.5).floor() as i64;
    let day_diff = day_number - 2_451_545;
    let day_stem = (4 + day_diff).rem_euclid(10) as u8;
    let day_branch = (6 + day_diff).rem_euclid(12) as u8;
    let local_solar_hours = ((local_jd + 0.5).rem_euclid(1.0)) * 24.0;
    (Pillar::new(day_stem, day_branch), local_solar_hours)
}

/// Calculate Day Pillar and Hour Pillar according to local hours and ZiHourMode.
pub fn calculate_day_and_hour_pillars(
    local_jd: f64,
    effective_solar_hours: f64,
    zi_mode: ZiHourMode,
) -> (Pillar, Pillar) {
    let (base_day, _) = calculate_day_pillar(local_jd);
    let hour_branch = calculate_hour_branch(effective_solar_hours);

    let day_pillar = if hour_branch == 0 && effective_solar_hours >= 23.0 && zi_mode == ZiHourMode::NextDayAt23 {
        Pillar::new(base_day.stem_index + 1, base_day.branch_index + 1)
    } else {
        base_day
    };

    let hour_stem = calculate_hour_stem(day_pillar.stem_index, hour_branch);
    (day_pillar, Pillar::new(hour_stem, hour_branch))
}

/// Calculate Earthly Branch index (0..11) for local solar hours (0.0..24.0).
pub fn calculate_hour_branch(local_solar_hours: f64) -> u8 {
    let normalized = local_solar_hours.rem_euclid(24.0);
    (((normalized + 1.0) / 2.0).floor() as u8) % 12
}

/// Calculate Hour Heavenly Stem index (0..9) using Five Rats rule (오서둔시법).
pub fn calculate_hour_stem(day_stem: u8, hour_branch: u8) -> u8 {
    ((day_stem % 5) * 2 + hour_branch) % 10
}

/// Calculate DaeUn pillar at a given progression step (1-indexed).
pub fn calculate_daeun_pillar(
    month_stem: u8,
    month_branch: u8,
    step: u8,
    is_forward: bool,
) -> Pillar {
    let delta = if is_forward { step as i16 } else { -(step as i16) };
    let s = (month_stem as i16 + delta).rem_euclid(10) as u8;
    let b = (month_branch as i16 + delta).rem_euclid(12) as u8;
    Pillar::new(s, b)
}
