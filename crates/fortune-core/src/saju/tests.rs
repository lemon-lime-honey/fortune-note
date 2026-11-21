use super::*;
use crate::celestial::{CalendarDateTime, GeoCoordinate};
use crate::ephemeris::compute_ephemeris;

const SEOUL: GeoCoordinate = GeoCoordinate::new(37.5665, 126.9780);

fn seoul_saju(dt: CalendarDateTime, gender: Gender, options: SajuOptions) -> SajuChart {
    let ephem = compute_ephemeris(dt, SEOUL).expect("ephem should succeed");
    calculate_saju_with_options(&ephem, dt.year, SEOUL.longitude, gender, options)
        .expect("saju should succeed")
}

#[test]
fn test_saju_pillars_and_daeun() {
    let saju = seoul_saju(CalendarDateTime::new(2000, 1, 1, 12, 0, 0.0), Gender::Male, SajuOptions::default());
    assert_eq!(saju.day_pillar.korean(), "무오");
    assert_eq!(saju.day_pillar.hanja(), "戊午");
    assert_eq!(saju.daeun_periods.len(), 10);
    assert!((1..=10).contains(&saju.daeun_number));
}

#[test]
fn test_saju_lichun_boundary_transition() {
    let before = seoul_saju(CalendarDateTime::new(2024, 2, 4, 8, 26, 0.0), Gender::Male, SajuOptions::default());
    assert_eq!((before.year_pillar.korean().as_str(), before.month_pillar.korean().as_str()), ("계묘", "을축"));

    let after = seoul_saju(CalendarDateTime::new(2024, 2, 4, 8, 28, 0.0), Gender::Male, SajuOptions::default());
    assert_eq!((after.year_pillar.korean().as_str(), after.month_pillar.korean().as_str()), ("갑진", "병인"));
}

#[test]
fn test_saju_daeun_gender_direction() {
    let dt = CalendarDateTime::new(2024, 3, 1, 12, 0, 0.0);
    let male = seoul_saju(dt, Gender::Male, SajuOptions::default());
    let female = seoul_saju(dt, Gender::Female, SajuOptions::default());
    assert!(male.is_forward_daeun);
    assert!(!female.is_forward_daeun);
    assert_ne!(male.daeun_periods[0].pillar.stem_index, female.daeun_periods[0].pillar.stem_index);
}

#[test]
fn test_hour_branch_stem_and_daeun_rules() {
    assert_eq!(calculate_hour_branch(22.9), 11);
    assert_eq!(calculate_hour_branch(23.5), 0);
    assert_eq!(calculate_hour_branch(0.5), 0);
    assert_eq!(calculate_hour_branch(1.5), 1);
    assert_eq!(calculate_hour_branch(24.0), 0);

    for (day_stem, expected) in [(0, 0), (5, 0), (1, 2), (6, 2), (2, 4), (7, 4), (3, 6), (8, 6), (4, 8), (9, 8)] {
        assert_eq!(calculate_hour_stem(day_stem, 0), expected);
    }

    let f_10 = calculate_daeun_pillar(0, 2, 10, true);
    assert_eq!((f_10.stem_index, f_10.branch_index), (0, 0));
    let r_1 = calculate_daeun_pillar(0, 2, 1, false);
    assert_eq!((r_1.stem_index, r_1.branch_index), (9, 1));
}

#[test]
fn test_saju_zi_hour_mode_and_equation_of_time() {
    let dt = CalendarDateTime::new(2000, 1, 1, 15, 30, 0.0);
    let next = seoul_saju(dt, Gender::Male, SajuOptions { zi_hour_mode: ZiHourMode::NextDayAt23, apply_equation_of_time: false });
    let midnight = seoul_saju(dt, Gender::Male, SajuOptions { zi_hour_mode: ZiHourMode::MidnightAt00, apply_equation_of_time: false });
    assert_eq!((next.hour_pillar.branch_index, midnight.hour_pillar.branch_index), (0, 0));
    assert_eq!(next.day_pillar.korean(), "기미");
    assert_eq!(midnight.day_pillar.korean(), "무오");

    let nov = CalendarDateTime::new(2024, 11, 3, 3, 0, 0.0);
    let mean = seoul_saju(nov, Gender::Male, SajuOptions { zi_hour_mode: ZiHourMode::NextDayAt23, apply_equation_of_time: false });
    let apparent = seoul_saju(nov, Gender::Male, SajuOptions { zi_hour_mode: ZiHourMode::NextDayAt23, apply_equation_of_time: true });
    assert_eq!((mean.year_pillar, mean.month_pillar), (apparent.year_pillar, apparent.month_pillar));
}
