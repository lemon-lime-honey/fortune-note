//! Divisional charts (Vargas: 16 BPHS Shodashavarga + D6, D8, D11).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use vedaksha::math::angle::normalize_degrees;

use super::{VargaChart, VargaPlacement, VedicPlanet};

/// Supported divisional charts in Fortune Note (16 BPHS Shodashavarga + D6, D8, D11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VargaDivision {
    D1, D2, D3, D4, D6, D7, D8, D9, D10, D11, D12, D16, D20, D24, D27, D30, D40, D45, D60,
}

impl VargaDivision {
    pub const ALL: [VargaDivision; 19] = [
        Self::D1, Self::D2, Self::D3, Self::D4, Self::D6, Self::D7, Self::D8, Self::D9,
        Self::D10, Self::D11, Self::D12, Self::D16, Self::D20, Self::D24, Self::D27,
        Self::D30, Self::D40, Self::D45, Self::D60,
    ];

    pub fn title(self) -> &'static str {
        const TITLES: [(VargaDivision, &str); 19] = [
            (VargaDivision::D1, "Rasi (D1)"),
            (VargaDivision::D2, "Hora (D2)"),
            (VargaDivision::D3, "Drekkana (D3)"),
            (VargaDivision::D4, "Chaturthamsha (D4)"),
            (VargaDivision::D6, "Shashthamsha (D6)"),
            (VargaDivision::D7, "Saptamsha (D7)"),
            (VargaDivision::D8, "Ashtamamsha (D8)"),
            (VargaDivision::D9, "Navamsha (D9)"),
            (VargaDivision::D10, "Dashamsha (D10)"),
            (VargaDivision::D11, "Rudramsha (D11)"),
            (VargaDivision::D12, "Dwadashamsha (D12)"),
            (VargaDivision::D16, "Shodashamsha (D16)"),
            (VargaDivision::D20, "Vimshamsha (D20)"),
            (VargaDivision::D24, "Chaturvimshamsha (D24)"),
            (VargaDivision::D27, "Saptavimshamsha (D27)"),
            (VargaDivision::D30, "Trimshamsha (D30)"),
            (VargaDivision::D40, "Khavedamsha (D40)"),
            (VargaDivision::D45, "Akshavedamsha (D45)"),
            (VargaDivision::D60, "Shashtiamsha (D60)"),
        ];
        TITLES.iter().find(|(d, _)| *d == self).map_or("Varga", |(_, t)| *t)
    }
}

#[derive(Clone, Copy)]
enum UniformVargaRule {
    Hora,
    StepFromSelf(u8),
    OddEvenBase(u8, u8),
    OddEvenOffset(u8, u8),
    ModalityBase([u8; 3]),
    ModalityOffset([u8; 3]),
    ElementBase([u8; 4]),
    ReverseFromAries,
}

#[derive(Clone, Copy)]
struct UniformVargaSpec {
    divisions: u8,
    rule: UniformVargaRule,
}

fn uniform_varga_spec(div: VargaDivision) -> UniformVargaSpec {
    use UniformVargaRule::*;
    const SPECS: [(VargaDivision, UniformVargaSpec); 17] = [
        (VargaDivision::D2, UniformVargaSpec { divisions: 2, rule: Hora }),
        (VargaDivision::D3, UniformVargaSpec { divisions: 3, rule: StepFromSelf(4) }),
        (VargaDivision::D4, UniformVargaSpec { divisions: 4, rule: StepFromSelf(3) }),
        (VargaDivision::D6, UniformVargaSpec { divisions: 6, rule: OddEvenBase(0, 6) }),
        (VargaDivision::D7, UniformVargaSpec { divisions: 7, rule: OddEvenOffset(0, 6) }),
        (VargaDivision::D8, UniformVargaSpec { divisions: 8, rule: ModalityBase([0, 8, 4]) }),
        (VargaDivision::D9, UniformVargaSpec { divisions: 9, rule: ModalityOffset([0, 8, 4]) }),
        (VargaDivision::D10, UniformVargaSpec { divisions: 10, rule: OddEvenOffset(0, 8) }),
        (VargaDivision::D11, UniformVargaSpec { divisions: 11, rule: ReverseFromAries }),
        (VargaDivision::D12, UniformVargaSpec { divisions: 12, rule: StepFromSelf(1) }),
        (VargaDivision::D16, UniformVargaSpec { divisions: 16, rule: ModalityBase([0, 4, 8]) }),
        (VargaDivision::D20, UniformVargaSpec { divisions: 20, rule: ModalityBase([0, 8, 4]) }),
        (VargaDivision::D24, UniformVargaSpec { divisions: 24, rule: OddEvenBase(4, 3) }),
        (VargaDivision::D27, UniformVargaSpec { divisions: 27, rule: ElementBase([0, 3, 6, 9]) }),
        (VargaDivision::D40, UniformVargaSpec { divisions: 40, rule: OddEvenBase(0, 6) }),
        (VargaDivision::D45, UniformVargaSpec { divisions: 45, rule: ModalityBase([0, 4, 8]) }),
        (VargaDivision::D60, UniformVargaSpec { divisions: 60, rule: StepFromSelf(1) }),
    ];
    SPECS
        .iter()
        .find(|(d, _)| *d == div)
        .map(|(_, s)| *s)
        .unwrap_or(UniformVargaSpec { divisions: 1, rule: StepFromSelf(0) })
}

fn apply_uniform_varga_rule(sign_idx: u8, part: u8, rule: UniformVargaRule) -> u8 {
    let is_odd = sign_idx.is_multiple_of(2);
    match rule {
        UniformVargaRule::Hora => if is_odd == (part == 0) { 4 } else { 3 },
        UniformVargaRule::StepFromSelf(step) => (sign_idx + part * step) % 12,
        UniformVargaRule::OddEvenBase(odd, even) => ((if is_odd { odd } else { even }) + part) % 12,
        UniformVargaRule::OddEvenOffset(odd, even) => (sign_idx + (if is_odd { odd } else { even }) + part) % 12,
        UniformVargaRule::ModalityBase(bases) => (bases[(sign_idx % 3) as usize] + part) % 12,
        UniformVargaRule::ModalityOffset(offs) => (sign_idx + offs[(sign_idx % 3) as usize] + part) % 12,
        UniformVargaRule::ElementBase(bases) => (bases[(sign_idx % 4) as usize] + part) % 12,
        UniformVargaRule::ReverseFromAries => ((12 - sign_idx) % 12 + part) % 12,
    }
}

#[inline]
fn uniform_varga_part_and_degree(pos_in_sign: f64, n: u8) -> (u8, f64) {
    let clamped = pos_in_sign.clamp(0.0, 30.0);
    let scaled = clamped * (n as f64);
    let raw_part = scaled / 30.0;
    let rounded = raw_part.round();
    let snapped = if (raw_part - rounded).abs() < 1e-9 { rounded } else { raw_part.floor() };
    let part = (snapped.max(0.0) as u8).min(n.saturating_sub(1));
    let mut degree = scaled - (part as f64 * 30.0);
    if degree.abs() < 1e-9 {
        degree = 0.0;
    }
    (part, degree.clamp(0.0, 30.0))
}

fn trimshamsha_sign_and_degree(sign_idx: u8, pos_in_sign: f64) -> (u8, f64) {
    const ODD_SEGMENTS: [(f64, u8, f64, f64); 5] = [
        (5.0, 0, 0.0, 5.0), (10.0, 10, 5.0, 5.0), (18.0, 8, 10.0, 8.0),
        (25.0, 2, 18.0, 7.0), (31.0, 6, 25.0, 5.0),
    ];
    const EVEN_SEGMENTS: [(f64, u8, f64, f64); 5] = [
        (5.0, 1, 0.0, 5.0), (12.0, 5, 5.0, 7.0), (20.0, 11, 12.0, 8.0),
        (25.0, 9, 20.0, 5.0), (31.0, 7, 25.0, 5.0),
    ];
    let segments = if sign_idx.is_multiple_of(2) { &ODD_SEGMENTS } else { &EVEN_SEGMENTS };
    let (_, sign, start, span) = segments
        .iter()
        .copied()
        .find(|&(upper, _, _, _)| pos_in_sign < upper - 1e-9)
        .unwrap_or(segments[4]);
    let mut degree = ((pos_in_sign - start).max(0.0) / span) * 30.0;
    if degree.abs() < 1e-9 {
        degree = 0.0;
    }
    (sign, degree.clamp(0.0, 30.0))
}

/// Computes the divisional sign index (0..11) and normalized degree in that sign (0.0..30.0).
pub fn calculate_varga_sign_and_degree(sidereal_lon: f64, div: VargaDivision) -> (u8, f64) {
    let norm = normalize_degrees(sidereal_lon);
    let raw_sign = norm / 30.0;
    let rounded_sign = raw_sign.round();
    let snapped = if (raw_sign - rounded_sign).abs() < 1e-11 && rounded_sign < 12.0 {
        rounded_sign
    } else {
        raw_sign.floor()
    };
    let sign_idx = (snapped.max(0.0) as u8).min(11);
    let mut pos_in_sign = (norm - (sign_idx as f64 * 30.0)).clamp(0.0, 30.0);
    if pos_in_sign.abs() < 1e-11 {
        pos_in_sign = 0.0;
    }

    match div {
        VargaDivision::D1 => (sign_idx, pos_in_sign),
        VargaDivision::D30 => trimshamsha_sign_and_degree(sign_idx, pos_in_sign),
        other => {
            let spec = uniform_varga_spec(other);
            let (part, degree) = uniform_varga_part_and_degree(pos_in_sign, spec.divisions);
            (apply_uniform_varga_rule(sign_idx, part, spec.rule), degree)
        }
    }
}

/// Computes the divisional sign index (0..11) for any supported varga division.
pub fn calculate_varga_sign(sidereal_lon: f64, div: VargaDivision) -> u8 {
    calculate_varga_sign_and_degree(sidereal_lon, div).0
}

pub(crate) fn build_all_varga_charts(
    lagna_sidereal: f64,
    rahu_sid_lon: f64,
    planets: &[VedicPlanet],
) -> BTreeMap<VargaDivision, VargaChart> {
    let mut vargas = BTreeMap::new();
    for div in VargaDivision::ALL {
        let (v_lagna, _) = calculate_varga_sign_and_degree(lagna_sidereal, div);
        let (rahu_v_sign, rahu_v_deg) = calculate_varga_sign_and_degree(rahu_sid_lon, div);

        let placements = planets
            .iter()
            .map(|p| {
                let (p_sign, p_degree) = if p.name == "Ketu" && div != VargaDivision::D2 {
                    ((rahu_v_sign + 6) % 12, rahu_v_deg)
                } else {
                    calculate_varga_sign_and_degree(p.sidereal_longitude, div)
                };
                let house = ((p_sign as i16 - v_lagna as i16 + 12) % 12 + 1) as u8;
                VargaPlacement {
                    planet_name: p.name.clone(),
                    sign_index: p_sign,
                    sign_degree: p_degree,
                    house,
                }
            })
            .collect();

        vargas.insert(div, VargaChart { division: div, lagna_sign: v_lagna, placements });
    }
    vargas
}
