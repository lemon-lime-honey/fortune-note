//! Natal chart deterministic calculation CLI tool.
//!
//! Excludes seconds according to birth chart requirements.
//! All 8 arguments (year, month, day, hour, minute, latitude, longitude, gender)
//! are strictly required with no arbitrary defaults.

use fortune_core::{FortuneCalculationInput, Gender, VargaDivision, calculate_fortune};
use std::env;

const ZODIAC_SIGNS: [&str; 12] = [
    "Aries",
    "Taurus",
    "Gemini",
    "Cancer",
    "Leo",
    "Virgo",
    "Libra",
    "Scorpio",
    "Sagittarius",
    "Capricorn",
    "Aquarius",
    "Pisces",
];

const RASHI_NAMES: [&str; 12] = [
    "Mesha",
    "Vrishabha",
    "Mithuna",
    "Karka",
    "Simha",
    "Kanya",
    "Tula",
    "Vrishchika",
    "Dhanu",
    "Makara",
    "Kumbha",
    "Meena",
];

fn format_deg(deg: f64) -> String {
    let sign_idx = ((deg / 30.0).floor() as usize) % 12;
    let rem = deg % 30.0;
    let d = rem.floor() as u32;
    let m = ((rem - d as f64) * 60.0).floor() as u32;
    let s = ((((rem - d as f64) * 60.0) - m as f64) * 60.0).round() as u32;
    format!("{} {:02}°{:02}'{:02}\"", ZODIAC_SIGNS[sign_idx], d, m, s)
}

fn format_rashi(sign_idx: u8, deg_in_sign: f64) -> String {
    let s_idx = (sign_idx as usize) % 12;
    let d = deg_in_sign.floor() as u32;
    let m = ((deg_in_sign - d as f64) * 60.0).floor() as u32;
    format!("{} {:02}°{:02}'", RASHI_NAMES[s_idx], d, m)
}

fn print_usage_and_exit() -> ! {
    eprintln!(
        "Usage: cargo run --example natal -- <year> <month> <day> <hour> <minute> <latitude> <longitude> <gender>"
    );
    eprintln!();
    eprintln!("All 8 parameters are strictly required for a natal chart:");
    eprintln!("  <year>      : Calendar year (e.g. 1990)");
    eprintln!("  <month>     : Month 1-12");
    eprintln!("  <day>       : Day 1-31");
    eprintln!("  <hour>      : Birth hour 0-23");
    eprintln!("  <minute>    : Birth minute 0-59");
    eprintln!("  <latitude>  : Observer latitude -90.0 to 90.0 (e.g. 37.5665)");
    eprintln!("  <longitude> : Observer longitude -180.0 to 180.0 (e.g. 126.9780)");
    eprintln!("  <gender>    : 'male' / '남' or 'female' / '여'");
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 9 {
        print_usage_and_exit();
    }

    let year: i32 = match args[1].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: Invalid year '{}'.", args[1]);
            std::process::exit(1);
        }
    };

    let month: u32 = match args[2].parse() {
        Ok(v) if (1..=12).contains(&v) => v,
        _ => {
            eprintln!("Error: Invalid month '{}' (must be 1-12).", args[2]);
            std::process::exit(1);
        }
    };

    let day: u32 = match args[3].parse() {
        Ok(v) if (1..=31).contains(&v) => v,
        _ => {
            eprintln!("Error: Invalid day '{}' (must be 1-31).", args[3]);
            std::process::exit(1);
        }
    };

    let hour: u32 = match args[4].parse() {
        Ok(v) if v <= 23 => v,
        _ => {
            eprintln!("Error: Invalid hour '{}' (must be 0-23).", args[4]);
            std::process::exit(1);
        }
    };

    let minute: u32 = match args[5].parse() {
        Ok(v) if v <= 59 => v,
        _ => {
            eprintln!("Error: Invalid minute '{}' (must be 0-59).", args[5]);
            std::process::exit(1);
        }
    };

    let latitude: f64 = match args[6].parse() {
        Ok(v) if (-90.0..=90.0).contains(&v) => v,
        _ => {
            eprintln!(
                "Error: Invalid latitude '{}' (must be between -90.0 and 90.0).",
                args[6]
            );
            std::process::exit(1);
        }
    };

    let longitude: f64 = match args[7].parse() {
        Ok(v) if (-180.0..=180.0).contains(&v) => v,
        _ => {
            eprintln!(
                "Error: Invalid longitude '{}' (must be between -180.0 and 180.0).",
                args[7]
            );
            std::process::exit(1);
        }
    };

    let gender = match args[8].trim().to_lowercase().as_str() {
        "m" | "male" | "남" | "남자" => Gender::Male,
        "f" | "female" | "여" | "여자" => Gender::Female,
        other => {
            eprintln!(
                "Error: Invalid gender '{}'. Must be 'male' or 'female'.",
                other
            );
            std::process::exit(1);
        }
    };

    // Natal charts exclude seconds (fixed at 0.0)
    let second = 0.0;

    let input = FortuneCalculationInput {
        year,
        month,
        day,
        hour,
        minute,
        second,
        latitude,
        longitude,
        gender,
    };

    let output = match calculate_fortune(input) {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Calculation failed: {}", e);
            std::process::exit(1);
        }
    };

    let offset_sign = if output.utc_offset_hours >= 0.0 {
        "+"
    } else {
        "-"
    };
    let offset_h = output.utc_offset_hours.abs().floor() as u32;
    let offset_m = ((output.utc_offset_hours.abs() - offset_h as f64) * 60.0).round() as u32;

    println!("============================================================");
    println!("  FORTUNE NOTE - NATAL CHART CALCULATION ENGINE");
    println!("============================================================");
    println!(
        "Local Birth : {:04}-{:02}-{:02} {:02}:{:02}",
        year, month, day, hour, minute
    );
    println!("Coordinates : Lat {:.4}°, Lon {:.4}°", latitude, longitude);
    println!(
        "Timezone    : {} (UTC{}{:02}:{:02})",
        output.timezone_id, offset_sign, offset_h, offset_m
    );
    println!(
        "UTC Time    : {:04}-{:02}-{:02} {:02}:{:02}",
        output.utc_year, output.utc_month, output.utc_day, output.utc_hour, output.utc_minute
    );
    println!("Gender      : {:?}", gender);
    println!("------------------------------------------------------------\n");

    // 1. Astronomical Ephemeris
    println!("[1. Astronomical Ephemeris (UTC)]");
    println!("  Julian Day (UT1) : {:.6}", output.ephemeris.jd_ut1);
    println!("  RAMC             : {:.4}°", output.ephemeris.ramc);
    println!(
        "  True Obliquity   : {:.4}°",
        output.ephemeris.true_obliquity_deg
    );
    println!(
        "  Lahiri Ayanamsha : {:.4}°\n",
        output.ephemeris.true_ayanamsha_deg
    );

    // 2. Saju (Four Pillars & DaeUn)
    println!("[2. Saju (Four Pillars of Destiny)]");
    println!(
        "  Year  Pillar : {} ({})",
        output.saju.year_pillar.korean(),
        output.saju.year_pillar.hanja()
    );
    println!(
        "  Month Pillar : {} ({})",
        output.saju.month_pillar.korean(),
        output.saju.month_pillar.hanja()
    );
    println!(
        "  Day   Pillar : {} ({})",
        output.saju.day_pillar.korean(),
        output.saju.day_pillar.hanja()
    );
    println!(
        "  Hour  Pillar : {} ({})",
        output.saju.hour_pillar.korean(),
        output.saju.hour_pillar.hanja()
    );
    println!(
        "  DaeUn Number : {} ({})",
        output.saju.daeun_number,
        if output.saju.is_forward_daeun {
            "Forward 순행"
        } else {
            "Reverse 역행"
        }
    );
    print!("  DaeUn Cycles : ");
    for period in &output.saju.daeun_periods {
        print!(
            "[{:02}세: {}({})] ",
            period.start_age,
            period.pillar.korean(),
            period.pillar.hanja()
        );
    }
    println!("\n");

    // 3. Western Astrology
    println!("[3. Western Astrology (Tropical, Placidus)]");
    println!(
        "  Ascendant (ASC) : {}",
        format_deg(output.western.ascendant)
    );
    println!(
        "  Midheaven (MC)  : {}",
        format_deg(output.western.midheaven)
    );
    println!("  Houses (Placidus Cusps):");
    for (i, cusp) in output.western.house_cusps.iter().enumerate() {
        println!("    House {:02} : {}", i + 1, format_deg(*cusp));
    }
    println!("  Classical Planets (7):");
    for planet in &output.western.planets {
        let retro = if planet.is_retrograde { " [R]" } else { "" };
        println!(
            "    {:<8} : {} (House {:02}){}",
            planet.name,
            format_deg(planet.longitude),
            planet.house_number,
            retro
        );
    }
    println!("  Lunar Nodes (Dragon's Head & Tail):");
    for node in &output.western.nodes {
        let retro = if node.is_retrograde { " [R]" } else { "" };
        println!(
            "    {:<10} : {} (House {:02}){}",
            node.name,
            format_deg(node.longitude),
            node.house_number,
            retro
        );
    }
    println!("  Major Aspects (Classical Planets):");
    if output.western.aspects.is_empty() {
        println!("    None detected within standard orbs.");
    } else {
        for asp in &output.western.aspects {
            println!(
                "    {:<7} {:<10} {:<7} (Actual: {:.2}°, Orb: {:.2}°)",
                asp.planet1, asp.aspect_type, asp.planet2, asp.actual_angle, asp.orb
            );
        }
    }
    println!();

    // 4. Vedic Astrology
    println!("[4. Vedic Astrology (Sidereal, Lahiri)]");
    println!(
        "  Lagna (ASC) : {}",
        format_rashi(output.vedic.lagna_sign, output.vedic.lagna_sidereal % 30.0)
    );
    println!("  Planets & Nakshatras:");
    for p in &output.vedic.planets {
        let retro = if p.is_retrograde { " [R]" } else { "" };
        println!(
            "    {:<8} : {} | Nakshatra: {:<14} Pada {} | Bhava {:02}{}",
            p.name,
            format_rashi(p.sign_index, p.sign_degree),
            p.nakshatra_name,
            p.nakshatra_pada,
            ((p.sign_index + 12 - output.vedic.lagna_sign) % 12) + 1,
            retro
        );
    }
    println!("  Divisional Charts (Vargas):");
    let divisions = [
        VargaDivision::D1,
        VargaDivision::D3,
        VargaDivision::D7,
        VargaDivision::D9,
        VargaDivision::D10,
        VargaDivision::D12,
    ];
    for div in divisions {
        if let Some(v_chart) = output.vedic.vargas.get(&div) {
            print!(
                "    {:<18} (Lagna: {:<9}) -> ",
                div.title(),
                RASHI_NAMES[v_chart.lagna_sign as usize]
            );
            for pl in &v_chart.placements {
                print!(
                    "{}:{} ",
                    pl.planet_name, RASHI_NAMES[pl.sign_index as usize]
                );
            }
            println!();
        }
    }
    println!("  Vimshottari Dasha (120-Year Timeline):");
    for md in &output.vedic.vimshottari_dashas {
        println!(
            "    Maha Dasha: {:<8} (Antar sub-periods: {})",
            md.lord,
            md.sub_periods.len()
        );
        for ad in &md.sub_periods {
            println!("      -> Antar Dasha: {:<8}", ad.lord);
        }
    }
    println!("============================================================");
}
