use serde::{Deserialize, Serialize};
use vedaksha::ephem::julian::calendar_to_jd;

use crate::celestial::{completed_age, CalendarDate};
use crate::western::dignity::DOMICILE_RULERS;
use super::{normalize_western_body_name, ZODIAC_SIGNS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualProfectionResult {
    pub target_age: u32,
    pub target_year: i32,
    pub profected_house: u8,
    pub profected_sign_index: u8,
    pub profected_sign_name: String,
    pub lord_of_the_year: String,
    pub active_natal_planets: Vec<String>,
    #[serde(default)]
    pub monthly_profected_house: u8,
    #[serde(default)]
    pub monthly_profected_sign_index: u8,
    #[serde(default)]
    pub monthly_profected_sign_name: String,
    #[serde(default)]
    pub lord_of_the_month: String,
    #[serde(default)]
    pub daily_profected_house: u8,
    #[serde(default)]
    pub daily_profected_sign_index: u8,
    #[serde(default)]
    pub daily_profected_sign_name: String,
    #[serde(default)]
    pub lord_of_the_day: String,
}

/// Zodiacal Releasing (ZR) period entry (Vettius Valens, Anthologies Book IV).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZrPeriod {
    pub level: u8,
    pub sign_index: u8,
    pub sign_name: String,
    pub ruler: String,
    pub start_day_offset: f64,
    pub end_day_offset: f64,
    pub duration_days: f64,
    pub is_loosing_of_the_bond: bool,
    #[serde(default)]
    pub sub_periods: Vec<ZrPeriod>,
}

/// Complete Zodiacal Releasing timeline from a starting Hellenistic Lot (Fortune or Spirit).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZodiacalReleasingResult {
    pub basis_lot_name: String,
    pub start_sign_index: u8,
    pub start_sign_name: String,
    pub periods: Vec<ZrPeriod>,
}

/// Hellenistic minor period years assigned to each zodiac sign in Zodiacal Releasing (Valens Book IV).
pub fn zr_sign_minor_years(sign_index: u8) -> u32 {
    match sign_index % 12 {
        0 | 7 => 15,
        1 | 6 => 8,
        2 | 5 => 20,
        3 => 25,
        4 => 19,
        8 | 11 => 12,
        _ => 30,
    }
}

fn build_zr_l2_periods(l1_sign: u8, l1_start_day: f64, l1_end_day: f64) -> Vec<ZrPeriod> {
    let mut sub_periods = Vec::new();
    let mut cur_day = l1_start_day;
    let mut l2_idx = 0usize;
    let mut l2_base_sign = l1_sign;
    let mut lob_triggered = false;

    while cur_day < l1_end_day {
        let mut is_lob = false;
        if l2_idx == 12 && !lob_triggered {
            l2_base_sign = (l1_sign + 6) % 12;
            l2_idx = 0;
            lob_triggered = true;
            is_lob = true;
        }
        let l2_sign = (l2_base_sign + l2_idx as u8) % 12;
        let l2_days = (zr_sign_minor_years(l2_sign) as f64) * 30.0;
        let l2_end = (cur_day + l2_days).min(l1_end_day);

        sub_periods.push(ZrPeriod {
            level: 2,
            sign_index: l2_sign,
            sign_name: ZODIAC_SIGNS[l2_sign as usize].to_string(),
            ruler: DOMICILE_RULERS[l2_sign as usize].name().to_string(),
            start_day_offset: cur_day,
            end_day_offset: l2_end,
            duration_days: l2_end - cur_day,
            is_loosing_of_the_bond: is_lob,
            sub_periods: Vec::new(),
        });

        cur_day = l2_end;
        l2_idx += 1;
    }
    sub_periods
}

/// Computes Vettius Valens Zodiacal Releasing (L1 & L2) with 360-day Hellenistic years and LOB jump.
pub fn calculate_zodiacal_releasing(
    basis_lot_name: &str,
    start_sign_index: u8,
    max_years: u32,
) -> ZodiacalReleasingResult {
    let start_sign = start_sign_index % 12;
    let max_days = (max_years as f64) * 360.0;
    let mut l1_periods = Vec::new();
    let mut cur_l1_day = 0.0;
    let mut l1_step = 0u8;

    while cur_l1_day < max_days {
        let l1_sign = (start_sign + l1_step) % 12;
        let l1_end_day = (cur_l1_day + (zr_sign_minor_years(l1_sign) as f64) * 360.0).min(max_days);
        l1_periods.push(ZrPeriod {
            level: 1,
            sign_index: l1_sign,
            sign_name: ZODIAC_SIGNS[l1_sign as usize].to_string(),
            ruler: DOMICILE_RULERS[l1_sign as usize].name().to_string(),
            start_day_offset: cur_l1_day,
            end_day_offset: l1_end_day,
            duration_days: l1_end_day - cur_l1_day,
            is_loosing_of_the_bond: false,
            sub_periods: build_zr_l2_periods(l1_sign, cur_l1_day, l1_end_day),
        });
        cur_l1_day = l1_end_day;
        l1_step = l1_step.wrapping_add(1);
    }

    ZodiacalReleasingResult {
        basis_lot_name: basis_lot_name.to_string(),
        start_sign_index: start_sign,
        start_sign_name: ZODIAC_SIGNS[start_sign as usize].to_string(),
        periods: l1_periods,
    }
}

/// Compute Hellenistic Annual, Monthly, and Daily Profections (Vettius Valens).
pub fn calculate_annual_profection(
    birth_date: CalendarDate,
    target_date: CalendarDate,
    natal_ascendant: f64,
    natal_planets: &[(&str, f64)],
) -> AnnualProfectionResult {
    let (age, last_bday_year) = completed_age(birth_date, target_date);

    let shift_signs = (age % 12) as u8;
    let asc_sign = (natal_ascendant.rem_euclid(360.0) / 30.0).floor() as u8 % 12;
    let profected_sign_index = (asc_sign + shift_signs) % 12;

    let jd_last_bday = calendar_to_jd(last_bday_year, birth_date.month, birth_date.day as f64);
    let jd_target = target_date.to_julian_day();
    let days_since_birthday = (jd_target - jd_last_bday).max(0.0);

    let month_shift = ((days_since_birthday / 30.0).floor() as u8).min(11);
    let monthly_sign = (profected_sign_index + month_shift) % 12;
    let days_in_month = days_since_birthday - (month_shift as f64 * 30.0);
    let daily_sign = (monthly_sign + (((days_in_month / 2.5).floor() as u8) % 12)) % 12;

    let active_natal_planets = natal_planets
        .iter()
        .filter(|&&(_, lon)| (lon.rem_euclid(360.0) / 30.0).floor() as u8 % 12 == profected_sign_index)
        .map(|&(name, _)| normalize_western_body_name(name))
        .collect();

    AnnualProfectionResult {
        target_age: age,
        target_year: target_date.year,
        profected_house: shift_signs + 1,
        profected_sign_index,
        profected_sign_name: ZODIAC_SIGNS[profected_sign_index as usize].to_string(),
        lord_of_the_year: DOMICILE_RULERS[profected_sign_index as usize].name().to_string(),
        active_natal_planets,
        monthly_profected_house: ((monthly_sign + 12 - asc_sign) % 12) + 1,
        monthly_profected_sign_index: monthly_sign,
        monthly_profected_sign_name: ZODIAC_SIGNS[monthly_sign as usize].to_string(),
        lord_of_the_month: DOMICILE_RULERS[monthly_sign as usize].name().to_string(),
        daily_profected_house: ((daily_sign + 12 - asc_sign) % 12) + 1,
        daily_profected_sign_index: daily_sign,
        daily_profected_sign_name: ZODIAC_SIGNS[daily_sign as usize].to_string(),
        lord_of_the_day: DOMICILE_RULERS[daily_sign as usize].name().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zodiacal_releasing_and_loosing_of_the_bond() {
        let zr = calculate_zodiacal_releasing("Part of Spirit", 3, 30);
        let first_l1 = &zr.periods[0];
        assert_eq!(first_l1.sign_index, 3);
        assert_eq!(first_l1.duration_days, 25.0 * 360.0);
        assert!(first_l1.sub_periods.len() > 12);
        let lob_sub = &first_l1.sub_periods[12];
        assert!(lob_sub.is_loosing_of_the_bond);
        assert_eq!(lob_sub.sign_index, 9);
    }
}
