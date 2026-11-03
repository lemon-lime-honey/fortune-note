use crate::models::{HoraryQuestion, Profile};
use fortune_core::FortuneCalculationOutput;

const ZODIAC_SIGNS: [&str; 12] = [
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces",
];

const VEDIC_SIGNS: [&str; 12] = [
    "Mesha (Aries)", "Vrishabha (Taurus)", "Mithuna (Gemini)", "Karka (Cancer)",
    "Simha (Leo)", "Kanya (Virgo)", "Tula (Libra)", "Vrishchika (Scorpio)",
    "Dhanu (Sagittarius)", "Makara (Capricorn)", "Kumbha (Aquarius)", "Meena (Pisces)",
];

fn append_profile_meta(out: &mut String, profile: &Profile, chart: &FortuneCalculationOutput, header: &str) {
    out.push_str(&format!("# {}: {}\n\n", header, profile.name));
    out.push_str("## 1. 인물 기본 정보\n");
    out.push_str(&format!("- 성별: {}\n", if profile.gender == "male" { "남성" } else { "여성" }));
    out.push_str(&format!(
        "- 출생 일시: {:04}-{:02}-{:02} {:02}:{:02}:{:02} (현지 기준)\n",
        profile.birth_year,
        profile.birth_month,
        profile.birth_day,
        profile.birth_hour,
        profile.birth_minute,
        profile.birth_second as u32
    ));
    out.push_str(&format!("- 관측 위치: 위도 {:.4}°, 경도 {:.4}°\n", profile.latitude, profile.longitude));
    out.push_str(&format!(
        "- 시간대 보정: {} (UTC {:+0.1}시간)\n",
        chart.timezone_id, chart.utc_offset_hours
    ));
    out.push_str(&format!(
        "- UTC 변환 일시: {:04}-{:02}-{:02} {:02}:{:02}:{:02}\n",
        chart.utc_year, chart.utc_month, chart.utc_day, chart.utc_hour, chart.utc_minute, chart.utc_second as u32
    ));
    if !profile.notes.is_empty() {
        out.push_str(&format!("- 비고: {}\n", profile.notes));
    }
    out.push('\n');
}

/// 사주명리 전용 마크다운 리포트 및 프롬프트 생성
pub fn format_profile_saju_markdown(profile: &Profile, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_profile_meta(&mut out, profile, chart, "사주명리 분석 데이터");

    out.push_str("## 2. 사주팔자 명식 (Four Pillars of Destiny)\n");
    out.push_str(&format!(
        "- 명식: 연주 [{} ({})], 월주 [{} ({})], 일주 [{} ({})], 시주 [{} ({})]\n",
        chart.saju.year_pillar.korean(), chart.saju.year_pillar.hanja(),
        chart.saju.month_pillar.korean(), chart.saju.month_pillar.hanja(),
        chart.saju.day_pillar.korean(), chart.saju.day_pillar.hanja(),
        chart.saju.hour_pillar.korean(), chart.saju.hour_pillar.hanja()
    ));
    out.push_str(&format!(
        "- 대운 방향: {} (대운수: {})\n",
        if chart.saju.is_forward_daeun { "순행" } else { "역행" },
        chart.saju.daeun_number
    ));
    out.push_str("- 10년 주기 대운 흐름:\n");
    for d in &chart.saju.daeun_periods {
        out.push_str(&format!(
            "  * {}세 ~ {}세: [{} ({})]\n",
            d.start_age,
            d.start_age + 9,
            d.pillar.korean(),
            d.pillar.hanja()
        ));
    }
    out.push('\n');

    out
}

fn append_western_details(out: &mut String, chart: &FortuneCalculationOutput, is_horary: bool) {
    let asc_sign = (chart.western.ascendant / 30.0).floor() as usize % 12;
    let mc_sign = (chart.western.midheaven / 30.0).floor() as usize % 12;
    out.push_str("## 2. Celestial Placements & House Cusps\n");
    let is_whole_sign = chart.western.house_system == fortune_core::WesternHouseSystem::WholeSign;
    let mc_house_desc = if is_whole_sign {
        let mc_h = fortune_core::western::determine_house(chart.western.midheaven, &chart.western.house_cusps);
        format!(" [Located in House {:02}]", mc_h)
    } else {
        String::new()
    };

    out.push_str(&format!(
        "- Ascendant (ASC): {:.2}° ({})\n",
        chart.western.ascendant, ZODIAC_SIGNS[asc_sign]
    ));
    out.push_str(&format!(
        "- Midheaven (MC): {:.2}° ({}){}\n",
        chart.western.midheaven, ZODIAC_SIGNS[mc_sign], mc_house_desc
    ));
    out.push_str(&format!("- House System: {}\n", chart.western.house_system.display_name()));
    if is_whole_sign {
        out.push_str("- House Signs (Whole Sign Houses):\n");
        let mc_h = fortune_core::western::determine_house(chart.western.midheaven, &chart.western.house_cusps);
        for (i, cusp) in chart.western.house_cusps.iter().enumerate() {
            let sign_idx = (cusp / 30.0).floor() as usize % 12;
            let ruler = fortune_core::western::dignity::DOMICILE_RULERS[sign_idx].name();
            let mc_tag = if (i + 1) as u8 == mc_h { " [MC Location]" } else { "" };
            out.push_str(&format!(
                "  * House {:02}{}: {} (Ruler: {})\n",
                i + 1, mc_tag, ZODIAC_SIGNS[sign_idx], ruler
            ));
        }
    } else {
        out.push_str("- House Cusps (12 Cusps):\n");
        for (i, cusp) in chart.western.house_cusps.iter().enumerate() {
            let sign_idx = (cusp / 30.0).floor() as usize % 12;
            let sign_deg = cusp - (sign_idx as f64 * 30.0);
            let tag = match i + 1 {
                1 => " (ASC)",
                4 => " (IC)",
                7 => " (DSC)",
                10 => " (MC)",
                _ => "",
            };
            out.push_str(&format!(
                "  * House {:02}{}: {:.2}° ({})\n",
                i + 1, tag, sign_deg, ZODIAC_SIGNS[sign_idx]
            ));
        }
    }
    out.push_str("- Planetary Placements:\n");
    for p in &chart.western.planets {
        let sign_name = ZODIAC_SIGNS[p.sign_index as usize % 12];
        let retro_str = if p.is_retrograde { " [Rx]" } else { "" };
        out.push_str(&format!(
            "  * {}: {:.2}° ({}) - House {}{} (Speed: {:+0.2}°/day)\n",
            p.name, p.longitude, sign_name, p.house_number, retro_str, p.speed
        ));
    }
    out.push_str("- Lunar Nodes:\n");
    for n in &chart.western.nodes {
        let sign_name = ZODIAC_SIGNS[n.sign_index as usize % 12];
        out.push_str(&format!(
            "  * {}: {:.2}° ({}) - House {}\n",
            n.name, n.longitude, sign_name, n.house_number
        ));
    }
    out.push_str("- Major Aspects:\n");
    if chart.western.aspects.is_empty() {
        out.push_str("  * No major aspects within orb\n");
    } else {
        for a in &chart.western.aspects {
            let dynamic_str = if a.is_applying { "Applying" } else { "Separating" };
            out.push_str(&format!(
                "  * {} - {}: {} (Orb: {:.2}°, {})\n",
                a.planet1, a.planet2, a.aspect_type, a.orb, dynamic_str
            ));
        }
    }
    out.push('\n');

    if is_whole_sign {
        if let Some(ref h) = chart.western.hellenistic {
            out.push_str("## 3. Hellenistic Traditional Analysis (Whole Sign)\n");

            out.push_str("### Independent Angles & Planetary Action Points\n");
            let asc_sign = ZODIAC_SIGNS[h.angles.ascendant_sign as usize % 12];
            let mc_sign = ZODIAC_SIGNS[h.angles.midheaven_sign as usize % 12];
            let mc_house_note = if h.angles.is_mc_in_10th {
                "House 10 (Culmination)".to_string()
            } else {
                format!("House {:02} (Special Action Point)", h.angles.midheaven_house)
            };
            out.push_str(&format!("- Ascendant (ASC): {:.2}° ({})\n", h.angles.ascendant_degree, asc_sign));
            out.push_str(&format!("- Midheaven (MC): {:.2}° ({}) - {}\n", h.angles.midheaven_degree, mc_sign, mc_house_note));

            out.push_str("\n### Qualitative Planet Conditions, Egyptian Bounds & Dorothean Triplicity\n");
            out.push_str("| Planet | Status | House | Bound Ruler | Triplicity (Day/Night/Part) | In Triplicity | Lord of ASC |\n");
            out.push_str("| :--- | :--- | :---: | :---: | :---: | :---: | :---: |\n");
            for pc in &h.planet_conditions {
                let asc_tag = if pc.is_ruler_of_asc { "Yes" } else { "-" };
                let trip_tag = if pc.is_in_triplicity { "Yes" } else { "-" };
                let bound = if pc.bound_ruler.is_empty() { "-" } else { pc.bound_ruler.as_str() };
                let trip_rulers = if pc.triplicity_day_ruler.is_empty() {
                    "-".to_string()
                } else {
                    format!("{}/{}/{}", pc.triplicity_day_ruler, pc.triplicity_night_ruler, pc.triplicity_participating_ruler)
                };
                out.push_str(&format!(
                    "| **{}** | {} | H{:02} | {} | {} | {} | {} |\n",
                    pc.name, pc.condition_label, pc.house_number, bound, trip_rulers, trip_tag, asc_tag
                ));
            }

            if !h.lots.is_empty() {
                out.push_str("\n### Hermetic Lots (Paulus Alexandrinus)\n");
                for lot in &h.lots {
                    let sign_name = ZODIAC_SIGNS[lot.sign_index as usize % 12];
                    out.push_str(&format!(
                        "- {}: {:.2}° ({}) - House {:02} (Ruler: {})\n",
                        lot.name, lot.longitude, sign_name, lot.house_number, lot.ruler
                    ));
                }
            }

            if !h.aspects.is_empty() {
                out.push_str("\n### Sign-Based Aspects & Degree Contacts\n");
                for ta in &h.aspects {
                    let dynamic_str = if ta.is_applying { "Applying" } else { "Separating" };
                    let partile_str = if ta.is_partile { " [Partile <= 1°]" } else { "" };
                    out.push_str(&format!(
                        "- {} - {}: {} (Orb: {:.2}°){}, {}\n",
                        ta.planet1, ta.planet2, ta.aspect_type, ta.orb, partile_str, dynamic_str
                    ));
                }
            }

            if let Some(ref san) = h.prenatal_syzygy {
                let san_sign = ZODIAC_SIGNS[san.sign_index as usize % 12];
                out.push_str("\n### Prenatal Syzygy (SAN)\n");
                out.push_str(&format!(
                    "- Type: {}, Longitude: {:.2}° ({}), Moment: {}\n",
                    san.syzygy_type, san.longitude, san_sign, san.exact_datetime_utc
                ));
            }

            if !h.fixed_star_conjunctions.is_empty() {
                out.push_str("\n### Fixed Star Conjunctions (Orb ≤ 1.5°)\n");
                for sc in &h.fixed_star_conjunctions {
                    let status_tag = if sc.is_qualified {
                        format!(" [Qualified: {}]", sc.qualification_reason)
                    } else {
                        format!(" [Filtered: {}]", sc.qualification_reason)
                    };
                    let vis_tag = if sc.is_visible {
                        format!(" [Visible (Culm: {:+.1}°)]", sc.max_altitude)
                    } else {
                        format!(" [Below Horizon (Culm: {:+.1}°)]", sc.max_altitude)
                    };
                    out.push_str(&format!(
                        "- {} ☌ {} (Orb: {:.2}°, Lat: {:+.1}°, Mag: {:.2}, Nature: {}){}{}: {}\n",
                        sc.star_name, sc.body_name, sc.orb, sc.star_latitude, sc.star_magnitude, sc.nature, status_tag, vis_tag, sc.meaning
                    ));
                }
            }
            out.push('\n');
        }
    } else if let Some(ref trad) = chart.western.traditional {
        out.push_str("## 3. Medieval Traditional Analysis (Regiomontanus)\n");
        if is_horary {
            out.push_str(&format!(
                "- Planetary Hours: Day of {}, Hour of {} (Radicality: {} - {})\n",
                trad.planetary_hours.day_ruler,
                trad.planetary_hours.hour_ruler,
                if trad.planetary_hours.is_radical { "Radical" } else { "Non-radical" },
                trad.planetary_hours.radicality_reason
            ));
            out.push_str(&format!(
                "- Moon Condition: Phase [{}], VOC [{}], Via Combusta [{}]\n",
                trad.moon_condition.lunar_phase_name,
                if trad.moon_condition.is_void_of_course { "Void of Course" } else { "None" },
                if trad.moon_condition.is_via_combusta { "Via Combusta (15° Lib - 15° Sco)" } else { "None" },
            ));
            if let Some(ref next_asp) = trad.moon_condition.next_applying_aspect {
                out.push_str(&format!("  * Next Applying Aspect: {}\n", next_asp));
            }
        } else {
            out.push_str(&format!(
                "- Planetary Hours: Day of {}, Hour of {} (Hour {}, {})\n",
                trad.planetary_hours.day_ruler,
                trad.planetary_hours.hour_ruler,
                trad.planetary_hours.hour_number,
                if trad.planetary_hours.is_day_hour { "Day" } else { "Night" },
            ));
            out.push_str(&format!(
                "- Moon Condition: Phase [{}]\n",
                trad.moon_condition.lunar_phase_name,
            ));
        }
        if let Some(ref san) = trad.prenatal_syzygy {
            let sign_name = ZODIAC_SIGNS[san.sign_index as usize % 12];
            out.push_str(&format!(
                "- Prenatal Syzygy (SAN): {} at {:.2}° ({}) on {}\n",
                san.syzygy_type, san.longitude, sign_name, san.exact_datetime_utc
            ));
        }

        if is_horary && !trad.planetary_hours.considerations.is_empty() {
            out.push_str("- Considerations Before Judgment:\n");
            for c in &trad.planetary_hours.considerations {
                out.push_str(&format!("  * {}\n", c));
            }
        }

        out.push_str("\n### Essential & Accidental Dignities\n");
        out.push_str("| Planet | Longitude | House | Dignity Status | Essential | Accidental | Total Score |\n");
        out.push_str("| :--- | :--- | :---: | :--- | :---: | :---: | :---: |\n");
        for d in &trad.planet_dignities {
            let mut status = Vec::new();
            if d.is_in_domicile { status.push("Domicile (+5)"); }
            if d.is_in_exaltation { status.push("Exaltation (+4)"); }
            if d.is_in_triplicity { status.push("Triplicity (+3)"); }
            if d.is_in_term { status.push("Term (+2)"); }
            if d.is_in_face { status.push("Face (+1)"); }
            if d.is_in_detriment { status.push("Detriment (-5)"); }
            if d.is_in_fall { status.push("Fall (-4)"); }
            if d.is_peregrine { status.push("Peregrine (-5)"); }
            if status.is_empty() { status.push("Neutral"); }

            out.push_str(&format!(
                "| {} | {:.2}° | H{} | {} | {:+} | {:+} | **{:+}** |\n",
                d.planet.name(),
                d.longitude,
                d.house_number,
                status.join(", "),
                d.essential_score,
                d.accidental_score,
                d.total_lilly_score
            ));
        }

        if !trad.traditional_aspects.is_empty() {
            out.push_str("\n### Traditional Dynamic Aspects (Applying / Separating, Moiety Orb)\n");
            for ta in &trad.traditional_aspects {
                let dynamic_str = if ta.is_applying { "Applying" } else { "Separating" };
                let partile_str = if ta.is_partile { " [Partile]" } else { "" };
                out.push_str(&format!(
                    "- {} - {}: {} (Orb: {:.2}° / Max: {:.2}°){}, {}\n",
                    ta.planet1, ta.planet2, ta.aspect_type, ta.orb, ta.max_allowed_orb, partile_str, dynamic_str
                ));
            }
        }

        if !trad.fixed_star_conjunctions.is_empty() {
            out.push_str("\n### Fixed Star Conjunctions (Orb ≤ 1.5°)\n");
            for sc in &trad.fixed_star_conjunctions {
                let status_tag = if sc.is_qualified {
                    format!(" [Qualified: {}]", sc.qualification_reason)
                } else {
                    format!(" [Filtered: {}]", sc.qualification_reason)
                };
                let vis_tag = if sc.is_visible {
                    format!(" [Visible (Culm: {:+.1}°)]", sc.max_altitude)
                } else {
                    format!(" [Below Horizon (Culm: {:+.1}°)]", sc.max_altitude)
                };
                out.push_str(&format!(
                    "- {} ☌ {} (Orb: {:.2}°, Lat: {:+.1}°, Mag: {:.2}, Nature: {}){}{}: {}\n",
                    sc.star_name, sc.body_name, sc.orb, sc.star_latitude, sc.star_magnitude, sc.nature, status_tag, vis_tag, sc.meaning
                ));
            }
        }

        if !trad.cusp_dignities.is_empty() {
            out.push_str("\n### House Cusp Dignities & Almutens\n");
            out.push_str("| House | Cusp Degree | Domicile | Exaltation | Triplicity | Term | Face | Almuten (Score) |\n");
            out.push_str("| :---: | :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");
            for cd in &trad.cusp_dignities {
                let sign_name = ZODIAC_SIGNS[cd.sign_index as usize % 12];
                let exalt_str = cd.exaltation.as_deref().unwrap_or("-");
                out.push_str(&format!(
                    "| H{:02} | {:.2}° ({}) | {} | {} | {} | {} | {} | **{}** ({}) |\n",
                    cd.house_number, cd.sign_degree, sign_name,
                    cd.domicile, exalt_str, cd.triplicity, cd.term, cd.face,
                    cd.almuten, cd.almuten_score
                ));
            }
        }

        if !trad.mutual_receptions.is_empty() {
            out.push_str("\n### Mutual Receptions\n");
            for mr in &trad.mutual_receptions {
                out.push_str(&format!("- [{}] {}\n", mr.reception_type, mr.description));
            }
        }

        let has_perfection = !trad.perfection.translation_of_light.is_empty()
            || !trad.perfection.collection_of_light.is_empty()
            || !trad.perfection.besiegement.is_empty();

        if is_horary && has_perfection {
            out.push_str("\n### Horary Perfection & Besiegement\n");
            for tol in &trad.perfection.translation_of_light {
                out.push_str(&format!("- [Translation of Light] {}\n", tol));
            }
            for col in &trad.perfection.collection_of_light {
                out.push_str(&format!("- [Collection of Light] {}\n", col));
            }
            for bsg in &trad.perfection.besiegement {
                out.push_str(&format!("- [Besiegement] {}\n", bsg));
            }
        }

        if !trad.antiscia.is_empty() {
            out.push_str("\n### Antiscia & Contra-Antiscia (Solstice Reflection)\n");
            for a in &trad.antiscia {
                let ant_sign = ZODIAC_SIGNS[a.antiscion_sign_index as usize % 12];
                out.push_str(&format!(
                    "- {}: Antiscion {:.2}° ({}) | Contra-Antiscion {:.2}°\n",
                    a.body_name, a.antiscion_degree, ant_sign, a.contra_antiscion_longitude
                ));
            }
        }

        if !trad.arabic_parts.is_empty() {
            out.push_str("\n### Arabic Parts\n");
            for part in &trad.arabic_parts {
                let sign_name = ZODIAC_SIGNS[part.sign_index as usize % 12];
                out.push_str(&format!(
                    "- {}: {:.2}° ({}) - House {} (Ruler: {})\n",
                    part.name, part.longitude, sign_name, part.house_number, part.ruler
                ));
            }
        }
        out.push('\n');
    }
}

pub fn append_western_predictive_details(
    out: &mut String,
    suite: &fortune_core::western::predictive::PredictiveSuite,
) {
    out.push_str(&format!(
        "## 4. Predictive Astrology Analysis (Target: {:04}-{:02}-{:02} {:02}:{:02}, Age {})\n",
        suite.target_year, suite.target_month, suite.target_day, suite.target_hour, suite.target_minute, suite.age
    ));

    // 1. Annual Profection
    let prof = &suite.profection;
    let occ_str = if prof.active_natal_planets.is_empty() {
        "None".to_string()
    } else {
        prof.active_natal_planets.join(", ")
    };
    out.push_str("\n### Annual Profection\n");
    out.push_str(&format!(
        "- Target Year: {} (Age {})\n- Profected House: House {:02} ({})\n- Lord of the Year (Time Lord): {}\n- Natal Occupants in Profected House: {}\n",
        prof.target_year, prof.target_age, prof.profected_house, prof.profected_sign_name, prof.lord_of_the_year, occ_str
    ));

    // 2. Solar & Planetary Returns
    if !suite.planetary_returns.is_empty() {
        out.push_str("\n### Solar & Planetary Returns\n");
        for ret in &suite.planetary_returns {
            let asc_sign = ZODIAC_SIGNS[(ret.ascendant / 30.0).floor() as usize % 12];
            let mc_sign = ZODIAC_SIGNS[(ret.midheaven / 30.0).floor() as usize % 12];
            out.push_str(&format!(
                "- **{} Return ({})**: Exact Moment `{}` | Return ASC {:.2}° ({}), Return MC {:.2}° ({})\n",
                ret.return_body, ret.target_year, ret.return_date_utc, ret.ascendant, asc_sign, ret.midheaven, mc_sign
            ));
            let mut pl_items = Vec::new();
            for p in &ret.return_planets {
                let s_name = ZODIAC_SIGNS[p.sign_index as usize % 12];
                let h_num = fortune_core::western::determine_house(p.progressed_longitude, &ret.house_cusps);
                let rx = if p.is_retrograde { " Rx" } else { "" };
                pl_items.push(format!("{}: {:.2}° {} (H{:02}{})", p.name, p.sign_degree, s_name, h_num, rx));
            }
            if !pl_items.is_empty() {
                out.push_str(&format!("  * Return Placements: {}\n", pl_items.join(", ")));
            }
            if !ret.natal_aspects.is_empty() {
                let asp_items: Vec<String> = ret
                    .natal_aspects
                    .iter()
                    .map(|a| format!("Return {} {} Natal {} (Orb {:.2}°)", a.return_point, a.aspect_type, a.natal_point, a.orb))
                    .collect();
                out.push_str(&format!("  * Return-to-Natal Aspects: {}\n", asp_items.join(", ")));
            }
        }
    }

    // 3. Solar Arc & Secondary Progressions
    out.push_str(&format!(
        "\n### Solar Arc Directions (Arc Offset: +{:.2}°)\n",
        suite.solar_arc.arc_degrees
    ));
    for p in &suite.solar_arc.directed_planets {
        let s_name = ZODIAC_SIGNS[p.sign_index as usize % 12];
        out.push_str(&format!(
            "- SA {}: {:.2}° ({}) [Natal: {:.2}°]\n",
            p.name, p.sign_degree, s_name, p.natal_longitude
        ));
    }
    if !suite.solar_arc.contacts.is_empty() {
        out.push_str("- Solar Arc Contacts (Orb ≤ 1.5°):\n");
        for c in &suite.solar_arc.contacts {
            out.push_str(&format!(
                "  * SA {} {} Natal {} (Orb: {:.2}°)\n",
                c.moving_body, c.aspect_type, c.natal_body, c.orb
            ));
        }
    }

    if let Some(ref sp) = suite.secondary_progression {
        out.push_str(&format!(
            "\n### Secondary Progressions (Ephemeris Date: {}, Phase: {})\n",
            sp.progressed_date_utc, sp.progressed_lunar_phase
        ));
        for p in &sp.planets {
            let s_name = ZODIAC_SIGNS[p.sign_index as usize % 12];
            let rx = if p.is_retrograde { " [Rx]" } else { "" };
            out.push_str(&format!(
                "- Progressed {}: {:.2}° ({}){}\n",
                p.name, p.sign_degree, s_name, rx
            ));
        }
        if !sp.contacts.is_empty() {
            out.push_str("- Progressed-to-Natal Contacts (Orb ≤ 1.5°):\n");
            for c in &sp.contacts {
                out.push_str(&format!(
                    "  * Progressed {} {} Natal {} (Orb: {:.2}°)\n",
                    c.moving_body, c.aspect_type, c.natal_body, c.orb
                ));
            }
        }
    }

    // 4. Transits & Primary Directions
    if !suite.transits.is_empty() {
        out.push_str("\n### Active Transits to Natal (Orb ≤ 2.0°)\n");
        for t in &suite.transits {
            out.push_str(&format!(
                "- Transit {} ({:.2}°) {} Natal {} ({:.2}°) (Orb: {:.2}°)\n",
                t.transiting_planet, t.transiting_longitude, t.aspect_type, t.natal_planet, t.natal_longitude, t.orb
            ));
        }
    }

    if !suite.primary_directions.is_empty() {
        out.push_str("\n### Primary Directions (Naibod Key)\n");
        for pd in &suite.primary_directions {
            out.push_str(&format!(
                "- {} {} {} (Arc: {:.2}°, Age: {:.1})\n",
                pd.promittor, pd.aspect_type, pd.significator, pd.direction_arc_deg, pd.realization_age
            ));
        }
    }
    out.push('\n');
}

/// 서양 점성술 전용 마크다운 리포트 생성
pub fn format_profile_western_markdown(profile: &Profile, chart: &FortuneCalculationOutput) -> String {
    format_profile_western_markdown_with_predictive(profile, chart, None)
}

/// 서양 점성술 전용 마크다운 리포트 생성 (예측 기법 포함)
pub fn format_profile_western_markdown_with_predictive(
    profile: &Profile,
    chart: &FortuneCalculationOutput,
    suite: Option<&fortune_core::western::predictive::PredictiveSuite>,
) -> String {
    let mut out = String::new();
    append_profile_meta(&mut out, profile, chart, "서양 점성술 (Western Tropical Natal Chart)");
    append_western_details(&mut out, chart, false);
    if let Some(s) = suite {
        append_western_predictive_details(&mut out, s);
    }
    out
}

fn jd_to_date_string(jd: f64) -> String {
    let z = (jd + 0.5).floor() as i64;
    let a = if z < 2_299_161 {
        z
    } else {
        let alpha = ((z as f64 - 1_867_216.25) / 36_524.25).floor() as i64;
        z + 1 + alpha - (alpha / 4)
    };
    let b = a + 1524;
    let c = ((b as f64 - 122.1) / 365.25).floor() as i64;
    let d = (365.25 * c as f64).floor() as i64;
    let e = ((b - d) as f64 / 30.6001).floor() as i64;
    let day = b - d - (30.6001 * e as f64).floor() as i64;
    let month = if e < 14 { e - 1 } else { e - 13 };
    let year = if month > 2 { c - 4716 } else { c - 4715 };
    format!("{:04}-{:02}-{:02}", year, month, day)
}

fn append_vedic_details(out: &mut String, chart: &FortuneCalculationOutput, is_prashna: bool) {
    if is_prashna {
        out.push_str("## 2. Prashna Chart Placements (Vedic Sidereal)\n");
    } else {
        out.push_str("## 2. 라시 차트 및 분점도 (Vedic Sidereal & Vargas)\n");
    }
    let lagna_sign_idx = chart.vedic.lagna_sign as usize % 12;
    let lagna_label = if is_prashna { "Prashna Lagna" } else { "Lagna" };
    out.push_str(&format!(
        "- Ayanamsha: {} ({:.4}°)\n",
        chart.vedic.ayanamsha_name, chart.vedic.ayanamsha_deg
    ));
    out.push_str(&format!(
        "- {}: {:.2}° ({})\n",
        lagna_label, chart.vedic.lagna_sidereal, VEDIC_SIGNS[lagna_sign_idx]
    ));
    out.push_str("- D1 Rasi Placements:\n");
    for p in &chart.vedic.planets {
        let sign_name = VEDIC_SIGNS[p.sign_index as usize % 12];
        let retro_str = if p.is_retrograde { " [Rx]" } else { "" };
        let comb_str = if p.combustion != "None" && !p.combustion.is_empty() {
            format!(" [{}]", p.combustion)
        } else {
            String::new()
        };
        out.push_str(&format!(
            "  * {}: {:.2}° (H{}, {} {:.2}°, {} Pada {}, Dignity: {}){}{}\n",
            p.name,
            p.sidereal_longitude,
            p.house,
            sign_name,
            p.sign_degree,
            p.nakshatra_name,
            p.nakshatra_pada,
            p.dignity,
            retro_str,
            comb_str
        ));
    }

    let pan = &chart.vedic.panchanga;
    if !pan.tithi_name.is_empty() {
        out.push_str("- Panchanga (5 Limbs of Time):\n");
        out.push_str(&format!(
            "  * Tithi: {} (#{}, {})\n",
            pan.tithi_name, pan.tithi_number, pan.paksha
        ));
        out.push_str(&format!("  * Vara: {} (Lord: {})\n", pan.vara_name, pan.vara_lord));
        out.push_str(&format!(
            "  * Nakshatra: {} Pada {} (Lord: {})\n",
            pan.nakshatra_name, pan.nakshatra_pada, pan.nakshatra_lord
        ));
        out.push_str(&format!("  * Yoga: {}\n", pan.yoga_name));
        out.push_str(&format!("  * Karana: {}\n", pan.karana_name));
    }

    if !is_prashna && !chart.vedic.chara_karakas.is_empty() {
        out.push_str("- Jaimini Chara Karakas (7-Karaka Scheme):\n");
        for ck in &chart.vedic.chara_karakas {
            out.push_str(&format!(
                "  * {}: {} ({:.2}°)\n",
                ck.karaka, ck.planet, ck.effective_degree
            ));
        }
    }

    if !chart.vedic.arudha_padas.is_empty() {
        out.push_str("- Arudha Padas (BPHS Ch. 29):\n");
        for ap in &chart.vedic.arudha_padas {
            out.push_str(&format!(
                "  * {} ({}): {} (H{} from Lagna)\n",
                ap.code, ap.name, ap.sign_name, ap.house_from_lagna
            ));
        }
    }

    let notable_drishti: Vec<_> = chart
        .vedic
        .drishti
        .iter()
        .filter(|d| !d.aspected_planets.is_empty() || d.aspect_offset != 7)
        .collect();
    if !notable_drishti.is_empty() {
        out.push_str("- Graha Drishti (Full Aspects incl. Special Bhava Aspects):\n");
        for d in notable_drishti {
            let target_sign_name = VEDIC_SIGNS[d.target_sign as usize % 12];
            let targets_str = if d.aspected_planets.is_empty() {
                "Empty Bhava".to_string()
            } else {
                d.aspected_planets.join(", ")
            };
            out.push_str(&format!(
                "  * {} (H{}) -> {}th Drishti on H{} ({}): {}\n",
                d.from_planet,
                d.from_house,
                d.aspect_offset,
                d.target_house,
                target_sign_name,
                targets_str
            ));
        }
    }

    if !is_prashna && chart.vedic.ashtakavarga.total_bindus > 0 {
        out.push_str("- Ashtakavarga (Sarvashtakavarga & Bhinna Totals):\n");
        let mut sav_parts = Vec::with_capacity(12);
        for (idx, &pts) in chart.vedic.ashtakavarga.sarva.iter().enumerate() {
            sav_parts.push(format!("{}: {}", VEDIC_SIGNS[idx], pts));
        }
        out.push_str(&format!(
            "  * Sarvashtakavarga (Total {}): {}\n",
            chart.vedic.ashtakavarga.total_bindus,
            sav_parts.join(", ")
        ));
        for row in &chart.vedic.ashtakavarga.bhinna {
            out.push_str(&format!(
                "  * Bhinna {} (Total {}): {:?}\n",
                row.planet, row.total, row.bindus
            ));
        }
    }

    out.push_str("- Divisional Charts (Vargas):\n");
    for (division, v) in &chart.vedic.vargas {
        if is_prashna
            && !matches!(
                *division,
                fortune_core::VargaDivision::D1
                    | fortune_core::VargaDivision::D3
                    | fortune_core::VargaDivision::D9
            )
        {
            continue;
        }
        let sign_name = VEDIC_SIGNS[v.lagna_sign as usize % 12];
        out.push_str(&format!(
            "  * {}: Lagna {}\n",
            division.title(), sign_name
        ));
        let mut placements_str = Vec::new();
        for p in &v.placements {
            let p_sign_name = VEDIC_SIGNS[p.sign_index as usize % 12];
            if *division == fortune_core::VargaDivision::D1 {
                placements_str.push(format!(
                    "{}: H{} {} {:.2}°",
                    p.planet_name, p.house, p_sign_name, p.sign_degree
                ));
            } else {
                placements_str.push(format!(
                    "{}: H{} {}",
                    p.planet_name, p.house, p_sign_name
                ));
            }
        }
        if !placements_str.is_empty() {
            out.push_str(&format!("    - Placements: {}\n", placements_str.join(", ")));
        }
    }

    if !is_prashna {
        out.push_str("- Vimshottari Dasha Sequence:\n");
        for d in &chart.vedic.vimshottari_dashas {
            out.push_str(&format!(
                "  * Mahadasha {}: {} ~ {}\n",
                d.lord,
                jd_to_date_string(d.start_jd),
                jd_to_date_string(d.end_jd)
            ));
            for sub in &d.sub_periods {
                out.push_str(&format!(
                    "    - Antardasha {}: {} ~ {}\n",
                    sub.lord,
                    jd_to_date_string(sub.start_jd),
                    jd_to_date_string(sub.end_jd)
                ));
            }
        }
    }
    out.push('\n');
}

/// 베다 점성술 전용 마크다운 리포트 및 프롬프트 생성
pub fn format_profile_vedic_markdown(profile: &Profile, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_profile_meta(&mut out, profile, chart, "베다 점성술 분석 데이터");
    append_vedic_details(&mut out, chart, false);
    out
}

fn append_horary_meta(out: &mut String, horary: &HoraryQuestion, chart: &FortuneCalculationOutput, header: &str) {
    out.push_str(&format!("# {}: {}\n\n", header, horary.title));
    out.push_str("## 1. 질문 메타 정보\n");
    out.push_str(&format!("- 분류: {}\n", horary.category));
    out.push_str(&format!("- 질문 본문: {}\n", horary.question_text));
    out.push_str(&format!(
        "- 질문 발생 일시: {:04}-{:02}-{:02} {:02}:{:02}:{:02} (현지 기준)\n",
        horary.query_year, horary.query_month, horary.query_day, horary.query_hour, horary.query_minute, horary.query_second as u32
    ));
    out.push_str(&format!("- 질문 장소 좌표: 위도 {:.4}°, 경도 {:.4}°\n", horary.latitude, horary.longitude));
    out.push_str(&format!("- 시간대: {} (UTC {:+0.1}시간)\n", chart.timezone_id, chart.utc_offset_hours));
    if !horary.result_notes.is_empty() {
        out.push_str(&format!("- 사후 실현 결과 판정: {}\n", horary.result_notes));
    }
    out.push('\n');
}

/// 서양 트로피컬 호라리 전용 마크다운 리포트 생성
pub fn format_horary_western_markdown(horary: &HoraryQuestion, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_horary_meta(&mut out, horary, chart, "호라리 질의 분석 데이터 (Western Tropical)");
    append_western_details(&mut out, chart, true);
    out
}

/// 베다 시데리얼 프라슈나 전용 마크다운 리포트 생성
pub fn format_prashna_vedic_markdown(horary: &HoraryQuestion, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_horary_meta(&mut out, horary, chart, "프라슈나 질의 분석 데이터 (Vedic Sidereal)");
    append_vedic_details(&mut out, chart, true);
    out
}
