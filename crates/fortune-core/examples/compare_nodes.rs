use vedaksha::astro::sidereal::{true_ayanamsha_value, Ayanamsha};
use vedaksha::ephem::analytical::AnalyticalProvider;
use vedaksha::ephem::bodies::Body;
use vedaksha::ephem::coordinates::apparent_position;
use vedaksha::ephem::julian::calendar_to_jd;
use vedaksha::math::angle::normalize_degrees;

fn format_dms(deg: f64) -> String {
    let d = deg.floor() as u32;
    let rem_m = (deg - d as f64) * 60.0;
    let m = rem_m.floor() as u32;
    let s = (rem_m - m as f64) * 60.0;
    format!("{:3}° {:02}' {:05.2}\"", d, m, s)
}

fn check_moment(label: &str, year: i32, month: u32, day: f64) {
    let jd_ut1 = calendar_to_jd(year, month, day);
    let true_ayanamsha = true_ayanamsha_value(Ayanamsha::IndianOfficial, jd_ut1);
    let provider = AnalyticalProvider;

    let true_pos = apparent_position(&provider, Body::TrueNode, jd_ut1).unwrap();
    let mean_pos = apparent_position(&provider, Body::MeanNode, jd_ut1).unwrap();

    let true_trop = normalize_degrees(true_pos.ecliptic.longitude.to_degrees());
    let mean_trop = normalize_degrees(mean_pos.ecliptic.longitude.to_degrees());

    let true_sid = normalize_degrees(true_trop - true_ayanamsha);
    let mean_sid = normalize_degrees(mean_trop - true_ayanamsha);

    let diff = normalize_degrees(true_trop - mean_trop);
    let diff_signed = if diff > 180.0 { diff - 360.0 } else { diff };

    println!("=== {} (JD UT1: {:.4}) ===", label, jd_ut1);
    println!("Lahiri Ayanamsha: {}", format_dms(true_ayanamsha));
    println!("True Node (Tropical) : {}", format_dms(true_trop));
    println!("Mean Node (Tropical) : {}", format_dms(mean_trop));
    println!("True Node (Sidereal) : {}", format_dms(true_sid));
    println!("Mean Node (Sidereal) : {}", format_dms(mean_sid));
    println!("Difference (True - Mean): {:+.4}° ({:+.2}')\n", diff_signed, diff_signed * 60.0);
}

fn main() {
    // 1. Current timestamp (2026-09-24 02:00:34 UTC)
    let day_frac_now = 24.0 + (2.0 + 34.0 / 60.0) / 24.0;
    check_moment("Current Time (2026-09-24 02:00:34 UTC)", 2026, 9, day_frac_now);

    // 2. Reference test date (2024-03-20 12:00:00 UTC)
    check_moment("Vernal Equinox 2024 (2024-03-20 12:00:00 UTC)", 2024, 3, 20.5);
}
