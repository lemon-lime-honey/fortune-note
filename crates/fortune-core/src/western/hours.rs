//! Planetary Hours, Day Rulers, and William Lilly's Considerations Before Judgment.
//!
//! Based on Christian Astrology (1647), Chapter XXI (p. 121-123).

use serde::{Deserialize, Serialize};

use crate::celestial::{compute_solar_horizon_state, SolarHorizonState};
use super::dignity::{ClassicalPlanet, DOMICILE_RULERS};

/// Planetary day and hour chronocrators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetaryHoursReport {
    pub day_ruler: String,
    pub hour_ruler: String,
    pub is_day_hour: bool,
    pub hour_number: u8,
    pub is_radical: bool,
    pub radicality_reason: String,
    pub considerations: Vec<String>,
}

/// Bundled astronomical inputs for planetary hour & horary radicality evaluation.
#[derive(Debug, Clone, Copy)]
pub struct AstroPlanetaryHoursInput {
    pub jd_ut1: f64,
    pub latitude: f64,
    pub ramc: f64,
    pub sun_longitude: f64,
    pub obliquity_deg: f64,
    pub asc_longitude: f64,
    pub saturn_house: u8,
    pub is_moon_voc: bool,
    pub is_moon_via_combusta: bool,
    pub is_horary: bool,
}

const CHALDEAN_ORDER: [ClassicalPlanet; 7] = [
    ClassicalPlanet::Saturn,
    ClassicalPlanet::Jupiter,
    ClassicalPlanet::Mars,
    ClassicalPlanet::Sun,
    ClassicalPlanet::Venus,
    ClassicalPlanet::Mercury,
    ClassicalPlanet::Moon,
];

const DAY_RULERS: [ClassicalPlanet; 7] = [
    ClassicalPlanet::Sun,
    ClassicalPlanet::Moon,
    ClassicalPlanet::Mars,
    ClassicalPlanet::Mercury,
    ClassicalPlanet::Jupiter,
    ClassicalPlanet::Venus,
    ClassicalPlanet::Saturn,
];

fn day_night_fraction(state: SolarHorizonState) -> (bool, f64, i64) {
    let h = state.hour_angle_deg;
    let semi = state.semi_diurnal_deg;

    if state.is_polar_day {
        (true, ((h + 180.0) / 360.0).clamp(0.0, 0.999_999), 0)
    } else if state.is_polar_night {
        (false, ((h + 180.0) / 360.0).clamp(0.0, 0.999_999), 0)
    } else if h >= -semi && h <= semi {
        let f = (h + semi) / (2.0 * semi);
        (true, f.clamp(0.0, 0.999_999), 0)
    } else {
        let night_span = (360.0 - 2.0 * semi).max(0.1);
        if h < -semi {
            let dist = (180.0 - semi) + (180.0 + h);
            (false, (dist / night_span).clamp(0.0, 0.999_999), -1)
        } else {
            let dist = h - semi;
            (false, (dist / night_span).clamp(0.0, 0.999_999), 0)
        }
    }
}

/// Compute day ruler and hour ruler using astronomical hour angles and true sunrise/sunset.
pub fn calculate_astronomical_planetary_hours_with_mode(
    input: &AstroPlanetaryHoursInput,
) -> PlanetaryHoursReport {
    let horizon = compute_solar_horizon_state(
        input.sun_longitude,
        input.obliquity_deg,
        input.latitude,
        input.ramc,
        input.jd_ut1,
    );
    let (is_daytime, fraction, day_shift) = day_night_fraction(horizon);

    // Local solar noon JD rolls over at noon; +1.5 maps Julian Day to 0 = Sunday .. 6 = Saturday.
    let base_weekday = ((horizon.local_noon_jd + 1.5).floor() as i64).rem_euclid(7);
    let astrological_weekday = (base_weekday + day_shift).rem_euclid(7) as u32;

    let mut report = calculate_planetary_hours(
        astrological_weekday,
        is_daytime,
        fraction,
        input.asc_longitude,
        input.saturn_house,
        input.is_moon_voc,
        input.is_moon_via_combusta,
    );

    if !input.is_horary {
        report.is_radical = false;
        report.radicality_reason.clear();
        report.considerations.clear();
    }

    report
}

fn evaluate_radicality(
    asc_ruler: ClassicalPlanet,
    hour_ruler: ClassicalPlanet,
    asc_sign: u8,
) -> (bool, String) {
    if asc_ruler == hour_ruler {
        (true, format!("Lord of ASC and Lord of Hour are identical ({})", asc_ruler.name()))
    } else if are_same_nature(asc_ruler, hour_ruler) {
        (true, format!(
            "Lord of ASC ({}) and Lord of Hour ({}) share the same elemental nature",
            asc_ruler.name(), hour_ruler.name()
        ))
    } else if are_same_triplicity(asc_ruler, hour_ruler, asc_sign) {
        (true, format!(
            "Lord of ASC ({}) and Lord of Hour ({}) are of the same Triplicity",
            asc_ruler.name(), hour_ruler.name()
        ))
    } else {
        (false, format!(
            "Lord of ASC ({}) and Lord of Hour ({}) do not match in nature or triplicity",
            asc_ruler.name(), hour_ruler.name()
        ))
    }
}

fn collect_considerations(
    asc_degree: f64,
    saturn_house: u8,
    is_moon_voc: bool,
    is_moon_via_combusta: bool,
) -> Vec<String> {
    let mut considerations = Vec::new();
    if asc_degree < 3.0 {
        considerations.push(format!("Early ASC ({:.1}°): Question is premature or matters not yet ripe for judgment", asc_degree));
    } else if asc_degree > 27.0 {
        considerations.push(format!("Late ASC ({:.1}°): Past remedy, or questioner has already settled or taken matters elsewhere", asc_degree));
    }
    if saturn_house == 1 {
        considerations.push("Saturn in 1st House: Querent likely dissatisfied, deceptive, or matter corrupted".into());
    } else if saturn_house == 7 {
        considerations.push("Saturn in 7th House: Danger of astrologer's judgment failing or matter brought to error".into());
    }
    if is_moon_voc {
        considerations.push("Moon is Void of Course: Little or nothing will come of the matter".into());
    }
    if is_moon_via_combusta {
        considerations.push("Moon in Via Combusta (15° Lib - 15° Sco): Querent in great fear, affliction, or unpredictable event".into());
    }
    considerations
}

/// Compute day ruler and hour ruler for a given local weekday and day/night fraction.
pub fn calculate_planetary_hours(
    day_of_week: u32,
    is_daytime: bool,
    fraction_of_day_or_night: f64,
    asc_longitude: f64,
    saturn_house: u8,
    is_moon_voc: bool,
    is_moon_via_combusta: bool,
) -> PlanetaryHoursReport {
    let day_ruler = DAY_RULERS[(day_of_week % 7) as usize];
    let day_start_idx = CHALDEAN_ORDER.iter().position(|&p| p == day_ruler).unwrap_or(0);
    let hour_in_half = (fraction_of_day_or_night * 12.0).floor().clamp(0.0, 11.0) as usize;
    let hour_idx = if is_daytime {
        (day_start_idx + hour_in_half) % 7
    } else {
        (day_start_idx + 12 + hour_in_half) % 7
    };
    let hour_ruler = CHALDEAN_ORDER[hour_idx];

    let asc_sign = (asc_longitude / 30.0).floor() as u8 % 12;
    let asc_degree = asc_longitude - (asc_sign as f64 * 30.0);
    let asc_ruler = DOMICILE_RULERS[asc_sign as usize];

    let (is_radical, radicality_reason) = evaluate_radicality(asc_ruler, hour_ruler, asc_sign);
    let considerations = collect_considerations(asc_degree, saturn_house, is_moon_voc, is_moon_via_combusta);

    PlanetaryHoursReport {
        day_ruler: day_ruler.name().to_string(),
        hour_ruler: hour_ruler.name().to_string(),
        is_day_hour: is_daytime,
        hour_number: (hour_in_half + 1) as u8,
        is_radical,
        radicality_reason,
        considerations,
    }
}

fn are_same_nature(p1: ClassicalPlanet, p2: ClassicalPlanet) -> bool {
    let nature = |p: ClassicalPlanet| match p {
        ClassicalPlanet::Sun | ClassicalPlanet::Mars => 1,
        ClassicalPlanet::Jupiter | ClassicalPlanet::Venus => 2,
        ClassicalPlanet::Saturn | ClassicalPlanet::Mercury => 3,
        ClassicalPlanet::Moon => 4,
    };
    nature(p1) == nature(p2)
}

fn are_same_triplicity(p1: ClassicalPlanet, p2: ClassicalPlanet, sign: u8) -> bool {
    let triplicity = sign % 4;
    let in_triplicity = |p: ClassicalPlanet, trip: u8| match trip {
        0 => matches!(p, ClassicalPlanet::Sun | ClassicalPlanet::Jupiter | ClassicalPlanet::Saturn),
        1 => matches!(p, ClassicalPlanet::Venus | ClassicalPlanet::Moon | ClassicalPlanet::Mars),
        2 => matches!(p, ClassicalPlanet::Saturn | ClassicalPlanet::Mercury | ClassicalPlanet::Jupiter),
        _ => matches!(p, ClassicalPlanet::Mars | ClassicalPlanet::Moon | ClassicalPlanet::Venus),
    };
    in_triplicity(p1, triplicity) && in_triplicity(p2, triplicity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::celestial::{CalendarDateTime, GeoCoordinate};
    use crate::ephemeris::compute_ephemeris;

    fn run_seoul_hours(utc_hour: u32, utc_min: u32) -> PlanetaryHoursReport {
        let ephem = compute_ephemeris(
            CalendarDateTime::new(2024, 2, 3, utc_hour, utc_min, 0.0),
            GeoCoordinate::new(37.5665, 126.9780),
        )
        .expect("ephemeris should succeed");
        let sun_lon = ephem.get_body("Sun").unwrap().tropical_longitude;
        calculate_astronomical_planetary_hours_with_mode(&AstroPlanetaryHoursInput {
            jd_ut1: ephem.jd_ut1,
            latitude: 37.5665,
            ramc: ephem.ramc,
            sun_longitude: sun_lon,
            obliquity_deg: ephem.true_obliquity_deg,
            asc_longitude: 120.0,
            saturn_house: 2,
            is_moon_voc: false,
            is_moon_via_combusta: false,
            is_horary: true,
        })
    }

    #[test]
    fn test_seoul_morning_and_predawn_day_ruler() {
        // 2024-02-04 07:45 KST (after Sunday sunrise) -> Sun
        assert_eq!(run_seoul_hours(22, 45).day_ruler, "Sun");
        // 2024-02-04 04:00 KST (predawn before Sunday sunrise) -> Saturday (Saturn)
        assert_eq!(run_seoul_hours(19, 0).day_ruler, "Saturn");
    }
}
