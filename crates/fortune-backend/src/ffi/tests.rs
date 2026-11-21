use super::*;

#[test]
fn test_client_ffi_calculations() {
    assert_eq!(fortune_backend_version(), "0.1.0");
    let client = FortuneClient::new(":memory:".to_string()).expect("client init failed");

    let direct_req = r#"{"year":2024,"month":3,"day":20,"hour":12,"minute":0,"second":0.0,"latitude":37.5665,"longitude":126.9780,"is_male":true}"#;
    let json = client.calculate_direct(direct_req.to_string()).expect("direct calculation failed");
    assert!(json.contains("\"timezone_id\"") && json.contains("\"western\"") && json.contains("\"vedic\"") && json.contains("\"saju\""));

    let prof_req = r#"{"birth_year":1990,"birth_month":5,"birth_day":15,"target_year":2026,"target_month":5,"target_day":15,"natal_ascendant":15.0,"natal_planets":[["Sun",50.0]]}"#;
    let prof_json = client.calculate_annual_profection(prof_req.to_string()).expect("profection ffi failed");
    assert!(prof_json.contains("\"target_age\":36") && prof_json.contains("\"profected_house\":1"));

    let sa_req = r#"{"birth_year":1990,"birth_month":5,"birth_day":15,"target_year":2026,"target_month":5,"target_day":15,"natal_planets":[["Sun",54.0]]}"#;
    let sa_json = client.calculate_solar_arc(sa_req.to_string()).expect("solar arc ffi failed");
    assert!(sa_json.contains("\"solar_arc_degrees\""));
}

#[test]
fn test_client_ffi_crud_and_exports() {
    let client = FortuneClient::new(":memory:".to_string()).expect("client init failed");
    let profile_str = client.create_profile(r#"{
        "name": "홍길동", "gender": "male", "birth_year": 1990, "birth_month": 5, "birth_day": 15,
        "birth_hour": 9, "birth_minute": 30, "latitude": 37.5665, "longitude": 126.9780
    }"#.to_string()).unwrap();
    let profile_id = serde_json::from_str::<serde_json::Value>(&profile_str).unwrap()["id"].as_str().unwrap().to_string();

    assert!(client.get_profile(profile_id.clone()).unwrap().unwrap().contains("홍길동"));
    assert!(client.list_profiles(Some("길동".to_string())).unwrap().contains("홍길동"));
    assert!(client.calculate_profile_chart(profile_id.clone()).unwrap().contains("\"western\""));

    let pred_json = client.calculate_profile_predictive_suite(
        profile_id.clone(),
        r#"{"target_year":2026,"target_month":5,"target_day":15,"target_hour":12,"target_minute":0,"house_system":"Regiomontanus"}"#.to_string(),
    ).unwrap();
    assert!(pred_json.contains("\"profection\"") && pred_json.contains("\"solar_return\""));

    assert!(client.export_profile_saju_markdown(profile_id.clone()).unwrap().contains("사주팔자"));
    assert!(client.export_profile_western_markdown(profile_id.clone(), None).unwrap().contains("Western Tropical Natal Chart"));
    assert!(client.export_profile_vedic_markdown(profile_id.clone()).unwrap().contains("Vedic Sidereal & Vargas"));

    let horary_str = client.create_horary(format!(
        r#"{{"title":"이직 문의","category":"career","question_text":"이직 가능?","query_year":2026,"query_month":9,"query_day":21,"query_hour":16,"query_minute":0,"query_second":0.0,"latitude":37.5665,"longitude":126.9780,"querent_profile_id":"{}"}}"#,
        profile_id
    )).unwrap();
    let horary_id = serde_json::from_str::<serde_json::Value>(&horary_str).unwrap()["id"].as_str().unwrap().to_string();

    assert!(client.calculate_horary_chart(horary_id.clone()).unwrap().contains("\"ephemeris\""));
    assert!(client.export_horary_western_markdown(horary_id.clone()).unwrap().contains("Western Tropical"));
    assert!(client.export_prashna_vedic_markdown(horary_id.clone()).unwrap().contains("Vedic Sidereal"));
    assert!(client.delete_horary(horary_id).unwrap());
    assert!(client.delete_profile(profile_id).unwrap());
}
