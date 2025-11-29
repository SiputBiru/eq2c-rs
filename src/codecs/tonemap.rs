use clap::ValueEnum;
use glam::{Mat3, Vec3};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ToneMapType {
    Reinhard,
    Aces,
    Khronos,
    Agx,
    None,
}

pub fn apply_tonemap(color: Vec3, method: ToneMapType) -> Vec3 {
    match method {
        ToneMapType::Reinhard => reinhard(color),
        ToneMapType::Aces => aces_tonemap(color),
        ToneMapType::Khronos => khronos_pbr_neutral(color),
        ToneMapType::Agx => agx_tonemap(color),
        ToneMapType::None => color.clamp(Vec3::ZERO, Vec3::ONE),
    }
}

// --- Algorithms ---

fn reinhard(v: Vec3) -> Vec3 {
    v / (v + 1.0)
}

pub fn aces_tonemap(color: Vec3) -> Vec3 {
    let m1 = Mat3::from_cols_array(&[
        0.59719, 0.07600, 0.02840, 0.35458, 0.90834, 0.13383, 0.04823, 0.01566, 0.83777,
    ]);

    let m2 = Mat3::from_cols_array(&[
        1.60475, -0.10208, -0.00327, -0.53108, 1.10813, -0.07276, -0.07367, -0.00605, 1.07602,
    ]);

    let v = m1 * color;
    let a = v * (v + 0.0245786) - 0.000090537;
    let b = v * (0.983729 * v + 0.432951) + 0.238081;

    let result = m2 * (a / b);

    result.clamp(Vec3::ZERO, Vec3::ONE)
}

// Khronos PBR Neutral Tone Mapper
fn khronos_pbr_neutral(mut color: Vec3) -> Vec3 {
    const START_COMPRESSION: f32 = 0.8 - 0.04;
    const DESATURATION: f32 = 0.15;

    let x = color.min_element();
    let offset = if x < 0.08 { x - 6.25 * x * x } else { 0.04 };

    color -= Vec3::splat(offset);

    let peak = color.max_element();

    if peak < START_COMPRESSION {
        return color + Vec3::splat(offset);
    }

    const D: f32 = 1.0 - START_COMPRESSION;
    let new_peak = 1.0 - D * D / (peak + D - START_COMPRESSION);

    color *= new_peak / peak;
    let g = 1.0 - 1.0 / (DESATURATION * (peak - new_peak) + 1.0);

    color.lerp(Vec3::splat(new_peak), g) + Vec3::splat(offset)
}

// --- AgX Implementation ---

const AGX_INPUT_MAT: Mat3 = Mat3::from_cols_array(&[
    0.84247906,
    0.0784336,
    0.079223745,
    0.04232824,
    0.87846864,
    0.07916613,
    0.04237565,
    0.0784336,
    0.879143,
]);

const AGX_OUTPUT_MAT: Mat3 = Mat3::from_cols_array(&[
    1.196879,
    -0.09802088,
    -0.09902974,
    -0.05289685,
    1.1519031,
    -0.09896118,
    -0.05297163,
    -0.09804345,
    1.1510737,
]);

fn agx_tonemap(color: Vec3) -> Vec3 {
    let val = AGX_INPUT_MAT * color;

    const MIN_EV: f32 = -12.47393;
    const MAX_EV: f32 = 4.026069;

    let val_log = Vec3::new(
        val.x.max(1e-10).log2().clamp(MIN_EV, MAX_EV),
        val.y.max(1e-10).log2().clamp(MIN_EV, MAX_EV),
        val.z.max(1e-10).log2().clamp(MIN_EV, MAX_EV),
    );

    let val_norm = (val_log - MIN_EV) / (MAX_EV - MIN_EV);
    let result = agx_default_contrast_approx(val_norm);
    let linear_result = AGX_OUTPUT_MAT * result;

    linear_result.clamp(Vec3::ZERO, Vec3::ONE)
}

fn agx_default_contrast_approx(x: Vec3) -> Vec3 {
    let x2 = x * x;
    let x4 = x2 * x2;

    (Vec3::splat(15.5) * x4 * x2) - (Vec3::splat(40.14) * x4 * x) + (Vec3::splat(31.96) * x4)
        - (Vec3::splat(6.868) * x2 * x)
        + (Vec3::splat(0.4298) * x2)
        + (Vec3::splat(0.1191) * x)
        - Vec3::splat(0.00232)
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reinhard_basics() {
        let black = apply_tonemap(Vec3::ZERO, ToneMapType::Reinhard);
        assert_eq!(black, Vec3::ZERO);

        let sun = apply_tonemap(Vec3::splat(10000.0), ToneMapType::Reinhard);
        assert!((sun.x - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_khronos_linearity() {
        let dark = Vec3::splat(0.5);
        let result = apply_tonemap(dark, ToneMapType::Khronos);
        assert!((result.x - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_agx_sanity() {
        let black = apply_tonemap(Vec3::ZERO, ToneMapType::Agx);
        assert!(black.length() < 0.01);

        let val = Vec3::new(0.0, 0.5, 10.0);
        let result = apply_tonemap(val, ToneMapType::Agx);

        assert!(result.z > 0.8); // Should be bright
        assert!(result.z <= 1.0); // Should be clamped

        assert!(result.x > 0.0);
    }
}
