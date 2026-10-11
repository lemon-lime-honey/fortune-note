//! Western Natal Chart Calculation CLI.
//!
//! Deterministically calculates and displays a complete Western tropical natal chart
//! including William Lilly traditional techniques (Essential Dignities, Almuten,
//! Moon Condition, Planetary Hours, Antiscia, Arabic Parts, Fixed Stars, Prenatal Syzygy).
//!
//! Excludes predictive time-dependent data (transits, returns, progressions, directions).

use std::env;
use fortune_core::{
    calculate_fortune_western, FortuneCalculationInput, Gender, WesternHouseSystem,
};

const ZODIAC_SIGNS: [&str; 12] = [
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces",
];

fn format_deg(deg: f64) -> String {
    let norm = deg.rem_euclid(360.0);
    let sign_idx = (norm / 30.0).floor() as usize % 12;
    let rem = norm % 30.0;
    let d = rem.floor() as u32;
    let m = ((rem - d as f64) * 60.0).floor() as u32;
    let s = ((((rem - d as f64) * 60.0) - m as f64) * 60.0).round() as u32;
    format!("{} {:02}°{:02}'{:02}\"", ZODIAC_SIGNS[sign_idx], d, m, s)
}

fn print_usage_and_exit() -> ! {
    eprintln!("============================================================");
    eprintln!("  FORTUNE NOTE - WESTERN NATAL CHART CLI");
    eprintln!("============================================================");
    eprintln!("Usage:");
    eprintln!("  cargo run -p fortune-core --example western_natal -- \\");
    eprintln!("    <year> <month> <day> <hour> <minute> <latitude> <longitude> <gender> [house_system]");
    eprintln!();
    eprintln!("Required Arguments:");
    eprintln!("  <year>         : Birth year (e.g. 1990)");
    eprintln!("  <month>        : Birth month 1-12");
    eprintln!("  <day>          : Birth day 1-31");
    eprintln!("  <hour>         : Birth hour 0-23");
    eprintln!("  <minute>       : Birth minute 0-59");
    eprintln!("  <latitude>     : Latitude -90.0 to 90.0 (e.g. 37.5665)");
    eprintln!("  <longitude>    : Longitude -180.0 to 180.0 (e.g. 126.9780)");
    eprintln!("  <gender>       : 'male' / '남' or 'female' / '여' (affects Part of Marriage)");
    eprintln!();
    eprintln!("Optional Arguments:");
    eprintln!("  [house_system] : 'regiomontanus' (default), 'placidus', 'koch', 'equal', 'wholesign', 'campanus'");
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 9 || args.len() > 10 {
        print_usage_and_exit();
    }

    let year: i32 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: Invalid year '{}'", args[1]);
        std::process::exit(1);
    });

    let month: u32 = match args[2].parse() {
        Ok(v) if (1..=12).contains(&v) => v,
        _ => {
            eprintln!("Error: Invalid month '{}' (1-12)", args[2]);
            std::process::exit(1);
        }
    };

    let day: u32 = match args[3].parse() {
        Ok(v) if (1..=31).contains(&v) => v,
        _ => {
            eprintln!("Error: Invalid day '{}' (1-31)", args[3]);
            std::process::exit(1);
        }
    };

    let hour: u32 = match args[4].parse() {
        Ok(v) if v <= 23 => v,
        _ => {
            eprintln!("Error: Invalid hour '{}' (0-23)", args[4]);
            std::process::exit(1);
        }
    };

    let minute: u32 = match args[5].parse() {
        Ok(v) if v <= 59 => v,
        _ => {
            eprintln!("Error: Invalid minute '{}' (0-59)", args[5]);
            std::process::exit(1);
        }
    };

    let latitude: f64 = match args[6].parse() {
        Ok(v) if (-90.0..=90.0).contains(&v) => v,
        _ => {
            eprintln!("Error: Invalid latitude '{}' (-90.0 to 90.0)", args[6]);
            std::process::exit(1);
        }
    };

    let longitude: f64 = match args[7].parse() {
        Ok(v) if (-180.0..=180.0).contains(&v) => v,
        _ => {
            eprintln!("Error: Invalid longitude '{}' (-180.0 to 180.0)", args[7]);
            std::process::exit(1);
        }
    };

    let gender = match args[8].trim().to_lowercase().as_str() {
        "m" | "male" | "남" | "남자" => Gender::Male,
        "f" | "female" | "여" | "여자" => Gender::Female,
        other => {
            eprintln!("Error: Invalid gender '{}'. Must be 'male' or 'female'.", other);
            std::process::exit(1);
        }
    };

    let house_system = if args.len() == 10 {
        match args[9].trim().to_lowercase().as_str() {
            "regiomontanus" | "regio" => WesternHouseSystem::Regiomontanus,
            "placidus" => WesternHouseSystem::Placidus,
            "koch" => WesternHouseSystem::Koch,
            "equal" => WesternHouseSystem::Equal,
            "wholesign" | "whole_sign" | "whole" => WesternHouseSystem::WholeSign,
            "campanus" => WesternHouseSystem::Campanus,
            other => {
                eprintln!("Error: Unknown house system '{}'. Using Regiomontanus.", other);
                WesternHouseSystem::Regiomontanus
            }
        }
    } else {
        WesternHouseSystem::Regiomontanus
    };

    let input = FortuneCalculationInput {
        year,
        month,
        day,
        hour,
        minute,
        second: 0.0,
        latitude,
        longitude,
        gender,
        western_house_system: Some(house_system),
        timezone_id: None,
    };

    let output = match calculate_fortune_western(input) {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Western calculation failed: {}", e);
            std::process::exit(1);
        }
    };

    let offset_sign = if output.utc_offset_hours >= 0.0 { "+" } else { "-" };
    let offset_h = output.utc_offset_hours.abs().floor() as u32;
    let offset_m = ((output.utc_offset_hours.abs() - offset_h as f64) * 60.0).round() as u32;

    println!("============================================================");
    println!("       FORTUNE NOTE - WESTERN TROPICAL NATAL CHART          ");
    println!("============================================================");
    println!("Local Birth    : {:04}-{:02}-{:02} {:02}:{:02}", year, month, day, hour, minute);
    println!("Timezone       : {} (UTC{}{:02}:{:02})", output.timezone_id, offset_sign, offset_h, offset_m);
    println!("Observer Coords: Lat {:.4}°, Lon {:.4}°", latitude, longitude);
    println!("Gender         : {:?} (Marriage lot branch applied)", gender);
    println!("House System   : {}", house_system.display_name());
    println!("------------------------------------------------------------\n");

    // 1. Astronomical Baseline
    println!("[1. Astronomical Baseline]");
    println!("  Julian Day (UT1) : {:.6}", output.ephemeris.jd_ut1);
    println!("  RAMC             : {:.4}°", output.ephemeris.ramc);
    println!("  True Obliquity   : {:.4}°\n", output.ephemeris.true_obliquity_deg);

    // 2. Angles & Houses
    let chart = &output.western;
    let asc = chart.ascendant;
    let mc = chart.midheaven;
    let dsc = (asc + 180.0).rem_euclid(360.0);
    let ic = (mc + 180.0).rem_euclid(360.0);

    println!("[2. Angles & House Cusps ({})]", house_system.display_name());
    println!("  Ascendant  (ASC) : {}", format_deg(asc));
    println!("  Midheaven  (MC)  : {}", format_deg(mc));
    println!("  Descendant (DSC) : {}", format_deg(dsc));
    println!("  Immum Coeli(IC)  : {}", format_deg(ic));
    println!("  Cusps:");
    for (i, cusp) in chart.house_cusps.iter().enumerate() {
        println!("    House {:02} : {}", i + 1, format_deg(*cusp));
    }
    println!();

    // 3. Classical Planets & Nodes
    println!("[3. Planetary Positions (7 Classical + Lunar Nodes)]");
    for planet in &chart.planets {
        let retro = if planet.is_retrograde { " [Rx]" } else { "     " };
        println!(
            "  {:<8} : {}  (House {:02}){}  Speed: {:+0.3}°/day  Lat: {:+0.2}°",
            planet.name,
            format_deg(planet.longitude),
            planet.house_number,
            retro,
            planet.speed,
            planet.latitude
        );
    }
    for node in &chart.nodes {
        let retro = if node.is_retrograde { " [Rx]" } else { "     " };
        println!(
            "  {:<10} : {}  (House {:02}){}",
            node.name,
            format_deg(node.longitude),
            node.house_number,
            retro
        );
    }
    println!();

    // 4. Major Aspects
    println!("[4. Major Aspects (Standard Angular Relationships)]");
    if chart.aspects.is_empty() {
        println!("  No major aspects detected.");
    } else {
        for asp in &chart.aspects {
            let movement = if asp.is_applying { "Applying" } else { "Separating" };
            println!(
                "  {:<7} {:<11} {:<7} : Actual {:6.2}°, Orb {:4.2}° ({})",
                asp.planet1, asp.aspect_type, asp.planet2, asp.actual_angle, asp.orb, movement
            );
        }
    }
    println!();

    // 5. Traditional Astrology (William Lilly CA 1647)
    if let Some(ref trad) = chart.traditional {
        println!("============================================================");
        println!("  WILLIAM LILLY TRADITIONAL ASTROLOGY ANALYSIS");
        println!("============================================================");

        // 5.1 Planetary Hours
        println!("[5.1 Planetary Hours]");
        println!(
            "  Day Ruler   : {} ({})",
            trad.planetary_hours.day_ruler,
            if trad.planetary_hours.is_day_hour { "Daytime" } else { "Nighttime" }
        );
        println!(
            "  Hour Ruler  : {} (Hour {:02})",
            trad.planetary_hours.hour_ruler,
            trad.planetary_hours.hour_number
        );
        println!();

        // 5.2 Moon Condition
        println!("[5.2 Condition of the Moon]");
        println!("  Phase             : {} (Elongation: {:.2}°)", trad.moon_condition.lunar_phase_name, trad.moon_condition.elongation_deg);
        if let Some(ref next_asp) = trad.moon_condition.next_applying_aspect {
            println!("  Next Application  : {}", next_asp);
        }
        println!();

        // 5.3 Prenatal Syzygy
        if let Some(ref san) = trad.prenatal_syzygy {
            println!("[5.3 Prenatal Syzygy (Syzygy Ante Nativitatem - SAN)]");
            println!("  Type      : {}", san.syzygy_type);
            println!("  Longitude : {}", format_deg(san.longitude));
            println!("  UTC Moment: {}\n", san.exact_datetime_utc);
        }

        // 5.4 Essential Dignities & Scores
        println!("[5.4 Essential Dignities & Lilly Scores]");
        for d in &trad.planet_dignities {
            let mut dignities = Vec::new();
            if d.is_in_domicile { dignities.push("Domicile(+5)"); }
            if d.is_in_exaltation { dignities.push("Exaltation(+4)"); }
            if d.is_in_triplicity { dignities.push("Triplicity(+3)"); }
            if d.is_in_term { dignities.push("Term(+2)"); }
            if d.is_in_face { dignities.push("Face(+1)"); }
            if d.is_in_detriment { dignities.push("Detriment(-5)"); }
            if d.is_in_fall { dignities.push("Fall(-4)"); }
            if d.is_peregrine { dignities.push("Peregrine(-5)"); }

            let dignities_str = if dignities.is_empty() { "-".to_string() } else { dignities.join(", ") };
            println!(
                "  {:<8} | Total Score: {:+3} (Essential: {:+2}, Accidental: {:+2}) | [{}]",
                d.planet.name(),
                d.total_lilly_score,
                d.essential_score,
                d.accidental_score,
                dignities_str
            );
        }
        println!();

        // 5.5 House Cusp Dignities & Almuten
        println!("[5.5 House Cusp Dignities & Almuten]");
        for cd in &trad.cusp_dignities {
            println!(
                "  House {:02} ({}) : Domicile: {:<7} | Exalt: {:<7} | Trip: {:<7} | Almuten: {:<7} (Score: {:02})",
                cd.house_number,
                format_deg(cd.cusp_degree),
                cd.domicile,
                cd.exaltation.as_deref().unwrap_or("-"),
                cd.triplicity,
                cd.almuten,
                cd.almuten_score
            );
        }
        println!();

        // 5.6 Traditional Aspects with Moiety Orbs
        println!("[5.6 Dynamic Traditional Aspects (Moiety of Orbs)]");
        if trad.traditional_aspects.is_empty() {
            println!("  None detected within moiety orbs.");
        } else {
            for asp in &trad.traditional_aspects {
                let motion = if asp.is_applying { "Applying" } else { "Separating" };
                let status = if asp.is_partile {
                    format!("Partile ({})", motion)
                } else {
                    motion.to_string()
                };
                println!(
                    "  {:<7} {:<11} {:<7} : Orb {:4.2}° / Max {:4.2}° | {} (To perfection: {:.2}°)",
                    asp.planet1, asp.aspect_type, asp.planet2, asp.orb, asp.max_allowed_orb, status, asp.degrees_to_perfection
                );
            }
        }
        println!();

        // 5.7 Mutual Receptions
        println!("[5.7 Mutual Receptions]");
        if trad.mutual_receptions.is_empty() {
            println!("  No classical mutual receptions detected.");
        } else {
            for mr in &trad.mutual_receptions {
                println!("  * {} ({})", mr.description, mr.reception_type);
            }
        }
        println!();

        // 5.8 Arabic Parts / Lots
        println!("[5.8 Arabic Parts & Hermetic Lots]");
        for part in &trad.arabic_parts {
            println!(
                "  {:<18} : {}  (House {:02}, Ruler: {})",
                part.name,
                format_deg(part.longitude),
                part.house_number,
                part.ruler
            );
        }
        println!();

        // 5.9 Antiscia Table
        println!("[5.9 Antiscia & Contra-Antiscia (Solstice Reflection)]");
        for ant in &trad.antiscia {
            println!(
                "  {:<10} : Antiscion: {} | Contra-Antiscion: {}",
                ant.body_name,
                format_deg(ant.antiscion_longitude),
                format_deg(ant.contra_antiscion_longitude)
            );
        }
        println!();

        // 5.10 Fixed Star Conjunctions
        println!("[5.10 Traditional Fixed Star Conjunctions (Orb <= 1.5°)]");
        if trad.fixed_star_conjunctions.is_empty() {
            println!("  No conjunctions with major traditional fixed stars detected.");
        } else {
            for fsc in &trad.fixed_star_conjunctions {
                println!(
                    "  * {} ({}) conj {} : Orb {:.2}° (Star at {})",
                    fsc.star_name, fsc.korean_name, fsc.body_name, fsc.orb, format_deg(fsc.star_longitude)
                );
                println!("    Nature : {}", fsc.nature);
                println!("    Meaning: {}", fsc.meaning);
            }
        }
    }

    println!("\n============================================================");
    println!("  END OF WESTERN NATAL CHART REPORT");
    println!("============================================================");
}
