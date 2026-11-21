//! Vedic auxiliary analyses: Vimshottari Dasha, Chara Karakas, Graha Drishti,
//! Arudha Padas, Ashtakavarga, and Suryodaya Panchanga.

use std::collections::HashMap;
use vedaksha::math::angle::normalize_degrees;
use vedaksha::vedic::arudha::{arudha_pada, ArudhaException};
use vedaksha::vedic::ashtakavarga::{
    bhinna_ashtakavarga, sarvashtakavarga, BhinnaAshtakavargaInput,
};
use vedaksha::vedic::dasha::vimshottari::compute_vimshottari;
use vedaksha::vedic::graha::Graha;
use vedaksha::vedic::nakshatra::Nakshatra;
use vedaksha::vedic::panchanga::{compute_karana, compute_panchanga_yoga};

use crate::celestial::compute_solar_horizon_state;
use crate::ephemeris::EphemerisSnapshot;
use super::dignity::arudha_sign_lord;
use super::{
    BhinnaAshtakavargaRow, DashaPeriodInfo, DashaYearType, VedicArudhaPada, VedicAshtakavarga,
    VedicDrishtiInfo, VedicKarakaInfo, VedicPanchanga, VedicPlanet,
};

const RASHI_NAMES: [&str; 12] = [
    "Mesha (Aries)", "Vrishabha (Taurus)", "Mithuna (Gemini)", "Karka (Cancer)",
    "Simha (Leo)", "Kanya (Virgo)", "Tula (Libra)", "Vrishchika (Scorpio)",
    "Dhanu (Sagittarius)", "Makara (Capricorn)", "Kumbha (Aquarius)", "Meena (Pisces)",
];

const TITHI_NAMES: [&str; 15] = [
    "Pratipada", "Dwitiya", "Tritiya", "Chaturthi", "Panchami",
    "Shashthi", "Saptami", "Ashtami", "Navami", "Dashami",
    "Ekadashi", "Dwadashi", "Trayodashi", "Chaturdashi", "Purnima / Amavasya",
];

const SEVEN_KARAKA_LABELS: [&str; 7] = [
    "Atmakaraka (AK)", "Amatyakaraka (AmK)", "Bhratrikaraka (BK)", "Matrikaraka (MK)",
    "Putrakaraka (PK)", "Gnatikaraka (GK)", "Darakaraka (DK)",
];

const ARUDHA_META: [(&str, &str); 12] = [
    ("AL", "Arudha Lagna"), ("A2", "Dhana Pada"), ("A3", "Vikrama Pada"),
    ("A4", "Matri Pada"), ("A5", "Mantra Pada"), ("A6", "Roga Pada"),
    ("A7", "Dara Pada"), ("A8", "Mrityu Pada"), ("A9", "Bhagya Pada"),
    ("A10", "Rajya Pada"), ("A11", "Labha Pada"), ("UL", "Upapada Lagna"),
];

const VARA_META: [(&str, &str); 7] = [
    ("Somavara (Monday)", "Moon"),
    ("Mangalavara (Tuesday)", "Mars"),
    ("Budhavara (Wednesday)", "Mercury"),
    ("Guruvara (Thursday)", "Jupiter"),
    ("Shukravara (Friday)", "Venus"),
    ("Shanivara (Saturday)", "Saturn"),
    ("Ravivara (Sunday)", "Sun"),
];

fn scale_dasha_period(p: &DashaPeriodInfo, birth_jd: f64, scale: f64) -> DashaPeriodInfo {
    DashaPeriodInfo {
        lord: p.lord.clone(),
        start_jd: birth_jd + (p.start_jd - birth_jd) * scale,
        end_jd: birth_jd + (p.end_jd - birth_jd) * scale,
        sub_periods: p.sub_periods.iter().map(|s| scale_dasha_period(s, birth_jd, scale)).collect(),
    }
}

pub(crate) fn compute_vimshottari_dashas(
    moon_sidereal_lon: f64,
    jd_ut1: f64,
    year_type: DashaYearType,
) -> Vec<DashaPeriodInfo> {
    let dasha_tree = compute_vimshottari(moon_sidereal_lon, jd_ut1, 2);
    let base: Vec<DashaPeriodInfo> = dasha_tree.maha_dashas.iter().map(DashaPeriodInfo::from).collect();
    match year_type {
        DashaYearType::Solar365_2425 => base,
        DashaYearType::Savana360 => {
            let scale = 360.0 / 365.2425;
            base.iter().map(|d| scale_dasha_period(d, jd_ut1, scale)).collect()
        }
    }
}

pub(crate) fn compute_chara_karakas(planets: &[VedicPlanet]) -> Vec<VedicKarakaInfo> {
    let mut candidates: Vec<(&str, f64)> = planets
        .iter()
        .filter(|p| p.name != "Rahu" && p.name != "Ketu")
        .map(|p| (p.name.as_str(), p.sign_degree))
        .collect();
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    candidates
        .into_iter()
        .zip(SEVEN_KARAKA_LABELS.iter())
        .map(|((p_name, deg), &label)| VedicKarakaInfo {
            karaka: label.to_string(),
            planet: p_name.to_string(),
            effective_degree: deg,
        })
        .collect()
}

pub(crate) fn compute_graha_drishti(planets: &[VedicPlanet], lagna_sign: u8) -> Vec<VedicDrishtiInfo> {
    let mut drishti = Vec::new();
    for p in planets {
        if p.name == "Ketu" {
            continue;
        }
        let offsets: &[u8] = match p.name.as_str() {
            "Mars" => &[4, 7, 8],
            "Jupiter" | "Rahu" => &[5, 7, 9],
            "Saturn" => &[3, 7, 10],
            _ => &[7],
        };
        for &offset in offsets {
            let target_sign = (p.sign_index + offset - 1) % 12;
            let target_house = ((target_sign as i16 - lagna_sign as i16 + 12) % 12 + 1) as u8;
            let aspected_planets = planets
                .iter()
                .filter(|other| other.name != p.name && other.sign_index == target_sign)
                .map(|other| other.name.clone())
                .collect();
            drishti.push(VedicDrishtiInfo {
                from_planet: p.name.clone(),
                from_sign: p.sign_index,
                from_house: p.house,
                aspect_offset: offset,
                target_sign,
                target_house,
                aspected_planets,
            });
        }
    }
    drishti
}

pub(crate) fn compute_arudha_padas(
    lagna_sign: u8,
    planet_signs: &HashMap<Graha, u8>,
    planet_lons: &HashMap<Graha, f64>,
) -> Vec<VedicArudhaPada> {
    (1..=12u8)
        .map(|bhava| {
            let bhava_rashi = (lagna_sign + bhava - 1) % 12;
            let lord = arudha_sign_lord(bhava_rashi, planet_signs, planet_lons);
            let lord_rashi = planet_signs.get(&lord).copied().unwrap_or(bhava_rashi);
            let pada_sign = arudha_pada(bhava_rashi, lord_rashi, ArudhaException::TenthFromLanded);
            let house_from_lagna = ((pada_sign as i16 - lagna_sign as i16 + 12) % 12 + 1) as u8;
            let (code, name) = ARUDHA_META[(bhava - 1) as usize];
            VedicArudhaPada {
                bhava,
                code: code.to_string(),
                name: name.to_string(),
                sign_index: pada_sign,
                sign_name: RASHI_NAMES[pada_sign as usize].to_string(),
                house_from_lagna,
            }
        })
        .collect()
}

pub(crate) fn compute_ashtakavarga(
    planet_signs: &HashMap<Graha, u8>,
    lagna_sign: u8,
) -> VedicAshtakavarga {
    let av_input = BhinnaAshtakavargaInput {
        sun: *planet_signs.get(&Graha::Sun).unwrap_or(&0),
        moon: *planet_signs.get(&Graha::Moon).unwrap_or(&0),
        mars: *planet_signs.get(&Graha::Mars).unwrap_or(&0),
        mercury: *planet_signs.get(&Graha::Mercury).unwrap_or(&0),
        jupiter: *planet_signs.get(&Graha::Jupiter).unwrap_or(&0),
        venus: *planet_signs.get(&Graha::Venus).unwrap_or(&0),
        saturn: *planet_signs.get(&Graha::Saturn).unwrap_or(&0),
        lagna: lagna_sign,
    };
    let bhinna_tables = bhinna_ashtakavarga(&av_input);
    let sarva = sarvashtakavarga(&bhinna_tables);
    let total_bindus: u16 = sarva.iter().map(|&b| b as u16).sum();
    let bhinna = bhinna_tables
        .iter()
        .map(|t| BhinnaAshtakavargaRow {
            planet: format!("{:?}", t.planet),
            bindus: t.bindus,
            total: t.total as u16,
        })
        .collect();
    VedicAshtakavarga { bhinna, sarva, total_bindus }
}

pub(crate) fn compute_panchanga(
    ephem: &EphemerisSnapshot,
    latitude: f64,
    sun_tropical_lon: f64,
    sun_sidereal_lon: f64,
    moon_sidereal_lon: f64,
) -> VedicPanchanga {
    let elongation = normalize_degrees(moon_sidereal_lon - sun_sidereal_lon);
    let tithi_number = ((elongation / 12.0).floor() as u8).min(29) + 1;
    let paksha = if tithi_number <= 15 { "Shukla Paksha" } else { "Krishna Paksha" }.to_string();
    let tithi_name = match tithi_number {
        15 => "Purnima",
        30 => "Amavasya",
        n => TITHI_NAMES[((n - 1) % 15) as usize],
    }
    .to_string();

    // Shared Suryodaya (sunrise) horizon calculation from `crate::celestial`.
    let horizon = compute_solar_horizon_state(
        sun_tropical_lon,
        ephem.true_obliquity_deg,
        latitude,
        ephem.ramc,
        ephem.jd_ut1,
    );
    let day_shift = if !horizon.is_polar_day && !horizon.is_polar_night
        && horizon.hour_angle_deg < -horizon.semi_diurnal_deg
    {
        -1i64
    } else {
        0i64
    };
    let base_weekday = ((horizon.local_noon_jd + 0.5).floor() as i64).rem_euclid(7);
    let weekday_idx = (base_weekday + day_shift).rem_euclid(7) as usize;
    let (vara_name, vara_lord) = VARA_META[weekday_idx];

    let moon_nak = Nakshatra::from_longitude(moon_sidereal_lon);
    let moon_pada = Nakshatra::pada_from_longitude(moon_sidereal_lon);
    let yoga = compute_panchanga_yoga(sun_sidereal_lon, moon_sidereal_lon);
    let karana = compute_karana(moon_sidereal_lon, sun_sidereal_lon);

    VedicPanchanga {
        tithi_name,
        tithi_number,
        paksha,
        vara_name: vara_name.to_string(),
        vara_lord: vara_lord.to_string(),
        nakshatra_name: moon_nak.name().to_string(),
        nakshatra_pada: moon_pada,
        nakshatra_lord: format!("{:?}", moon_nak.dasha_lord()),
        yoga_name: yoga.name.to_string(),
        karana_name: karana.name.to_string(),
    }
}
