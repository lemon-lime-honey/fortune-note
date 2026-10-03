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
        "- 출생 일시: {:04}-{:02}-{:02} {:02}:{:02} (현지 기준)\n",
        profile.birth_year, profile.birth_month, profile.birth_day, profile.birth_hour, profile.birth_minute
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

fn append_western_details(out: &mut String, chart: &FortuneCalculationOutput) {
    let asc_sign = (chart.western.ascendant / 30.0).floor() as usize % 12;
    let mc_sign = (chart.western.midheaven / 30.0).floor() as usize % 12;
    out.push_str("## 2. 천체 및 하우스 기본 배치\n");
    out.push_str(&format!(
        "- Ascendant: {:.2}° ({})\n",
        chart.western.ascendant, ZODIAC_SIGNS[asc_sign]
    ));
    out.push_str(&format!(
        "- Midheaven (MC): {:.2}° ({})\n",
        chart.western.midheaven, ZODIAC_SIGNS[mc_sign]
    ));
    out.push_str(&format!("- 하우스 시스템: {}\n", chart.western.house_system.display_name()));
    out.push_str("- 행성 배치:\n");
    for p in &chart.western.planets {
        let sign_name = ZODIAC_SIGNS[p.sign_index as usize % 12];
        let retro_str = if p.is_retrograde { " [Rx]" } else { "" };
        out.push_str(&format!(
            "  * {}: {:.2}° ({}) - House {}{}\n",
            p.name, p.longitude, sign_name, p.house_number, retro_str
        ));
    }
    out.push_str("- 노드 배치:\n");
    for n in &chart.western.nodes {
        let sign_name = ZODIAC_SIGNS[n.sign_index as usize % 12];
        out.push_str(&format!(
            "  * {}: {:.2}° ({}) - House {}\n",
            n.name, n.longitude, sign_name, n.house_number
        ));
    }
    out.push_str("- 메이저 아스펙트 (Major Aspects):\n");
    if chart.western.aspects.is_empty() {
        out.push_str("  * No major aspects within orb\n");
    } else {
        for a in &chart.western.aspects {
            out.push_str(&format!(
                "  * {} - {}: {} (오차: {:.2}°)\n",
                a.planet1, a.planet2, a.aspect_type, a.orb
            ));
        }
    }
    out.push('\n');

    if let Some(ref trad) = chart.western.traditional {
        out.push_str("## 3. 윌리엄 릴리 전통 점성술 정밀 분석\n");
        out.push_str(&format!(
            "- 플래너터리 아워: {}의 날, {}의 시간 (적격성: {} - {})\n",
            trad.planetary_hours.day_ruler,
            trad.planetary_hours.hour_ruler,
            if trad.planetary_hours.is_radical { "적격 (Radical)" } else { "부적격 (Non-radical)" },
            trad.planetary_hours.radicality_reason
        ));
        out.push_str(&format!(
            "- 달의 상태: 위상 [{}], VOC [{}], Via Combusta [{}]\n",
            trad.moon_condition.lunar_phase_name,
            if trad.moon_condition.is_void_of_course { "보이드 오브 코스 (VOC)" } else { "정상 운행" },
            if trad.moon_condition.is_via_combusta { "연소도 (Via Combusta)" } else { "정상 위치" },
        ));
        if let Some(ref next_asp) = trad.moon_condition.next_applying_aspect {
            out.push_str(&format!("  * 달의 다음 접근 각: {}\n", next_asp));
        }

        if !trad.planetary_hours.considerations.is_empty() {
            out.push_str("- 판단 전 고려사항 (Considerations Before Judgment):\n");
            for c in &trad.planetary_hours.considerations {
                out.push_str(&format!("  * {}\n", c));
            }
        }

        out.push_str("\n### 5중 본질적 품계 및 릴리 위계 총점\n");
        out.push_str("| 행성 | 황경 | 하우스 | 품계 상태 | 본질 점수 | 우발 점수 | 릴리 총점 |\n");
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
            out.push_str("\n### 전통 동적 아스펙트 (접근 / 분리 및 모이어티 오브)\n");
            for ta in &trad.traditional_aspects {
                let dynamic_str = if ta.is_applying { "접근 (Applying, 미래)" } else { "분리 (Separating, 과거)" };
                let partile_str = if ta.is_partile { " [Partile]" } else { "" };
                out.push_str(&format!(
                    "- {} - {}: {} (오차: {:.2}° / 허용: {:.2}°){}, {}\n",
                    ta.planet1, ta.planet2, ta.aspect_type, ta.orb, ta.max_allowed_orb, partile_str, dynamic_str
                ));
            }
        }

        if !trad.fixed_star_conjunctions.is_empty() {
            out.push_str("\n### 주요 항성 회합 (Fixed Star Conjunctions, Orb ≤ 1.5°)\n");
            for sc in &trad.fixed_star_conjunctions {
                out.push_str(&format!(
                    "- {} ({}) ☌ {} (오차: {:.2}°, 본성: {}): {}\n",
                    sc.star_name, sc.korean_name, sc.body_name, sc.orb, sc.nature, sc.meaning
                ));
            }
        }

        if !trad.arabic_parts.is_empty() {
            out.push_str("\n### 주요 아라빅 파트 (가상점)\n");
            for part in &trad.arabic_parts {
                let sign_name = ZODIAC_SIGNS[part.sign_index as usize % 12];
                out.push_str(&format!(
                    "- {}: {:.2}° ({}) - House {} (지배성: {})\n",
                    part.name, part.longitude, sign_name, part.house_number, part.ruler
                ));
            }
        }
        out.push('\n');
    }
}

/// 서양 점성술 전용 마크다운 리포트 생성
pub fn format_profile_western_markdown(profile: &Profile, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_profile_meta(&mut out, profile, chart, "서양 점성술 (Western Tropical Natal Chart)");
    append_western_details(&mut out, chart);
    out
}

/// 베다 점성술 전용 마크다운 리포트 및 프롬프트 생성
pub fn format_profile_vedic_markdown(profile: &Profile, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_profile_meta(&mut out, profile, chart, "베다 점성술 분석 데이터");

    out.push_str("## 2. 라시 차트 및 분점도 (Vedic Sidereal & Vargas)\n");
    let lagna_sign_idx = chart.vedic.lagna_sign as usize % 12;
    out.push_str(&format!(
        "- Lagna: {:.2}° ({})\n",
        chart.vedic.lagna_sidereal, VEDIC_SIGNS[lagna_sign_idx]
    ));
    out.push_str("- D1 Rasi Placements:\n");
    for p in &chart.vedic.planets {
        let sign_name = VEDIC_SIGNS[p.sign_index as usize % 12];
        let retro_str = if p.is_retrograde { " [Rx]" } else { "" };
        out.push_str(&format!(
            "  * {}: {:.2}° ({}, {} Pada {}){}\n",
            p.name, p.sidereal_longitude, sign_name, p.nakshatra_name, p.nakshatra_pada, retro_str
        ));
    }
    out.push_str("- Divisional Charts (Vargas):\n");
    for (division, v) in &chart.vedic.vargas {
        let sign_name = VEDIC_SIGNS[v.lagna_sign as usize % 12];
        out.push_str(&format!(
            "  * {}: Lagna {}\n",
            division.title(), sign_name
        ));
        let mut placements_str = Vec::new();
        for p in &v.placements {
            let p_sign_name = VEDIC_SIGNS[p.sign_index as usize % 12];
            placements_str.push(format!(
                "{}: H{} {} {:.1}°",
                p.planet_name, p.house, p_sign_name, p.sign_degree
            ));
        }
        if !placements_str.is_empty() {
            out.push_str(&format!("    - Placements: {}\n", placements_str.join(", ")));
        }
    }
    out.push_str("- Vimshottari Dasha Sequence:\n");
    for d in &chart.vedic.vimshottari_dashas {
        out.push_str(&format!(
            "  * Mahadasha {}: JD {:.1} ~ JD {:.1}\n",
            d.lord, d.start_jd, d.end_jd
        ));
        for sub in &d.sub_periods {
            out.push_str(&format!(
                "    - Antardasha {}: JD {:.1} ~ JD {:.1}\n",
                sub.lord, sub.start_jd, sub.end_jd
            ));
        }
    }
    out.push('\n');

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
    append_western_details(&mut out, chart);
    out
}

/// 베다 시데리얼 프라슈나 전용 마크다운 리포트 생성
pub fn format_prashna_vedic_markdown(horary: &HoraryQuestion, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_horary_meta(&mut out, horary, chart, "프라슈나 질의 분석 데이터 (Vedic Sidereal)");

    out.push_str("## 2. Prashna Chart Placements (Vedic Sidereal)\n");
    let lagna_sign_idx = chart.vedic.lagna_sign as usize % 12;
    out.push_str(&format!(
        "- Prashna Lagna: {:.2}° ({})\n",
        chart.vedic.lagna_sidereal, VEDIC_SIGNS[lagna_sign_idx]
    ));
    for p in &chart.vedic.planets {
        let sign_name = VEDIC_SIGNS[p.sign_index as usize % 12];
        let retro = if p.is_retrograde { " [Rx]" } else { "" };
        out.push_str(&format!(
            "- {}: {:.2}° ({}, {} Pada {}){}\n",
            p.name, p.sidereal_longitude, sign_name, p.nakshatra_name, p.nakshatra_pada, retro
        ));
    }
    out.push('\n');

    out
}
