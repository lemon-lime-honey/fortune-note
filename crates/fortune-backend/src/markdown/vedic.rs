use fortune_core::celestial::format_jd_date;
use fortune_core::{FortuneCalculationOutput, VargaDivision};

const VEDIC_SIGNS: [&str; 12] = [
    "Mesha (Aries)", "Vrishabha (Taurus)", "Mithuna (Gemini)", "Karka (Cancer)",
    "Simha (Leo)", "Kanya (Virgo)", "Tula (Libra)", "Vrishchika (Scorpio)",
    "Dhanu (Sagittarius)", "Makara (Capricorn)", "Kumbha (Aquarius)", "Meena (Pisces)",
];

fn append_vedic_indicators(out: &mut String, chart: &FortuneCalculationOutput, is_prashna: bool) {
    let pan = &chart.vedic.panchanga;
    if !pan.tithi_name.is_empty() {
        out.push_str("- Panchanga (5 Limbs of Time):\n");
        out.push_str(&format!("  * Tithi: {} (#{}, {})\n", pan.tithi_name, pan.tithi_number, pan.paksha));
        out.push_str(&format!("  * Vara: {} (Lord: {})\n", pan.vara_name, pan.vara_lord));
        out.push_str(&format!("  * Nakshatra: {} Pada {} (Lord: {})\n", pan.nakshatra_name, pan.nakshatra_pada, pan.nakshatra_lord));
        out.push_str(&format!("  * Yoga: {}\n  * Karana: {}\n", pan.yoga_name, pan.karana_name));
    }
    if !is_prashna && !chart.vedic.chara_karakas.is_empty() {
        out.push_str("- Jaimini Chara Karakas (7-Karaka Scheme):\n");
        for ck in &chart.vedic.chara_karakas {
            out.push_str(&format!("  * {}: {} ({:.2}°)\n", ck.karaka, ck.planet, ck.effective_degree));
        }
    }
    if !chart.vedic.arudha_padas.is_empty() {
        out.push_str("- Arudha Padas (BPHS Ch. 29):\n");
        for ap in &chart.vedic.arudha_padas {
            out.push_str(&format!("  * {} ({}): {} (H{} from Lagna)\n", ap.code, ap.name, ap.sign_name, ap.house_from_lagna));
        }
    }
    let notable_drishti: Vec<_> = chart.vedic.drishti.iter()
        .filter(|d| !d.aspected_planets.is_empty() || d.aspect_offset != 7)
        .collect();
    if !notable_drishti.is_empty() {
        out.push_str("- Graha Drishti (Full Aspects incl. Special Bhava Aspects):\n");
        for d in notable_drishti {
            let targets = if d.aspected_planets.is_empty() { "Empty Bhava".to_string() } else { d.aspected_planets.join(", ") };
            out.push_str(&format!(
                "  * {} (H{}) -> {}th Drishti on H{} ({}): {}\n",
                d.from_planet, d.from_house, d.aspect_offset, d.target_house, VEDIC_SIGNS[d.target_sign as usize % 12], targets
            ));
        }
    }
    if !is_prashna && chart.vedic.ashtakavarga.total_bindus > 0 {
        out.push_str("- Ashtakavarga (Sarvashtakavarga & Bhinna Totals):\n");
        let sav_parts: Vec<String> = chart.vedic.ashtakavarga.sarva.iter().enumerate()
            .map(|(idx, &pts)| format!("{}: {}", VEDIC_SIGNS[idx], pts))
            .collect();
        out.push_str(&format!("  * Sarvashtakavarga (Total {}): {}\n", chart.vedic.ashtakavarga.total_bindus, sav_parts.join(", ")));
        for row in &chart.vedic.ashtakavarga.bhinna {
            out.push_str(&format!("  * Bhinna {} (Total {}): {:?}\n", row.planet, row.total, row.bindus));
        }
    }
}

fn append_vedic_vargas_and_dasha(out: &mut String, chart: &FortuneCalculationOutput, is_prashna: bool) {
    out.push_str("- Divisional Charts (Vargas):\n");
    for (division, v) in &chart.vedic.vargas {
        if is_prashna && !matches!(*division, VargaDivision::D1 | VargaDivision::D3 | VargaDivision::D9) {
            continue;
        }
        out.push_str(&format!("  * {}: Lagna {}\n", division.title(), VEDIC_SIGNS[v.lagna_sign as usize % 12]));
        let placements: Vec<String> = v.placements.iter().map(|p| {
            let p_sign = VEDIC_SIGNS[p.sign_index as usize % 12];
            if *division == VargaDivision::D1 {
                format!("{}: H{} {} {:.2}°", p.planet_name, p.house, p_sign, p.sign_degree)
            } else {
                format!("{}: H{} {}", p.planet_name, p.house, p_sign)
            }
        }).collect();
        if !placements.is_empty() {
            out.push_str(&format!("    - Placements: {}\n", placements.join(", ")));
        }
    }

    if !is_prashna {
        out.push_str("- Vimshottari Dasha Sequence:\n");
        for d in &chart.vedic.vimshottari_dashas {
            out.push_str(&format!("  * Mahadasha {}: {} ~ {}\n", d.lord, format_jd_date(d.start_jd), format_jd_date(d.end_jd)));
            for sub in &d.sub_periods {
                out.push_str(&format!("    - Antardasha {}: {} ~ {}\n", sub.lord, format_jd_date(sub.start_jd), format_jd_date(sub.end_jd)));
            }
        }
    }
}

pub(crate) fn append_vedic_details(out: &mut String, chart: &FortuneCalculationOutput, is_prashna: bool) {
    if is_prashna {
        out.push_str("## 2. Prashna Chart Placements (Vedic Sidereal)\n");
    } else {
        out.push_str("## 2. 라시 차트 및 분점도 (Vedic Sidereal & Vargas)\n");
    }
    let lagna_label = if is_prashna { "Prashna Lagna" } else { "Lagna" };
    out.push_str(&format!("- Ayanamsha: {} ({:.4}°)\n", chart.vedic.ayanamsha_name, chart.vedic.ayanamsha_deg));
    out.push_str(&format!("- {}: {:.2}° ({})\n", lagna_label, chart.vedic.lagna_sidereal, VEDIC_SIGNS[chart.vedic.lagna_sign as usize % 12]));
    out.push_str("- D1 Rasi Placements:\n");
    for p in &chart.vedic.planets {
        let rx = if p.is_retrograde { " [Rx]" } else { "" };
        let comb = if p.combustion != "None" && !p.combustion.is_empty() { format!(" [{}]", p.combustion) } else { String::new() };
        out.push_str(&format!(
            "  * {}: {:.2}° (H{}, {} {:.2}°, {} Pada {}, Dignity: {}){}{}\n",
            p.name, p.sidereal_longitude, p.house, VEDIC_SIGNS[p.sign_index as usize % 12],
            p.sign_degree, p.nakshatra_name, p.nakshatra_pada, p.dignity, rx, comb
        ));
    }

    append_vedic_indicators(out, chart, is_prashna);
    append_vedic_vargas_and_dasha(out, chart, is_prashna);
    out.push('\n');
}
