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
