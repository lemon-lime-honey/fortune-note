use serde::{Deserialize, Serialize};
use super::normalize_western_body_name;

/// Convert ecliptic longitude (at ecliptic latitude = 0) to equatorial coordinates (RA, Dec).
pub fn ecliptic_to_equatorial(longitude_deg: f64, obliquity_deg: f64) -> (f64, f64) {
    let l_rad = longitude_deg.to_radians();
    let eps_rad = obliquity_deg.to_radians();

    let sin_delta = eps_rad.sin() * l_rad.sin();
    let delta_rad = sin_delta.clamp(-1.0, 1.0).asin();
    let delta_deg = delta_rad.to_degrees();

    let alpha_rad = (eps_rad.cos() * l_rad.sin()).atan2(l_rad.cos());
    let alpha_deg = alpha_rad.to_degrees().rem_euclid(360.0);

    (alpha_deg, delta_deg)
}

/// Calculate ascensional difference (AD) in degrees: sin(AD) = tan(phi) * tan(delta).
pub fn ascensional_difference(declination_deg: f64, latitude_deg: f64) -> f64 {
    let tan_phi = latitude_deg.to_radians().tan();
    let tan_delta = declination_deg.to_radians().tan();
    let product = (tan_phi * tan_delta).clamp(-1.0, 1.0);
    product.asin().to_degrees()
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

/// Computes Ptolemaic / Placidean Primary Directions with equatorial arcs and Naibod key.
/// Formatted and windowed around `target_age` to prevent the infant-direction lock defect.
pub fn calculate_primary_directions(
    natal_planets: &[(&str, f64)],
    natal_ascendant: f64,
    natal_midheaven: f64,
    latitude: f64,
    obliquity_deg: f64,
    target_age: f64,
    max_age: f64,
) -> Vec<PrimaryDirectionContact> {
    let obliquity = if obliquity_deg > 0.0 { obliquity_deg } else { 23.4392911 };
    let naibod_key = 59.0 / 60.0 + 8.0 / 3600.0; // 0.985647 deg per tropical year
    let max_arc = max_age * naibod_key;

    // 1. Compute equatorial coordinates for primary significators
    let (ramc, _) = ecliptic_to_equatorial(natal_midheaven, obliquity);
    // In celestial mechanics, OA of Ascendant = RA - AD of natal Ascendant
    let (ra_asc, dec_asc) = ecliptic_to_equatorial(natal_ascendant, obliquity);
    let ad_asc = ascensional_difference(dec_asc, latitude);
    let oa_asc = oblique_ascension(ra_asc, ad_asc);

    let aspects: [(f64, &str); 5] = [
        (0.0, "Conjunction"),
        (60.0, "Sextile"),
        (90.0, "Square"),
        (120.0, "Trine"),
        (180.0, "Opposition"),
    ];

    let mut all_directions = Vec::new();

    for &(prom_raw_name, prom_lon) in natal_planets {
        let prom_name = normalize_western_body_name(prom_raw_name);

        for (angle, asp_name) in aspects {
            // Test both forward and backward aspect rays (except for 0° and 180°)
            let ray_offsets = if angle == 0.0 || angle == 180.0 {
                vec![angle]
            } else {
                vec![angle, 360.0 - angle]
            };

            for ray_offset in ray_offsets {
                let asp_ecliptic_lon = (prom_lon + ray_offset).rem_euclid(360.0);
                let (ra_asp, dec_asp) = ecliptic_to_equatorial(asp_ecliptic_lon, obliquity);
                let ad_asp = ascensional_difference(dec_asp, latitude);
                let oa_asp = oblique_ascension(ra_asp, ad_asp);

                // A. Direct direction to MC: Diurnal motion carries promittor to Meridian
                let arc_mc = (ra_asp - ramc).rem_euclid(360.0);
                if arc_mc > 0.05 && arc_mc <= max_arc && arc_mc <= 110.0 {
                    let age = arc_mc / naibod_key;
                    all_directions.push(PrimaryDirectionContact {
                        promittor: prom_name.clone(),
                        significator: "MC".to_string(),
                        aspect_type: asp_name.to_string(),
                        direction_arc_deg: arc_mc,
                        realization_age: age,
                    });
                }

                // Converse direction to MC: Diurnal motion carries MC to promittor
                let arc_mc_conv = (ramc - ra_asp).rem_euclid(360.0);
                if arc_mc_conv > 0.05 && arc_mc_conv <= max_arc && arc_mc_conv <= 110.0 {
                    let age = arc_mc_conv / naibod_key;
                    all_directions.push(PrimaryDirectionContact {
                        promittor: prom_name.clone(),
                        significator: "MC".to_string(),
                        aspect_type: format!("{} (Converse)", asp_name),
                        direction_arc_deg: arc_mc_conv,
                        realization_age: age,
                    });
                }

                // B. Direct direction to ASC: Diurnal motion carries promittor to Horizon
                let arc_asc = (oa_asp - oa_asc).rem_euclid(360.0);
                if arc_asc > 0.05 && arc_asc <= max_arc && arc_asc <= 110.0 {
                    let age = arc_asc / naibod_key;
                    all_directions.push(PrimaryDirectionContact {
                        promittor: prom_name.clone(),
                        significator: "ASC".to_string(),
                        aspect_type: asp_name.to_string(),
                        direction_arc_deg: arc_asc,
                        realization_age: age,
                    });
                }

                // Converse direction to ASC: Diurnal motion carries ASC to promittor
                let arc_asc_conv = (oa_asc - oa_asp).rem_euclid(360.0);
                if arc_asc_conv > 0.05 && arc_asc_conv <= max_arc && arc_asc_conv <= 110.0 {
                    let age = arc_asc_conv / naibod_key;
                    all_directions.push(PrimaryDirectionContact {
                        promittor: prom_name.clone(),
                        significator: "ASC".to_string(),
                        aspect_type: format!("{} (Converse)", asp_name),
                        direction_arc_deg: arc_asc_conv,
                        realization_age: age,
                    });
                }
            }
        }
    }

    // Deduplicate any duplicate contacts that might arise from ray symmetry
    all_directions.sort_by(|a, b| {
        a.realization_age.partial_cmp(&b.realization_age).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut unique_directions: Vec<PrimaryDirectionContact> = Vec::new();
    for d in all_directions {
        let is_dup = unique_directions.iter().any(|u| {
            u.promittor == d.promittor
                && u.significator == d.significator
                && u.aspect_type == d.aspect_type
                && (u.direction_arc_deg - d.direction_arc_deg).abs() < 0.01
        });
        if !is_dup {
            unique_directions.push(d);
        }
    }

    // Window around target_age: prioritize directions closest to target_age,
    // breaking ties by lower realization_age
    unique_directions.sort_by(|a, b| {
        let diff_a = (a.realization_age - target_age).abs();
        let diff_b = (b.realization_age - target_age).abs();
        diff_a.partial_cmp(&diff_b)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.realization_age.partial_cmp(&b.realization_age).unwrap_or(std::cmp::Ordering::Equal))
    });

    unique_directions
}
