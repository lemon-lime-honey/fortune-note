//! William Lilly Medieval / Renaissance Traditional Western Astrology assembler.
//!
//! Extracted from `western/mod.rs` to keep chart orchestration and traditional
//! sub-engine assembly cleanly separated under SRP.

use serde::{Deserialize, Serialize};

use crate::ephemeris::EphemerisSnapshot;
use super::{
    almuten, antiscia, aspects, determine_house_traditional, dignity, fixed_stars, hours, moon,
    parts, reception, syzygy, WesternHouseSystem, WesternPlanet, DEFAULT_FIXED_STAR_MAX_ORB,
};

/// Complete William Lilly Traditional Astrology Analysis Data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WesternTraditionalData {
    pub planet_dignities: Vec<dignity::PlanetDignityReport>,
    pub moon_condition: moon::MoonCondition,
    pub planetary_hours: hours::PlanetaryHoursReport,
    pub traditional_aspects: Vec<aspects::TraditionalAspect>,
    pub antiscia: Vec<antiscia::AntiscionPoint>,
    pub mutual_receptions: Vec<reception::MutualReception>,
    pub perfection: reception::PerfectionReport,
    pub cusp_dignities: Vec<almuten::HouseCuspDignity>,
    pub arabic_parts: Vec<parts::ArabicPart>,
    pub fixed_stars: Vec<fixed_stars::FixedStarInfo>,
    pub fixed_star_conjunctions: Vec<fixed_stars::FixedStarConjunction>,
    pub prenatal_syzygy: Option<syzygy::PrenatalSyzygy>,
}

/// Bundled input context for building medieval traditional data.
pub(crate) struct TraditionalBuildContext<'a> {
    pub ephem: &'a EphemerisSnapshot,
    pub planets: &'a [WesternPlanet],
    pub planet_tuples: &'a [(&'a str, f64, f64)],
    pub planet_pos_tuples: &'a [(&'a str, f64)],
    pub cusps: &'a [f64; 12],
    pub ascendant: f64,
    pub midheaven: f64,
    pub latitude: f64,
    pub system: WesternHouseSystem,
    pub is_day_chart: bool,
    pub is_male: bool,
    pub is_horary: bool,
}

fn body_tropical_lon(ephem: &EphemerisSnapshot, name: &str, default_val: f64) -> f64 {
    ephem.get_body(name).map_or(default_val, |b| b.tropical_longitude)
}

fn compute_fixed_star_bundle(
    ctx: &TraditionalBuildContext<'_>,
) -> (Vec<fixed_stars::FixedStarInfo>, Vec<fixed_stars::FixedStarConjunction>) {
    let fixed_stars = fixed_stars::calculate_traditional_fixed_stars(
        ctx.ephem.jd_tt,
        ctx.latitude,
        ctx.ephem.true_obliquity_deg,
    );
    let dsc = (ctx.ascendant + 180.0).rem_euclid(360.0);
    let ic = (ctx.midheaven + 180.0).rem_euclid(360.0);
    let mut star_check_bodies = ctx.planet_pos_tuples.to_vec();
    star_check_bodies.push(("Ascendant", ctx.ascendant));
    star_check_bodies.push(("Midheaven", ctx.midheaven));
    star_check_bodies.push(("Descendant", dsc));
    star_check_bodies.push(("Imum Coeli", ic));

    let conjunctions = fixed_stars::detect_fixed_star_conjunctions(
        &fixed_stars,
        &star_check_bodies,
        DEFAULT_FIXED_STAR_MAX_ORB,
    );
    (fixed_stars, conjunctions)
}

fn build_planet_dignities(
    ctx: &TraditionalBuildContext<'_>,
    sun_lon: f64,
    conjunctions: &[fixed_stars::FixedStarConjunction],
) -> Vec<dignity::PlanetDignityReport> {
    let north_node_lon = body_tropical_lon(ctx.ephem, "Rahu", 0.0);
    let south_node_lon = body_tropical_lon(ctx.ephem, "Ketu", 180.0);
    let mut reports = Vec::with_capacity(ctx.planets.len());

    for p in ctx.planets {
        if let Some(cp) = dignity::ClassicalPlanet::from_name(&p.name) {
            let trad_house = determine_house_traditional(p.longitude, ctx.cusps, ctx.system);
            reports.push(dignity::calculate_planet_dignity_with_context(
                cp,
                p.longitude,
                p.speed,
                trad_house,
                sun_lon,
                ctx.is_day_chart,
                north_node_lon,
                south_node_lon,
                ctx.planet_pos_tuples,
                conjunctions,
            ));
        }
    }
    reports
}

fn build_planetary_hours(
    ctx: &TraditionalBuildContext<'_>,
    sun_lon: f64,
    moon_condition: &moon::MoonCondition,
) -> hours::PlanetaryHoursReport {
    let saturn_house = ctx
        .ephem
        .get_body("Saturn")
        .map_or(1, |b| determine_house_traditional(b.tropical_longitude, ctx.cusps, ctx.system));

    hours::calculate_astronomical_planetary_hours_with_mode(&hours::AstroPlanetaryHoursInput {
        jd_ut1: ctx.ephem.jd_ut1,
        latitude: ctx.latitude,
        ramc: ctx.ephem.ramc,
        sun_longitude: sun_lon,
        obliquity_deg: ctx.ephem.true_obliquity_deg,
        asc_longitude: ctx.ascendant,
        saturn_house,
        is_moon_voc: moon_condition.is_void_of_course,
        is_moon_via_combusta: moon_condition.is_via_combusta,
        is_horary: ctx.is_horary,
    })
}

fn build_arabic_parts(
    ctx: &TraditionalBuildContext<'_>,
    sun_lon: f64,
    moon_lon: f64,
) -> Vec<parts::ArabicPart> {
    parts::calculate_arabic_parts(
        ctx.ascendant,
        sun_lon,
        moon_lon,
        body_tropical_lon(ctx.ephem, "Mercury", 0.0),
        body_tropical_lon(ctx.ephem, "Venus", 0.0),
        body_tropical_lon(ctx.ephem, "Mars", 0.0),
        body_tropical_lon(ctx.ephem, "Jupiter", 0.0),
        body_tropical_lon(ctx.ephem, "Saturn", 0.0),
        ctx.cusps,
        ctx.is_day_chart,
        ctx.is_male,
    )
}

/// Assemble the complete William Lilly medieval dataset for quadrant house charts.
pub(crate) fn build_traditional_data(ctx: &TraditionalBuildContext<'_>) -> WesternTraditionalData {
    let sun_lon = body_tropical_lon(ctx.ephem, "Sun", 0.0);
    let moon_opt = ctx.ephem.get_body("Moon");
    let moon_lon = moon_opt.map_or(0.0, |b| b.tropical_longitude);
    let moon_speed = moon_opt.map_or(13.176, |b| b.speed);

    let (fixed_stars, fixed_star_conjunctions) = compute_fixed_star_bundle(ctx);
    let planet_dignities = build_planet_dignities(ctx, sun_lon, &fixed_star_conjunctions);

    let moon_condition = if ctx.is_horary {
        moon::evaluate_moon_condition(moon_lon, moon_speed, sun_lon, ctx.planet_tuples)
    } else {
        moon::evaluate_natal_moon_condition(moon_lon, sun_lon)
    };
    let planetary_hours = build_planetary_hours(ctx, sun_lon, &moon_condition);

    let mut antiscia_input = ctx.planet_pos_tuples.to_vec();
    antiscia_input.push(("Ascendant", ctx.ascendant));
    antiscia_input.push(("Midheaven", ctx.midheaven));

    let perfection = if ctx.is_horary {
        reception::analyze_perfection_mechanisms(ctx.planet_tuples)
    } else {
        reception::PerfectionReport::default()
    };

    WesternTraditionalData {
        planet_dignities,
        moon_condition,
        planetary_hours,
        traditional_aspects: aspects::calculate_traditional_aspects(ctx.planet_tuples),
        antiscia: antiscia::calculate_antiscia_table(&antiscia_input),
        mutual_receptions: reception::find_mutual_receptions(ctx.planet_pos_tuples, ctx.is_day_chart),
        perfection,
        cusp_dignities: almuten::calculate_house_cusp_dignities(ctx.cusps, ctx.is_day_chart),
        arabic_parts: build_arabic_parts(ctx, sun_lon, moon_lon),
        fixed_stars,
        fixed_star_conjunctions,
        prenatal_syzygy: syzygy::calculate_prenatal_syzygy_with_coords(ctx.ephem.jd_ut1, sun_lon, moon_lon).ok(),
    }
}
