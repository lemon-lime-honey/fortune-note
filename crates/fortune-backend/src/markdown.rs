use crate::models::{HoraryQuestion, Profile};
use fortune_core::FortuneCalculationOutput;

const ZODIAC_SIGNS: [&str; 12] = [
    "양자리", "황소자리", "쌍둥이자리", "게자리", "사자자리", "처녀자리",
    "천칭자리", "전갈자리", "사수자리", "염소자리", "물병자리", "물고기자리",
];

const VEDIC_SIGNS: [&str; 12] = [
    "Mesha (양자리)", "Vrishabha (황소자리)", "Mithuna (쌍둥이자리)", "Karka (게자리)",
    "Simha (사자자리)", "Kanya (처녀자리)", "Tula (천칭자리)", "Vrishchika (전갈자리)",
    "Dhanu (사수자리)", "Makara (염소자리)", "Kumbha (물병자리)", "Meena (물고기자리)",
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
        "- 대운 방향: {} (대운수: {}세)\n",
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
        "- 어센던트 (Ascendant): {:.2}° ({})\n",
        chart.western.ascendant, ZODIAC_SIGNS[asc_sign]
    ));
    out.push_str(&format!(
        "- 천정 (Midheaven): {:.2}° ({})\n",
        chart.western.midheaven, ZODIAC_SIGNS[mc_sign]
    ));
    out.push_str("- 7개 고전 행성 위치:\n");
    for p in &chart.western.planets {
        let sign_name = ZODIAC_SIGNS[p.sign_index as usize % 12];
        let retro_str = if p.is_retrograde { " [역행]" } else { "" };
        out.push_str(&format!(
            "  * {}: {:.2}° ({}) - 제{}하우스{}\n",
            p.name, p.longitude, sign_name, p.house_number, retro_str
        ));
    }
    out.push_str("- 달의 교점 (Lunar Nodes):\n");
    for n in &chart.western.nodes {
        let sign_name = ZODIAC_SIGNS[n.sign_index as usize % 12];
        out.push_str(&format!(
            "  * {}: {:.2}° ({}) - 제{}하우스\n",
            n.name, n.longitude, sign_name, n.house_number
        ));
    }
    out.push_str("- 메이저 아스펙트 (Major Aspects):\n");
    if chart.western.aspects.is_empty() {
        out.push_str("  * 유효 오브 범위 내 메이저 아스펙트 없음\n");
    } else {
        for a in &chart.western.aspects {
            out.push_str(&format!(
                "  * {} - {}: {} (각도: {:.2}°, 오차: {:.2}°)\n",
                a.planet1, a.planet2, a.aspect_type, a.actual_angle, a.orb
            ));
        }
    }
    out.push('\n');

    out.push_str("## 4. 베다 점성술 (Vedic Sidereal / Jyotish)\n");
    let lagna_sign_idx = chart.vedic.lagna_sign as usize % 12;
    out.push_str(&format!(
        "- 라그나 (Lagna): {:.2}° ({})\n",
        chart.vedic.lagna_sidereal, VEDIC_SIGNS[lagna_sign_idx]
    ));
    out.push_str("- 항성황도대 행성 위치 (D1 Rasi):\n");
    for p in &chart.vedic.planets {
        let sign_name = VEDIC_SIGNS[p.sign_index as usize % 12];
        let retro_str = if p.is_retrograde { " [역행]" } else { "" };
        out.push_str(&format!(
            "  * {}: {:.2}° ({}, {} 제{}파다){}\n",
            p.name, p.sidereal_longitude, sign_name, p.nakshatra_name, p.nakshatra_pada, retro_str
        ));
    }
    out.push_str("- 6종 바르가 차트 라그나 별자리 요약:\n");
    for (division, v) in &chart.vedic.vargas {
        let sign_name = VEDIC_SIGNS[v.lagna_sign as usize % 12];
        out.push_str(&format!(
            "  * {}: 라그나 {}\n",
            division.title(), sign_name
        ));
    }
    out.push_str("- 빔쇼타리 다샤 (Vimshottari Dasha) 대운 주기 요약:\n");
    for d in &chart.vedic.vimshottari_dashas {
        out.push_str(&format!(
            "  * 주성 {}: JD {:.1} ~ JD {:.1}\n",
            d.lord, d.start_jd, d.end_jd
        ));
    }
    out.push('\n');

    out.push_str("## 5. LLM 해석 지침 프롬프트\n");
    out.push_str("위의 사주팔자 4주 간지 및 대운 흐름, 서양 점성술의 행성 배치와 메이저 아스펙트, 베다 점성술의 바르가 분점도 및 빔쇼타리 다샤 팩트를 상호 교차 대조하여, 내담자의 타고난 기질적 장단점, 핵심 잠재 역량, 인생의 중요한 시기적 변화 흐름을 종합적으로 심층 분석해 주십시오.\n");

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

    out.push_str("## 2. 호라리 차트 행성 및 하우스 상태 (Western Tropical)\n");
    let asc_sign = (chart.western.ascendant / 30.0).floor() as usize % 12;
    let mc_sign = (chart.western.midheaven / 30.0).floor() as usize % 12;
    out.push_str(&format!(
        "- 어센던트: {:.2}° ({})\n",
        chart.western.ascendant, ZODIAC_SIGNS[asc_sign]
    ));
    out.push_str(&format!(
        "- 천정: {:.2}° ({})\n",
        chart.western.midheaven, ZODIAC_SIGNS[mc_sign]
    ));
    for p in &chart.western.planets {
        let sign_name = ZODIAC_SIGNS[p.sign_index as usize % 12];
        let retro = if p.is_retrograde { " [역행]" } else { "" };
        out.push_str(&format!(
            "- {}: {:.2}° ({}) - 제{}하우스{}\n",
            p.name, p.longitude, sign_name, p.house_number, retro
        ));
    }
    out.push_str("- 메이저 아스펙트:\n");
    for a in &chart.western.aspects {
        out.push_str(&format!(
            "  * {} - {}: {} (오차: {:.2}°)\n",
            a.planet1, a.planet2, a.aspect_type, a.orb
        ));
    }
    out.push('\n');

    out.push_str("## 3. 프라슈나 차트 (Vedic Sidereal)\n");
    let lagna_sign_idx = chart.vedic.lagna_sign as usize % 12;
    out.push_str(&format!(
        "- 프라슈나 라그나: {:.2}° ({})\n",
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
