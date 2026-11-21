use std::collections::HashMap;
use vedaksha::vedic::graha::Graha;

use super::dignity::{arudha_sign_lord, compute_classical_vedic_combustion, compute_vedic_dignity};
use super::*;
use crate::celestial::{CalendarDateTime, GeoCoordinate};
use crate::ephemeris::compute_ephemeris;

#[test]
fn test_vedic_chart_generation() {
    let ephem = compute_ephemeris(
        CalendarDateTime::new(2024, 3, 20, 12, 0, 0.0),
        GeoCoordinate::new(28.6139, 77.2090),
    )
    .expect("ephem should succeed");
    let chart = calculate_vedic(&ephem, 28.6139);

    assert_eq!(chart.vargas.len(), 19);
    for div in VargaDivision::ALL {
        let v_chart = chart.vargas.get(&div).expect("varga must exist");
        assert_eq!(v_chart.placements.len(), 9);
        let rahu = v_chart.placements.iter().find(|p| p.planet_name == "Rahu").unwrap();
        let ketu = v_chart.placements.iter().find(|p| p.planet_name == "Ketu").unwrap();
        if div == VargaDivision::D2 {
            assert!(matches!(rahu.sign_index, 3 | 4) && matches!(ketu.sign_index, 3 | 4));
        } else {
            assert_eq!((rahu.sign_index + 6) % 12, ketu.sign_index);
        }
    }

    assert_eq!(chart.planets.len(), 9);
    assert_eq!(chart.chara_karakas.len(), 7);
    assert!(chart.drishti.iter().all(|d| d.from_planet != "Ketu"));
    assert_eq!(chart.arudha_padas.len(), 12);
    assert_eq!(chart.ashtakavarga.total_bindus, 337);
    assert_eq!(chart.vimshottari_dashas.len(), 9);
    assert!(chart.vimshottari_dashas.iter().all(|d| d.sub_periods.len() == 9));
}

#[test]
fn test_varga_calculations_accuracy() {
    let cases: [(f64, VargaDivision, u8); 34] = [
        (15.5, VargaDivision::D1, 0),
        (7.5, VargaDivision::D2, 4), (20.0, VargaDivision::D2, 3), (35.0, VargaDivision::D2, 3), (50.0, VargaDivision::D2, 4),
        (5.0, VargaDivision::D3, 0), (15.0, VargaDivision::D3, 4), (25.0, VargaDivision::D3, 8),
        (2.0, VargaDivision::D4, 0), (8.0, VargaDivision::D4, 3), (38.0, VargaDivision::D4, 4),
        (2.0, VargaDivision::D6, 0), (32.0, VargaDivision::D6, 6),
        (2.0, VargaDivision::D7, 0), (32.0, VargaDivision::D7, 7),
        (1.0, VargaDivision::D8, 0), (31.0, VargaDivision::D8, 8), (61.0, VargaDivision::D8, 4),
        (1.0, VargaDivision::D9, 0), (31.0, VargaDivision::D9, 9), (10.0, VargaDivision::D9, 3), (20.0, VargaDivision::D9, 6),
        (31.0, VargaDivision::D10, 9), (61.0, VargaDivision::D11, 10), (31.0, VargaDivision::D12, 1),
        (31.0, VargaDivision::D16, 4), (31.0, VargaDivision::D20, 8),
        (0.5, VargaDivision::D24, 4), (30.5, VargaDivision::D24, 3), (59.5, VargaDivision::D24, 2),
        (30.5, VargaDivision::D27, 3), (2.5, VargaDivision::D30, 0), (38.0, VargaDivision::D30, 5), (0.7, VargaDivision::D60, 1),
    ];
    for (lon, div, expected_sign) in cases {
        assert_eq!(calculate_varga_sign(lon, div), expected_sign, "Mismatch at lon={} {:?}", lon, div);
    }
    let (d9_sign, d9_deg) = calculate_varga_sign_and_degree(10.0, VargaDivision::D9);
    assert_eq!(d9_sign, 3);
    assert!(d9_deg.abs() < 1e-6);
}

#[test]
fn test_panchanga_vara_and_dignities() {
    let empty_signs = HashMap::new();
    assert_eq!(compute_vedic_dignity(Graha::Ketu, 8, 10.0, &empty_signs), "Exalted");
    assert_eq!(compute_vedic_dignity(Graha::Ketu, 7, 10.0, &empty_signs), "Own Sign");

    let mut signs = HashMap::new();
    signs.insert(Graha::Mars, 0);
    signs.insert(Graha::Ketu, 2);
    let mut lons = HashMap::new();
    lons.insert(Graha::Mars, 15.0);
    lons.insert(Graha::Ketu, 60.0);
    assert_eq!(arudha_sign_lord(7, &signs, &lons), Graha::Mars);

    assert_eq!(compute_classical_vedic_combustion(Graha::Mars, 20.0, 10.0, true), "None");
    assert_eq!(compute_classical_vedic_combustion(Graha::Mars, 20.0, 10.0, false), "Combust");
    assert_eq!(compute_classical_vedic_combustion(Graha::Venus, 15.0, 10.0, false), "Combust (Unimpaired per Saravali)");
    assert_eq!(compute_classical_vedic_combustion(Graha::Saturn, 22.0, 10.0, false), "Combust (Unimpaired per Saravali)");

    let seoul = GeoCoordinate::new(37.5665, 126.9780);
    let ephem_predawn = compute_ephemeris(CalendarDateTime::new(2024, 3, 19, 19, 0, 0.0), seoul).unwrap();
    let predawn = calculate_vedic(&ephem_predawn, seoul.latitude);
    assert_eq!((predawn.panchanga.vara_name.as_str(), predawn.panchanga.vara_lord.as_str()), ("Mangalavara (Tuesday)", "Mars"));

    let ephem_morning = compute_ephemeris(CalendarDateTime::new(2024, 3, 19, 22, 30, 0.0), seoul).unwrap();
    let morning = calculate_vedic(&ephem_morning, seoul.latitude);
    assert_eq!((morning.panchanga.vara_name.as_str(), morning.panchanga.vara_lord.as_str()), ("Budhavara (Wednesday)", "Mercury"));

    let savana = calculate_vedic_with_options(
        &ephem_morning,
        seoul.latitude,
        VedicOptions { dasha_year_type: DashaYearType::Savana360 },
    );
    let solar_span = morning.vimshottari_dashas[1].end_jd - morning.vimshottari_dashas[1].start_jd;
    let savana_span = savana.vimshottari_dashas[1].end_jd - savana.vimshottari_dashas[1].start_jd;
    assert!(((savana_span / solar_span) - (360.0 / 365.2425)).abs() < 1e-6);
}
