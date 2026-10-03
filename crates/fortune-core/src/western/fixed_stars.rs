//! 16 Major Traditional Fixed Stars with Precession and Proper Motion.
//!
//! Propagated dynamically to any epoch using Hipparcos catalogue astrometry
//! via `vedaksha_ephem_core::stars::mean_ecliptic_of_date`.

use serde::{Deserialize, Serialize};
use vedaksha::ephem::stars::{CatalogueStar, mean_ecliptic_of_date};

const HIPPARCOS_EPOCH_JD: f64 = 2_448_349.062_5;

struct FixedStarDef {
    name: &'static str,
    korean_name: &'static str,
    nature: &'static str,
    meaning: &'static str,
    star: CatalogueStar,
}

const TRADITIONAL_STARS: &[FixedStarDef] = &[
    FixedStarDef {
        name: "Algol",
        korean_name: "알골",
        nature: "Saturn / Mars",
        meaning: "Losing head, extreme danger, misfortune, decapitation, sudden destruction",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 14576",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 47.042_213,
            dec_deg: 40.955_648,
            pm_ra_cosdec_mas_per_year: 2.47,
            pm_dec_mas_per_year: -1.41,
        },
    },
    FixedStarDef {
        name: "Alcyone (Pleiades)",
        korean_name: "알키오네 (플레이아데스)",
        nature: "Moon / Mars",
        meaning: "Blindness, weeping, eye afflictions, ambition followed by downfall",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 17702",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 56.871_145,
            dec_deg: 24.105_138,
            pm_ra_cosdec_mas_per_year: 19.34,
            pm_dec_mas_per_year: -43.67,
        },
    },
    FixedStarDef {
        name: "Aldebaran",
        korean_name: "알데바란 (동쪽의 수호자)",
        nature: "Mars",
        meaning: "Royal star of the East, great honor, military success, courage, danger of quarrel",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 21421",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 68.980_163,
            dec_deg: 16.509_302,
            pm_ra_cosdec_mas_per_year: 62.78,
            pm_dec_mas_per_year: -189.36,
        },
    },
    FixedStarDef {
        name: "Rigel",
        korean_name: "리겔",
        nature: "Jupiter / Mars",
        meaning: "Great honor, mechanical ability, rapid rise, lasting fame",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 24436",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 78.634_467,
            dec_deg: -8.201_638,
            pm_ra_cosdec_mas_per_year: 1.87,
            pm_dec_mas_per_year: -0.56,
        },
    },
    FixedStarDef {
        name: "Capella",
        korean_name: "카펠라",
        nature: "Mars / Mercury",
        meaning: "Curiosity, thirst for knowledge, civic honors, wealth",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 24608",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 79.172_328,
            dec_deg: 45.997_991,
            pm_ra_cosdec_mas_per_year: 75.52,
            pm_dec_mas_per_year: -427.11,
        },
    },
    FixedStarDef {
        name: "Betelgeuse",
        korean_name: "베텔게우스",
        nature: "Mars / Mercury",
        meaning: "Great fortune, martial honors, brilliance, sudden wealth",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 27989",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 88.792_939,
            dec_deg: 7.407_064,
            pm_ra_cosdec_mas_per_year: 27.33,
            pm_dec_mas_per_year: 10.86,
        },
    },
    FixedStarDef {
        name: "Sirius",
        korean_name: "시리우스",
        nature: "Jupiter / Mars",
        meaning: "Brightest star, vast ambition, guardianship, renown, bite of passion",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 32349",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 101.287_155,
            dec_deg: -16.716_116,
            pm_ra_cosdec_mas_per_year: -546.01,
            pm_dec_mas_per_year: -1223.07,
        },
    },
    FixedStarDef {
        name: "Castor",
        korean_name: "카스토르",
        nature: "Mercury",
        meaning: "Intellectual brilliance, writing, sudden downfall, sudden blindness or bone injury",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 36850",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 113.649_432,
            dec_deg: 31.888_315,
            pm_ra_cosdec_mas_per_year: -192.40,
            pm_dec_mas_per_year: -146.74,
        },
    },
    FixedStarDef {
        name: "Pollux",
        korean_name: "폴룩스",
        nature: "Mars",
        meaning: "Pugnacity, rashness, cruelty, martial courage",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 37826",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 116.328_957,
            dec_deg: 28.026_199,
            pm_ra_cosdec_mas_per_year: -625.69,
            pm_dec_mas_per_year: -45.94,
        },
    },
    FixedStarDef {
        name: "Procyon",
        korean_name: "프로키온",
        nature: "Mercury / Mars",
        meaning: "Impatience, petulance, sudden wealth followed by downfall",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 37279",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 114.825_494,
            dec_deg: 5.224_993,
            pm_ra_cosdec_mas_per_year: -716.57,
            pm_dec_mas_per_year: -1034.58,
        },
    },
    FixedStarDef {
        name: "Regulus",
        korean_name: "레굴루스 (북쪽의 수호자)",
        nature: "Mars / Jupiter",
        meaning: "Royal star of the North, supreme dignity, high command, ultimate fall if revenge taken",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 49669",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 152.092_962,
            dec_deg: 11.967_209,
            pm_ra_cosdec_mas_per_year: -249.40,
            pm_dec_mas_per_year: 4.88,
        },
    },
    FixedStarDef {
        name: "Vindemiatrix",
        korean_name: "빈데미아트릭스 (과부의 별)",
        nature: "Saturn / Mercury",
        meaning: "Widowhood, loss of partner, deception, depression",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 63608",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 195.544_525,
            dec_deg: 10.959_917,
            pm_ra_cosdec_mas_per_year: -270.44,
            pm_dec_mas_per_year: 14.52,
        },
    },
    FixedStarDef {
        name: "Spica",
        korean_name: "스피카",
        nature: "Venus / Mars",
        meaning: "Supreme beneficence, lasting talent, honor, protection against misfortune",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 65474",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 201.298_352,
            dec_deg: -11.161_245,
            pm_ra_cosdec_mas_per_year: -42.35,
            pm_dec_mas_per_year: -30.67,
        },
    },
    FixedStarDef {
        name: "Antares",
        korean_name: "안타레스 (서쪽의 수호자)",
        nature: "Mars / Jupiter",
        meaning: "Royal star of the West, destructive rage, military daring, danger of sudden ruin",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 80763",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 247.351_915,
            dec_deg: -26.432_002,
            pm_ra_cosdec_mas_per_year: -10.16,
            pm_dec_mas_per_year: -23.21,
        },
    },
    FixedStarDef {
        name: "Vega",
        korean_name: "베가 (직녀성)",
        nature: "Venus / Mercury",
        meaning: "Music, poetry, magical charm, artistic genius, transient prosperity",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 91262",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 279.234_735,
            dec_deg: 38.783_689,
            pm_ra_cosdec_mas_per_year: 200.94,
            pm_dec_mas_per_year: 286.23,
        },
    },
    FixedStarDef {
        name: "Fomalhaut",
        korean_name: "포말하우트 (남쪽의 수호자)",
        nature: "Venus / Mercury",
        meaning: "Royal star of the South, spiritual power, noble vision, transmutation of sorrow",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 113368",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 344.412_693,
            dec_deg: -29.622_237,
            pm_ra_cosdec_mas_per_year: 329.22,
            pm_dec_mas_per_year: -164.22,
        },
    },
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedStarInfo {
    pub name: String,
    pub korean_name: String,
    pub longitude: f64,
    pub sign_index: u8,
    pub sign_degree: f64,
    pub nature: String,
    pub meaning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedStarConjunction {
    pub star_name: String,
    pub korean_name: String,
    pub body_name: String,
    pub star_longitude: f64,
    pub body_longitude: f64,
    pub orb: f64,
    pub nature: String,
    pub meaning: String,
}

/// Compute all 16 traditional fixed star positions at a given Julian Day (TT).
pub fn calculate_traditional_fixed_stars(jd_tt: f64) -> Vec<FixedStarInfo> {
    TRADITIONAL_STARS.iter().map(|def| {
        let (lon, _) = mean_ecliptic_of_date(&def.star, jd_tt);
        let sign_index = (lon / 30.0).floor() as u8 % 12;
        let sign_degree = lon - (sign_index as f64 * 30.0);

        FixedStarInfo {
            name: def.name.to_string(),
            korean_name: def.korean_name.to_string(),
            longitude: lon,
            sign_index,
            sign_degree,
            nature: def.nature.to_string(),
            meaning: def.meaning.to_string(),
        }
    }).collect()
}

/// Detect conjunctions between chart bodies (planets, ASC, MC) and fixed stars within max_orb (default 1.5°).
pub fn detect_fixed_star_conjunctions(
    stars: &[FixedStarInfo],
    bodies: &[(&str, f64)], // (name, longitude)
    max_orb: f64,
) -> Vec<FixedStarConjunction> {
    let mut conjunctions = Vec::new();

    for star in stars {
        for &(body_name, body_lon) in bodies {
            let diff = (star.longitude - body_lon).abs();
            let orb = if diff > 180.0 { 360.0 - diff } else { diff };

            if orb <= max_orb {
                conjunctions.push(FixedStarConjunction {
                    star_name: star.name.clone(),
                    korean_name: star.korean_name.clone(),
                    body_name: body_name.to_string(),
                    star_longitude: star.longitude,
                    body_longitude: body_lon,
                    orb,
                    nature: star.nature.clone(),
                    meaning: star.meaning.clone(),
                });
            }
        }
    }

    conjunctions
}
