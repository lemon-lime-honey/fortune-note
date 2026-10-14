//! Planetary Return Chart & Inter-Chart Aspects CLI Tool.
//!
//! Computes exact return moment(s) for any celestial body (Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn, North Node, South Node)
//! in a given target year, along with full return chart placements and inter-chart aspects
//! between return points (Return Planets, Return ASC, Return MC) and natal points (Natal Planets, Natal ASC, Natal MC).
//!
//! Usage:
//!   cargo run --example return_chart -- <birth_year> <birth_month> <birth_day> <birth_hour> <birth_minute> <latitude> <longitude> <target_year> <return_body> [--orb <deg>] [--house <system>]
//!
//! Examples:
//!   # 1. 2026 Solar Return:
//!   cargo run --example return_chart -- 1990 5 15 14 30 37.5665 126.9780 2026 Sun
//!
//!   # 2. 2026 Mars Return with 1.5° orb:
//!   cargo run --example return_chart -- 1990 5 15 14 30 37.5665 126.9780 2026 Mars --orb 1.5
//!
//!   # 3. 2026 Jupiter Return using Placidus houses:
//!   cargo run --example return_chart -- 1990 5 15 14 30 37.5665 126.9780 2026 Jupiter --house Placidus

use fortune_core::western::predictive::calculate_planetary_return;
use fortune_core::{calculate_fortune_western, FortuneCalculationInput, Gender, WesternHouseSystem};
use std::env;

const ZODIAC_SIGNS: [&str; 12] = [
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces",
];

fn format_deg(deg: f64) -> String {
    let norm = deg.rem_euclid(360.0);
    let sign_idx = ((norm / 30.0).floor() as usize) % 12;
    let rem = norm % 30.0;
    let d = rem.floor() as u32;
    let m = ((rem - d as f64) * 60.0).floor() as u32;
    format!("{} {:02}°{:02}'", ZODIAC_SIGNS[sign_idx], d, m)
}

fn main() {
    let raw_args: Vec<String> = env::args().collect();

    let mut orb_limit: Option<f64> = None;
    let mut house_system_name: Option<String> = None;
    let mut args: Vec<String> = Vec::new();
    let mut skip_next = false;

    for (i, arg) in raw_args.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg == "--orb" && i + 1 < raw_args.len() {
            orb_limit = raw_args[i + 1].parse().ok();
            skip_next = true;
            continue;
        }
        if arg == "--house" && i + 1 < raw_args.len() {
            house_system_name = Some(raw_args[i + 1].clone());
            skip_next = true;
            continue;
        }
        args.push(arg.clone());
    }

    let (
        birth_year, birth_month, birth_day, birth_hour, birth_minute,
        lat, lon,
        target_year, return_body,
    ) = if args.len() >= 10 {
        (
            args[1].parse::<i32>().unwrap_or(1990),
            args[2].parse::<u32>().unwrap_or(5),
            args[3].parse::<u32>().unwrap_or(15),
            args[4].parse::<u32>().unwrap_or(14),
            args[5].parse::<u32>().unwrap_or(30),
            args[6].parse::<f64>().unwrap_or(37.5665),
            args[7].parse::<f64>().unwrap_or(126.9780),
            args[8].parse::<i32>().unwrap_or(2026),
            args[9].clone(),
        )
    } else {
        println!("===============================================================================");
        println!("Planetary Return Chart & Inter-Chart Aspects CLI Tool");
        println!("===============================================================================");
        println!("Usage: cargo run --example return_chart -- <birth_y> <birth_m> <birth_d> <birth_h> <birth_min> <lat> <lon> <target_year> <return_body> [--orb <deg>] [--house <system>]");
        println!("Defaulting to: 1990-05-15 14:30 Seoul (37.5665N, 126.9780E), 2026 Sun Return, Orb 2.0°");
        println!("===============================================================================");
        (1990, 5, 15, 14, 30, 37.5665, 126.9780, 2026, "Sun".to_string())
    };

    let house_sys_enum = match house_system_name.as_deref().map(|s| s.to_lowercase()).as_deref() {
        Some("placidus") => WesternHouseSystem::Placidus,
        Some("koch") => WesternHouseSystem::Koch,
        Some("equal") => WesternHouseSystem::Equal,
        Some("wholesign") | Some("whole") => WesternHouseSystem::WholeSign,
        Some("campanus") => WesternHouseSystem::Campanus,
        _ => WesternHouseSystem::Regiomontanus,
    };

    let natal_input = FortuneCalculationInput {
        year: birth_year,
        month: birth_month,
        day: birth_day,
        hour: birth_hour,
        minute: birth_minute,
        second: 0.0,
        gender: Gender::Male,
        latitude: lat,
        longitude: lon,
        western_house_system: Some(house_sys_enum),
        timezone_id: None,
        is_horary: false,
        saju_options: Default::default(),
    };

    let natal_chart = calculate_fortune_western(natal_input)
        .expect("Natal chart calculation failed");

    println!("===============================================================================");
    println!("Natal Chart Placements ({:04}-{:02}-{:02} {:02}:{:02} Local, Lat: {:.4}, Lon: {:.4}):",
        birth_year, birth_month, birth_day, birth_hour, birth_minute, lat, lon
    );
    println!("  • ASC: {}", format_deg(natal_chart.western.ascendant));
    println!("  • MC : {}", format_deg(natal_chart.western.midheaven));
    for p in &natal_chart.western.planets {
        let rx_str = if p.is_retrograde { " [Rx]" } else { "" };
        println!("  • {:<8}{}: {}", p.name, rx_str, format_deg(p.longitude));
    }
    for n in &natal_chart.western.nodes {
        println!("  • {:<8}    : {}", n.name, format_deg(n.longitude));
    }
    println!("-------------------------------------------------------------------------------");

    let orb = orb_limit.unwrap_or(2.0);
    println!("Calculating {} Return(s) in {} (House: {}, Orb Limit: {:.2}°)...",
        return_body, target_year, house_sys_enum.display_name(), orb
    );

    let start_time = std::time::Instant::now();
    let returns = calculate_planetary_return(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, 0.0,
        lat, lon, target_year, &return_body,
        Some(house_sys_enum.to_vedaksha_system()),
        Some(orb),
    ).expect("Planetary return calculation failed");
    let elapsed = start_time.elapsed();

    println!("Found {} return event(s) in {:.2?}:", returns.len(), elapsed);
    println!();

    if returns.is_empty() {
        println!("No return event found for {} in year {}.", return_body, target_year);
        println!("(Outer planets like Jupiter, Saturn, Uranus take multiple years to complete a cycle)");
        println!("===============================================================================");
        return;
    }

    for (idx, ret) in returns.iter().enumerate() {
        let rx_str = if ret.is_retrograde { " [Retrograde]" } else { " [Direct]" };
        let speed_str = format!("{:+0.3}°/day", ret.speed);

        println!("###############################################################################");
        println!("### [{:02}] {} Return: {} (JD: {:.4})", idx + 1, ret.return_body, ret.return_date_utc, ret.exact_jd_ut1);
        println!("* Exact Return Body Longitude: {} ({}, {})", format_deg(ret.body_longitude), rx_str, speed_str);
        println!("* Return Angles & Cusps (House: {}):", house_sys_enum.display_name());
        println!("  • Return ASC: {}", format_deg(ret.ascendant));
        println!("  • Return MC : {}", format_deg(ret.midheaven));
        println!("  • Cusps: 1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}",
            format_deg(ret.house_cusps[0]), format_deg(ret.house_cusps[1]),
            format_deg(ret.house_cusps[2]), format_deg(ret.house_cusps[3]),
            format_deg(ret.house_cusps[4]), format_deg(ret.house_cusps[5])
        );
        println!("           7: {}, 8: {}, 9: {}, 10: {}, 11: {}, 12: {}",
            format_deg(ret.house_cusps[6]), format_deg(ret.house_cusps[7]),
            format_deg(ret.house_cusps[8]), format_deg(ret.house_cusps[9]),
            format_deg(ret.house_cusps[10]), format_deg(ret.house_cusps[11])
        );

        println!();
        println!("* Return Planetary Positions:");
        for p in &ret.return_planets {
            let p_rx = if p.is_retrograde { " [Rx]" } else { "" };
            println!("  • {:<10}{}: {}", p.name, p_rx, format_deg(p.progressed_longitude));
        }

        println!();
        println!("* Inter-Chart Aspects (Return Points vs Natal Points, Orb <= {:.2}°):", orb);
        println!("| Return Point | Return Lon | Aspect | Natal Point | Natal Lon | Orb |");
        println!("|---|---|---|---|---|---|");

        for a in &ret.natal_aspects {
            println!(
                "| {:<12} | {:<15} | {:<11} | {:<11} | {:<15} | {:.2}° |",
                format!("Ret {}", a.return_point),
                format_deg(a.return_longitude),
                a.aspect_type,
                format!("Nat {}", a.natal_point),
                format_deg(a.natal_longitude),
                a.orb
            );
        }

        if !ret.return_internal_aspects.is_empty() {
            println!();
            println!("* Return Chart Internal Aspects (Orb <= {:.2}°):", orb);
            for ia in &ret.return_internal_aspects {
                println!("  - Ret {:<8} {:<11} Ret {:<8} (Orb: {:.2}°)",
                    ia.moving_body, ia.aspect_type, ia.natal_body, ia.orb
                );
            }
        }

        println!("-------------------------------------------------------------------------------");
    }

    println!("===============================================================================");
}
