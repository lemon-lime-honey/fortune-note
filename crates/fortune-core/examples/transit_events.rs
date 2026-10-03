//! Bounded Range Transit Events Timeline CLI Tool.
//!
//! Searches exact transit aspect hits (orb = 0.00°) between transiting planets
//! and natal points (Planets, ASC, MC) within a strictly bounded date range.
//!
//! Usage:
//!   cargo run --example transit_events -- <birth_year> <birth_month> <birth_day> <birth_hour> <birth_minute> <latitude> <longitude> <start_year> <start_month> <start_day> <end_year> <end_month> <end_day> [moving_bodies] [natal_targets]
//!
//! If no arguments are provided, defaults to a sample natal chart for the year 2026.

use fortune_core::western::predictive::{find_transit_events, TransitSearchQuery};
use fortune_core::{calculate_fortune_western, FortuneCalculationInput, Gender, WesternHouseSystem};
use std::env;

const ZODIAC_SIGNS: [&str; 12] = [
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces",
];

fn format_deg(deg: f64) -> String {
    let sign_idx = ((deg / 30.0).floor() as usize) % 12;
    let rem = deg % 30.0;
    let d = rem.floor() as u32;
    let m = ((rem - d as f64) * 60.0).floor() as u32;
    format!("{} {:02}°{:02}'", ZODIAC_SIGNS[sign_idx], d, m)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (
        birth_year, birth_month, birth_day, birth_hour, birth_minute,
        lat, lon,
        start_year, start_month, start_day,
        end_year, end_month, end_day,
        custom_moving, custom_targets,
    ) = if args.len() >= 14 {
        let by: i32 = args[1].parse().expect("Invalid birth_year");
        let bm: u32 = args[2].parse().expect("Invalid birth_month");
        let bd: u32 = args[3].parse().expect("Invalid birth_day");
        let bh: u32 = args[4].parse().expect("Invalid birth_hour");
        let bmin: u32 = args[5].parse().expect("Invalid birth_minute");
        let latitude: f64 = args[6].parse().expect("Invalid latitude");
        let longitude: f64 = args[7].parse().expect("Invalid longitude");

        let sy: i32 = args[8].parse().expect("Invalid start_year");
        let sm: u32 = args[9].parse().expect("Invalid start_month");
        let sd: u32 = args[10].parse().expect("Invalid start_day");

        let ey: i32 = args[11].parse().expect("Invalid end_year");
        let em: u32 = args[12].parse().expect("Invalid end_month");
        let ed: u32 = args[13].parse().expect("Invalid end_day");

        let moving = if args.len() > 14 {
            Some(args[14].split(',').map(|s| s.trim().to_string()).collect())
        } else {
            None
        };

        let targets = if args.len() > 15 {
            Some(args[15].split(',').map(|s| s.trim().to_string()).collect())
        } else {
            None
        };

        (by, bm, bd, bh, bmin, latitude, longitude, sy, sm, sd, ey, em, ed, moving, targets)
    } else {
        println!("===============================================================================");
        println!(" Bounded Range Transit Events Timeline CLI");
        println!("===============================================================================");
        println!("Usage: cargo run --example transit_events -- <by> <bm> <bd> <bh> <bmin> <lat> <lon> <sy> <sm> <sd> <ey> <em > <ed> [moving] [targets]");
        println!("(No arguments provided: Running with default 1990-05-15 Seoul Natal & 2026 Year Range)");
        println!("-------------------------------------------------------------------------------");
        (1990, 5, 15, 14, 30, 37.5665, 126.9780, 2026, 1, 1, 2026, 12, 31, None, None)
    };

    // 1. Calculate natal chart
    let input = FortuneCalculationInput {
        year: birth_year,
        month: birth_month,
        day: birth_day,
        hour: birth_hour,
        minute: birth_minute,
        second: 0.0,
        latitude: lat,
        longitude: lon,
        gender: Gender::Male,
        western_house_system: Some(WesternHouseSystem::Regiomontanus),
        timezone_id: None,
    };

    let western = calculate_fortune_western(input).expect("Natal calculation failed");

    // 2. Prepare natal points (ASC, MC, and Planets)
    let mut natal_points: Vec<(&str, f64)> = Vec::new();
    natal_points.push(("ASC", western.western.ascendant));
    natal_points.push(("MC", western.western.midheaven));
    for p in &western.western.planets {
        if p.name != "Rahu" && p.name != "Ketu" {
            natal_points.push((p.name.as_str(), p.longitude));
        }
    }

    println!("Natal Chart Placements:");
    println!("  • ASC: {}", format_deg(western.western.ascendant));
    println!("  • MC : {}", format_deg(western.western.midheaven));
    for &(name, lon) in &natal_points {
        if name != "ASC" && name != "MC" {
            println!("  • {:<7}: {}", name, format_deg(lon));
        }
    }
    println!("-------------------------------------------------------------------------------");

    // 3. Configure bounded search query
    let query = TransitSearchQuery {
        start_year,
        start_month,
        start_day,
        end_year,
        end_month,
        end_day,
        moving_bodies: custom_moving,
        natal_targets: custom_targets,
        aspect_types: Some(vec![
            "Conjunction".to_string(),
            "Opposition".to_string(),
            "Square".to_string(),
            "Trine".to_string(),
            "Sextile".to_string(),
        ]),
    };

    println!(
        "Searching exact transit hits between {:04}-{:02}-{:02} and {:04}-{:02}-{:02}...",
        start_year, start_month, start_day, end_year, end_month, end_day
    );

    let start_time = std::time::Instant::now();
    let hits = find_transit_events(&query, &natal_points).expect("Search failed");
    let elapsed = start_time.elapsed();

    println!("Found {} exact transit events in {:.2?}:", hits.len(), elapsed);
    println!();
    println!("| Exact UTC Date/Time | Transiting Planet | Aspect | Natal Target | Tr Lon | Nat Lon | Status |");
    println!("|---|---|---|---|---|---|---|");

    for hit in &hits {
        let rx_str = if hit.is_retrograde { "Rx" } else { "Direct" };
        let speed_str = format!("{:+0.2}°/d", hit.moving_speed);
        println!(
            "| {} | {:<10} | {:<11} | {:<12} | {} | {} | {} ({}) |",
            hit.utc_date_time,
            hit.moving_body,
            hit.aspect_type,
            hit.natal_point,
            format_deg(hit.moving_longitude),
            format_deg(hit.natal_longitude),
            rx_str,
            speed_str
        );
    }

    println!();
    println!("===============================================================================");
}
