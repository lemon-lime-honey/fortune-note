//! Saju (Four Pillars and DaeUn) pure character calculation layer.
//!
//! Provides lightweight computation strictly focused on:
//! - Four Pillars (Year, Month, Day, Hour) GanZhi characters
//! - DaeUn (10-year major luck) start age and pillar sequence

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ephemeris::{find_solar_longitude_moment, EphemerisError, EphemerisSnapshot};
use vedaksha::ephem::julian::calendar_to_jd;

#[derive(Error, Debug)]
pub enum SajuError {
    #[error("Ephemeris calculation failed: {0}")]
    Ephemeris(#[from] EphemerisError),
}

/// 10 Heavenly Stems (천간).
pub const HEAVENLY_STEMS: [&str; 10] = ["갑", "을", "병", "정", "무", "기", "경", "신", "임", "계"];
pub const HEAVENLY_STEMS_HANJA: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

/// 12 Earthly Branches (지지).
pub const EARTHLY_BRANCHES: [&str; 12] = ["자", "축", "인", "묘", "진", "사", "오", "미", "신", "유", "술", "해"];
pub const EARTHLY_BRANCHES_HANJA: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

/// A single pillar consisting of Heavenly Stem and Earthly Branch indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pillar {
    pub stem_index: u8,   // 0..9
    pub branch_index: u8, // 0..11
}

impl Pillar {
    pub fn new(stem: u8, branch: u8) -> Self {
        Self {
            stem_index: stem % 10,
            branch_index: branch % 12,
        }
    }

    pub fn korean(&self) -> String {
        format!(
            "{}{}",
            HEAVENLY_STEMS[self.stem_index as usize],
            EARTHLY_BRANCHES[self.branch_index as usize]
        )
    }

    pub fn hanja(&self) -> String {
        format!(
            "{}{}",
            HEAVENLY_STEMS_HANJA[self.stem_index as usize],
            EARTHLY_BRANCHES_HANJA[self.branch_index as usize]
        )
    }
}

/// Biological gender for forward/reverse fortune direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

/// 10-year major luck period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaeUnPeriod {
    pub order: u8,
    pub start_age: u32,
    pub start_jd: f64,
    pub pillar: Pillar,
}

/// Complete Saju Four Pillars and DaeUn result.
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

/// Zi hour (자시) day pillar transition mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZiHourMode {
    /// 23:00~24:00 advances to the next day's pillar (Classical single Zi hour tradition).
    NextDayAt23,
    /// 23:00~24:00 retains today's day pillar (Night Zi / Early Zi division tradition).
    MidnightAt00,
}

/// Options for Saju pillar calculations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SajuOptions {
    pub zi_hour_mode: ZiHourMode,
    pub apply_equation_of_time: bool,
}

impl Default for SajuOptions {
    fn default() -> Self {
        Self {
            zi_hour_mode: ZiHourMode::NextDayAt23,
            apply_equation_of_time: false,
        }
    }
}

/// 12 Major Solar Terms defining months (LiChun 315°, JingZhe 345°, etc.).
const MAJOR_TERMS: [(f64, u8); 12] = [
    (315.0, 2),  // 입춘 -> 인월 (寅: 2)
    (345.0, 3),  // 경칩 -> 묘월 (卯: 3)
    (15.0, 4),   // 청명 -> 진월 (辰: 4)
    (45.0, 5),   // 입하 -> 사월 (巳: 5)
    (75.0, 6),   // 망종 -> 오월 (午: 6)
    (105.0, 7),  // 소서 -> 미월 (未: 7)
    (135.0, 8),  // 입추 -> 신월 (申: 8)
    (165.0, 9),  // 백로 -> 유월 (酉: 9)
    (195.0, 10), // 한로 -> 술월 (戌: 10)
    (225.0, 11), // 입동 -> 해월 (亥: 11)
    (255.0, 0),  // 대설 -> 자월 (子: 0)
    (285.0, 1),  // 소한 -> 축월 (丑: 1)
];

fn ecliptic_to_ra(lon_deg: f64, eps_deg: f64) -> f64 {
    let l_rad = lon_deg.to_radians();
    let eps_rad = eps_deg.to_radians();
    let alpha_rad = (eps_rad.cos() * l_rad.sin()).atan2(l_rad.cos());
    alpha_rad.to_degrees().rem_euclid(360.0)
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

    // Find current major solar term boundary
    let mut prev_idx = 0;
    let mut next_idx = 1;
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
            next_idx = (i + 1) % 12;
            break;
        }
    }

    let (prev_target_lon, month_branch) = MAJOR_TERMS[prev_idx];
    let (next_target_lon, _) = MAJOR_TERMS[next_idx];

    let mut prev_diff = sun_lon - prev_target_lon;
    if prev_diff < 0.0 { prev_diff += 360.0; }
    let prev_jd = find_solar_longitude_moment(jd - (prev_diff / 0.9856), prev_target_lon)?;

    let mut next_diff = next_target_lon - sun_lon;
    if next_diff < 0.0 { next_diff += 360.0; }
    let next_jd = find_solar_longitude_moment(jd + (next_diff / 0.9856), next_target_lon)?;

    // 1. Year Pillar: Begins at LiChun (315°)
    let approx_lichun = calendar_to_jd(year, 2, 4.0);
    let lichun_jd = find_solar_longitude_moment(approx_lichun, 315.0)?;
    let saju_year = if jd < lichun_jd { year - 1 } else { year };

    let year_stem = ((saju_year - 4).rem_euclid(10)) as u8;
    let year_branch = ((saju_year - 4).rem_euclid(12)) as u8;
    let year_pillar = Pillar::new(year_stem, year_branch);

    // 2. Month Pillar: Five Tigers rule (오호둔월법)
    let base_month_stem = (year_stem % 5) * 2 + 2;
    let branch_offset = (month_branch + 12 - 2) % 12; // Offset from Yin (2)
    let month_stem = (base_month_stem + branch_offset) % 10;
    let month_pillar = Pillar::new(month_stem, month_branch);

    // 3. Day Pillar & Hour Pillar (True Solar Time or Local Mean Solar Time)
    let local_jd = jd + (longitude / 15.0 / 24.0);
    let effective_solar_hours = if options.apply_equation_of_time {
        let sun_ra = ecliptic_to_ra(sun_lon, ephem.true_obliquity_deg);
        ((ephem.ramc - sun_ra + 180.0).rem_euclid(360.0)) / 15.0
    } else {
        ((local_jd + 0.5).rem_euclid(1.0)) * 24.0
    };

    let (day_pillar, hour_pillar) = calculate_day_and_hour_pillars(
        local_jd,
        effective_solar_hours,
        options.zi_hour_mode,
    );

    // 4. DaeUn (10-year major luck progression)
    let is_year_yang = year_stem.is_multiple_of(2);
    let is_forward_daeun = match gender {
        Gender::Male => is_year_yang,
        Gender::Female => !is_year_yang,
    };

    let term_diff_days = if is_forward_daeun { next_jd - jd } else { jd - prev_jd };
    let daeun_number = (term_diff_days / 3.0).round().clamp(1.0, 10.0) as u32;

    let mut daeun_periods = Vec::with_capacity(10);
    for i in 1..=10u8 {
        let pillar = calculate_daeun_pillar(month_stem, month_branch, i, is_forward_daeun);
        let start_age = daeun_number + ((i - 1) as u32 * 10);
        let start_jd = jd + term_diff_days + ((i - 1) as f64 * 10.0 * 365.25);

        daeun_periods.push(DaeUnPeriod {
            order: i,
            start_age,
            start_jd,
            pillar,
        });
    }

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

/// Calculate Day Pillar and Hour Pillar according to local hours and ZiHourMode.
pub fn calculate_day_and_hour_pillars(
    local_jd: f64,
    effective_solar_hours: f64,
    zi_mode: ZiHourMode,
) -> (Pillar, Pillar) {
    let base_day_number = (local_jd + 0.5).floor() as i64;
    let base_day_diff = base_day_number - 2451545;
    let base_day_stem = (4 + base_day_diff).rem_euclid(10) as u8;
    let base_day_branch = (6 + base_day_diff).rem_euclid(12) as u8;

    let hour_branch = calculate_hour_branch(effective_solar_hours);

    let (day_stem, day_branch, hour_stem_base) = if hour_branch == 0 && effective_solar_hours >= 23.0 {
        match zi_mode {
            ZiHourMode::NextDayAt23 => {
                let next_stem = (base_day_stem + 1) % 10;
                let next_branch = (base_day_branch + 1) % 12;
                (next_stem, next_branch, next_stem)
            }
            ZiHourMode::MidnightAt00 => {
                (base_day_stem, base_day_branch, base_day_stem)
            }
        }
    } else {
        (base_day_stem, base_day_branch, base_day_stem)
    };

    let hour_stem = calculate_hour_stem(hour_stem_base, hour_branch);
    (Pillar::new(day_stem, day_branch), Pillar::new(hour_stem, hour_branch))
}

/// Calculate Day Pillar from local Julian Date.
/// Epoch: 2000-01-01 12:00 UT was Wu-Wu (戊午: stem 4, branch 6).
pub fn calculate_day_pillar(local_jd: f64) -> (Pillar, f64) {
    let day_number = (local_jd + 0.5).floor() as i64;
    let day_diff = day_number - 2451545;
    let day_stem = (4 + day_diff).rem_euclid(10) as u8;
    let day_branch = (6 + day_diff).rem_euclid(12) as u8;

    let day_fraction = (local_jd + 0.5).rem_euclid(1.0);
    let local_solar_hours = day_fraction * 24.0;

    (Pillar::new(day_stem, day_branch), local_solar_hours)
}

/// Calculate Earthly Branch index (0..11) for local solar hours (0.0..24.0).
/// Zi hour (자시) spans 23:00 to 01:00.
pub fn calculate_hour_branch(local_solar_hours: f64) -> u8 {
    let normalized_hours = local_solar_hours.rem_euclid(24.0);
    (((normalized_hours + 1.0) / 2.0).floor() as u8) % 12
}

/// Calculate Hour Heavenly Stem index (0..9) using Five Rats rule (오서둔시법).
pub fn calculate_hour_stem(day_stem: u8, hour_branch: u8) -> u8 {
    let base_hour_stem = (day_stem % 5) * 2;
    (base_hour_stem + hour_branch) % 10
}

/// Calculate DaeUn pillar at a given progression step (1-indexed).
/// Uses mathematical Euclidean remainder for guaranteed safe modular arithmetic.
pub fn calculate_daeun_pillar(
    month_stem: u8,
    month_branch: u8,
    step: u8,
    is_forward: bool,
) -> Pillar {
    let step_i16 = step as i16;
    let s = if is_forward {
        (month_stem as i16 + step_i16).rem_euclid(10) as u8
    } else {
        (month_stem as i16 - step_i16).rem_euclid(10) as u8
    };
    let b = if is_forward {
        (month_branch as i16 + step_i16).rem_euclid(12) as u8
    } else {
        (month_branch as i16 - step_i16).rem_euclid(12) as u8
    };
    Pillar::new(s, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ephemeris::compute_ephemeris;

    #[test]
    fn test_saju_pillars_and_daeun() {
        let ephem = compute_ephemeris(2000, 1, 1, 12, 0, 0.0, 37.5665, 126.9780)
            .expect("ephem should succeed");

        let saju = calculate_saju(&ephem, 2000, 126.9780, Gender::Male)
            .expect("saju should succeed");

        assert_eq!(saju.day_pillar.korean(), "무오");
        assert_eq!(saju.day_pillar.hanja(), "戊午");
        assert_eq!(saju.daeun_periods.len(), 10);
        assert!((1..=10).contains(&saju.daeun_number));
    }

    #[test]
    fn test_saju_lichun_boundary_transition() {
        // 2024 LiChun occurred at 2024-02-04 08:27 UTC (17:27 KST).
        // 1 minute before LiChun (08:26 UTC): Still Gye-Myo year (癸卯), Eul-Chuk month (乙丑)
        let ephem_before = compute_ephemeris(2024, 2, 4, 8, 26, 0.0, 37.5665, 126.9780)
            .expect("ephem before should succeed");
        let saju_before = calculate_saju(&ephem_before, 2024, 126.9780, Gender::Male)
            .expect("saju before should succeed");
        assert_eq!(saju_before.year_pillar.korean(), "계묘");
        assert_eq!(saju_before.month_pillar.korean(), "을축");

        // 1 minute after LiChun (08:28 UTC): Transitions to Gap-Jin year (甲辰), Byeong-In month (丙寅)
        let ephem_after = compute_ephemeris(2024, 2, 4, 8, 28, 0.0, 37.5665, 126.9780)
            .expect("ephem after should succeed");
        let saju_after = calculate_saju(&ephem_after, 2024, 126.9780, Gender::Male)
            .expect("saju after should succeed");
        assert_eq!(saju_after.year_pillar.korean(), "갑진");
        assert_eq!(saju_after.month_pillar.korean(), "병인");
    }

    #[test]
    fn test_saju_daeun_gender_direction() {
        // Gap-Jin year (Yang wood stem):
        // Male -> Forward progression
        // Female -> Reverse progression
        let ephem = compute_ephemeris(2024, 3, 1, 12, 0, 0.0, 37.5665, 126.9780)
            .expect("ephem should succeed");

        let saju_male = calculate_saju(&ephem, 2024, 126.9780, Gender::Male)
            .expect("male saju should succeed");
        assert!(saju_male.is_forward_daeun, "Yang male must be forward DaeUn");
        assert_eq!(saju_male.daeun_periods.len(), 10);

        let saju_female = calculate_saju(&ephem, 2024, 126.9780, Gender::Female)
            .expect("female saju should succeed");
        assert!(!saju_female.is_forward_daeun, "Yang female must be reverse DaeUn");
        assert_eq!(saju_female.daeun_periods.len(), 10);

        // Next daeun pillar direction check
        let m_first = &saju_male.daeun_periods[0].pillar;
        let f_first = &saju_female.daeun_periods[0].pillar;
        assert_ne!(m_first.stem_index, f_first.stem_index);
    }

    #[test]
    fn test_hour_branch_zi_boundary() {
        // Hai hour (해시: 21:00 ~ 23:00, index 11)
        assert_eq!(calculate_hour_branch(22.9), 11);

        // Zi hour (자시: 23:00 ~ 01:00, index 0)
        assert_eq!(calculate_hour_branch(23.0), 0);
        assert_eq!(calculate_hour_branch(23.5), 0);
        assert_eq!(calculate_hour_branch(0.0), 0);
        assert_eq!(calculate_hour_branch(0.5), 0);
        assert_eq!(calculate_hour_branch(0.99), 0);

        // Chou hour (축시: 01:00 ~ 03:00, index 1)
        assert_eq!(calculate_hour_branch(1.0), 1);
        assert_eq!(calculate_hour_branch(1.5), 1);

        // Wrap around safety
        assert_eq!(calculate_hour_branch(24.0), 0);
        assert_eq!(calculate_hour_branch(24.5), 0);
    }

    #[test]
    fn test_hour_stem_five_rats() {
        // Gap (0) or Gi (5) day -> Zi hour is Gap-Zi (0)
        assert_eq!(calculate_hour_stem(0, 0), 0);
        assert_eq!(calculate_hour_stem(5, 0), 0);

        // Eul (1) or Gyeong (6) day -> Zi hour is Byeong-Zi (2)
        assert_eq!(calculate_hour_stem(1, 0), 2);
        assert_eq!(calculate_hour_stem(6, 0), 2);

        // Byeong (2) or Shin (7) day -> Zi hour is Mu-Zi (4)
        assert_eq!(calculate_hour_stem(2, 0), 4);
        assert_eq!(calculate_hour_stem(7, 0), 4);

        // Jeong (3) or Im (8) day -> Zi hour is Gyeong-Zi (6)
        assert_eq!(calculate_hour_stem(3, 0), 6);
        assert_eq!(calculate_hour_stem(8, 0), 6);

        // Mu (4) or Gye (9) day -> Zi hour is Im-Zi (8)
        assert_eq!(calculate_hour_stem(4, 0), 8);
        assert_eq!(calculate_hour_stem(9, 0), 8);
    }

    #[test]
    fn test_daeun_modular_progression() {
        // Month: Gap-In (stem 0, branch 2)
        // Forward: step 1 -> Eul-Myo (1, 3), step 10 -> Gye-Hae (9, 11)
        let f_1 = calculate_daeun_pillar(0, 2, 1, true);
        assert_eq!(f_1.stem_index, 1);
        assert_eq!(f_1.branch_index, 3);

        let f_10 = calculate_daeun_pillar(0, 2, 10, true);
        assert_eq!(f_10.stem_index, 0); // 0 + 10 = 10 % 10 = 0 (Gap)
        assert_eq!(f_10.branch_index, 0); // 2 + 10 = 12 % 12 = 0 (Zi)

        // Reverse: step 1 -> Gye-Chuk (9, 1)
        let r_1 = calculate_daeun_pillar(0, 2, 1, false);
        assert_eq!(r_1.stem_index, 9); // (0 - 1).rem_euclid(10) = 9 (Gye)
        assert_eq!(r_1.branch_index, 1); // (2 - 1).rem_euclid(12) = 1 (Chuk)
    }

    #[test]
    fn test_saju_zi_hour_mode() {
        // Ephemeris at 2000-01-01 15:30 UTC -> Seoul local solar time 23:58 (during 23:00~24:00 Zi hour)
        let ephem = compute_ephemeris(2000, 1, 1, 15, 30, 0.0, 37.5665, 126.9780)
            .expect("ephem should succeed");

        // 1. Classical NextDayAt23: Day advances to next day (기미 己未)
        let saju_next = calculate_saju_with_options(
            &ephem, 2000, 126.9780, Gender::Male,
            SajuOptions {
                zi_hour_mode: ZiHourMode::NextDayAt23,
                apply_equation_of_time: false,
            },
        ).expect("saju_next should succeed");

        // 2. MidnightAt00 (야자시): Day retains today's day (무오 戊午)
        let saju_midnight = calculate_saju_with_options(
            &ephem, 2000, 126.9780, Gender::Male,
            SajuOptions {
                zi_hour_mode: ZiHourMode::MidnightAt00,
                apply_equation_of_time: false,
            },
        ).expect("saju_midnight should succeed");

        assert_eq!(saju_next.hour_pillar.branch_index, 0, "Both should be Zi hour");
        assert_eq!(saju_midnight.hour_pillar.branch_index, 0, "Both should be Zi hour");

        // NextDayAt23 advances day pillar from 戊午 (4, 6) to 己未 (5, 7)
        assert_eq!(saju_next.day_pillar.korean(), "기미");
        // MidnightAt00 keeps today's day pillar 戊午 (4, 6)
        assert_eq!(saju_midnight.day_pillar.korean(), "무오");
    }

    #[test]
    fn test_saju_equation_of_time() {
        // Ephemeris at Autumn equinox or November where Equation of Time is significant (~14-16 minutes)
        let ephem = compute_ephemeris(2024, 11, 3, 3, 0, 0.0, 37.5665, 126.9780)
            .expect("ephem should succeed");

        let saju_mean = calculate_saju_with_options(
            &ephem, 2024, 126.9780, Gender::Male,
            SajuOptions {
                zi_hour_mode: ZiHourMode::NextDayAt23,
                apply_equation_of_time: false,
            },
        ).expect("saju_mean should succeed");

        let saju_true = calculate_saju_with_options(
            &ephem, 2024, 126.9780, Gender::Male,
            SajuOptions {
                zi_hour_mode: ZiHourMode::NextDayAt23,
                apply_equation_of_time: true,
            },
        ).expect("saju_true should succeed");

        assert_eq!(saju_mean.year_pillar, saju_true.year_pillar);
        assert_eq!(saju_mean.month_pillar, saju_true.month_pillar);
    }
}

