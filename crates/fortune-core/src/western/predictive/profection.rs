use serde::{Deserialize, Serialize};
use vedaksha::ephem::julian::calendar_to_jd;
use crate::western::dignity::DOMICILE_RULERS;
use super::{normalize_western_body_name, ZODIAC_SIGNS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualProfectionResult {
    pub target_age: u32,
    pub target_year: i32,
    pub profected_house: u8, // 1 .. 12
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
    pub level: u8, // 1 = L1 (years), 2 = L2 (months / 30-day sub-periods)
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

/// Hellenistic minor period years assigned to each zodiac sign in Zodiacal Releasing (Valens Book IV):
/// - Leo (Sun): 19
/// - Cancer (Moon): 25
/// - Gemini / Virgo (Mercury): 20
/// - Taurus / Libra (Venus): 8
/// - Aries / Scorpio (Mars): 15
/// - Sagittarius / Pisces (Jupiter): 12
/// - Capricorn / Aquarius (Saturn): 30
pub fn zr_sign_minor_years(sign_index: u8) -> u32 {
    match sign_index % 12 {
        0 | 7 => 15,  // Aries, Scorpio (Mars)
        1 | 6 => 8,   // Taurus, Libra (Venus)
        2 | 5 => 20,  // Gemini, Virgo (Mercury)
        3 => 25,      // Cancer (Moon)
        4 => 19,      // Leo (Sun)
        8 | 11 => 12, // Sagittarius, Pisces (Jupiter)
        _ => 30,      // Capricorn, Aquarius (Saturn)
    }
}

/// Computes Vettius Valens Zodiacal Releasing (L1 & L2) using the 360-day Hellenistic year
/// and the Loosing of the Bond (LOB) jump rule when L2 completes a full 12-sign cycle and reaches
/// the 7th sign (opposition) from the L1 sign.
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
        let l1_years = zr_sign_minor_years(l1_sign);
        let l1_duration_days = (l1_years as f64) * 360.0;
        let l1_end_day = (cur_l1_day + l1_duration_days).min(max_days);

        // Build L2 sub-periods (each L2 sign lasts `zr_sign_minor_years(l2_sign) * 30.0` days).
        // Loosing of the Bond (LOB) occurs after the first 12 L2 signs complete (i.e. at the 13th L2 step),
        // where instead of repeating the L1 sign a second time, the sequence jumps to the opposite (7th) sign!
        let mut sub_periods = Vec::new();
        let mut cur_l2_day = cur_l1_day;
        let mut l2_idx = 0usize;
        let mut l2_base_sign = l1_sign;
        let mut lob_triggered_in_l1 = false;

        while cur_l2_day < l1_end_day {
            let mut is_lob = false;
            if l2_idx == 12 && !lob_triggered_in_l1 {
                // Jump to the opposite sign (Loosing of the Bond)
                l2_base_sign = (l1_sign + 6) % 12;
                l2_idx = 0;
                lob_triggered_in_l1 = true;
                is_lob = true;
            }
            let l2_sign = (l2_base_sign + (l2_idx as u8)) % 12;
            let l2_days = (zr_sign_minor_years(l2_sign) as f64) * 30.0;
            let l2_end = (cur_l2_day + l2_days).min(l1_end_day);

            sub_periods.push(ZrPeriod {
                level: 2,
                sign_index: l2_sign,
                sign_name: ZODIAC_SIGNS[l2_sign as usize].to_string(),
                ruler: DOMICILE_RULERS[l2_sign as usize].name().to_string(),
                start_day_offset: cur_l2_day,
                end_day_offset: l2_end,
                duration_days: l2_end - cur_l2_day,
                is_loosing_of_the_bond: is_lob,
                sub_periods: Vec::new(),
            });

            cur_l2_day = l2_end;
            l2_idx += 1;
        }

        l1_periods.push(ZrPeriod {
            level: 1,
            sign_index: l1_sign,
            sign_name: ZODIAC_SIGNS[l1_sign as usize].to_string(),
            ruler: DOMICILE_RULERS[l1_sign as usize].name().to_string(),
            start_day_offset: cur_l1_day,
            end_day_offset: l1_end_day,
            duration_days: l1_end_day - cur_l1_day,
            is_loosing_of_the_bond: false,
            sub_periods,
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

pub fn calculate_annual_profection(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    target_year: i32,
    target_month: u32,
    target_day: u32,
    natal_ascendant: f64,
    natal_planets: &[(&str, f64)],
) -> AnnualProfectionResult {
    let (completed_age, last_bday_year) = if (target_month, target_day) >= (birth_month, birth_day) {
        ((target_year - birth_year).max(0) as u32, target_year)
    } else {
        ((target_year - birth_year - 1).max(0) as u32, target_year - 1)
    };
    let shift_signs = (completed_age % 12) as u8;

    let asc_sign = (natal_ascendant / 30.0).floor() as u8 % 12;
    let profected_sign_index = (asc_sign + shift_signs) % 12;
    let profected_house = shift_signs + 1;
    let profected_sign_name = ZODIAC_SIGNS[profected_sign_index as usize].to_string();
    let lord_of_the_year = DOMICILE_RULERS[profected_sign_index as usize].name().to_string();

    // Days elapsed since the last birthday solar anniversary for Monthly (30.4375d / 1 month per sign)
    // and Daily (2.5 days per sign) Hellenistic Profections (Valens):
    let jd_last_bday = calendar_to_jd(last_bday_year, birth_month, birth_day as f64);
    let jd_target = calendar_to_jd(target_year, target_month, target_day as f64);
    let days_since_birthday = (jd_target - jd_last_bday).max(0.0);

    // Monthly profection: starts at the Annual Profection sign on the birthday and advances 1 sign per month (30 days)
    let month_shift = ((days_since_birthday / 30.0).floor() as u8).min(11);
    let monthly_profected_sign_index = (profected_sign_index + month_shift) % 12;
    let monthly_profected_house = ((monthly_profected_sign_index + 12 - asc_sign) % 12) + 1;
    let monthly_profected_sign_name = ZODIAC_SIGNS[monthly_profected_sign_index as usize].to_string();
    let lord_of_the_month = DOMICILE_RULERS[monthly_profected_sign_index as usize].name().to_string();

    // Daily profection: starts at the Monthly Profection sign and advances 1 sign every 2.5 days (2d 12h)
    let days_in_profection_month = days_since_birthday - (month_shift as f64 * 30.0);
    let day_shift = ((days_in_profection_month / 2.5).floor() as u8) % 12;
    let daily_profected_sign_index = (monthly_profected_sign_index + day_shift) % 12;
    let daily_profected_house = ((daily_profected_sign_index + 12 - asc_sign) % 12) + 1;
    let daily_profected_sign_name = ZODIAC_SIGNS[daily_profected_sign_index as usize].to_string();
    let lord_of_the_day = DOMICILE_RULERS[daily_profected_sign_index as usize].name().to_string();

    let mut active_natal_planets = Vec::new();
    for &(name, lon) in natal_planets {
        let p_sign = (lon / 30.0).floor() as u8 % 12;
        if p_sign == profected_sign_index {
            active_natal_planets.push(normalize_western_body_name(name));
        }
    }

    AnnualProfectionResult {
        target_age: completed_age,
        target_year,
        profected_house,
        profected_sign_index,
        profected_sign_name,
        lord_of_the_year,
        active_natal_planets,
        monthly_profected_house,
        monthly_profected_sign_index,
        monthly_profected_sign_name,
        lord_of_the_month,
        daily_profected_house,
        daily_profected_sign_index,
        daily_profected_sign_name,
        lord_of_the_day,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zodiacal_releasing_and_loosing_of_the_bond() {
        // Cancer (sign_index 3) has L1 duration of 25 years (9000 days).
        // The sum of all 12 signs' L2 periods is 129 months = 3870 days (10.75 years).
        // Since 9000 > 3870, at the 13th L2 sub-period (day 3870.0), Loosing of the Bond (LOB)
        // must trigger and jump to Capricorn (opposite Cancer, sign_index 9)!
        let zr = calculate_zodiacal_releasing("Part of Spirit", 3, 30);
        assert!(!zr.periods.is_empty());
        let first_l1 = &zr.periods[0];
        assert_eq!(first_l1.sign_index, 3); // Cancer
        assert_eq!(first_l1.duration_days, 25.0 * 360.0);
        assert!(first_l1.sub_periods.len() > 12);

        let lob_sub = &first_l1.sub_periods[12];
        assert!(lob_sub.is_loosing_of_the_bond, "13th L2 sub-period must trigger Loosing of the Bond");
        assert_eq!(lob_sub.sign_index, 9, "LOB from Cancer (3) must jump to Capricorn (9)");
    }
}
