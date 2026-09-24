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

    out.push_str("## 3. 사주명리학 감명 지침 프롬프트\n");
    out.push_str("당신은 자평진전, 적천수, 궁통보감의 정통 이론에 통달한 수십 년 경력의 사주명리학 대가입니다.\n");
    out.push_str("제시된 사주 4주 간지와 10년 대운 팩트만을 철저히 근거로 삼아 다음 단계에 따라 전문적으로 감명하십시오.\n\n");
    out.push_str("### 1단계: 원국 분석 (선천적 그릇과 기질)\n");
    out.push_str("- 일간을 중심으로 오행의 강약과 음양의 조화를 정밀 진단하십시오.\n");
    out.push_str("- 월령을 기준으로 격국을 판별하고, 원국의 균형을 잡는 용신, 희신, 기신을 명확히 규명하십시오.\n");
    out.push_str("- 일간의 통근 여부와 조후 상태를 짚어 타고난 천성과 성향을 분석하십시오.\n\n");
    out.push_str("### 2단계: 사회적 성취 및 적성 (직업·재물)\n");
    out.push_str("- 격국과 용신의 건전성에 기반하여 가장 적합한 직업군과 사회적 활동 방식을 제시하십시오.\n");
    out.push_str("- 재성과 관성의 배치를 통해 재물 획득 역량과 조직 내 성취 잠재력을 평가하십시오.\n\n");
    out.push_str("### 3단계: 대인관계 및 건강 주의점\n");
    out.push_str("- 육친의 동태와 합·충·형·파·해 작용을 분석하여 대인관계 및 가정운의 특성을 짚으십시오.\n");
    out.push_str("- 오행의 편중이나 결핍으로 인해 취약해질 수 있는 신체 부위와 건강 관리 포인트를 지적하십시오.\n\n");
    out.push_str("### 4단계: 10년 대운 흐름 및 실천적 조언\n");
    out.push_str("- 마주할 10년 대운의 간지가 용신·희신에 부합하는지 분석하여 인생의 도약기와 안정기를 짚으십시오.\n");
    out.push_str("- 내담자가 운의 흐름을 주도적으로 활용할 수 있는 구체적이고 현실적인 행동 전략을 제시하십시오.\n\n");
    out.push_str("[규칙] 주어진 사주 간지와 대운 데이터 외에 없는 글자나 신살을 임의로 날조하지 마십시오.\n");

    out
}

/// 서양 점성술 전용 마크다운 리포트 및 프롬프트 생성
pub fn format_profile_western_markdown(profile: &Profile, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_profile_meta(&mut out, profile, chart, "서양 점성술 분석 데이터");

    out.push_str("## 2. 나탈 차트 배치 (Western Tropical Natal Chart)\n");
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

    out.push_str("## 3. 서양 점성술 차트 리딩 지침 프롬프트\n");
    out.push_str("당신은 헬레니즘 전통 점성술의 구조적 엄밀함과 현대 심리 점성술의 통찰력을 겸비한 전문 점성학자입니다.\n");
    out.push_str("제시된 서양 트로피컬 나탈 차트 팩트(행성, 하우스, 아스펙트)만을 엄격히 기반으로 다음 순서에 따라 차트를 리딩하십시오.\n\n");
    out.push_str("### 1단계: 차트 핵심 축과 원소 균형\n");
    out.push_str("- 어센던트(ASC)와 차트 룰러(ASC 지배성)의 상태를 분석하여 내담자의 페르소나와 기본 행동 양식을 규명하십시오.\n");
    out.push_str("- MC(천정점)와 10하우스를 중심으로 사회적 지향점과 공적 성취 목표를 평가하십시오.\n");
    out.push_str("- 태양(본질적 정체성)과 달(정서적 반응)의 사인 및 하우스 배치를 조율하여 핵심 기질을 분석하십시오.\n\n");
    out.push_str("### 2단계: 행성 품계 및 하우스 룰러십\n");
    out.push_str("- 주요 행성들의 디그니티와 하우스 위치를 통해 각 행성의 기능적 강약을 판별하십시오.\n");
    out.push_str("- 각 하우스의 룰러가 어느 하우스로 연결되는지 추적하여 삶의 영역 간 상호작용을 설명하십시오.\n\n");
    out.push_str("### 3단계: 메이저 아스펙트 다이내믹스\n");
    out.push_str("- 컨정션, 트라인, 섹스타일 등 조화로운 아스펙트가 부여하는 타고난 재능과 기회를 도출하십시오.\n");
    out.push_str("- 스퀘어, 오포지션 등 긴장 아스펙트가 일으키는 심리적 갈등과 현실적 도전 과제를 짚고 해법을 제시하십시오.\n\n");
    out.push_str("### 4단계: 영혼의 진화와 실천 가이드\n");
    out.push_str("- 노드 축을 통해 과거의 익숙한 패턴에서 벗어나 영혼이 진화해야 할 성장 방향을 제시하십시오.\n");
    out.push_str("- 내담자가 자신의 천체 배치를 현실에서 건설적으로 발현할 수 있도록 구체적인 조언으로 마무리하십시오.\n\n");
    out.push_str("[규칙] 차트에 없는 가상의 천체나 확인되지 않은 배치를 임의로 만들어내지 마십시오.\n");

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

    out.push_str("## 3. 베다 점성술(조티쉬) 감명 지침 프롬프트\n");
    out.push_str("당신은 정통 파라샤라 및 자이미니 체계에 통달한 베다 점성술(Jyotish)의 판디트입니다.\n");
    out.push_str("제시된 시데리얼 라시 차트, 11개 바르가 분점도, 낙샤트라 및 빔쇼타리 다샤 팩트를 토대로 다음 원칙에 입각하여 심층 감명하십시오.\n\n");
    out.push_str("### 1단계: 라시(D1) 차트 기본 골격과 낙샤트라 분석\n");
    out.push_str("- 라그나와 라그나 로드의 위치 및 품계를 통해 생애 전반의 힘과 활력을 진단하십시오.\n");
    out.push_str("- 달의 위치와 잔마 낙샤트라 및 파다를 정밀 분석하여 내담자의 심식과 카르마적 잠재력을 평가하십시오.\n\n");
    out.push_str("### 2단계: 핵심 분점도(Vargas) 교차 검증\n");
    out.push_str("- D9 나밤샤: 다르마, 내면의 영적 성숙도, 배우자와의 인연 및 생애 후반부의 결실을 분석하십시오.\n");
    out.push_str("- D10 다샴샤: 직업적 명성과 권위, 사회적 지위 및 성취의 크기를 판별하십시오.\n");
    out.push_str("- D2(재물), D3(형제/용기), D4(부동산/운), D7(자녀/창의성) 등 특수 분점도 배치를 검증하십시오.\n\n");
    out.push_str("### 3단계: 빔쇼타리 다샤 운로 판정\n");
    out.push_str("- 진행 중인 마하다샤와 안타르다샤의 주재성을 D1 및 분점도와 연결하여 시기별 운의 상승과 하강을 예측하십시오.\n");
    out.push_str("- 각 다샤 기간에 마주할 현실적 과제와 성취 가능한 영역을 짚고 실천적 개운 조언을 제시하십시오.\n\n");
    out.push_str("[규칙] 서양 점성술이나 사주명리학 개념을 일절 혼용하지 말고 순수 조티쉬 원리로만 분석하십시오.\n");

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

/// 서양 트로피컬 호라리 전용 마크다운 리포트 및 프롬프트 생성
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
    out.push_str("위 서양 트로피컬 호라리 차트의 시그니피케이터(제1하우스 룰러인 질문자 주성 및 해당 질문 사안을 주관하는 해당 하우스 룰러), 달의 현재 위치와 접근/분리 아스펙트, 보이드 오브 코스 여부, 주요 행성 간의 리셉션을 정밀 분석하여 질문 사안의 성사 여부와 실현 시기, 주의점을 명확히 판단해 주십시오.\n");

    out
}

/// 베다 시데리얼 프라슈나 전용 마크다운 리포트 및 프롬프트 생성
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
    out.push_str("위 베다 시데리얼 프라슈나 차트의 라그나 로드(질문자), 사안 주관 행성(카랴카), 달의 위치 및 낙샤트라/파다, 타지카 요가를 정밀 분석하여 질문 사안의 성사 여부와 실현 시기, 방해요인을 베다 점성술 원리에 입각하여 명확히 판정해 주십시오.\n");

    out
}
