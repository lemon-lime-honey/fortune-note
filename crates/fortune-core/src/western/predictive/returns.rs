use serde::{Deserialize, Serialize};
use crate::ephemeris::{compute_ephemeris, find_solar_longitude_moment};
use crate::timezone::{resolve_local_to_utc, resolve_local_to_utc_with_tz};
use vedaksha::ephem::julian::calendar_to_jd;
use super::progressions::{PredictiveAspect, ProgressedBody};
use super::{
    body_from_name, find_aspect, format_jd_utc_seconds, get_body_tropical_lon, jd_to_calendar,
    normalize_western_body_name, separation_error,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnAspectContact {
    pub return_point: String,
    pub return_longitude: f64,
    pub natal_point: String,
    pub natal_longitude: f64,
    pub aspect_type: String,
    pub orb: f64,
}

pub fn compute_return_natal_aspects(
    return_points: &[(&str, f64)],
    natal_points: &[(&str, f64)],
    max_orb: f64,
) -> Vec<ReturnAspectContact> {
    let mut contacts = Vec::new();
    for &(ret_name, ret_lon) in return_points {
        for &(nat_name, nat_lon) in natal_points {
            if let Some((asp, orb)) = find_aspect(ret_lon, nat_lon, max_orb) {
                contacts.push(ReturnAspectContact {
                    return_point: normalize_western_body_name(ret_name),
                    return_longitude: ret_lon,
                    natal_point: normalize_western_body_name(nat_name),
                    natal_longitude: nat_lon,
                    aspect_type: asp.to_string(),
                    orb,
                });
            }
        }
    }
    contacts
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarReturnResult {
    pub return_year: i32,
    pub return_date_utc: String,
    pub exact_jd_ut1: f64,
    pub sun_longitude: f64,
    pub ascendant: f64,
    pub midheaven: f64,
    pub house_cusps: [f64; 12],
    pub planets: Vec<ProgressedBody>,
    #[serde(default)]
    pub natal_aspects: Vec<ReturnAspectContact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetaryReturnResult {
    pub target_year: i32,
    pub return_body: String,
    pub return_date_utc: String,
    pub exact_jd_ut1: f64,
    pub body_longitude: f64,
    pub is_retrograde: bool,
    pub speed: f64,
    pub ascendant: f64,
    pub midheaven: f64,
    pub house_cusps: [f64; 12],
    pub return_planets: Vec<ProgressedBody>,
    pub natal_aspects: Vec<ReturnAspectContact>,
    pub return_internal_aspects: Vec<PredictiveAspect>,
}

/// Core solar return calculation with optional natal aspects computation.
pub fn calculate_solar_return_with_natal_sun_detailed(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    birth_hour: u32,
    birth_minute: u32,
    birth_second: f64,
    latitude: f64,
    longitude: f64,
    target_year: i32,
    target_sun_lon: f64,
    house_system: Option<vedaksha::astro::houses::HouseSystem>,
    natal_points: Option<&[(&str, f64)]>,
) -> Result<SolarReturnResult, String> {
    calculate_solar_return_with_natal_sun_detailed_tz(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude, target_year, target_sun_lon, house_system, natal_points, None,
    )
}

pub fn calculate_solar_return_with_natal_sun_detailed_tz(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    birth_hour: u32,
    birth_minute: u32,
    birth_second: f64,
    latitude: f64,
    longitude: f64,
    target_year: i32,
    target_sun_lon: f64,
    house_system: Option<vedaksha::astro::houses::HouseSystem>,
    natal_points: Option<&[(&str, f64)]>,
    timezone_id: Option<&str>,
) -> Result<SolarReturnResult, String> {
    // 1. Resolve exact natal UTC timestamp
    let tz_res = resolve_local_to_utc_with_tz(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude, timezone_id,
    ).map_err(|e| format!("Timezone resolution failed: {}", e))?;

    // 2. Initial guess in target_year using exact birth UTC JD
    let birth_time_frac = (tz_res.utc_hour as f64 + (tz_res.utc_minute as f64 + tz_res.utc_second / 60.0) / 60.0) / 24.0;
    let birth_utc_jd = calendar_to_jd(tz_res.utc_year, tz_res.utc_month, tz_res.utc_day as f64 + birth_time_frac);
    let year_diff = (target_year - birth_year) as f64;
    let approx_jd = birth_utc_jd + year_diff * 365.242_198_78;

    // 3. Newton-Raphson exact solar return solver (1e-7 deg precision)
    let exact_jd = find_solar_longitude_moment(approx_jd, target_sun_lon)
        .map_err(|e| format!("Solar return solver failed: {}", e))?;

    let (ry, rm, rd, rh, rmin, rs) = jd_to_calendar(exact_jd);

    // 4. Return chart observation at natal coordinates
    let final_ephem = compute_ephemeris(ry, rm, rd, rh, rmin, rs, latitude, longitude)
        .map_err(|e| format!("Solar return ephemeris calculation failed: {}", e))?;

    let h_sys = house_system.unwrap_or(vedaksha::astro::houses::HouseSystem::Regiomontanus);
    let house_result = vedaksha::astro::houses::compute_houses(
        final_ephem.ramc,
        latitude,
        final_ephem.true_obliquity_deg,
        h_sys,
    );

    let planets: Vec<ProgressedBody> = final_ephem.bodies.iter().map(|b| {
        let lon = b.tropical_longitude;
        let sign_index = (lon / 30.0).floor() as u8 % 12;
        let sign_degree = lon - (sign_index as f64 * 30.0);
        ProgressedBody {
            name: normalize_western_body_name(&b.name),
            progressed_longitude: lon,
            sign_index,
            sign_degree,
            is_retrograde: b.is_retrograde,
        }
    }).collect();

    let mut natal_aspects = Vec::new();
    if let Some(nat_pts) = natal_points {
        let mut ret_points = Vec::with_capacity(planets.len() + 2);
        ret_points.push(("ASC", house_result.asc));
        ret_points.push(("MC", house_result.mc));
        for p in &planets {
            ret_points.push((p.name.as_str(), p.progressed_longitude));
        }
        natal_aspects = compute_return_natal_aspects(&ret_points, nat_pts, 2.0);
    }

    Ok(SolarReturnResult {
        return_year: target_year,
        return_date_utc: format_jd_utc_seconds(exact_jd),
        exact_jd_ut1: exact_jd,
        sun_longitude: target_sun_lon,
        ascendant: house_result.asc,
        midheaven: house_result.mc,
        house_cusps: house_result.cusps,
        planets,
        natal_aspects,
    })
}

pub fn calculate_solar_return_with_natal_sun(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    birth_hour: u32,
    birth_minute: u32,
    birth_second: f64,
    latitude: f64,
    longitude: f64,
    target_year: i32,
    target_sun_lon: f64,
    house_system: Option<vedaksha::astro::houses::HouseSystem>,
) -> Result<SolarReturnResult, String> {
    calculate_solar_return_with_natal_sun_detailed(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude, target_year, target_sun_lon, house_system, None,
    )
}

pub fn calculate_solar_return(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    birth_hour: u32,
    birth_minute: u32,
    birth_second: f64,
    latitude: f64,
    longitude: f64,
    target_year: i32,
    house_system: Option<vedaksha::astro::houses::HouseSystem>,
) -> Result<SolarReturnResult, String> {
    // 1. Resolve exact natal UTC timestamp
    let tz_res = resolve_local_to_utc(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude,
    ).map_err(|e| format!("Timezone resolution failed: {}", e))?;

    let natal_ephem = compute_ephemeris(
        tz_res.utc_year, tz_res.utc_month, tz_res.utc_day,
        tz_res.utc_hour, tz_res.utc_minute, tz_res.utc_second,
        latitude, longitude,
    ).map_err(|e| format!("Natal ephemeris error: {}", e))?;

    let natal_sun = natal_ephem.get_body("Sun")
        .ok_or_else(|| "Sun not found in natal ephemeris".to_string())?;

    calculate_solar_return_with_natal_sun(
        birth_year,
        birth_month,
        birth_day,
        birth_hour,
        birth_minute,
        birth_second,
        latitude,
        longitude,
        target_year,
        natal_sun.tropical_longitude,
        house_system,
    )
}

/// Calculate planetary returns for any celestial body in a target year.
/// Computes exact return moment(s), return chart placements, and aspects to natal points (including ASC and MC).
pub fn calculate_planetary_return(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    birth_hour: u32,
    birth_minute: u32,
    birth_second: f64,
    latitude: f64,
    longitude: f64,
    target_year: i32,
    return_body_name: &str,
    house_system: Option<vedaksha::astro::houses::HouseSystem>,
    aspect_max_orb: Option<f64>,
) -> Result<Vec<PlanetaryReturnResult>, String> {
    let tz_res = resolve_local_to_utc(
        birth_year, birth_month, birth_day, birth_hour, birth_minute, birth_second,
        latitude, longitude,
    ).map_err(|e| format!("Timezone resolution failed: {}", e))?;

    let birth_time_frac = (tz_res.utc_hour as f64 + (tz_res.utc_minute as f64 + tz_res.utc_second / 60.0) / 60.0) / 24.0;
    let birth_utc_jd = calendar_to_jd(tz_res.utc_year, tz_res.utc_month, tz_res.utc_day as f64 + birth_time_frac);

    let natal_ephem = compute_ephemeris(
        tz_res.utc_year, tz_res.utc_month, tz_res.utc_day,
        tz_res.utc_hour, tz_res.utc_minute, tz_res.utc_second,
        latitude, longitude,
    ).map_err(|e| format!("Natal ephemeris error: {}", e))?;

    let h_sys = house_system.unwrap_or(vedaksha::astro::houses::HouseSystem::Regiomontanus);
    let natal_house = vedaksha::astro::houses::compute_houses(
        natal_ephem.ramc,
        latitude,
        natal_ephem.true_obliquity_deg,
        h_sys,
    );

    let natal_target_body = natal_ephem.bodies.iter().find(|b| {
        b.name.eq_ignore_ascii_case(return_body_name)
            || (return_body_name.eq_ignore_ascii_case("Rahu") && b.name == "Rahu")
            || (return_body_name.eq_ignore_ascii_case("Ketu") && b.name == "Ketu")
            || (return_body_name.eq_ignore_ascii_case("North Node") && b.name == "Rahu")
            || (return_body_name.eq_ignore_ascii_case("South Node") && b.name == "Ketu")
    }).ok_or_else(|| format!("Body {} not found in natal chart", return_body_name))?;

    let mut all_natal_points = Vec::with_capacity(natal_ephem.bodies.len() + 2);
    all_natal_points.push(("ASC", natal_house.asc));
    all_natal_points.push(("MC", natal_house.mc));
    for b in &natal_ephem.bodies {
        all_natal_points.push((b.name.as_str(), b.tropical_longitude));
    }

    calculate_planetary_return_from_natal(
        birth_year,
        birth_utc_jd,
        latitude,
        longitude,
        target_year,
        return_body_name,
        natal_target_body.tropical_longitude,
        &all_natal_points,
        h_sys,
        aspect_max_orb.unwrap_or(2.0),
    )
}

pub(crate) fn calculate_planetary_return_from_natal(
    birth_year: i32,
    birth_utc_jd: f64,
    latitude: f64,
    longitude: f64,
    target_year: i32,
    return_body_name: &str,
    natal_target_lon: f64,
    all_natal_points: &[(&str, f64)],
    h_sys: vedaksha::astro::houses::HouseSystem,
    orb_limit: f64,
) -> Result<Vec<PlanetaryReturnResult>, String> {
    let vbody = body_from_name(return_body_name)
        .ok_or_else(|| format!("Unknown return celestial body: {}", return_body_name))?;

    let mut exact_jds = Vec::new();

    if return_body_name.eq_ignore_ascii_case("Sun") {
        let approx_jd = birth_utc_jd + (target_year - birth_year) as f64 * 365.242_198_78;
        let exact_jd = find_solar_longitude_moment(approx_jd, natal_target_lon)
            .map_err(|e| format!("Solar return solver failed: {}", e))?;
        exact_jds.push(exact_jd);
    } else {
        let start_jd = calendar_to_jd(target_year, 1, 1.0);
        let end_jd = calendar_to_jd(target_year, 12, 31.9999);

        let step_days = if return_body_name.eq_ignore_ascii_case("Moon") {
            0.5
        } else if return_body_name.eq_ignore_ascii_case("Mercury") || return_body_name.eq_ignore_ascii_case("Venus") {
            1.0
        } else {
            2.0
        };

        let mut cur_t = start_jd;
        let mut prev_lon = get_body_tropical_lon(vbody, cur_t)?;

        while cur_t < end_jd {
            let next_t = (cur_t + step_days).min(end_jd);
            let next_lon = match get_body_tropical_lon(vbody, next_t) {
                Ok(l) => l,
                Err(_) => {
                    cur_t = next_t;
                    continue;
                }
            };

            let e0 = separation_error(prev_lon, natal_target_lon, 0.0);
            let e1 = separation_error(next_lon, natal_target_lon, 0.0);

            if e0.abs() < 12.0 && e1.abs() < 12.0 && (e0 * e1 <= 0.0) && (e0 != e1) {
                let mut left_t = cur_t;
                let mut right_t = next_t;
                for _ in 0..16 {
                    let mid_t = (left_t + right_t) * 0.5;
                    if let Ok(mid_lon) = get_body_tropical_lon(vbody, mid_t) {
                        let mid_e = separation_error(mid_lon, natal_target_lon, 0.0);
                        if mid_e.abs() < 1e-5 {
                            left_t = mid_t;
                            break;
                        }
                        let e_left = separation_error(get_body_tropical_lon(vbody, left_t).unwrap_or(prev_lon), natal_target_lon, 0.0);
                        if e_left * mid_e <= 0.0 {
                            right_t = mid_t;
                        } else {
                            left_t = mid_t;
                        }
                    } else {
                        break;
                    }
                }
                let cand_jd = left_t;
                if !exact_jds.iter().any(|&u: &f64| (u - cand_jd).abs() < 0.25) {
                    exact_jds.push(cand_jd);
                }
            }

            cur_t = next_t;
            prev_lon = next_lon;
        }
    }

    let mut results = Vec::new();

    for exact_jd in exact_jds {
        let (ry, rm, rd, rh, rmin, rs) = jd_to_calendar(exact_jd);
        let return_ephem = compute_ephemeris(ry, rm, rd, rh, rmin, rs, latitude, longitude)
            .map_err(|e| format!("Return ephemeris calculation failed: {}", e))?;

        let ret_house = vedaksha::astro::houses::compute_houses(
            return_ephem.ramc,
            latitude,
            return_ephem.true_obliquity_deg,
            h_sys,
        );

        let mut ret_planets = Vec::new();
        let mut cur_body_lon = natal_target_lon;
        let mut cur_body_spd = 0.0;
        let mut cur_body_rx = false;

        for b in &return_ephem.bodies {
            let lon = b.tropical_longitude;
            let sign_index = (lon / 30.0).floor() as u8 % 12;
            let sign_degree = lon - (sign_index as f64 * 30.0);

            if b.name.eq_ignore_ascii_case(return_body_name)
                || (return_body_name.eq_ignore_ascii_case("Rahu") && b.name == "Rahu")
                || (return_body_name.eq_ignore_ascii_case("Ketu") && b.name == "Ketu")
                || (return_body_name.eq_ignore_ascii_case("North Node") && b.name == "Rahu")
                || (return_body_name.eq_ignore_ascii_case("South Node") && b.name == "Ketu")
            {
                cur_body_lon = lon;
                cur_body_spd = b.speed;
                cur_body_rx = b.is_retrograde;
            }

            ret_planets.push(ProgressedBody {
                name: normalize_western_body_name(&b.name),
                progressed_longitude: lon,
                sign_index,
                sign_degree,
                is_retrograde: b.is_retrograde,
            });
        }

        let mut all_ret_points = Vec::with_capacity(ret_planets.len() + 2);
        all_ret_points.push(("ASC", ret_house.asc));
        all_ret_points.push(("MC", ret_house.mc));
        for p in &ret_planets {
            all_ret_points.push((p.name.as_str(), p.progressed_longitude));
        }

        let natal_aspects = compute_return_natal_aspects(&all_ret_points, all_natal_points, orb_limit);

        let mut internal_aspects = Vec::new();
        for i in 0..ret_planets.len() {
            for j in (i + 1)..ret_planets.len() {
                if let Some((asp, orb)) = find_aspect(ret_planets[i].progressed_longitude, ret_planets[j].progressed_longitude, orb_limit) {
                    internal_aspects.push(PredictiveAspect {
                        moving_body: ret_planets[i].name.clone(),
                        natal_body: ret_planets[j].name.clone(),
                        aspect_type: asp.to_string(),
                        orb,
                    });
                }
            }
        }

        results.push(PlanetaryReturnResult {
            target_year,
            return_body: normalize_western_body_name(return_body_name),
            return_date_utc: format_jd_utc_seconds(exact_jd),
            exact_jd_ut1: exact_jd,
            body_longitude: cur_body_lon,
            is_retrograde: cur_body_rx,
            speed: cur_body_spd,
            ascendant: ret_house.asc,
            midheaven: ret_house.mc,
            house_cusps: ret_house.cusps,
            return_planets: ret_planets,
            natal_aspects,
            return_internal_aspects: internal_aspects,
        });
    }

    Ok(results)
}
