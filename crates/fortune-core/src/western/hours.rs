//! Planetary Hours, Day Rulers, and William Lilly's Considerations Before Judgment.
//!
//! Based on Christian Astrology (1647), Chapter XXI (p. 121-123).

use serde::{Deserialize, Serialize};
use super::dignity::{ClassicalPlanet, DOMICILE_RULERS};

/// Planetary day and hour chronocrators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetaryHoursReport {
    pub day_ruler: String,
    pub hour_ruler: String,
    pub is_day_hour: bool,
    pub hour_number: u8, // 1 .. 12
    pub is_radical: bool,
    pub radicality_reason: String,
    pub considerations: Vec<String>,
}

/// Chaldean order of the planets (from highest sphere to lowest).
const CHALDEAN_ORDER: [ClassicalPlanet; 7] = [
    ClassicalPlanet::Saturn,
    ClassicalPlanet::Jupiter,
    ClassicalPlanet::Mars,
    ClassicalPlanet::Sun,
    ClassicalPlanet::Venus,
    ClassicalPlanet::Mercury,
    ClassicalPlanet::Moon,
];

/// Day rulers by day of week (0 = Sunday .. 6 = Saturday).
const DAY_RULERS: [ClassicalPlanet; 7] = [
    ClassicalPlanet::Sun,     // Sunday
    ClassicalPlanet::Moon,    // Monday
    ClassicalPlanet::Mars,    // Tuesday
    ClassicalPlanet::Mercury, // Wednesday
    ClassicalPlanet::Jupiter, // Thursday
    ClassicalPlanet::Venus,   // Friday
    ClassicalPlanet::Saturn,  // Saturday
];

/// Compute day ruler and hour ruler using astronomical hour angles and true sunrise/sunset.
pub fn calculate_astronomical_planetary_hours(
    jd_ut1: f64,
    latitude: f64,
    ramc: f64,
    sun_longitude: f64,
    obliquity_deg: f64,
    asc_longitude: f64,
    saturn_house: u8,
    is_moon_voc: bool,
    is_moon_via_combusta: bool,
) -> PlanetaryHoursReport {
    let sun_lon_rad = sun_longitude.to_radians();
    let eps_rad = obliquity_deg.to_radians();
    let lat_rad = latitude.to_radians();

    // Equatorial coordinates of Sun
    let sin_delta = eps_rad.sin() * sun_lon_rad.sin();
    let cos_delta = (1.0 - sin_delta * sin_delta).max(0.0).sqrt();
    let alpha_rad = (eps_rad.cos() * sun_lon_rad.sin()).atan2(sun_lon_rad.cos());
    let alpha_deg = alpha_rad.to_degrees().rem_euclid(360.0);

    // Standard solar altitude at sunrise/sunset: -50 arcminutes (-0.8333333°)
    let h0_rad = (-0.833_333_3_f64).to_radians();
    let cos_h0 = if lat_rad.cos() * cos_delta > 1e-7 {
        (h0_rad.sin() - lat_rad.sin() * sin_delta) / (lat_rad.cos() * cos_delta)
    } else {
        0.0
    };

    let (semi_diurnal_deg, is_polar_day, is_polar_night) = if cos_h0 < -1.0 {
        (180.0, true, false) // Midnight sun (all day)
    } else if cos_h0 > 1.0 {
        (0.0, false, true)  // Polar night (all night)
    } else {
        (cos_h0.clamp(-1.0, 1.0).acos().to_degrees(), false, false)
    };

    // Hour angle H = RAMC - Alpha, normalized to [-180°, +180°]
    let mut h = (ramc - alpha_deg).rem_euclid(360.0);
    if h > 180.0 {
        h -= 360.0;
    }

    let (is_daytime, fraction, day_shift) = if is_polar_day {
        let f = (h + 180.0) / 360.0;
        (true, f.clamp(0.0, 0.999999), 0i64)
    } else if is_polar_night {
        let f = (h + 180.0) / 360.0;
        (false, f.clamp(0.0, 0.999999), 0i64)
    } else if h >= -semi_diurnal_deg && h <= semi_diurnal_deg {
        // Daytime: from sunrise (-H0) to sunset (+H0)
        let f = (h + semi_diurnal_deg) / (2.0 * semi_diurnal_deg);
        (true, f.clamp(0.0, 0.999999), 0i64)
    } else {
        // Nighttime
        let night_span = (360.0 - 2.0 * semi_diurnal_deg).max(0.1);
        if h < -semi_diurnal_deg {
            // Before sunrise (between midnight and sunrise): day ruler belongs to previous calendar day
            let dist_from_sunset = (180.0 - semi_diurnal_deg) + (180.0 + h);
            let f = dist_from_sunset / night_span;
            (false, f.clamp(0.0, 0.999999), -1i64)
        } else {
            // After sunset (between sunset and midnight): day ruler belongs to today
            let dist_from_sunset = h - semi_diurnal_deg;
            let f = dist_from_sunset / night_span;
            (false, f.clamp(0.0, 0.999999), 0i64)
        }
    };

    // Calendar day of week for local observer:
    // Determine the local true solar noon JD by subtracting the hour angle from current UT1 JD.
    // Julian Day integer dates roll over at noon, so local noon JD accurately reflects the local calendar day.
    let local_noon_jd = jd_ut1 - (h / 360.0);
    let base_weekday = ((local_noon_jd + 1.5).floor() as i64).rem_euclid(7);
    let astrological_weekday = (base_weekday + day_shift).rem_euclid(7) as u32;

    calculate_planetary_hours(
        astrological_weekday,
        is_daytime,
        fraction,
        asc_longitude,
        saturn_house,
        is_moon_voc,
        is_moon_via_combusta,
    )
}

/// Compute day ruler and hour ruler for a given local time, weekday, and day/night fraction.
pub fn calculate_planetary_hours(
    day_of_week: u32, // 0 = Sunday .. 6 = Saturday (starting at sunrise)
    is_daytime: bool,
    fraction_of_day_or_night: f64, // 0.0 .. 1.0 (from sunrise or sunset)
    asc_longitude: f64,
    saturn_house: u8,
    is_moon_voc: bool,
    is_moon_via_combusta: bool,
) -> PlanetaryHoursReport {
    let day_ruler = DAY_RULERS[(day_of_week % 7) as usize];

    // Find starting Chaldean index for the day ruler
    let day_start_idx = CHALDEAN_ORDER.iter().position(|&p| p == day_ruler).unwrap_or(0);

    let hour_in_half = (fraction_of_day_or_night * 12.0).floor().clamp(0.0, 11.0) as usize;
    let hour_number = (hour_in_half + 1) as u8;

    let hour_idx = if is_daytime {
        (day_start_idx + hour_in_half) % 7
    } else {
        // Night starts 12 hours after day start
        (day_start_idx + 12 + hour_in_half) % 7
    };
    let hour_ruler = CHALDEAN_ORDER[hour_idx];

    // ASC ruler
    let asc_sign = (asc_longitude / 30.0).floor() as u8 % 12;
    let asc_degree = asc_longitude - (asc_sign as f64 * 30.0);
    let asc_ruler = DOMICILE_RULERS[asc_sign as usize];

    // Radicality Test (Lilly CA p. 121):
    // "The question then shall be taken for Radical, or fit for judgment,
    // when the Lord of the Ascendant and the Lord of the Hour are of one nature, or of one Triplicity..."
    let (is_radical, radicality_reason) = if asc_ruler == hour_ruler {
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
    };

    // Considerations Before Judgment (Lilly CA p. 122)
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

    PlanetaryHoursReport {
        day_ruler: day_ruler.name().to_string(),
        hour_ruler: hour_ruler.name().to_string(),
        is_day_hour: is_daytime,
        hour_number,
        is_radical,
        radicality_reason,
        considerations,
    }
}

fn are_same_nature(p1: ClassicalPlanet, p2: ClassicalPlanet) -> bool {
    // Hot & Dry: Sun, Mars
    // Cold & Moist: Moon, Venus
    // Cold & Dry: Saturn, Mercury
    // Hot & Moist: Jupiter
    let nature = |p: ClassicalPlanet| match p {
        ClassicalPlanet::Sun | ClassicalPlanet::Mars => 1,
        ClassicalPlanet::Moon | ClassicalPlanet::Venus => 2,
        ClassicalPlanet::Saturn | ClassicalPlanet::Mercury => 3,
        ClassicalPlanet::Jupiter => 4,
    };
    nature(p1) == nature(p2)
}

fn are_same_triplicity(p1: ClassicalPlanet, p2: ClassicalPlanet, sign: u8) -> bool {
    let triplicity = sign % 4;
    let in_triplicity = |p: ClassicalPlanet, trip: u8| match trip {
        0 => matches!(p, ClassicalPlanet::Sun | ClassicalPlanet::Jupiter | ClassicalPlanet::Saturn), // Fire
        1 => matches!(p, ClassicalPlanet::Venus | ClassicalPlanet::Moon | ClassicalPlanet::Mars),    // Earth
        2 => matches!(p, ClassicalPlanet::Saturn | ClassicalPlanet::Mercury | ClassicalPlanet::Jupiter), // Air
        _ => matches!(p, ClassicalPlanet::Mars | ClassicalPlanet::Moon | ClassicalPlanet::Venus),    // Water
    };
    in_triplicity(p1, triplicity) && in_triplicity(p2, triplicity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ephemeris::compute_ephemeris;

    #[test]
    fn test_seoul_morning_and_predawn_day_ruler() {
        // 2024-02-04 was Sunday in Seoul (KST).
        // A: 2024-02-04 07:45 KST (just after sunrise): Sunday day ruler (Sun)
        // 07:45 KST = 2024-02-03 22:45 UTC
        let ephem_morning = compute_ephemeris(2024, 2, 3, 22, 45, 0.0, 37.5665, 126.9780)
            .expect("ephemeris should succeed");
        let sun_lon = ephem_morning.get_body("Sun").unwrap().tropical_longitude;

        let rep_morning = calculate_astronomical_planetary_hours(
            ephem_morning.jd_ut1,
            37.5665,
            ephem_morning.ramc,
            sun_lon,
            ephem_morning.true_obliquity_deg,
            120.0,
            2,
            false,
            false,
        );
        assert_eq!(rep_morning.day_ruler, "Sun", "After sunrise on Sunday must be Sun day");

        // B: 2024-02-04 04:00 KST (predawn before sunrise): Previous astrological day ruler (Saturn)
        // 04:00 KST = 2024-02-03 19:00 UTC
        let ephem_predawn = compute_ephemeris(2024, 2, 3, 19, 0, 0.0, 37.5665, 126.9780)
            .expect("ephemeris should succeed");
        let sun_lon_pre = ephem_predawn.get_body("Sun").unwrap().tropical_longitude;

        let rep_predawn = calculate_astronomical_planetary_hours(
            ephem_predawn.jd_ut1,
            37.5665,
            ephem_predawn.ramc,
            sun_lon_pre,
            ephem_predawn.true_obliquity_deg,
            120.0,
            2,
            false,
            false,
        );
        assert_eq!(rep_predawn.day_ruler, "Saturn", "Before sunrise on Sunday must still be Saturday (Saturn)");
    }
}
