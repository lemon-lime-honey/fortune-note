use fortune_core::celestial::ZODIAC_SIGNS;
use fortune_core::western::fixed_stars::FixedStarConjunction;
use fortune_core::western::predictive::{PredictiveSuite, PREDICTIVE_CONTACT_ORB_DEG};
use fortune_core::western::{HellenisticData, WesternTraditionalData};
use fortune_core::{FortuneCalculationOutput, WesternHouseSystem};

fn append_fixed_star_conjunctions(out: &mut String, conjunctions: &[FixedStarConjunction]) {
    if conjunctions.is_empty() {
        return;
    }
    out.push_str("\n### Fixed Star Conjunctions\n");
    for sc in conjunctions {
        let qual = if sc.is_qualified { "Qualified" } else { "Filtered" };
        let vis = if sc.is_visible { "Visible" } else { "Below Horizon" };
        out.push_str(&format!(
            "- {} ☌ {} (Orb: {:.2}°, Lat: {:+.1}°, Mag: {:.2}, Nature: {}) [{}: {}] [{} (Culm: {:+.1}°)]: {}\n",
            sc.star_name, sc.body_name, sc.orb, sc.star_latitude, sc.star_magnitude,
            sc.nature, qual, sc.qualification_reason, vis, sc.max_altitude, sc.meaning
        ));
    }
}

fn append_house_cusps(out: &mut String, chart: &FortuneCalculationOutput, is_whole_sign: bool) {
    if is_whole_sign {
        out.push_str("- House Signs (Whole Sign Houses):\n");
        let mc_h = fortune_core::western::determine_house(chart.western.midheaven, &chart.western.house_cusps);
        for (i, cusp) in chart.western.house_cusps.iter().enumerate() {
            let sign_idx = (cusp / 30.0).floor() as usize % 12;
            let ruler = fortune_core::western::dignity::DOMICILE_RULERS[sign_idx].name();
            let mc_tag = if (i + 1) as u8 == mc_h { " [MC Location]" } else { "" };
            out.push_str(&format!("  * House {:02}{}: {} (Ruler: {})\n", i + 1, mc_tag, ZODIAC_SIGNS[sign_idx], ruler));
        }
        return;
    }

    out.push_str("- House Cusps (12 Cusps):\n");
    for (i, cusp) in chart.western.house_cusps.iter().enumerate() {
        let sign_idx = (cusp / 30.0).floor() as usize % 12;
        let sign_deg = cusp - (sign_idx as f64 * 30.0);
        let tag = match i + 1 { 1 => " (ASC)", 4 => " (IC)", 7 => " (DSC)", 10 => " (MC)", _ => "" };
        out.push_str(&format!("  * House {:02}{}: {:.2}° ({})\n", i + 1, tag, sign_deg, ZODIAC_SIGNS[sign_idx]));
    }
}

fn append_hellenistic_conditions_table(out: &mut String, h: &HellenisticData) {
    out.push_str("\n### Qualitative Planet Conditions, Egyptian Bounds & Dorothean Triplicity\n");
    out.push_str("| Planet | Status | House | Bound Ruler | Triplicity (Day/Night/Part) | In Triplicity | Lord of ASC |\n");
    out.push_str("| :--- | :--- | :---: | :---: | :---: | :---: | :---: |\n");
    for pc in &h.planet_conditions {
        let bound = if pc.bound_ruler.is_empty() { "-" } else { pc.bound_ruler.as_str() };
        let trip = if pc.triplicity_day_ruler.is_empty() {
            "-".to_string()
        } else {
            format!("{}/{}/{}", pc.triplicity_day_ruler, pc.triplicity_night_ruler, pc.triplicity_participating_ruler)
        };
        out.push_str(&format!(
            "| **{}** | {} | H{:02} | {} | {} | {} | {} |\n",
            pc.name, pc.condition_label, pc.house_number, bound, trip,
            if pc.is_in_triplicity { "Yes" } else { "-" },
            if pc.is_ruler_of_asc { "Yes" } else { "-" },
        ));
    }
}

fn append_hellenistic_lots_and_aspects(out: &mut String, h: &HellenisticData) {
    if !h.lots.is_empty() {
        out.push_str("\n### Hermetic Lots (Paulus Alexandrinus)\n");
        for lot in &h.lots {
            out.push_str(&format!(
                "- {}: {:.2}° ({}) - House {:02} (Ruler: {})\n",
                lot.name, lot.longitude, ZODIAC_SIGNS[lot.sign_index as usize % 12], lot.house_number, lot.ruler
            ));
        }
    }
    if !h.aspects.is_empty() {
        out.push_str("\n### Sign-Based Aspects & Degree Contacts\n");
        for ta in &h.aspects {
            let partile = if ta.is_partile { " [Partile <= 1°]" } else { "" };
            let dyn_str = if ta.is_applying { "Applying" } else { "Separating" };
            out.push_str(&format!("- {} - {}: {} (Orb: {:.2}°){}, {}\n", ta.planet1, ta.planet2, ta.aspect_type, ta.orb, partile, dyn_str));
        }
    }
}

fn append_hellenistic_details(out: &mut String, h: &HellenisticData) {
    out.push_str("## 3. Hellenistic Traditional Analysis (Whole Sign)\n");
    out.push_str("### Independent Angles & Planetary Action Points\n");
    let mc_note = if h.angles.is_mc_in_10th {
        "House 10 (Culmination)".to_string()
    } else {
        format!("House {:02} (Special Action Point)", h.angles.midheaven_house)
    };
    out.push_str(&format!("- Ascendant (ASC): {:.2}° ({})\n", h.angles.ascendant_degree, ZODIAC_SIGNS[h.angles.ascendant_sign as usize % 12]));
    out.push_str(&format!("- Midheaven (MC): {:.2}° ({}) - {}\n", h.angles.midheaven_degree, ZODIAC_SIGNS[h.angles.midheaven_sign as usize % 12], mc_note));

    append_hellenistic_conditions_table(out, h);
    append_hellenistic_lots_and_aspects(out, h);
    if let Some(ref san) = h.prenatal_syzygy {
        out.push_str(&format!("\n### Prenatal Syzygy (SAN)\n- Type: {}, Longitude: {:.2}° ({}), Moment: {}\n", san.syzygy_type, san.longitude, ZODIAC_SIGNS[san.sign_index as usize % 12], san.exact_datetime_utc));
    }
    append_fixed_star_conjunctions(out, &h.fixed_star_conjunctions);
    out.push('\n');
}

fn append_medieval_dignities(out: &mut String, trad: &WesternTraditionalData) {
    out.push_str("\n### Essential & Accidental Dignities\n");
    out.push_str("| Planet | Longitude | House | Dignity Status | Essential | Accidental | Total Score |\n");
    out.push_str("| :--- | :--- | :---: | :--- | :---: | :---: | :---: |\n");
    for d in &trad.planet_dignities {
        let flags = [
            (d.is_in_domicile, "Domicile (+5)"), (d.is_in_exaltation, "Exaltation (+4)"),
            (d.is_in_triplicity, "Triplicity (+3)"), (d.is_in_term, "Term (+2)"),
            (d.is_in_face, "Face (+1)"), (d.is_in_detriment, "Detriment (-5)"),
            (d.is_in_fall, "Fall (-4)"), (d.is_peregrine, "Peregrine (-5)"),
        ];
        let mut status: Vec<&str> = flags.iter().filter_map(|&(ok, lbl)| ok.then_some(lbl)).collect();
        if status.is_empty() { status.push("Neutral"); }
        out.push_str(&format!(
            "| {} | {:.2}° | H{} | {} | {:+} | {:+} | **{:+}** |\n",
            d.planet.name(), d.longitude, d.house_number, status.join(", "),
            d.essential_score, d.accidental_score, d.total_lilly_score
        ));
    }
}

fn append_medieval_secondary_tables(out: &mut String, trad: &WesternTraditionalData, is_horary: bool) {
    if !trad.cusp_dignities.is_empty() {
        out.push_str("\n### House Cusp Dignities & Almutens\n| House | Cusp Degree | Domicile | Exaltation | Triplicity | Term | Face | Almuten (Score) |\n| :---: | :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");
        for cd in &trad.cusp_dignities {
            out.push_str(&format!(
                "| H{:02} | {:.2}° ({}) | {} | {} | {} | {} | {} | **{}** ({}) |\n",
                cd.house_number, cd.sign_degree, ZODIAC_SIGNS[cd.sign_index as usize % 12],
                cd.domicile, cd.exaltation.as_deref().unwrap_or("-"),
                cd.triplicity, cd.term, cd.face, cd.almuten, cd.almuten_score
            ));
        }
    }
    if !trad.mutual_receptions.is_empty() {
        out.push_str("\n### Mutual Receptions\n");
        for mr in &trad.mutual_receptions {
            out.push_str(&format!("- [{}] {}\n", mr.reception_type, mr.description));
        }
    }
    let perf = &trad.perfection;
    if is_horary && (!perf.translation_of_light.is_empty() || !perf.collection_of_light.is_empty() || !perf.besiegement.is_empty()) {
        out.push_str("\n### Horary Perfection & Besiegement\n");
        for tol in &perf.translation_of_light { out.push_str(&format!("- [Translation of Light] {}\n", tol)); }
        for col in &perf.collection_of_light { out.push_str(&format!("- [Collection of Light] {}\n", col)); }
        for bsg in &perf.besiegement { out.push_str(&format!("- [Besiegement] {}\n", bsg)); }
    }
    if !trad.antiscia.is_empty() {
        out.push_str("\n### Antiscia & Contra-Antiscia (Solstice Reflection)\n");
        for a in &trad.antiscia {
            out.push_str(&format!("- {}: Antiscion {:.2}° ({}) | Contra-Antiscion {:.2}°\n", a.body_name, a.antiscion_degree, ZODIAC_SIGNS[a.antiscion_sign_index as usize % 12], a.contra_antiscion_longitude));
        }
    }
    if !trad.arabic_parts.is_empty() {
        out.push_str("\n### Arabic Parts\n");
        for part in &trad.arabic_parts {
            out.push_str(&format!("- {}: {:.2}° ({}) - House {} (Ruler: {})\n", part.name, part.longitude, ZODIAC_SIGNS[part.sign_index as usize % 12], part.house_number, part.ruler));
        }
    }
}

fn append_medieval_header(out: &mut String, trad: &WesternTraditionalData, is_horary: bool) {
    out.push_str("## 3. Medieval Traditional Analysis (Regiomontanus)\n");
    if is_horary {
        let rad = if trad.planetary_hours.is_radical { "Radical" } else { "Non-radical" };
        let voc = if trad.moon_condition.is_void_of_course { "Void of Course" } else { "None" };
        let vc = if trad.moon_condition.is_via_combusta { "Via Combusta (15° Lib - 15° Sco)" } else { "None" };
        out.push_str(&format!(
            "- Planetary Hours: Day of {}, Hour of {} (Radicality: {} - {})\n- Moon Condition: Phase [{}], VOC [{}], Via Combusta [{}]\n",
            trad.planetary_hours.day_ruler, trad.planetary_hours.hour_ruler, rad,
            trad.planetary_hours.radicality_reason, trad.moon_condition.lunar_phase_name, voc, vc
        ));
        if let Some(ref next_asp) = trad.moon_condition.next_applying_aspect {
            out.push_str(&format!("  * Next Applying Aspect: {}\n", next_asp));
        }
        if !trad.planetary_hours.considerations.is_empty() {
            out.push_str("- Considerations Before Judgment:\n");
            for c in &trad.planetary_hours.considerations { out.push_str(&format!("  * {}\n", c)); }
        }
    } else {
        let span = if trad.planetary_hours.is_day_hour { "Day" } else { "Night" };
        out.push_str(&format!(
            "- Planetary Hours: Day of {}, Hour of {} (Hour {}, {})\n- Moon Condition: Phase [{}]\n",
            trad.planetary_hours.day_ruler, trad.planetary_hours.hour_ruler,
            trad.planetary_hours.hour_number, span, trad.moon_condition.lunar_phase_name
        ));
    }
    if let Some(ref san) = trad.prenatal_syzygy {
        out.push_str(&format!("- Prenatal Syzygy (SAN): {} at {:.2}° ({}) on {}\n", san.syzygy_type, san.longitude, ZODIAC_SIGNS[san.sign_index as usize % 12], san.exact_datetime_utc));
    }
}

fn append_medieval_details(out: &mut String, trad: &WesternTraditionalData, is_horary: bool) {
    append_medieval_header(out, trad, is_horary);
    append_medieval_dignities(out, trad);
    if !trad.traditional_aspects.is_empty() {
        out.push_str("\n### Traditional Dynamic Aspects (Applying / Separating, Moiety Orb)\n");
        for ta in &trad.traditional_aspects {
            out.push_str(&format!(
                "- {} - {}: {} (Orb: {:.2}° / Max: {:.2}°){}, {}\n",
                ta.planet1, ta.planet2, ta.aspect_type, ta.orb, ta.max_allowed_orb,
                if ta.is_partile { " [Partile]" } else { "" },
                if ta.is_applying { "Applying" } else { "Separating" }
            ));
        }
    }
    append_fixed_star_conjunctions(out, &trad.fixed_star_conjunctions);
    append_medieval_secondary_tables(out, trad, is_horary);
    out.push('\n');
}

pub(crate) fn append_western_details(out: &mut String, chart: &FortuneCalculationOutput, is_horary: bool) {
    let asc_sign = (chart.western.ascendant / 30.0).floor() as usize % 12;
    let mc_sign = (chart.western.midheaven / 30.0).floor() as usize % 12;
    let is_whole_sign = chart.western.house_system == WesternHouseSystem::WholeSign;
    let mc_desc = if is_whole_sign {
        let mc_h = fortune_core::western::determine_house(chart.western.midheaven, &chart.western.house_cusps);
        format!(" [Located in House {:02}]", mc_h)
    } else {
        String::new()
    };

    out.push_str("## 2. Celestial Placements & House Cusps\n");
    out.push_str(&format!("- Ascendant (ASC): {:.2}° ({})\n", chart.western.ascendant, ZODIAC_SIGNS[asc_sign]));
    out.push_str(&format!("- Midheaven (MC): {:.2}° ({}){}\n", chart.western.midheaven, ZODIAC_SIGNS[mc_sign], mc_desc));
    out.push_str(&format!("- House System: {}\n", chart.western.house_system.display_name()));
    append_house_cusps(out, chart, is_whole_sign);

    out.push_str("- Planetary Placements:\n");
    for p in &chart.western.planets {
        let rx = if p.is_retrograde { " [Rx]" } else { "" };
        out.push_str(&format!("  * {}: {:.2}° ({}) - House {}{} (Speed: {:+0.2}°/day)\n", p.name, p.longitude, ZODIAC_SIGNS[p.sign_index as usize % 12], p.house_number, rx, p.speed));
    }
    out.push_str("- Lunar Nodes:\n");
    for n in &chart.western.nodes {
        out.push_str(&format!("  * {}: {:.2}° ({}) - House {}\n", n.name, n.longitude, ZODIAC_SIGNS[n.sign_index as usize % 12], n.house_number));
    }
    out.push_str("- Major Aspects:\n");
    if chart.western.aspects.is_empty() {
        out.push_str("  * No major aspects within orb\n");
    } else {
        for a in &chart.western.aspects {
            out.push_str(&format!("  * {} - {}: {} (Orb: {:.2}°, {})\n", a.planet1, a.planet2, a.aspect_type, a.orb, if a.is_applying { "Applying" } else { "Separating" }));
        }
    }
    out.push('\n');

    if is_whole_sign {
        if let Some(ref h) = chart.western.hellenistic { append_hellenistic_details(out, h); }
    } else if let Some(ref trad) = chart.western.traditional {
        append_medieval_details(out, trad, is_horary);
    }
}

fn append_predictive_returns(out: &mut String, suite: &PredictiveSuite) {
    if suite.planetary_returns.is_empty() {
        return;
    }
    out.push_str("\n### Solar & Planetary Returns\n");
    for ret in &suite.planetary_returns {
        let asc_sign = ZODIAC_SIGNS[(ret.ascendant / 30.0).floor() as usize % 12];
        let mc_sign = ZODIAC_SIGNS[(ret.midheaven / 30.0).floor() as usize % 12];
        out.push_str(&format!(
            "- **{} Return ({})**: Exact Moment `{}` | Return ASC {:.2}° ({}), Return MC {:.2}° ({})\n",
            ret.return_body, ret.target_year, ret.return_date_utc, ret.ascendant, asc_sign, ret.midheaven, mc_sign
        ));
        let pl_items: Vec<String> = ret.return_planets.iter().map(|p| {
            let h_num = fortune_core::western::determine_house(p.progressed_longitude, &ret.house_cusps);
            format!("{}: {:.2}° {} (H{:02}{})", p.name, p.sign_degree, ZODIAC_SIGNS[p.sign_index as usize % 12], h_num, if p.is_retrograde { " Rx" } else { "" })
        }).collect();
        if !pl_items.is_empty() {
            out.push_str(&format!("  * Return Placements: {}\n", pl_items.join(", ")));
        }
        if !ret.natal_aspects.is_empty() {
            let asp_items: Vec<String> = ret.natal_aspects.iter()
                .map(|a| format!("Return {} {} Natal {} (Orb {:.2}°)", a.return_point, a.aspect_type, a.natal_point, a.orb))
                .collect();
            out.push_str(&format!("  * Return-to-Natal Aspects: {}\n", asp_items.join(", ")));
        }
    }
}

fn append_solar_arc_and_progressions(out: &mut String, suite: &PredictiveSuite) {
    out.push_str(&format!("\n### Solar Arc Directions (Arc Offset: +{:.2}°)\n", suite.solar_arc.arc_degrees));
    for p in &suite.solar_arc.directed_planets {
        out.push_str(&format!("- SA {}: {:.2}° ({}) [Natal: {:.2}°]\n", p.name, p.sign_degree, ZODIAC_SIGNS[p.sign_index as usize % 12], p.natal_longitude));
    }
    if !suite.solar_arc.contacts.is_empty() {
        out.push_str(&format!("- Solar Arc Contacts (Orb ≤ {:.1}°):\n", PREDICTIVE_CONTACT_ORB_DEG));
        for c in &suite.solar_arc.contacts {
            out.push_str(&format!("  * SA {} {} Natal {} (Orb: {:.2}°)\n", c.moving_body, c.aspect_type, c.natal_body, c.orb));
        }
    }
    if let Some(ref sp) = suite.secondary_progression {
        out.push_str(&format!("\n### Secondary Progressions (Ephemeris Date: {}, Phase: {})\n", sp.progressed_date_utc, sp.progressed_lunar_phase));
        for p in &sp.planets {
            out.push_str(&format!("- Progressed {}: {:.2}° ({}){}\n", p.name, p.sign_degree, ZODIAC_SIGNS[p.sign_index as usize % 12], if p.is_retrograde { " [Rx]" } else { "" }));
        }
        if !sp.contacts.is_empty() {
            out.push_str(&format!("- Progressed-to-Natal Contacts (Orb ≤ {:.1}°):\n", PREDICTIVE_CONTACT_ORB_DEG));
            for c in &sp.contacts {
                out.push_str(&format!("  * Progressed {} {} Natal {} (Orb: {:.2}°)\n", c.moving_body, c.aspect_type, c.natal_body, c.orb));
            }
        }
    }
}

fn append_transits_and_directions(out: &mut String, suite: &PredictiveSuite) {
    if !suite.transits.is_empty() {
        out.push_str("\n### Active Transits to Natal (Orb ≤ 2.0°)\n");
        for t in &suite.transits {
            out.push_str(&format!("- Transit {} ({:.2}°) {} Natal {} ({:.2}°) (Orb: {:.2}°)\n", t.transiting_planet, t.transiting_longitude, t.aspect_type, t.natal_planet, t.natal_longitude, t.orb));
        }
    }
    if !suite.primary_directions.is_empty() {
        out.push_str("\n### Primary Directions (Naibod Key)\n");
        for pd in &suite.primary_directions {
            out.push_str(&format!("- {} {} {} (Arc: {:.2}°, Age: {:.1})\n", pd.promittor, pd.aspect_type, pd.significator, pd.direction_arc_deg, pd.realization_age));
        }
    }
}

pub fn append_western_predictive_details(out: &mut String, suite: &PredictiveSuite) {
    out.push_str(&format!(
        "## 4. Predictive Astrology Analysis (Target: {:04}-{:02}-{:02} {:02}:{:02}, Age {})\n",
        suite.target_year, suite.target_month, suite.target_day, suite.target_hour, suite.target_minute, suite.age
    ));
    let prof = &suite.profection;
    let occ_str = if prof.active_natal_planets.is_empty() { "None".to_string() } else { prof.active_natal_planets.join(", ") };
    out.push_str(&format!(
        "\n### Annual Profection\n- Target Year: {} (Age {})\n- Profected House: House {:02} ({})\n- Lord of the Year (Time Lord): {}\n- Natal Occupants in Profected House: {}\n",
        prof.target_year, prof.target_age, prof.profected_house, prof.profected_sign_name, prof.lord_of_the_year, occ_str
    ));
    append_predictive_returns(out, suite);
    append_solar_arc_and_progressions(out, suite);
    append_transits_and_directions(out, suite);
    out.push('\n');
}
