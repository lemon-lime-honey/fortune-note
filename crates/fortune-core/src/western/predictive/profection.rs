use serde::{Deserialize, Serialize};
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
    let completed_age = if (target_month, target_day) >= (birth_month, birth_day) {
        (target_year - birth_year).max(0) as u32
    } else {
        (target_year - birth_year - 1).max(0) as u32
    };
    let shift_signs = (completed_age % 12) as u8;

    let asc_sign = (natal_ascendant / 30.0).floor() as u8 % 12;
    let profected_sign_index = (asc_sign + shift_signs) % 12;
    let profected_house = shift_signs + 1;
    let profected_sign_name = ZODIAC_SIGNS[profected_sign_index as usize].to_string();

    let lord_of_the_year = DOMICILE_RULERS[profected_sign_index as usize].name().to_string();

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
    }
}
