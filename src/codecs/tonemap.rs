use clap::ValueEnum;
use glam::{Mat3, Vec3};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ToneMapType {
    Reinhard,
    Aces,
    Khronos,
    None,
}

pub fn apply_tonemap(color: Vec3, method: ToneMapType) -> Vec3 {
    match method {
        ToneMapType::Reinhard => reinhard(color),
        ToneMapType::Aces => aces_tonemap(color),
        ToneMapType::Khronos => khronos_pbr_neutral(color),
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

    result.clamp(Vec3::ZERO, Vec3::ONE).powf(1.0 / 2.2)
}

// Khronos PBR Neutral Tone Mapper
// Source: https://github.com/KhronosGroup/ToneMapping/tree/main/PBR_Neutral
fn khronos_pbr_neutral(mut color: Vec3) -> Vec3 {
    const START_COMPRESSION: f32 = 0.8 - 0.04;
    const DESATURATION: f32 = 0.15;

    // float x = min(color.r, min(color.g, color.b));
    let x = color.min_element();

    // float offset = x < 0.08 ? x - 6.25 * x * x : 0.04;
    let offset = if x < 0.08 { x - 6.25 * x * x } else { 0.04 };

    // color -= offset;
    // We use splat to subtract a scalar from all 3 vector components
    color -= Vec3::splat(offset);

    // float peak = max(color.r, max(color.g, color.b));
    let peak = color.max_element();

    // if (peak < startCompression) return color;
    if peak < START_COMPRESSION {
        return color;
    }

    // const float d = 1. - startCompression;
    let d = 1.0 - START_COMPRESSION;

    // float newPeak = 1. - d * d / (peak + d - startCompression);
    let new_peak = 1.0 - d * d / (peak + d - START_COMPRESSION);

    // color *= newPeak / peak;
    color *= new_peak / peak;

    // float g = 1. - 1. / (desaturation * (peak - newPeak) + 1.);
    let g = 1.0 - 1.0 / (DESATURATION * (peak - new_peak) + 1.0);

    // return mix(color, newPeak * vec3(1, 1, 1), g);
    color.lerp(Vec3::splat(new_peak), g)
}
