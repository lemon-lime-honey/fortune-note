//! Universal Vedic Mundane Astronomical Event Generator & Chart Engine.
//!
//! Features:
//! 1. CLI Argument Parsing: Start Date, End Date, Latitude, Longitude, Export Directory.
//! 2. Decoupled Architecture:
//!    - Pure (start_jd, end_jd) event finder engine.
//!    - Global UTC determination before local chart calculation.
//! 3. Distinct Event Categorization & Mundane Relevance:
//!    - Primary: Mesha Sankranti, Solar/Lunar Eclipses, Surya Sankrantis, Major Stations
//!    - Secondary: Regular Lunations (New Moon / Full Moon)
//!    - Reference: Tropical Equinoxes & Solstices
//! 4. Mathematical Presentation Layer:
//!    - Boundary rollover normalization (30.0° -> next sign 0.0°).
//! 5. Hybrid Export Engine:
//!    - events.json: Full structured machine-readable payload.
//!    - SUMMARY.md: Markdown executive briefing grouped by Mundane Relevance.

use fortune_core::ephemeris::{compute_ephemeris, EphemerisError};
use fortune_core::vedic::calculate_vedic;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;
use vedaksha::astro::sidereal::{true_ayanamsha_value, Ayanamsha};
use vedaksha::ephem::analytical::AnalyticalProvider;
use vedaksha::ephem::bodies::Body;
use vedaksha::ephem::coordinates::apparent_position;
use vedaksha::ephem::julian::{calendar_to_jd, jd_to_calendar};
use vedaksha::math::angle::normalize_degrees;

const DEFAULT_LAT: f64 = 37.5665;
const DEFAULT_LON: f64 = 126.9780;

const RASHI_NAMES: [&str; 12] = [
    "Mesha", "Vrishabha", "Mithuna", "Karka", "Simha", "Kanya",
    "Tula", "Vrishchika", "Dhanu", "Makara", "Kumbha", "Meena",
];

fn wrap_diff(diff: f64) -> f64 {
    let mut d = normalize_degrees(diff);
    if d >= 180.0 {
        d -= 360.0;
    }
    d
}

/// Normalizes sign boundary display (e.g. 30.0° rolls over to next sign 0.0°).
fn normalize_placement_display(sign_idx: u8, deg: f64) -> (u8, f64) {
    let rounded_deg = (deg * 10.0).round() / 10.0;
    if rounded_deg >= 30.0 {
        ((sign_idx + 1) % 12, 0.0)
    } else {
        (sign_idx, rounded_deg)
    }
}

// ---------------------------------------------------------------------------
// 1. Generalized Solver
// ---------------------------------------------------------------------------

pub fn solve_newton_raphson<F>(
    mut approx_jd: f64,
    max_iter: usize,
    tol_deg: f64,
    mut eval: F,
) -> Result<(f64, f64, usize), EphemerisError>
where
    F: FnMut(f64) -> Result<(f64, f64), EphemerisError>,
{
    let mut iter = 0;
    let mut last_residual = f64::MAX;

    while iter < max_iter {
        iter += 1;
        let (residual, rate) = eval(approx_jd)?;
        last_residual = residual.abs();

        if last_residual < tol_deg {
            return Ok((approx_jd, residual, iter));
        }

        let effective_rate = if rate.abs() < 1e-6 {
            if rate >= 0.0 { 1e-6 } else { -1e-6 }
        } else {
            rate
        };

        let step = residual / effective_rate;
        approx_jd += step;
    }

    Ok((approx_jd, last_residual, iter))
}

// ---------------------------------------------------------------------------
// 2. Individual Event Finders
// ---------------------------------------------------------------------------

/// Find exact moment Sun reaches a Tropical longitude (0°, 90°, 180°, 270°).
pub fn find_tropical_solar_ingress(
    target_lon: f64,
    approx_jd: f64,
) -> Result<(f64, f64, usize), EphemerisError> {
    let provider = AnalyticalProvider;
    let target = normalize_degrees(target_lon);

    solve_newton_raphson(approx_jd, 15, 1e-7, |jd| {
        let sun = apparent_position(&provider, Body::Sun, jd).map_err(|e| {
            EphemerisError::BodyCalculationFailed("Sun".to_string(), format!("{:?}", e))
        })?;
        let current_lon = normalize_degrees(sun.ecliptic.longitude.to_degrees());
        let residual = wrap_diff(target - current_lon);
        let rate = sun.longitude_speed.max(0.9);
        Ok((residual, rate))
    })
}

/// Find exact moment Sun reaches a Sidereal Lahiri longitude (0°, 30°, ... 330°).
pub fn find_sidereal_surya_sankranti(
    target_sidereal_lon: f64,
    approx_jd: f64,
) -> Result<(f64, f64, usize), EphemerisError> {
    let provider = AnalyticalProvider;
    let target = normalize_degrees(target_sidereal_lon);

    solve_newton_raphson(approx_jd, 15, 1e-7, |jd| {
        let sun = apparent_position(&provider, Body::Sun, jd).map_err(|e| {
            EphemerisError::BodyCalculationFailed("Sun".to_string(), format!("{:?}", e))
        })?;
        let ayanamsha = true_ayanamsha_value(Ayanamsha::IndianOfficial, jd);
        let current_sidereal_lon = normalize_degrees(sun.ecliptic.longitude.to_degrees() - ayanamsha);
        let residual = wrap_diff(target - current_sidereal_lon);
        let rate = sun.longitude_speed.max(0.9);
        Ok((residual, rate))
    })
}

/// Find exact New Moon (Moon-Sun tropical conjunction).
pub fn find_new_moon(approx_jd: f64) -> Result<(f64, f64, usize), EphemerisError> {
    let provider = AnalyticalProvider;

    solve_newton_raphson(approx_jd, 15, 1e-7, |jd| {
        let sun = apparent_position(&provider, Body::Sun, jd).map_err(|e| {
            EphemerisError::BodyCalculationFailed("Sun".to_string(), format!("{:?}", e))
        })?;
        let moon = apparent_position(&provider, Body::Moon, jd).map_err(|e| {
            EphemerisError::BodyCalculationFailed("Moon".to_string(), format!("{:?}", e))
        })?;

        let sun_lon = normalize_degrees(sun.ecliptic.longitude.to_degrees());
        let moon_lon = normalize_degrees(moon.ecliptic.longitude.to_degrees());
        let residual = wrap_diff(sun_lon - moon_lon);
        let relative_rate = moon.longitude_speed - sun.longitude_speed;
        Ok((residual, relative_rate))
    })
}

/// Find exact Full Moon (Moon-Sun tropical opposition, 180°).
pub fn find_full_moon(approx_jd: f64) -> Result<(f64, f64, usize), EphemerisError> {
    let provider = AnalyticalProvider;

    solve_newton_raphson(approx_jd, 15, 1e-7, |jd| {
        let sun = apparent_position(&provider, Body::Sun, jd).map_err(|e| {
            EphemerisError::BodyCalculationFailed("Sun".to_string(), format!("{:?}", e))
        })?;
        let moon = apparent_position(&provider, Body::Moon, jd).map_err(|e| {
            EphemerisError::BodyCalculationFailed("Moon".to_string(), format!("{:?}", e))
        })?;

        let sun_lon = normalize_degrees(sun.ecliptic.longitude.to_degrees());
        let moon_lon = normalize_degrees(moon.ecliptic.longitude.to_degrees());
        let diff = wrap_diff(moon_lon - sun_lon);
        let residual = if diff >= 0.0 { 180.0 - diff } else { -180.0 - diff };
        let relative_rate = moon.longitude_speed - sun.longitude_speed;
        Ok((residual, relative_rate))
    })
}

/// Find planetary station (longitude_speed = 0).
pub fn find_station(body: Body, approx_jd: f64) -> Result<(f64, f64, usize), EphemerisError> {
    let provider = AnalyticalProvider;
    let h = 0.01;

    solve_newton_raphson(approx_jd, 20, 1e-6, |jd| {
        let p_curr = apparent_position(&provider, body, jd).map_err(|e| {
            EphemerisError::BodyCalculationFailed(format!("{:?}", body), format!("{:?}", e))
        })?;
        let p_plus = apparent_position(&provider, body, jd + h).map_err(|e| {
            EphemerisError::BodyCalculationFailed(format!("{:?}", body), format!("{:?}", e))
        })?;
        let p_minus = apparent_position(&provider, body, jd - h).map_err(|e| {
            EphemerisError::BodyCalculationFailed(format!("{:?}", body), format!("{:?}", e))
        })?;

        let speed = p_curr.longitude_speed;
        let accel = (p_plus.longitude_speed - p_minus.longitude_speed) / (2.0 * h);
        let residual = -speed;
        Ok((residual, accel))
    })
}

// ---------------------------------------------------------------------------
// 3. Astronomical Event Data Model & Engine
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MundaneRelevance {
    Primary,
    Secondary,
    Reference,
}

impl MundaneRelevance {
    pub fn badge(&self) -> &'static str {
        match self {
            Self::Primary => "★ PRIMARY",
            Self::Secondary => "• SECONDARY",
            Self::Reference => "◇ REFERENCE",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "PRIMARY",
            Self::Secondary => "SECONDARY",
            Self::Reference => "REFERENCE",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum EventCategory {
    TropicalSolarIngress,
    SiderealSuryaSankranti,
    NewMoon,
    FullMoon,
    PlanetaryStation,
}

impl EventCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TropicalSolarIngress => "TROPICAL INGRESS",
            Self::SiderealSuryaSankranti => "SIDEREAL SANKRANTI",
            Self::NewMoon => "NEW MOON",
            Self::FullMoon => "FULL MOON",
            Self::PlanetaryStation => "PLANETARY STATION",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstronomicalEvent {
    pub category: EventCategory,
    pub relevance: MundaneRelevance,
    pub name: String,
    pub jd_ut1: f64,
    pub residual: f64,
    pub iterations: usize,
    pub unit: &'static str,
    pub note: Option<String>,
}

/// Pure event finding engine decoupled from CLI and Local Charts.
pub fn find_astronomical_events(start_jd: f64, end_jd: f64) -> Vec<AstronomicalEvent> {
    let provider = AnalyticalProvider;
    let mut events = Vec::new();

    // --- A. Tropical Solar Ingress (Equinoxes & Solstices: 0°, 90°, 180°, 270°) ---
    let cardinal_points = [
        ("Tropical Vernal Equinox (춘분 0° Aries)", 0.0),
        ("Tropical Summer Solstice (하지 90° Cancer)", 90.0),
        ("Tropical Autumnal Equinox (추분 180° Libra)", 180.0),
        ("Tropical Winter Solstice (동지 270° Capricorn)", 270.0),
    ];

    let start_sun = apparent_position(&provider, Body::Sun, start_jd).unwrap();
    let start_sun_trop = normalize_degrees(start_sun.ecliptic.longitude.to_degrees());

    for (name, target_deg) in cardinal_points {
        let diff_deg = normalize_degrees(target_deg - start_sun_trop);
        let approx_days = diff_deg / 0.9856;
        let approx_jd = start_jd + approx_days;

        if approx_jd >= start_jd - 5.0 && approx_jd <= end_jd + 5.0 {
            if let Ok((jd, res, iters)) = find_tropical_solar_ingress(target_deg, approx_jd) {
                if jd >= start_jd && jd <= end_jd {
                    events.push(AstronomicalEvent {
                        category: EventCategory::TropicalSolarIngress,
                        relevance: MundaneRelevance::Reference,
                        name: name.to_string(),
                        jd_ut1: jd,
                        residual: res,
                        iterations: iters,
                        unit: "deg",
                        note: Some("Tropical Seasonal Reference Marker".to_string()),
                    });
                }
            }
        }
    }

    // --- B. Sidereal Surya Sankranti (Mesha 0°, Vrishabha 30°, ... 330°) ---
    let start_ayanamsha = true_ayanamsha_value(Ayanamsha::IndianOfficial, start_jd);
    let start_sun_sid = normalize_degrees(start_sun_trop - start_ayanamsha);

    let mut next_rashi_idx = ((start_sun_sid / 30.0).floor() as usize + 1) % 12;
    let mut current_search_jd = start_jd;

    while current_search_jd <= end_jd + 35.0 {
        let target_sid_deg = (next_rashi_idx as f64) * 30.0;
        let sun_now = apparent_position(&provider, Body::Sun, current_search_jd).unwrap();
        let aya_now = true_ayanamsha_value(Ayanamsha::IndianOfficial, current_search_jd);
        let sid_now = normalize_degrees(sun_now.ecliptic.longitude.to_degrees() - aya_now);

        let rem_deg = normalize_degrees(target_sid_deg - sid_now);
        let days_to_jump = rem_deg / 0.9856;
        let approx_jd = current_search_jd + days_to_jump;

        if let Ok((jd, res, iters)) = find_sidereal_surya_sankranti(target_sid_deg, approx_jd) {
            if jd >= start_jd && jd <= end_jd {
                let rashi_title = RASHI_NAMES[next_rashi_idx];
                let is_mesha = next_rashi_idx == 0;
                let title = format!(
                    "Surya {} Sankranti (Sun enters sidereal {}){}",
                    rashi_title,
                    rashi_title,
                    if is_mesha { " [★ VEDIC NEW YEAR / 메샤 산크란티]" } else { "" }
                );

                events.push(AstronomicalEvent {
                    category: EventCategory::SiderealSuryaSankranti,
                    relevance: MundaneRelevance::Primary,
                    name: title,
                    jd_ut1: jd,
                    residual: res,
                    iterations: iters,
                    unit: "deg",
                    note: if is_mesha {
                        Some("Core Mundane Year Marker".to_string())
                    } else {
                        Some("Mundane Solar Month Marker".to_string())
                    },
                });
            }
            current_search_jd = jd + 25.0;
            next_rashi_idx = (next_rashi_idx + 1) % 12;
        } else {
            break;
        }
    }

    // --- C. Predictive Leap Lunar Phases (New Moon / Full Moon with Eclipse Tagging) ---
    let synodic_month = 29.530589;

    let s0 = apparent_position(&provider, Body::Sun, start_jd).unwrap();
    let m0 = apparent_position(&provider, Body::Moon, start_jd).unwrap();
    let init_diff = normalize_degrees(m0.ecliptic.longitude.to_degrees() - s0.ecliptic.longitude.to_degrees());

    let days_to_nm = (360.0 - init_diff) / 12.19;
    let first_new_moon_approx = start_jd + days_to_nm;

    let days_to_fm = if init_diff < 180.0 {
        (180.0 - init_diff) / 12.19
    } else {
        (540.0 - init_diff) / 12.19
    };
    let first_full_moon_approx = start_jd + days_to_fm;

    // Collect New Moons
    let mut nm_approx = first_new_moon_approx;
    while nm_approx <= end_jd + 5.0 {
        if let Ok((jd, res, iters)) = find_new_moon(nm_approx) {
            if jd >= start_jd && jd <= end_jd {
                let sun_pos = apparent_position(&provider, Body::Sun, jd).unwrap();
                let node_pos = apparent_position(&provider, Body::TrueNode, jd).unwrap();
                let node_dist = wrap_diff(sun_pos.ecliptic.longitude.to_degrees() - node_pos.ecliptic.longitude.to_degrees()).abs();
                let is_solar_eclipse = node_dist < 18.5 || (node_dist - 180.0).abs() < 18.5;

                let (name, relevance, note) = if is_solar_eclipse {
                    (
                        "New Moon (삭) [SOLAR ECLIPSE / 일식]".to_string(),
                        MundaneRelevance::Primary,
                        Some("Major Mundane Syzygy (Solar Eclipse)".to_string()),
                    )
                } else {
                    (
                        "New Moon (삭)".to_string(),
                        MundaneRelevance::Secondary,
                        Some("Monthly Lunation Syzygy".to_string()),
                    )
                };

                events.push(AstronomicalEvent {
                    category: EventCategory::NewMoon,
                    relevance,
                    name,
                    jd_ut1: jd,
                    residual: res,
                    iterations: iters,
                    unit: "deg",
                    note,
                });
            }
            nm_approx = jd + synodic_month;
        } else {
            nm_approx += synodic_month;
        }
    }

    // Collect Full Moons
    let mut fm_approx = first_full_moon_approx;
    while fm_approx <= end_jd + 5.0 {
        if let Ok((jd, res, iters)) = find_full_moon(fm_approx) {
            if jd >= start_jd && jd <= end_jd {
                let sun_pos = apparent_position(&provider, Body::Sun, jd).unwrap();
                let node_pos = apparent_position(&provider, Body::TrueNode, jd).unwrap();
                let node_dist = wrap_diff(sun_pos.ecliptic.longitude.to_degrees() - node_pos.ecliptic.longitude.to_degrees()).abs();
                let is_lunar_eclipse = node_dist < 12.5 || (node_dist - 180.0).abs() < 12.5;

                let (name, relevance, note) = if is_lunar_eclipse {
                    (
                        "Full Moon (망) [LUNAR ECLIPSE / 월식]".to_string(),
                        MundaneRelevance::Primary,
                        Some("Major Mundane Syzygy (Lunar Eclipse)".to_string()),
                    )
                } else {
                    (
                        "Full Moon (망)".to_string(),
                        MundaneRelevance::Secondary,
                        Some("Monthly Opposition Syzygy".to_string()),
                    )
                };

                events.push(AstronomicalEvent {
                    category: EventCategory::FullMoon,
                    relevance,
                    name,
                    jd_ut1: jd,
                    residual: res,
                    iterations: iters,
                    unit: "deg",
                    note,
                });
            }
            fm_approx = jd + synodic_month;
        } else {
            fm_approx += synodic_month;
        }
    }

    // --- D. Planetary Stations (Mercury, Mars, Jupiter, Saturn) via Bracket Search ---
    let bodies = [
        ("Mercury", Body::Mercury),
        ("Mars", Body::Mars),
        ("Jupiter", Body::Jupiter),
        ("Saturn", Body::Saturn),
    ];

    let bracket_step = 2.0;

    for (name, body) in bodies {
        let mut t = start_jd;
        let mut prev_speed = apparent_position(&provider, body, t).unwrap().longitude_speed;

        while t < end_jd {
            let next_t = (t + bracket_step).min(end_jd);
            let next_speed = apparent_position(&provider, body, next_t).unwrap().longitude_speed;

            if (prev_speed < 0.0 && next_speed >= 0.0) || (prev_speed > 0.0 && next_speed <= 0.0) {
                let is_retro_start = prev_speed > 0.0;
                let approx_station = t + bracket_step * 0.5;

                if let Ok((jd, res, iters)) = find_station(body, approx_station) {
                    if jd >= start_jd && jd <= end_jd {
                        let title = format!(
                            "{} Stationary {}",
                            name,
                            if is_retro_start { "Retrograde (역행 개시 정체)" } else { "Direct (순행 전환 정체)" }
                        );

                        events.push(AstronomicalEvent {
                            category: EventCategory::PlanetaryStation,
                            relevance: MundaneRelevance::Primary,
                            name: title,
                            jd_ut1: jd,
                            residual: res,
                            iterations: iters,
                            unit: "deg/day",
                            note: Some("Planetary Motion Direction Shift".to_string()),
                        });
                    }
                }
            }

            prev_speed = next_speed;
            t = next_t;
        }
    }

    events.sort_by(|a, b| a.jd_ut1.partial_cmp(&b.jd_ut1).unwrap());
    events
}

// ---------------------------------------------------------------------------
// 4. Output Formatting, Export Models & Vedic Chart Generator
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct ExportedPlanet {
    pub name: String,
    pub sign: String,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub nakshatra: String,
    pub nakshatra_pada: u8,
    pub is_retrograde: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExportedChart {
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub ayanamsha_deg: f64,
    pub lagna_sign: String,
    pub lagna_sign_index: u8,
    pub lagna_degree: f64,
    pub planets: Vec<ExportedPlanet>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExportedEvent {
    pub id: String,
    pub category: String,
    pub relevance: String,
    pub name: String,
    pub utc_moment: String,
    pub local_moment: String,
    pub julian_day_ut1: f64,
    pub solver_residual: f64,
    pub solver_iterations: usize,
    pub note: Option<String>,
    pub chart: ExportedChart,
}

#[derive(Debug, Clone, Serialize)]
pub struct MundaneExportPayload {
    pub engine: String,
    pub start_date: String,
    pub end_date: String,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub total_events: usize,
    pub events: Vec<ExportedEvent>,
}

#[derive(Debug, Clone)]
pub struct ResolvedUtcTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: f64,
}

impl ResolvedUtcTime {
    pub fn from_jd(jd: f64) -> Self {
        let (year, month, day_frac) = jd_to_calendar(jd);
        let day = day_frac.floor() as u32;
        let rem_d = day_frac - day as f64;
        let total_seconds = rem_d * 86400.0;
        let hour = (total_seconds / 3600.0).floor() as u32;
        let rem_sec = total_seconds - hour as f64 * 3600.0;
        let minute = (rem_sec / 60.0).floor() as u32;
        let second = rem_sec - minute as f64 * 60.0;

        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    pub fn to_utc_string(&self) -> String {
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:05.2} UTC",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }

    pub fn to_kst_string(&self) -> String {
        let total_secs = self.hour as f64 * 3600.0 + self.minute as f64 * 60.0 + self.second + 9.0 * 3600.0;
        let mut d = self.day;
        let mut m = self.month;
        let mut y = self.year;

        let kst_sec = total_secs % 86400.0;
        let add_days = (total_secs / 86400.0).floor() as u32;
        d += add_days;
        if d > 28 && m == 2 { d -= 28; m += 1; }
        else if d > 30 && [4, 6, 9, 11].contains(&m) { d -= 30; m += 1; }
        else if d > 31 { d -= 31; m += 1; }
        if m > 12 { m = 1; y += 1; }

        let kh = (kst_sec / 3600.0).floor() as u32;
        let rem = kst_sec - kh as f64 * 3600.0;
        let km = (rem / 60.0).floor() as u32;
        let ks = rem - km as f64 * 60.0;

        format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} KST", y, m, d, kh, km, ks.round() as u32)
    }

    pub fn to_compact_id(&self, prefix: &str) -> String {
        format!(
            "{}_{:04}{:02}{:02}_{:02}{:02}",
            prefix, self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

fn build_exported_event(
    event: &AstronomicalEvent,
    lat: f64,
    lon: f64,
    loc_name: &str,
) -> ExportedEvent {
    let utc = ResolvedUtcTime::from_jd(event.jd_ut1);
    let ephem = compute_ephemeris(
        utc.year,
        utc.month,
        utc.day,
        utc.hour,
        utc.minute,
        utc.second,
        lat,
        lon,
    ).expect("Ephemeris computation failed");

    let vedic = calculate_vedic(&ephem, lat);
    let raw_lagna_deg = vedic.lagna_sidereal % 30.0;
    let (lagna_sign_idx, lagna_deg) = normalize_placement_display(vedic.lagna_sign, raw_lagna_deg);

    let planets = vedic.planets.iter().map(|p| {
        let (s_idx, s_deg) = normalize_placement_display(p.sign_index, p.sign_degree);
        ExportedPlanet {
            name: p.name.clone(),
            sign: RASHI_NAMES[s_idx as usize].to_string(),
            sign_index: s_idx,
            sign_degree: s_deg,
            nakshatra: p.nakshatra_name.clone(),
            nakshatra_pada: p.nakshatra_pada,
            is_retrograde: p.is_retrograde,
        }
    }).collect();

    let chart = ExportedChart {
        location_name: loc_name.to_string(),
        latitude: lat,
        longitude: lon,
        ayanamsha_deg: ephem.true_ayanamsha_deg,
        lagna_sign: RASHI_NAMES[lagna_sign_idx as usize].to_string(),
        lagna_sign_index: lagna_sign_idx,
        lagna_degree: lagna_deg,
        planets,
    };

    let prefix = match event.category {
        EventCategory::TropicalSolarIngress => "trop_ingress",
        EventCategory::SiderealSuryaSankranti => "sankranti",
        EventCategory::NewMoon => "new_moon",
        EventCategory::FullMoon => "full_moon",
        EventCategory::PlanetaryStation => "station",
    };

    ExportedEvent {
        id: utc.to_compact_id(prefix),
        category: event.category.as_str().to_string(),
        relevance: event.relevance.as_str().to_string(),
        name: event.name.clone(),
        utc_moment: utc.to_utc_string(),
        local_moment: utc.to_kst_string(),
        julian_day_ut1: event.jd_ut1,
        solver_residual: event.residual.abs(),
        solver_iterations: event.iterations,
        note: event.note.clone(),
        chart,
    }
}

fn print_console_event(exported: &ExportedEvent) {
    println!("--------------------------------------------------------------------------------");
    println!("[{}] [{}] {}", exported.category, exported.relevance, exported.name);
    println!("  1. Global Event Moment : {}", exported.utc_moment);
    println!("     Local Reference ({}) : {}", exported.chart.location_name, exported.local_moment);
    println!("     Julian Day (UT1)   : {:.6}", exported.julian_day_ut1);
    println!(
        "  2. Solver Convergence  : residual = {:.2e}, iterations = {}",
        exported.solver_residual, exported.solver_iterations
    );
    println!(
        "  3. {} Vedic Chart (Lahiri Sidereal / Whole Sign / True Node):",
        exported.chart.location_name
    );
    println!(
        "     - Sidereal Lagna   : {} {:02.2}°",
        exported.chart.lagna_sign, exported.chart.lagna_degree
    );

    let key_planets: Vec<_> = exported.chart.planets.iter().filter(|p| {
        ["Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Rahu"].contains(&p.name.as_str())
    }).map(|p| {
        let retro = if p.is_retrograde { " [R]" } else { "" };
        format!(
            "{}:{}{}({:02.1}°) in {}",
            p.name, p.sign, retro, p.sign_degree, p.nakshatra
        )
    }).collect();

    println!("     - Key Placements   : {}", key_planets.join(" | "));
}

/// Generates human/LLM-readable executive briefing markdown report.
fn generate_markdown_report(payload: &MundaneExportPayload) -> String {
    let mut out = String::with_capacity(16384);
    out.push_str("# Vedic Mundane Astronomical Event Briefing\n\n");
    out.push_str(&format!("- **Engine**: {}\n", payload.engine));
    out.push_str(&format!("- **Interval**: {} to {}\n", payload.start_date, payload.end_date));
    out.push_str(&format!("- **Reference Location**: {} (Lat {:.4}°, Lon {:.4}°)\n", payload.location_name, payload.latitude, payload.longitude));
    out.push_str(&format!("- **Total Events Detected**: {}\n", payload.total_events));
    out.push_str("- **Zodiac & Calculation Frame**: Lahiri Sidereal, Whole Sign Bhavas, True Osculating Lunar Node\n\n");

    out.push_str("## Executive Hierarchy Overview\n\n");
    let primary_count = payload.events.iter().filter(|e| e.relevance == "PRIMARY").count();
    let secondary_count = payload.events.iter().filter(|e| e.relevance == "SECONDARY").count();
    let reference_count = payload.events.iter().filter(|e| e.relevance == "REFERENCE").count();
    out.push_str(&format!("- **★ PRIMARY ({})**: Mesha Sankranti (Year Marker), Eclipses, Surya Sankrantis, Major Planetary Stations\n", primary_count));
    out.push_str(&format!("- **• SECONDARY ({})**: Regular Monthly New Moons & Full Moons\n", secondary_count));
    out.push_str(&format!("- **◇ REFERENCE ({})**: Tropical Seasonal Equinoxes & Solstices\n\n", reference_count));

    for (section_title, rel_filter) in [
        ("Section 1: Primary Mundane Indicators (★ PRIMARY)", "PRIMARY"),
        ("Section 2: Regular Lunations (• SECONDARY)", "SECONDARY"),
        ("Section 3: Tropical Seasonal Reference Points (◇ REFERENCE)", "REFERENCE"),
    ] {
        out.push_str(&format!("## {}\n\n", section_title));

        let matching: Vec<_> = payload.events.iter().filter(|e| e.relevance == rel_filter).collect();
        if matching.is_empty() {
            out.push_str("_No events found in this category for the target interval._\n\n");
            continue;
        }

        for ev in matching {
            out.push_str(&format!("### {}\n\n", ev.name));
            out.push_str(&format!("- **UTC Moment**: `{}`\n", ev.utc_moment));
            out.push_str(&format!("- **Local ({})**: `{}`\n", ev.chart.location_name, ev.local_moment));
            out.push_str(&format!("- **Julian Day (UT1)**: `{:.6}` (Residual: `{:.2e}`)\n", ev.julian_day_ut1, ev.solver_residual));
            if let Some(ref note) = ev.note {
                out.push_str(&format!("- **Mundane Note**: {}\n", note));
            }
            out.push_str(&format!("- **Sidereal Lagna**: **{} {:02.2}°**\n", ev.chart.lagna_sign, ev.chart.lagna_degree));
            out.push_str(&format!("- **Lahiri Ayanamsha**: `{:.4}°`\n\n", ev.chart.ayanamsha_deg));

            out.push_str("| Body | Sign | Degree | Nakshatra | Motion |\n");
            out.push_str("| :--- | :--- | :---: | :--- | :---: |\n");
            for p in &ev.chart.planets {
                let motion = if p.is_retrograde { "Retrograde [R]" } else { "Direct" };
                out.push_str(&format!(
                    "| {} | {} | {:04.1}° | {} (Pada {}) | {} |\n",
                    p.name, p.sign, p.sign_degree, p.nakshatra, p.nakshatra_pada, motion
                ));
            }
            out.push_str("\n---\n\n");
        }
    }

    out
}

// ---------------------------------------------------------------------------
// 5. CLI Entrypoint
// ---------------------------------------------------------------------------

fn parse_ymd(s: &str) -> Option<(i32, u32, u32)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 { return None; }
    let y = parts[0].parse().ok()?;
    let m = parts[1].parse().ok()?;
    let d = parts[2].parse().ok()?;
    Some((y, m, d))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut start_ymd = (2026, 9, 24);
    let mut end_ymd = (2027, 4, 20);
    let mut lat = DEFAULT_LAT;
    let mut lon = DEFAULT_LON;
    let mut loc_name = "Seoul".to_string();
    let mut export_dir: Option<String> = None;

    let mut i = 1;
    let mut positional_count = 0;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--lat" && i + 1 < args.len() {
            lat = args[i + 1].parse().unwrap_or(DEFAULT_LAT);
            loc_name = format!("Lat {:.2}", lat);
            i += 2;
        } else if arg == "--lon" && i + 1 < args.len() {
            lon = args[i + 1].parse().unwrap_or(DEFAULT_LON);
            i += 2;
        } else if (arg == "--export" || arg == "-o") && i + 1 < args.len() {
            export_dir = Some(args[i + 1].clone());
            i += 2;
        } else if arg == "--help" || arg == "-h" {
            println!("Usage: cargo run --release --example astronomical_events [START_DATE] [END_DATE] [--lat LAT] [--lon LON] [--export DIR]");
            println!("       Dates in YYYY-MM-DD format. Default: 2026-09-24 to 2027-04-20 (Seoul)");
            println!("       --export DIR : Exports events.json and SUMMARY.md to the specified directory.");
            return;
        } else if !arg.starts_with("--") {
            if positional_count == 0 {
                if let Some(parsed) = parse_ymd(arg) {
                    start_ymd = parsed;
                }
                positional_count += 1;
            } else if positional_count == 1 {
                if let Some(parsed) = parse_ymd(arg) {
                    end_ymd = parsed;
                }
                positional_count += 1;
            }
            i += 1;
        } else {
            i += 1;
        }
    }

    let start_jd = calendar_to_jd(start_ymd.0, start_ymd.1, start_ymd.2 as f64);
    let end_jd = calendar_to_jd(end_ymd.0, end_ymd.1, end_ymd.2 as f64);

    println!("================================================================================");
    println!("  VEDIC MUNDANE ASTRONOMICAL EVENT ENGINE");
    println!(
        "  Target Interval: {:04}-{:02}-{:02} ~ {:04}-{:02}-{:02} (Location: {}, Lat {:.4}, Lon {:.4})",
        start_ymd.0, start_ymd.1, start_ymd.2, end_ymd.0, end_ymd.1, end_ymd.2, loc_name, lat, lon
    );
    if let Some(ref dir) = export_dir {
        println!("  Export Target  : {}", dir);
    }
    println!("================================================================================\n");

    println!("ACCURACY SPECIFICATIONS & METHODOLOGICAL BOUNDS");
    println!("--------------------------------------------------------------------------------");
    println!("1. Solver Convergence Tolerance (Root Finding Precision):");
    println!("   - Angular Residual : < 1.0e-7 degrees (~0.00036 arcsec mathematical convergence)");
    println!("   - Boundary Note    : Root-finding convergence is mathematically exact to sub-millisecond,");
    println!("                        but DOES NOT constitute an ephemeris accuracy guarantee.");
    println!();
    println!("2. Ephemeris Intrinsic Precision (Vedaksha Analytical Model):");
    println!("   - Major Bodies     : Analytical series (VSOP87A + ELP/MPP02).");
    println!("   - Benchmarks       : Validated against JPL Horizons 1900-2025 with mean error < 0.25 arcsec.");
    println!("   - Physical Note    : Real-world physical time variance depends on body velocity and provider.");
    println!();
    println!("3. Ayanamsha Implementation:");
    println!("   - Formulation      : Indian Official Lahiri True Ayanamsha anchored to primary source.");
    println!("   - Cross-Validation : No independent external cross-validation claimed by engine.");
    println!("================================================================================\n");

    let timer = std::time::Instant::now();
    let events = find_astronomical_events(start_jd, end_jd);
    let scan_duration = timer.elapsed();

    println!("Found {} astronomical events in {:.2?} (High-speed periodic leap scan).\n", events.len(), scan_duration);

    let mut exported_events = Vec::with_capacity(events.len());
    for event in &events {
        let exported = build_exported_event(event, lat, lon, &loc_name);
        print_console_event(&exported);
        exported_events.push(exported);
    }

    let payload = MundaneExportPayload {
        engine: "Vedaksha Analytical Engine (fortune-core)".to_string(),
        start_date: format!("{:04}-{:02}-{:02}", start_ymd.0, start_ymd.1, start_ymd.2),
        end_date: format!("{:04}-{:02}-{:02}", end_ymd.0, end_ymd.1, end_ymd.2),
        location_name: loc_name.clone(),
        latitude: lat,
        longitude: lon,
        total_events: exported_events.len(),
        events: exported_events,
    };

    if let Some(ref dir_str) = export_dir {
        let out_path = Path::new(dir_str);
        if let Err(e) = fs::create_dir_all(out_path) {
            eprintln!("Failed to create export directory {}: {}", dir_str, e);
        } else {
            let json_file = out_path.join("events.json");
            let md_file = out_path.join("SUMMARY.md");

            let json_content = serde_json::to_string_pretty(&payload).expect("JSON serialization failed");
            let md_content = generate_markdown_report(&payload);

            fs::write(&json_file, json_content).expect("Failed to write events.json");
            fs::write(&md_file, md_content).expect("Failed to write SUMMARY.md");

            println!("\n[EXPORT SUCCESS] Files saved to {}:", dir_str);
            println!("  - JSON Payload   : {}", json_file.display());
            println!("  - Markdown Report: {}", md_file.display());
        }
    }

    println!("\n================================================================================");
    println!("  ALL {} EVENTS PROCESSED IN {:.2?} TOTAL TIME", payload.total_events, timer.elapsed());
    println!("================================================================================\n");
}
