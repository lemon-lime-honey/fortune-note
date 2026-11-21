use serde::{Deserialize, Serialize};

pub use crate::celestial::ecliptic_to_equatorial;
use crate::celestial::{
    normalize_western_body_name, MEAN_OBLIQUITY_J2000_DEG, NAIBOD_DAILY_RATE_DEG,
};
use super::MAJOR_PTOLEMAIC_ASPECTS;

/// Calculate ascensional difference (AD) in degrees: sin(AD) = tan(phi) * tan(delta).
pub fn ascensional_difference(declination_deg: f64, latitude_deg: f64) -> f64 {
    let tan_phi = latitude_deg.to_radians().tan();
    let tan_delta = declination_deg.to_radians().tan();
    (tan_phi * tan_delta).clamp(-1.0, 1.0).asin().to_degrees()
}

/// Calculate Oblique Ascension (OA): OA = RA - AD.
pub fn oblique_ascension(ra_deg: f64, ad_deg: f64) -> f64 {
    (ra_deg - ad_deg).rem_euclid(360.0)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryDirectionContact {
    pub promittor: String,
    pub significator: String,
    pub aspect_type: String,
    pub direction_arc_deg: f64,
    pub realization_age: f64,
}

#[derive(Clone, Copy)]
struct DirectionWindow {
    ramc: f64,
    oa_asc: f64,
    max_arc: f64,
}

fn push_contact_if_in_range(
    out: &mut Vec<PrimaryDirectionContact>,
    promittor: &str,
    significator: &str,
    aspect_type: String,
    arc: f64,
    max_arc: f64,
) {
    if arc > 0.05 && arc <= max_arc && arc <= 110.0 {
        out.push(PrimaryDirectionContact {
            promittor: promittor.to_string(),
            significator: significator.to_string(),
            aspect_type,
            direction_arc_deg: arc,
            realization_age: arc / NAIBOD_DAILY_RATE_DEG,
        });
    }
}

fn collect_ray_directions(
    out: &mut Vec<PrimaryDirectionContact>,
    prom_name: &str,
    asp_name: &str,
    ra_asp: f64,
    oa_asp: f64,
    win: DirectionWindow,
) {
    let conv_label = format!("{} (Converse)", asp_name);
    push_contact_if_in_range(out, prom_name, "MC", asp_name.to_string(), (ra_asp - win.ramc).rem_euclid(360.0), win.max_arc);
    push_contact_if_in_range(out, prom_name, "MC", conv_label.clone(), (win.ramc - ra_asp).rem_euclid(360.0), win.max_arc);
    push_contact_if_in_range(out, prom_name, "ASC", asp_name.to_string(), (oa_asp - win.oa_asc).rem_euclid(360.0), win.max_arc);
    push_contact_if_in_range(out, prom_name, "ASC", conv_label, (win.oa_asc - oa_asp).rem_euclid(360.0), win.max_arc);
}

fn deduplicate_and_window(
    mut all_directions: Vec<PrimaryDirectionContact>,
    target_age: f64,
) -> Vec<PrimaryDirectionContact> {
    all_directions.sort_by(|a, b| {
        a.realization_age.partial_cmp(&b.realization_age).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut unique: Vec<PrimaryDirectionContact> = Vec::new();
    for d in all_directions {
        let is_dup = unique.iter().any(|u| {
            u.promittor == d.promittor
                && u.significator == d.significator
                && u.aspect_type == d.aspect_type
                && (u.direction_arc_deg - d.direction_arc_deg).abs() < 0.01
        });
        if !is_dup {
            unique.push(d);
        }
    }

    unique.sort_by(|a, b| {
        let diff_a = (a.realization_age - target_age).abs();
        let diff_b = (b.realization_age - target_age).abs();
        diff_a
            .partial_cmp(&diff_b)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.realization_age.partial_cmp(&b.realization_age).unwrap_or(std::cmp::Ordering::Equal))
    });
    unique
}

/// Computes Ptolemaic / Placidean Primary Directions with equatorial arcs and Naibod key.
pub fn calculate_primary_directions(
    natal_planets: &[(&str, f64)],
    natal_ascendant: f64,
    natal_midheaven: f64,
    latitude: f64,
    obliquity_deg: f64,
    target_age: f64,
    max_age: f64,
) -> Vec<PrimaryDirectionContact> {
    let obliquity = if obliquity_deg > 0.0 { obliquity_deg } else { MEAN_OBLIQUITY_J2000_DEG };
    let (ramc, _) = ecliptic_to_equatorial(natal_midheaven, obliquity);
    let (ra_asc, dec_asc) = ecliptic_to_equatorial(natal_ascendant, obliquity);
    let win = DirectionWindow {
        ramc,
        oa_asc: oblique_ascension(ra_asc, ascensional_difference(dec_asc, latitude)),
        max_arc: max_age * NAIBOD_DAILY_RATE_DEG,
    };

    let mut all_directions = Vec::new();
    for &(prom_raw_name, prom_lon) in natal_planets {
        let prom_name = normalize_western_body_name(prom_raw_name);
        for (angle, asp_name) in MAJOR_PTOLEMAIC_ASPECTS {
            let rays: &[f64] = if angle == 0.0 || angle == 180.0 {
                &[angle]
            } else {
                &[angle, 360.0 - angle]
            };
            for &ray_offset in rays {
                let asp_lon = (prom_lon + ray_offset).rem_euclid(360.0);
                let (ra_asp, dec_asp) = ecliptic_to_equatorial(asp_lon, obliquity);
                let oa_asp = oblique_ascension(ra_asp, ascensional_difference(dec_asp, latitude));
                collect_ray_directions(&mut all_directions, &prom_name, asp_name, ra_asp, oa_asp, win);
            }
        }
    }

    deduplicate_and_window(all_directions, target_age)
}
