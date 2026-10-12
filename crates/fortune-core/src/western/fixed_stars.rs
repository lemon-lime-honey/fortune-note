//! 31 Elite Traditional Fixed Stars with Astrometry, Precession, and Geographic Visibility.
//!
//! Exclusively includes authentic historical standards:
//! - 15 Behenian Stars (Medieval & Renaissance talismanic and natal astrology)
//! - 4 Royal Stars of Persia (Aldebaran, Regulus, Antares, Fomalhaut)
//! - Hellenistic 1st-magnitude luminaries (Anonymous of 379, Ptolemy, Vettius Valens)
//! - Malefic clusters and sight-afflicting stars (Praesepe, Aselli, Facies, Algol)
//! - William Lilly's primary natal/horary significators (Vindemiatrix, Scheat, Markab)
//!
//! Propagated dynamically to any epoch using Hipparcos catalogue astrometry (I/311/hip2)
//! via `vedaksha::ephem::stars::mean_ecliptic_of_date`.
//!
//! Includes geographic horizon visibility calculation:
//! Maximum culmination altitude: `Alt_max = 90° - |observer_lat - declination|`.
//! A star is visible if `Alt_max > 0°`; otherwise it is below the horizon (never rises).

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
    // 1. Algol (15 Behenian Stars, Caput Medusae)
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
    // 2. Alcyone / Pleiades (15 Behenian Stars, Nebulous Cluster)
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
    // 3. Aldebaran (15 Behenian Stars, Royal Star of the East)
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
    // 4. Rigel (Hellenistic 1st Magnitude)
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
    // 5. Capella (15 Behenian Stars)
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
    // 6. Betelgeuse (Hellenistic 1st Magnitude)
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
    // 7. Sirius (15 Behenian Stars, Hellenistic #1 Luminary)
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
    // 8. Canopus (Hellenistic #2 Luminary, Alpha Carinae)
    FixedStarDef {
        name: "Canopus",
        korean_name: "카노푸스 (노인성)",
        nature: "Saturn / Jupiter",
        meaning: "Second brightest star, longevity, voyages, navigation, conservatism, profound wisdom",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 30438",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 95.987_877,
            dec_deg: -52.695_661,
            pm_ra_cosdec_mas_per_year: 19.99,
            pm_dec_mas_per_year: 23.67,
        },
    },
    // 9. Castor (Hellenistic Alpha Geminorum)
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
    // 10. Pollux (Hellenistic Beta Geminorum)
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
    // 11. Procyon (15 Behenian Stars, Alpha Canis Minoris)
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
    // 12. Praesepe / M44 (Ancient Malefic Sight-Afflicting Cluster, Epsilon Cancri)
    FixedStarDef {
        name: "Praesepe (M44)",
        korean_name: "프라에세페 (벌집성단)",
        nature: "Moon / Mars",
        meaning: "Blindness, eye afflictions, severe defects, fatal disease, sudden tragedy",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 42556",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 130.112_542,
            dec_deg: 19.544_806,
            pm_ra_cosdec_mas_per_year: -36.29,
            pm_dec_mas_per_year: -12.13,
        },
    },
    // 13. Asellus Borealis (Ancient Malefic Asses of Cancer, Gamma Cancri)
    FixedStarDef {
        name: "Asellus Borealis",
        korean_name: "아셀루스 보레알리스 (북쪽 당나귀)",
        nature: "Mars / Sun",
        meaning: "Heroic boldness, military honors, sudden attacks, eye afflictions",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 42806",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 130.821_458,
            dec_deg: 21.468_500,
            pm_ra_cosdec_mas_per_year: -105.76,
            pm_dec_mas_per_year: -39.16,
        },
    },
    // 14. Asellus Australis (Ancient Malefic Asses of Cancer, Delta Cancri)
    FixedStarDef {
        name: "Asellus Australis",
        korean_name: "아셀루스 아우스트랄리스 (남쪽 당나귀)",
        nature: "Mars / Sun",
        meaning: "Violent fevers, eye afflictions, burns, military courage, danger of quarrel",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 42911",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 131.171_250,
            dec_deg: 18.154_308,
            pm_ra_cosdec_mas_per_year: -18.44,
            pm_dec_mas_per_year: -227.81,
        },
    },
    // 15. Regulus (15 Behenian Stars, Royal Star of the North)
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
    // 16. Algorab (15 Behenian Stars, Delta Corvi)
    FixedStarDef {
        name: "Algorab",
        korean_name: "알고라브 (까마귀)",
        nature: "Saturn / Mars",
        meaning: "Scavenging, deceit, malice, repulsion, danger of poison and treachery",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 60965",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 187.466_083,
            dec_deg: -16.515_417,
            pm_ra_cosdec_mas_per_year: -210.49,
            pm_dec_mas_per_year: -138.74,
        },
    },
    // 17. Vindemiatrix (William Lilly CA Primary Significator, Epsilon Virginis)
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
    // 18. Spica (15 Behenian Stars, Alpha Virginis)
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
    // 19. Alkaid / Benetnasch (15 Behenian Stars, Eta Ursae Majoris)
    FixedStarDef {
        name: "Alkaid",
        korean_name: "알카이드 (파군성)",
        nature: "Mars / Saturn",
        meaning: "Destruction, mourning, loss of partner, severe mortality, sudden conflict",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 67301",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 206.885_157,
            dec_deg: 49.313_265,
            pm_ra_cosdec_mas_per_year: -121.17,
            pm_dec_mas_per_year: -14.91,
        },
    },
    // 20. Arcturus (15 Behenian Stars, Alpha Boötis)
    FixedStarDef {
        name: "Arcturus",
        korean_name: "아르크투루스 (목동자리 대길성)",
        nature: "Mars / Jupiter",
        meaning: "Fourth brightest star, supreme justice, prosperity, naval honors, protective renown",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 69673",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 213.915_300,
            dec_deg: 19.182_410,
            pm_ra_cosdec_mas_per_year: -1093.45,
            pm_dec_mas_per_year: -1999.40,
        },
    },
    // 21. Alphecca / Gemma (15 Behenian Stars, Alpha Coronae Borealis)
    FixedStarDef {
        name: "Alphecca",
        korean_name: "알페카 (보석/젬마)",
        nature: "Venus / Mercury",
        meaning: "Artistic genius, poetic dignity, beloved standing, fruitful honor",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 76267",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 233.671_954,
            dec_deg: 26.714_693,
            pm_ra_cosdec_mas_per_year: -120.27,
            pm_dec_mas_per_year: -89.44,
        },
    },
    // 22. Antares (15 Behenian Stars, Royal Star of the West)
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
    // 23. Facies (Ancient Malefic Nebula in Sagittarius, M22)
    FixedStarDef {
        name: "Facies",
        korean_name: "파키에스 (궁수자리 성단)",
        nature: "Sun / Mars",
        meaning: "Extreme ruthless focus, eye afflictions, violent danger, unyielding conviction",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 91100",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 279.100_000,
            dec_deg: -23.900_000,
            pm_ra_cosdec_mas_per_year: -2.0,
            pm_dec_mas_per_year: -5.0,
        },
    },
    // 24. Vega (15 Behenian Stars, Alpha Lyrae)
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
    // 25. Altair (Hellenistic Alpha Aquilae)
    FixedStarDef {
        name: "Altair",
        korean_name: "알타이르 (견우성/독수리)",
        nature: "Mars / Jupiter",
        meaning: "Audacity, unconquerable ambition, martial honors, swift daring",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 97649",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 297.695_827,
            dec_deg: 8.868_322,
            pm_ra_cosdec_mas_per_year: 536.87,
            pm_dec_mas_per_year: 385.54,
        },
    },
    // 26. Deneb (Hellenistic Alpha Cygni)
    FixedStarDef {
        name: "Deneb",
        korean_name: "데네브 (백조자리 알파)",
        nature: "Venus / Mercury",
        meaning: "Lucid intelligence, scientific scholarship, swift learning, artistic taste",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 102098",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 310.357_979,
            dec_deg: 45.280_338,
            pm_ra_cosdec_mas_per_year: 1.56,
            pm_dec_mas_per_year: 1.55,
        },
    },
    // 27. Deneb Algedi (15 Behenian Stars, Delta Capricorni)
    FixedStarDef {
        name: "Deneb Algedi",
        korean_name: "데넵 알게디 (사법관의 별)",
        nature: "Saturn / Jupiter",
        meaning: "Supreme justice, judicial discernment, leadership won through sorrow, enduring victory",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 107556",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 326.760_195,
            dec_deg: -16.126_042,
            pm_ra_cosdec_mas_per_year: 263.76,
            pm_dec_mas_per_year: -296.85,
        },
    },
    // 28. Fomalhaut (Royal Star of the South, Alpha Piscis Austrini)
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
    // 29. Scheat (William Lilly CA Primary Malefic, Beta Pegasi)
    FixedStarDef {
        name: "Scheat",
        korean_name: "셰아트 (페가수스자리 베타)",
        nature: "Mars / Mercury",
        meaning: "Catastrophic flooding, imprisonment, misfortune, suicide, mental turmoil, acute peril",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 113881",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 345.943_564,
            dec_deg: 28.082_789,
            pm_ra_cosdec_mas_per_year: 187.67,
            pm_dec_mas_per_year: 137.69,
        },
    },
    // 30. Markab (William Lilly CA Primary Significator, Alpha Pegasi)
    FixedStarDef {
        name: "Markab",
        korean_name: "마르카브 (페가수스자리 알파)",
        nature: "Mars / Mercury",
        meaning: "Dignity, honor, brilliant mind, danger of fires and weapon wounds, sudden ambition",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 113963",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 346.190_214,
            dec_deg: 15.205_267,
            pm_ra_cosdec_mas_per_year: 60.40,
            pm_dec_mas_per_year: -41.30,
        },
    },
    // 31. Achernar (Hellenistic 1st Magnitude, Alpha Eridani)
    FixedStarDef {
        name: "Achernar",
        korean_name: "아케르나르 (강의 끝)",
        nature: "Jupiter",
        meaning: "High religious office, supreme beneficence, philosophical honor, success",
        star: CatalogueStar {
            catalogue: "I/311/hip2",
            identifier: "HIP 7588",
            epoch_jd_tt: HIPPARCOS_EPOCH_JD,
            ra_deg: 24.428_524,
            dec_deg: -57.236_754,
            pm_ra_cosdec_mas_per_year: 88.02,
            pm_dec_mas_per_year: -40.08,
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
    pub declination: f64,
    pub is_visible: bool,
    pub max_altitude: f64,
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
    pub is_visible: bool,
    pub max_altitude: f64,
    pub nature: String,
    pub meaning: String,
}

/// Compute the 31 elite traditional fixed star positions and horizon visibility at a given Julian Day (TT)
/// for the observer's geographic latitude.
pub fn calculate_traditional_fixed_stars(jd_tt: f64, observer_lat: f64, obliquity_deg: f64) -> Vec<FixedStarInfo> {
    let eps_rad = obliquity_deg.to_radians();

    TRADITIONAL_STARS.iter().map(|def| {
        let (lon, lat) = mean_ecliptic_of_date(&def.star, jd_tt);
        let sign_index = (lon / 30.0).floor() as u8 % 12;
        let sign_degree = lon - (sign_index as f64 * 30.0);

        let lon_rad = lon.to_radians();
        let lat_rad = lat.to_radians();

        // Convert ecliptic coordinates (lon, lat) to equatorial declination:
        // sin(dec) = sin(lat)*cos(eps) + cos(lat)*sin(eps)*sin(lon)
        let sin_dec = lat_rad.sin() * eps_rad.cos() + lat_rad.cos() * eps_rad.sin() * lon_rad.sin();
        let dec = sin_dec.clamp(-1.0, 1.0).asin().to_degrees();

        // Maximum upper meridian transit altitude at observer's latitude:
        // Alt_max = 90° - |observer_lat - dec|
        let max_altitude = 90.0 - (observer_lat - dec).abs();
        let is_visible = max_altitude > 0.0;

        FixedStarInfo {
            name: def.name.to_string(),
            korean_name: def.korean_name.to_string(),
            longitude: lon,
            sign_index,
            sign_degree,
            declination: dec,
            is_visible,
            max_altitude,
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
                    is_visible: star.is_visible,
                    max_altitude: star.max_altitude,
                    nature: star.nature.clone(),
                    meaning: star.meaning.clone(),
                });
            }
        }
    }

    conjunctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elite_fixed_stars_catalog() {
        let jd = 2447965.677778; // 1990-03-15
        let seoul_lat = 37.5665;
        let obliquity = 23.4425;
        let stars = calculate_traditional_fixed_stars(jd, seoul_lat, obliquity);

        // Exact 31 elite stars
        assert_eq!(stars.len(), 31);

        // Verify 15 Behenian stars are all present
        let behenian = [
            "Algol", "Alcyone (Pleiades)", "Aldebaran", "Capella", "Sirius",
            "Procyon", "Regulus", "Algorab", "Spica", "Alkaid",
            "Arcturus", "Alphecca", "Antares", "Vega", "Deneb Algedi"
        ];
        for b in &behenian {
            assert!(stars.iter().any(|s| s.name == *b), "Missing Behenian star: {}", b);
        }

        // Verify 4 Royal stars
        let royal = ["Aldebaran", "Regulus", "Antares", "Fomalhaut"];
        for r in &royal {
            assert!(stars.iter().any(|s| s.name == *r), "Missing Royal star: {}", r);
        }

        // Verify Malefic clusters
        let clusters = ["Praesepe (M44)", "Asellus Borealis", "Asellus Australis", "Facies"];
        for c in &clusters {
            assert!(stars.iter().any(|s| s.name == *c), "Missing cluster: {}", c);
        }

        // Verify visibility at Seoul (Lat 37.57°):
        // Sirius (Dec ≈ -16.7°) -> Visible (Alt_max ≈ 35.7°)
        let sirius = stars.iter().find(|s| s.name == "Sirius").unwrap();
        assert!(sirius.is_visible);
        assert!(sirius.max_altitude > 30.0);

        // Canopus (Dec ≈ -52.7°) -> Below horizon in Seoul (Lat 37.57°)
        let canopus = stars.iter().find(|s| s.name == "Canopus").unwrap();
        assert!(!canopus.is_visible);
        assert!(canopus.max_altitude < 0.0);

        // Achernar (Dec ≈ -57.2°) -> Below horizon in Seoul
        let achernar = stars.iter().find(|s| s.name == "Achernar").unwrap();
        assert!(!achernar.is_visible);
        assert!(achernar.max_altitude < 0.0);
    }
}
