use crate::models::{HoraryQuestion, Profile};
use fortune_core::FortuneCalculationOutput;

const ZODIAC_SIGNS: [&str; 12] = [
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
];

const VEDIC_SIGNS: [&str; 12] = [
    "Mesha (Aries)", "Vrishabha (Taurus)", "Mithuna (Gemini)", "Karka (Cancer)",
    "Simha (Leo)", "Kanya (Virgo)", "Tula (Libra)", "Vrishchika (Scorpio)",
    "Dhanu (Sagittarius)", "Makara (Capricorn)", "Kumbha (Aquarius)", "Meena (Pisces)",
];

pub fn format_profile_markdown(profile: &Profile, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();

    out.push_str(&format!("# 운세 분석 데이터: {}\n\n", profile.name));

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

    out.push_str("## 2. 사주팔자 (Four Pillars of Destiny)\n");
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

    out.push_str("## 3. 서양 점성술 (Western Tropical Astrology)\n");
    let asc_sign = (chart.western.ascendant / 30.0).floor() as usize % 12;
    let mc_sign = (chart.western.midheaven / 30.0).floor() as usize % 12;
    out.push_str(&format!(
        "- Ascendant: {:.2}° ({})\n",
        chart.western.ascendant, ZODIAC_SIGNS[asc_sign]
    ));
    out.push_str(&format!(
        "- Midheaven (MC): {:.2}° ({})\n",
        chart.western.midheaven, ZODIAC_SIGNS[mc_sign]
    ));
    out.push_str("- Planets:\n");
    for p in &chart.western.planets {
        let sign_name = ZODIAC_SIGNS[p.sign_index as usize % 12];
        let retro_str = if p.is_retrograde { " [Rx]" } else { "" };
        out.push_str(&format!(
            "  * {}: {:.2}° ({}) - House {}{}\n",
            p.name, p.longitude, sign_name, p.house_number, retro_str
        ));
    }
    out.push_str("- Nodes:\n");
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
            out.push_str(&format!(
                "  * {} - {}: {} (Actual: {:.2}°, Orb: {:.2}°)\n",
                a.planet1, a.planet2, a.aspect_type, a.actual_angle, a.orb
            ));
        }
    }
    out.push('\n');

    out.push_str("## 4. 베다 점성술 (Vedic Sidereal / Jyotish)\n");
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

    out.push_str("## 5. LLM 해석 지침 프롬프트\n");
    out.push_str("위의 사주팔자 4주 간지 및 대운 흐름, 서양 점성술의 행성 배치와 메이저 아스펙트, 베다 점성술의 바르가 분점도 및 빔쇼타리 다샤 팩트를 상호 교차 대조하여, 내담자의 타고난 기질적 장단점, 핵심 잠재 역량, 인생의 중요한 시기적 변화 흐름을 종합적으로 심층 분석해 주십시오.\n");

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

pub fn format_horary_western_markdown(horary: &HoraryQuestion, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_horary_meta(&mut out, horary, chart, "호라리 질의 분석 데이터 (Western Tropical)");

    out.push_str("## 2. Horary Chart Placements (Western Tropical)\n");
    let asc_sign = (chart.western.ascendant / 30.0).floor() as usize % 12;
    let mc_sign = (chart.western.midheaven / 30.0).floor() as usize % 12;
    out.push_str(&format!(
        "- Ascendant: {:.2}° ({})\n",
        chart.western.ascendant, ZODIAC_SIGNS[asc_sign]
    ));
    out.push_str(&format!(
        "- Midheaven (MC): {:.2}° ({})\n",
        chart.western.midheaven, ZODIAC_SIGNS[mc_sign]
    ));
    for p in &chart.western.planets {
        let sign_name = ZODIAC_SIGNS[p.sign_index as usize % 12];
        let retro = if p.is_retrograde { " [Rx]" } else { "" };
        out.push_str(&format!(
            "- {}: {:.2}° ({}) - House {}{}\n",
            p.name, p.longitude, sign_name, p.house_number, retro
        ));
    }
    for n in &chart.western.nodes {
        let sign_name = ZODIAC_SIGNS[n.sign_index as usize % 12];
        out.push_str(&format!(
            "- {}: {:.2}° ({}) - House {}\n",
            n.name, n.longitude, sign_name, n.house_number
        ));
    }
    out.push_str("- Major Aspects:\n");
    if chart.western.aspects.is_empty() {
        out.push_str("  * No major aspects within orb\n");
    } else {
        for a in &chart.western.aspects {
            out.push_str(&format!(
                "  * {} - {}: {} (Orb: {:.2}°)\n",
                a.planet1, a.planet2, a.aspect_type, a.orb
            ));
        }
    }
    out.push('\n');

    out.push_str("## 3. LLM 해석 지침 (Western Horary)\n");
    out.push_str("위 서양 트로피컬 호라리 차트의 시그니피케이터(제1하우스 룰러인 질문자 주성 및 해당 질문 사안을 주관하는 해당 하우스 룰러), 달의 현재 위치와 접근/분리 아스펙트, 보이드 오브 코스 여부, 주요 행성 간의 리셉션(Exaltation, Domicile 등)을 정밀 분석하여 질문 사안의 성사 여부와 실현 시기, 주의점을 명확히 판단해 주십시오.\n");

    out
}

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

    out.push_str("## 3. LLM 해석 지침 (Vedic Prashna)\n");
    out.push_str("위 베다 시데리얼 프라슈나 차트의 라그나 로드(질문자), 사안 주관 행성(카랴카), 달의 위치 및 낙샤트라/파다, 타지카 요가(Ithasala, Ishrafa 등)를 정밀 분석하여 질문 사안의 성사 여부와 실현 시기, 방해요인을 베다 점성술 원리에 입각하여 명확히 판정해 주십시오.\n");

    out
}

pub fn format_horary_markdown(horary: &HoraryQuestion, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();

    out.push_str(&format!("# 호라리 질의 분석 데이터: {}\n\n", horary.title));

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

    out.push_str("## 2. Horary Chart Placements (Western Tropical)\n");
    let asc_sign = (chart.western.ascendant / 30.0).floor() as usize % 12;
    let mc_sign = (chart.western.midheaven / 30.0).floor() as usize % 12;
    out.push_str(&format!(
        "- Ascendant: {:.2}° ({})\n",
        chart.western.ascendant, ZODIAC_SIGNS[asc_sign]
    ));
    out.push_str(&format!(
        "- Midheaven (MC): {:.2}° ({})\n",
        chart.western.midheaven, ZODIAC_SIGNS[mc_sign]
    ));
    for p in &chart.western.planets {
        let sign_name = ZODIAC_SIGNS[p.sign_index as usize % 12];
        let retro = if p.is_retrograde { " [Rx]" } else { "" };
        out.push_str(&format!(
            "- {}: {:.2}° ({}) - House {}{}\n",
            p.name, p.longitude, sign_name, p.house_number, retro
        ));
    }
    out.push_str("- Major Aspects:\n");
    for a in &chart.western.aspects {
        out.push_str(&format!(
            "  * {} - {}: {} (Orb: {:.2}°)\n",
            a.planet1, a.planet2, a.aspect_type, a.orb
        ));
    }
    out.push('\n');

    out.push_str("## 3. Prashna Chart (Vedic Sidereal)\n");
    let lagna_sign_idx = chart.vedic.lagna_sign as usize % 12;
    out.push_str(&format!(
        "- Prashna Lagna: {:.2}° ({})\n",
        chart.vedic.lagna_sidereal, VEDIC_SIGNS[lagna_sign_idx]
    ));
    for p in &chart.vedic.planets {
        let sign_name = VEDIC_SIGNS[p.sign_index as usize % 12];
        out.push_str(&format!(
            "- {}: {:.2}° ({})\n",
            p.name, p.sidereal_longitude, sign_name
        ));
    }
    out.push('\n');

    out.push_str("## 4. LLM 해석 지침 프롬프트\n");
    out.push_str("위 호라리/프라슈나 질문의 성격에 해당하는 하우스 룰러(시그니피케이터)와 질문자(제1하우스 룰러 및 달)의 위치, 상호 아스펙트 및 접근/분리 상태를 면밀히 분석하여, 질문한 사안의 성사 여부와 시기, 주의점을 명확히 판단해 주십시오.\n");

    out
}
