use serde::{Deserialize, Serialize};
use vedaksha::astro::houses::{compute_houses, HouseSystem};
use vedaksha::ephem::julian::calendar_to_jd;

use crate::celestial::{
    format_jd_utc_seconds, jd_to_calendar, matches_western_body, normalize_western_body_name,
    CalendarDateTime, GeoCoordinate, DIURNAL_ROTATION_DEG_PER_DAY, TROPICAL_YEAR_DAYS,
};
use crate::ephemeris::{compute_ephemeris, find_solar_longitude_moment, EphemerisSnapshot};
use crate::timezone::resolve_local_to_utc_with_tz;
use crate::western::aspects::is_aspect_applying;
use super::progressions::{PredictiveAspect, ProgressedBody};
use super::{body_from_name, find_aspect_with_target, get_body_tropical_lon, separation_error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnAspectContact {
    pub return_point: String,
    pub return_longitude: f64,
    pub natal_point: String,
    pub natal_longitude: f64,
    pub aspect_type: String,
    pub orb: f64,
    #[serde(default)]
    pub is_applying: bool,
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

impl PlanetaryReturnResult {
    /// Convert a Sun planetary return into a `SolarReturnResult` without recomputing ephemeris.
    pub fn to_solar_return(&self) -> SolarReturnResult {
        SolarReturnResult {
            return_year: self.target_year,
            return_date_utc: self.return_date_utc.clone(),
            exact_jd_ut1: self.exact_jd_ut1,
            sun_longitude: self.body_longitude,
            ascendant: self.ascendant,
            midheaven: self.midheaven,
            house_cusps: self.house_cusps,
            planets: self.return_planets.clone(),
            natal_aspects: self.natal_aspects.clone(),
        }
    }
}

/// Request object for computing planetary or solar returns from natal birth data.
#[derive(Debug, Clone, Copy)]
pub struct PlanetaryReturnRequest<'a> {
    pub birth_dt: CalendarDateTime,
    pub location: GeoCoordinate,
    pub timezone_id: Option<&'a str>,
    pub target_year: i32,
    pub return_body_name: &'a str,
    pub house_system: Option<HouseSystem>,
    pub aspect_max_orb: Option<f64>,
}

/// Internal bundled context shared across all body returns in a predictive suite.
pub(crate) struct NatalReturnContext<'a> {
    pub birth_year: i32,
    pub birth_utc_jd: f64,
    pub location: GeoCoordinate,
    pub target_year: i32,
    pub all_natal_points: &'a [(&'a str, f64)],
    pub house_system: HouseSystem,
    pub orb_limit: f64,
}

pub fn compute_return_natal_aspects(
    return_points: &[(&str, f64)],
    natal_points: &[(&str, f64)],
    max_orb: f64,
) -> Vec<ReturnAspectContact> {
    let with_speeds: Vec<(&str, f64, f64)> = return_points.iter().map(|&(n, l)| (n, l, 0.0)).collect();
    compute_return_natal_aspects_with_speeds(&with_speeds, natal_points, max_orb)
}

pub(crate) fn compute_return_natal_aspects_with_speeds(
    return_points: &[(&str, f64, f64)],
    natal_points: &[(&str, f64)],
    max_orb: f64,
) -> Vec<ReturnAspectContact> {
    let mut contacts = Vec::new();
    for &(ret_name, ret_lon, ret_speed) in return_points {
        for &(nat_name, nat_lon) in natal_points {
            if let Some((asp, target_angle, orb)) = find_aspect_with_target(ret_lon, nat_lon, max_orb) {
                contacts.push(ReturnAspectContact {
                    return_point: normalize_western_body_name(ret_name),
                    return_longitude: ret_lon,
                    natal_point: normalize_western_body_name(nat_name),
                    natal_longitude: nat_lon,
                    aspect_type: asp.to_string(),
                    orb,
                    is_applying: is_aspect_applying(ret_lon, ret_speed, nat_lon, 0.0, target_angle),
                });
            }
        }
    }
    contacts
}

fn find_return_jds(
    birth_year: i32,
    birth_utc_jd: f64,
    target_year: i32,
    return_body_name: &str,
    natal_target_lon: f64,
) -> Result<Vec<f64>, String> {
    let vbody = body_from_name(return_body_name)
        .ok_or_else(|| format!("Unknown return celestial body: {}", return_body_name))?;

    if return_body_name.eq_ignore_ascii_case("Sun") {
        let approx_jd = birth_utc_jd + (target_year - birth_year) as f64 * TROPICAL_YEAR_DAYS;
        let exact_jd = find_solar_longitude_moment(approx_jd, natal_target_lon)
            .map_err(|e| format!("Solar return solver failed: {}", e))?;
        return Ok(vec![exact_jd]);
    }

    let start_jd = calendar_to_jd(target_year, 1, 1.0);
    let end_jd = calendar_to_jd(target_year, 12, 31.9999);
    let step_days = if return_body_name.eq_ignore_ascii_case("Moon") {
        0.5
    } else if return_body_name.eq_ignore_ascii_case("Mercury") || return_body_name.eq_ignore_ascii_case("Venus") {
        1.0
    } else {
        2.0
    };

    let mut exact_jds = Vec::new();
    let mut cur_t = start_jd;
    let mut prev_lon = get_body_tropical_lon(vbody, cur_t)?;

    while cur_t < end_jd {
        let next_t = (cur_t + step_days).min(end_jd);
        let Ok(next_lon) = get_body_tropical_lon(vbody, next_t) else {
            cur_t = next_t;
            continue;
        };
        let e0 = separation_error(prev_lon, natal_target_lon, 0.0);
        let e1 = separation_error(next_lon, natal_target_lon, 0.0);

        if e0.abs() < 12.0 && e1.abs() < 12.0 && (e0 * e1 <= 0.0) && (e0 != e1) {
            let cand_jd = refine_return_bisection(vbody, cur_t, next_t, prev_lon, natal_target_lon);
            if !exact_jds.iter().any(|&u: &f64| (u - cand_jd).abs() < 0.25) {
                exact_jds.push(cand_jd);
            }
        }
        cur_t = next_t;
        prev_lon = next_lon;
    }
    Ok(exact_jds)
}

fn refine_return_bisection(
    vbody: vedaksha::ephem::bodies::Body,
    mut left_t: f64,
    mut right_t: f64,
    prev_lon: f64,
    natal_target_lon: f64,
) -> f64 {
    for _ in 0..16 {
        let mid_t = (left_t + right_t) * 0.5;
        let Ok(mid_lon) = get_body_tropical_lon(vbody, mid_t) else {
            break;
        };
        let mid_e = separation_error(mid_lon, natal_target_lon, 0.0);
        if mid_e.abs() < 1e-5 {
            return mid_t;
        }
        let left_lon = get_body_tropical_lon(vbody, left_t).unwrap_or(prev_lon);
        let e_left = separation_error(left_lon, natal_target_lon, 0.0);
        if e_left * mid_e <= 0.0 {
            right_t = mid_t;
        } else {
            left_t = mid_t;
        }
    }
    left_t
}

fn compute_internal_return_aspects(ephem: &EphemerisSnapshot, orb_limit: f64) -> Vec<PredictiveAspect> {
    let mut aspects = Vec::new();
    for i in 0..ephem.bodies.len() {
        for j in (i + 1)..ephem.bodies.len() {
            let (b1, b2) = (&ephem.bodies[i], &ephem.bodies[j]);
            if let Some((asp, target_angle, orb)) =
                find_aspect_with_target(b1.tropical_longitude, b2.tropical_longitude, orb_limit)
            {
                aspects.push(PredictiveAspect {
                    moving_body: normalize_western_body_name(&b1.name),
                    natal_body: normalize_western_body_name(&b2.name),
                    aspect_type: asp.to_string(),
                    orb,
                    is_applying: is_aspect_applying(
                        b1.tropical_longitude,
                        b1.speed,
                        b2.tropical_longitude,
                        b2.speed,
                        target_angle,
                    ),
                });
            }
        }
    }
    aspects
}

fn build_return_at_jd(
    ctx: &NatalReturnContext<'_>,
    exact_jd: f64,
    return_body_name: &str,
) -> Result<PlanetaryReturnResult, String> {
    let return_ephem = compute_ephemeris(jd_to_calendar(exact_jd), ctx.location)
        .map_err(|e| format!("Return ephemeris calculation failed: {}", e))?;
    let ret_house = compute_houses(
        return_ephem.ramc,
        ctx.location.latitude,
        return_ephem.true_obliquity_deg,
        ctx.house_system,
    );

    let mut ret_planets = Vec::with_capacity(return_ephem.bodies.len());
    let (mut cur_lon, mut cur_spd, mut cur_rx) = (0.0, 0.0, false);

    for b in &return_ephem.bodies {
        let lon = b.tropical_longitude;
        let sign_index = (lon / 30.0).floor() as u8 % 12;
        if matches_western_body(&b.name, return_body_name) {
            cur_lon = lon;
            cur_spd = b.speed;
            cur_rx = b.is_retrograde;
        }
        ret_planets.push(ProgressedBody {
            name: normalize_western_body_name(&b.name),
            progressed_longitude: lon,
            sign_index,
            sign_degree: lon - (sign_index as f64 * 30.0),
            is_retrograde: b.is_retrograde,
        });
    }

    let mut all_ret_points = Vec::with_capacity(ret_planets.len() + 2);
    all_ret_points.push(("ASC", ret_house.asc, DIURNAL_ROTATION_DEG_PER_DAY));
    all_ret_points.push(("MC", ret_house.mc, DIURNAL_ROTATION_DEG_PER_DAY));
    for b in &return_ephem.bodies {
        all_ret_points.push((b.name.as_str(), b.tropical_longitude, b.speed));
    }

    Ok(PlanetaryReturnResult {
        target_year: ctx.target_year,
        return_body: normalize_western_body_name(return_body_name),
        return_date_utc: format_jd_utc_seconds(exact_jd),
        exact_jd_ut1: exact_jd,
        body_longitude: cur_lon,
        is_retrograde: cur_rx,
        speed: cur_spd,
        ascendant: ret_house.asc,
        midheaven: ret_house.mc,
        house_cusps: ret_house.cusps,
        return_planets: ret_planets,
        natal_aspects: compute_return_natal_aspects_with_speeds(&all_ret_points, ctx.all_natal_points, ctx.orb_limit),
        return_internal_aspects: compute_internal_return_aspects(&return_ephem, ctx.orb_limit),
    })
}

pub(crate) fn calculate_planetary_return_from_natal(
    ctx: &NatalReturnContext<'_>,
    return_body_name: &str,
    natal_target_lon: f64,
) -> Result<Vec<PlanetaryReturnResult>, String> {
    let exact_jds = find_return_jds(
        ctx.birth_year,
        ctx.birth_utc_jd,
        ctx.target_year,
        return_body_name,
        natal_target_lon,
    )?;
    let mut results = Vec::with_capacity(exact_jds.len());
    for exact_jd in exact_jds {
        results.push(build_return_at_jd(ctx, exact_jd, return_body_name)?);
    }
    Ok(results)
}

/// Calculate planetary returns for any celestial body in a target year (with optional explicit timezone).
pub fn calculate_planetary_return(req: &PlanetaryReturnRequest<'_>) -> Result<Vec<PlanetaryReturnResult>, String> {
    let tz_res = resolve_local_to_utc_with_tz(req.birth_dt, req.location, req.timezone_id)
        .map_err(|e| format!("Timezone resolution failed: {}", e))?;
    let utc_dt = tz_res.utc_datetime();
    let birth_utc_jd = utc_dt.to_julian_day();

    let natal_ephem = compute_ephemeris(utc_dt, req.location)
        .map_err(|e| format!("Natal ephemeris error: {}", e))?;
    let house_system = req.house_system.unwrap_or(HouseSystem::Regiomontanus);
    let natal_house = compute_houses(
        natal_ephem.ramc,
        req.location.latitude,
        natal_ephem.true_obliquity_deg,
        house_system,
    );

    let natal_target = natal_ephem
        .bodies
        .iter()
        .find(|b| matches_western_body(&b.name, req.return_body_name))
        .ok_or_else(|| format!("Body {} not found in natal chart", req.return_body_name))?;

    let mut all_natal_points = Vec::with_capacity(natal_ephem.bodies.len() + 2);
    all_natal_points.push(("ASC", natal_house.asc));
    all_natal_points.push(("MC", natal_house.mc));
    for b in &natal_ephem.bodies {
        all_natal_points.push((b.name.as_str(), b.tropical_longitude));
    }

    let ctx = NatalReturnContext {
        birth_year: req.birth_dt.year,
        birth_utc_jd,
        location: req.location,
        target_year: req.target_year,
        all_natal_points: &all_natal_points,
        house_system,
        orb_limit: req.aspect_max_orb.unwrap_or(2.0),
    };
    calculate_planetary_return_from_natal(&ctx, req.return_body_name, natal_target.tropical_longitude)
}

/// Calculate Solar Return for a target year using explicit optional timezone support.
pub fn calculate_solar_return(req: &PlanetaryReturnRequest<'_>) -> Result<SolarReturnResult, String> {
    let sun_req = PlanetaryReturnRequest {
        return_body_name: "Sun",
        ..*req
    };
    let returns = calculate_planetary_return(&sun_req)?;
    returns
        .first()
        .map(PlanetaryReturnResult::to_solar_return)
        .ok_or_else(|| "Solar return could not be determined".to_string())
}
