use vedaksha::ephem::stars::CatalogueStar;

const HIPPARCOS_EPOCH_JD: f64 = 2_448_349.062_5;

pub(crate) struct FixedStarDef {
    pub name: &'static str,
    pub korean_name: &'static str,
    pub magnitude: f64,
    pub nature: &'static str,
    pub meaning: &'static str,
    pub star: CatalogueStar,
}

const fn hip_star(
    identifier: &'static str,
    ra_deg: f64,
    dec_deg: f64,
    pm_ra: f64,
    pm_dec: f64,
) -> CatalogueStar {
    CatalogueStar {
        catalogue: "I/311/hip2",
        identifier,
        epoch_jd_tt: HIPPARCOS_EPOCH_JD,
        ra_deg,
        dec_deg,
        pm_ra_cosdec_mas_per_year: pm_ra,
        pm_dec_mas_per_year: pm_dec,
    }
}

pub(crate) const TRADITIONAL_STARS: &[FixedStarDef] = &[
    FixedStarDef {
        name: "Algol", korean_name: "알골", magnitude: 2.12, nature: "Saturn / Mars",
        meaning: "Losing head, extreme danger, misfortune, decapitation, sudden destruction",
        star: hip_star("HIP 14576", 47.042_213, 40.955_648, 2.47, -1.41),
    },
    FixedStarDef {
        name: "Alcyone (Pleiades)", korean_name: "알키오네 (플레이아데스)", magnitude: 2.87, nature: "Moon / Mars",
        meaning: "Blindness, weeping, eye afflictions, ambition followed by downfall",
        star: hip_star("HIP 17702", 56.871_145, 24.105_138, 19.34, -43.67),
    },
    FixedStarDef {
        name: "Aldebaran", korean_name: "알데바란 (동쪽의 수호자)", magnitude: 0.85, nature: "Mars",
        meaning: "Royal star of the East, great honor, military success, courage, danger of quarrel",
        star: hip_star("HIP 21421", 68.980_163, 16.509_302, 62.78, -189.36),
    },
    FixedStarDef {
        name: "Rigel", korean_name: "리겔", magnitude: 0.13, nature: "Jupiter / Mars",
        meaning: "Great honor, mechanical ability, rapid rise, lasting fame",
        star: hip_star("HIP 24436", 78.634_467, -8.201_638, 1.87, -0.56),
    },
    FixedStarDef {
        name: "Capella", korean_name: "카펠라", magnitude: 0.08, nature: "Mars / Mercury",
        meaning: "Curiosity, thirst for knowledge, civic honors, wealth",
        star: hip_star("HIP 24608", 79.172_328, 45.997_991, 75.52, -427.11),
    },
    FixedStarDef {
        name: "Betelgeuse", korean_name: "베텔게우스", magnitude: 0.50, nature: "Mars / Mercury",
        meaning: "Great fortune, martial honors, brilliance, sudden wealth",
        star: hip_star("HIP 27989", 88.792_939, 7.407_064, 27.33, 10.86),
    },
    FixedStarDef {
        name: "Sirius", korean_name: "시리우스", magnitude: -1.46, nature: "Jupiter / Mars",
        meaning: "Brightest star, vast ambition, guardianship, renown, bite of passion",
        star: hip_star("HIP 32349", 101.287_155, -16.716_116, -546.01, -1223.07),
    },
    FixedStarDef {
        name: "Canopus", korean_name: "카노푸스 (노인성)", magnitude: -0.74, nature: "Saturn / Jupiter",
        meaning: "Second brightest star, longevity, voyages, navigation, conservatism, profound wisdom",
        star: hip_star("HIP 30438", 95.987_877, -52.695_661, 19.99, 23.67),
    },
    FixedStarDef {
        name: "Castor", korean_name: "카스토르", magnitude: 1.58, nature: "Mercury",
        meaning: "Intellectual brilliance, writing, sudden downfall, sudden blindness or bone injury",
        star: hip_star("HIP 36850", 113.649_432, 31.888_315, -192.40, -146.74),
    },
    FixedStarDef {
        name: "Pollux", korean_name: "폴룩스", magnitude: 1.14, nature: "Mars",
        meaning: "Pugnacity, rashness, cruelty, martial courage",
        star: hip_star("HIP 37826", 116.328_957, 28.026_199, -625.69, -45.94),
    },
    FixedStarDef {
        name: "Procyon", korean_name: "프로키온", magnitude: 0.38, nature: "Mercury / Mars",
        meaning: "Impatience, petulance, sudden wealth followed by downfall",
        star: hip_star("HIP 37279", 114.825_494, 5.224_993, -716.57, -1034.58),
    },
    FixedStarDef {
        name: "Praesepe (M44)", korean_name: "프라에세페 (벌집성단)", magnitude: 3.70, nature: "Moon / Mars",
        meaning: "Blindness, eye afflictions, severe defects, fatal disease, sudden tragedy",
        star: hip_star("HIP 42556", 130.112_542, 19.544_806, -36.29, -12.13),
    },
    FixedStarDef {
        name: "Asellus Borealis", korean_name: "아셀루스 보레알리스 (북쪽 당나귀)", magnitude: 4.67, nature: "Mars / Sun",
        meaning: "Heroic boldness, military honors, sudden attacks, eye afflictions",
        star: hip_star("HIP 42806", 130.821_458, 21.468_500, -105.76, -39.16),
    },
    FixedStarDef {
        name: "Asellus Australis", korean_name: "아셀루스 아우스트랄리스 (남쪽 당나귀)", magnitude: 3.94, nature: "Mars / Sun",
        meaning: "Violent fevers, eye afflictions, burns, military courage, danger of quarrel",
        star: hip_star("HIP 42911", 131.171_250, 18.154_308, -18.44, -227.81),
    },
    FixedStarDef {
        name: "Regulus", korean_name: "레굴루스 (북쪽의 수호자)", magnitude: 1.36, nature: "Mars / Jupiter",
        meaning: "Royal star of the North, supreme dignity, high command, ultimate fall if revenge taken",
        star: hip_star("HIP 49669", 152.092_962, 11.967_209, -249.40, 4.88),
    },
    FixedStarDef {
        name: "Algorab", korean_name: "알고라브 (까마귀)", magnitude: 2.59, nature: "Saturn / Mars",
        meaning: "Scavenging, deceit, malice, repulsion, danger of poison and treachery",
        star: hip_star("HIP 60965", 187.466_083, -16.515_417, -210.49, -138.74),
    },
    FixedStarDef {
        name: "Vindemiatrix", korean_name: "빈데미아트릭스 (과부의 별)", magnitude: 2.83, nature: "Saturn / Mercury",
        meaning: "Widowhood, loss of partner, deception, depression",
        star: hip_star("HIP 63608", 195.544_525, 10.959_917, -270.44, 14.52),
    },
    FixedStarDef {
        name: "Spica", korean_name: "스피카", magnitude: 0.98, nature: "Venus / Mars",
        meaning: "Supreme beneficence, lasting talent, honor, protection against misfortune",
        star: hip_star("HIP 65474", 201.298_352, -11.161_245, -42.35, -30.67),
    },
    FixedStarDef {
        name: "Alkaid", korean_name: "알카이드 (파군성)", magnitude: 1.86, nature: "Mars / Saturn",
        meaning: "Destruction, mourning, loss of partner, severe mortality, sudden conflict",
        star: hip_star("HIP 67301", 206.885_157, 49.313_265, -121.17, -14.91),
    },
    FixedStarDef {
        name: "Arcturus", korean_name: "아르크투루스 (목동자리 대길성)", magnitude: -0.05, nature: "Mars / Jupiter",
        meaning: "Fourth brightest star, supreme justice, prosperity, naval honors, protective renown",
        star: hip_star("HIP 69673", 213.915_300, 19.182_410, -1093.45, -1999.40),
    },
    FixedStarDef {
        name: "Alphecca", korean_name: "알페카 (보석/젬마)", magnitude: 2.22, nature: "Venus / Mercury",
        meaning: "Artistic genius, poetic dignity, beloved standing, fruitful honor",
        star: hip_star("HIP 76267", 233.671_954, 26.714_693, -120.27, -89.44),
    },
    FixedStarDef {
        name: "Antares", korean_name: "안타레스 (서쪽의 수호자)", magnitude: 1.06, nature: "Mars / Jupiter",
        meaning: "Royal star of the West, destructive rage, military daring, danger of sudden ruin",
        star: hip_star("HIP 80763", 247.351_915, -26.432_002, -10.16, -23.21),
    },
    FixedStarDef {
        name: "Facies", korean_name: "파키에스 (궁수자리 성단)", magnitude: 5.90, nature: "Sun / Mars",
        meaning: "Extreme ruthless focus, eye afflictions, violent danger, unyielding conviction",
        star: hip_star("HIP 91100", 279.100_000, -23.900_000, -2.0, -5.0),
    },
    FixedStarDef {
        name: "Vega", korean_name: "베가 (직녀성)", magnitude: 0.03, nature: "Venus / Mercury",
        meaning: "Music, poetry, magical charm, artistic genius, transient prosperity",
        star: hip_star("HIP 91262", 279.234_735, 38.783_689, 200.94, 286.23),
    },
    FixedStarDef {
        name: "Altair", korean_name: "알타이르 (견우성/독수리)", magnitude: 0.77, nature: "Mars / Jupiter",
        meaning: "Audacity, unconquerable ambition, martial honors, swift daring",
        star: hip_star("HIP 97649", 297.695_827, 8.868_322, 536.87, 385.54),
    },
    FixedStarDef {
        name: "Deneb", korean_name: "데네브 (백조자리 알파)", magnitude: 1.25, nature: "Venus / Mercury",
        meaning: "Lucid intelligence, scientific scholarship, swift learning, artistic taste",
        star: hip_star("HIP 102098", 310.357_979, 45.280_338, 1.56, 1.55),
    },
    FixedStarDef {
        name: "Deneb Algedi", korean_name: "데넵 알게디 (사법관의 별)", magnitude: 2.85, nature: "Saturn / Jupiter",
        meaning: "Supreme justice, judicial discernment, leadership won through sorrow, enduring victory",
        star: hip_star("HIP 107556", 326.760_195, -16.126_042, 263.76, -296.85),
    },
    FixedStarDef {
        name: "Fomalhaut", korean_name: "포말하우트 (남쪽의 수호자)", magnitude: 1.17, nature: "Venus / Mercury",
        meaning: "Royal star of the South, spiritual power, noble vision, transmutation of sorrow",
        star: hip_star("HIP 113368", 344.412_693, -29.622_237, 329.22, -164.22),
    },
    FixedStarDef {
        name: "Scheat", korean_name: "셰아트 (페가수스자리 베타)", magnitude: 2.42, nature: "Mars / Mercury",
        meaning: "Catastrophic flooding, imprisonment, misfortune, suicide, mental turmoil, acute peril",
        star: hip_star("HIP 113881", 345.943_564, 28.082_789, 187.67, 137.69),
    },
    FixedStarDef {
        name: "Markab", korean_name: "마르카브 (페가수스자리 알파)", magnitude: 2.49, nature: "Mars / Mercury",
        meaning: "Dignity, honor, brilliant mind, danger of fires and weapon wounds, sudden ambition",
        star: hip_star("HIP 113963", 346.190_214, 15.205_267, 60.40, -41.30),
    },
    FixedStarDef {
        name: "Achernar", korean_name: "아케르나르 (강의 끝)", magnitude: 0.45, nature: "Jupiter",
        meaning: "High religious office, supreme beneficence, philosophical honor, success",
        star: hip_star("HIP 7588", 24.428_524, -57.236_754, 88.02, -40.08),
    },
];
