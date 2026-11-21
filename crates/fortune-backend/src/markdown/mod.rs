//! LLM-friendly Markdown report formatters split by astrological domain.

mod vedic;
mod western;

use crate::models::{HoraryQuestion, Profile};
use fortune_core::FortuneCalculationOutput;

pub use western::append_western_predictive_details;

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

/// 사주명리 전용 마크다운 리포트 생성
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
        "- 대운 방향: {} (대운수: {})\n- 10년 주기 대운 흐름:\n",
        if chart.saju.is_forward_daeun { "순행" } else { "역행" },
        chart.saju.daeun_number
    ));
    for d in &chart.saju.daeun_periods {
        out.push_str(&format!(
            "  * {}세 ~ {}세: [{} ({})]\n",
            d.start_age, d.start_age + 9, d.pillar.korean(), d.pillar.hanja()
        ));
    }
    out.push('\n');
    out
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
    western::append_western_details(&mut out, chart, false);
    if let Some(s) = suite {
        western::append_western_predictive_details(&mut out, s);
    }
    out
}

/// 베다 점성술 전용 마크다운 리포트 생성
pub fn format_profile_vedic_markdown(profile: &Profile, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_profile_meta(&mut out, profile, chart, "베다 점성술 분석 데이터");
    vedic::append_vedic_details(&mut out, chart, false);
    out
}

/// 서양 트로피컬 호라리 전용 마크다운 리포트 생성
pub fn format_horary_western_markdown(horary: &HoraryQuestion, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_horary_meta(&mut out, horary, chart, "호라리 질의 분석 데이터 (Western Tropical)");
    western::append_western_details(&mut out, chart, true);
    out
}

/// 베다 시데리얼 프라슈나 전용 마크다운 리포트 생성
pub fn format_prashna_vedic_markdown(horary: &HoraryQuestion, chart: &FortuneCalculationOutput) -> String {
    let mut out = String::new();
    append_horary_meta(&mut out, horary, chart, "프라슈나 질의 분석 데이터 (Vedic Sidereal)");
    vedic::append_vedic_details(&mut out, chart, true);
    out
}
