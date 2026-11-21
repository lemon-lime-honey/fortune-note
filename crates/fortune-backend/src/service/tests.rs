use super::*;

fn sample_profile(name: &str, gender: &str, tz: Option<&str>) -> CreateProfileParams {
    CreateProfileParams {
        name: name.to_string(),
        gender: gender.to_string(),
        birth_year: 1995,
        birth_month: 10,
        birth_day: 25,
        birth_hour: 14,
        birth_minute: 30,
        birth_second: 15.0,
        latitude: 37.5665,
        longitude: 126.9780,
        timezone_id: tz.map(str::to_string),
        notes: Some("비고".to_string()),
    }
}

fn sample_horary(title: &str, querent_profile_id: Option<String>) -> CreateHoraryParams {
    CreateHoraryParams {
        title: title.to_string(),
        category: "career".to_string(),
        question_text: "신규 프로젝트 계약을 지금 체결하는 것이 유리한가?".to_string(),
        query_year: 2026,
        query_month: 9,
        query_day: 21,
        query_hour: 15,
        query_minute: 45,
        query_second: 30.0,
        latitude: 37.5665,
        longitude: 126.9780,
        timezone_id: None,
        querent_profile_id,
        result_notes: None,
    }
}

#[test]
fn test_service_profile_and_markdown_flow() {
    let service = FortuneBackendService::new(":memory:").expect("service init failed");
    let profile = service.create_profile(sample_profile("테스트 인물", "female", None)).unwrap();
    assert_eq!(profile.timezone_id, "Asia/Seoul");

    let chart = service.calculate_profile_chart(&profile.id).unwrap();
    assert_eq!(chart.saju.daeun_periods.len(), 10);
    assert!((chart.utc_second - 15.0).abs() < 1e-6);
    assert_eq!(service.calculate_profile_saju(&profile.id).unwrap().saju.daeun_periods.len(), 10);
    assert_eq!(service.calculate_profile_western(&profile.id, None).unwrap().western.planets.len(), 7);
    assert_eq!(service.calculate_profile_vedic(&profile.id).unwrap().vedic.vargas.len(), 19);

    let saju_md = service.export_profile_saju_markdown(&profile.id).unwrap();
    assert!(saju_md.contains("테스트 인물") && saju_md.contains("사주팔자") && !saju_md.contains("Western Tropical"));

    let western_md = service.export_profile_western_markdown(&profile.id, None).unwrap();
    assert!(western_md.contains("Western Tropical Natal Chart") && western_md.contains("Major Aspects"));

    let vedic_md = service.export_profile_vedic_markdown(&profile.id).unwrap();
    assert!(vedic_md.contains("Vedic Sidereal & Vargas") && vedic_md.contains("Vimshottari Dasha Sequence"));
}

#[test]
fn test_service_horary_flow_and_querent_unlink() {
    let service = FortuneBackendService::new(":memory:").unwrap();
    let female = service.create_profile(sample_profile("여성 의뢰인", "female", Some("Asia/Seoul"))).unwrap();

    let linked = service.create_horary(sample_horary("이직 문의", Some(female.id.clone()))).unwrap();
    assert_eq!(linked.querent_profile_id, Some(female.id.clone()));
    assert_eq!(linked.timezone_id, "Asia/Seoul");

    // Regression: updating horary with querent_profile_id = None must actually unlink the profile.
    let unlinked = service.update_horary(&linked.id, sample_horary("이직 문의 (독립)", None)).unwrap();
    assert_eq!(unlinked.querent_profile_id, None, "update_horary must clear querent_profile_id when None is sent");

    let chart_female = {
        let re_linked = service.update_horary(&linked.id, sample_horary("결혼 질문", Some(female.id.clone()))).unwrap();
        service.calculate_horary_chart(&re_linked.id).unwrap()
    };
    let independent = service.create_horary(sample_horary("결혼 질문 (독립)", None)).unwrap();
    let chart_default = service.calculate_horary_chart(&independent.id).unwrap();

    let m_female = chart_female.western.traditional.unwrap().arabic_parts
        .into_iter().find(|p| p.name == "Part of Marriage").unwrap();
    let m_default = chart_default.western.traditional.unwrap().arabic_parts
        .into_iter().find(|p| p.name == "Part of Marriage").unwrap();
    assert_ne!(m_female.longitude, m_default.longitude);

    let western_md = service.export_horary_western_markdown(&independent.id).unwrap();
    assert!(western_md.contains("Western Tropical") && !western_md.contains("Prashna Lagna"));
    let prashna_md = service.export_prashna_vedic_markdown(&independent.id).unwrap();
    assert!(prashna_md.contains("Vedic Sidereal") && prashna_md.contains("Prashna Lagna"));
}

#[test]
fn test_default_house_system_and_western_json_contract() {
    let service = FortuneBackendService::new(":memory:").unwrap();
    let p = service.create_profile(sample_profile("하우스 검증", "male", Some("Asia/Seoul"))).unwrap();

    let default_out = service.calculate_profile_western(&p.id, None).unwrap();
    let regio_out = service.calculate_profile_western(&p.id, Some("Regiomontanus")).unwrap();
    let wholesign_out = service.calculate_profile_western(&p.id, Some("WholeSign")).unwrap();

    assert_eq!(default_out.western.house_cusps, regio_out.western.house_cusps);
    assert!(regio_out.western.traditional.is_some() && regio_out.western.hellenistic.is_none());
    assert!(wholesign_out.western.hellenistic.is_some() && wholesign_out.western.traditional.is_none());

    let parsed: serde_json::Value = serde_json::to_value(&regio_out).unwrap();
    let trad = &parsed["western"]["traditional"];
    assert!(!trad["antiscia"].as_array().unwrap().is_empty());
    assert_eq!(trad["cusp_dignities"].as_array().unwrap().len(), 12);
    assert!(trad.get("fixed_star_conjunctions").is_some());

    let hellenistic_md = service.export_profile_western_markdown(&p.id, Some("WholeSign")).unwrap();
    assert!(hellenistic_md.contains("Hellenistic Traditional Analysis (Whole Sign)"));
    assert!(!hellenistic_md.contains("Total Score"));
}

#[test]
fn test_backup_restore_and_rollback_on_fk_failure() {
    let s1 = FortuneBackendService::new(":memory:").unwrap();
    let p1 = s1.create_profile(sample_profile("사용자A", "male", None)).unwrap();
    s1.create_horary(sample_horary("이직 성공 여부", Some(p1.id.clone()))).unwrap();

    let backup_json = s1.export_backup_json().unwrap();
    let s2 = FortuneBackendService::new(":memory:").unwrap();
    let summary = s2.import_backup_json(&backup_json, true).unwrap();
    assert_eq!((summary.imported_profiles, summary.imported_horary), (1, 1));
    assert_eq!(s2.list_horary(None, None).unwrap()[0].querent_profile_id, Some(p1.id.clone()));

    let invalid_backup = serde_json::json!({
        "app": "fortune-note", "version": 1, "exported_at": "2026-10-14T00:00:00Z",
        "profiles": [],
        "horary_questions": [{
            "id": "bad-h", "title": "FK 오류", "category": "career", "question_text": "err",
            "query_year": 2026, "query_month": 1, "query_day": 1,
            "query_hour": 12, "query_minute": 0, "query_second": 0.0,
            "latitude": 37.5, "longitude": 127.0, "timezone_id": "Asia/Seoul",
            "querent_profile_id": "non-existent-profile", "result_notes": "",
            "created_at": "2026-10-14T00:00:00Z", "updated_at": "2026-10-14T00:00:00Z"
        }]
    }).to_string();

    assert!(s2.import_backup_json(&invalid_backup, true).is_err());
    assert_eq!(s2.list_profiles(None).unwrap().len(), 1, "Rollback must preserve existing profiles");
}
