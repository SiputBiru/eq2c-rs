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

    result.clamp(Vec3::ZERO, Vec3::ONE).powf(1.0 / 2.2)
}

// Khronos PBR Neutral Tone Mapper
// Source: https://github.com/KhronosGroup/ToneMapping/tree/main/PBR_Neutral
fn khronos_pbr_neutral(mut color: Vec3) -> Vec3 {
    const START_COMPRESSION: f32 = 0.8 - 0.04;
    const DESATURATION: f32 = 0.15;

    let x = color.min_element();

    let offset = if x < 0.08 { x - 6.25 * x * x } else { 0.04 };

    // We use splat to subtract a scalar from all 3 vector components
    color -= Vec3::splat(offset);

    let peak = color.max_element();

    if peak < START_COMPRESSION {
        return color;
    }

    let d = 1.0 - START_COMPRESSION;

    let new_peak = 1.0 - d * d / (peak + d - START_COMPRESSION);

    color *= new_peak / peak;

    let g = 1.0 - 1.0 / (DESATURATION * (peak - new_peak) + 1.0);

    color.lerp(Vec3::splat(new_peak), g)
}

// AGX Algorithms

const AGX_INPUT_MAT: Mat3 = Mat3::from_cols_array(&[
    0.84247906,
    0.0784336,
    0.079223745, // Column 0
    0.04232824,
    0.87846864,
    0.07916613, // Column 1
    0.04237565,
    0.0784336,
    0.879143, // Column 2
]);

const AGX_OUTPUT_MAT: Mat3 = Mat3::from_cols_array(&[
    1.196879,
    -0.09802088,
    -0.09902974, // Column 0
    -0.05289685,
    1.1519031,
    -0.09896118, // Column 1
    -0.05297163,
    -0.09804345,
    1.1510737, // Column 2
]);

pub fn agx_tonemap(color: Vec3) -> Vec3 {
    // 1. Gamut Mapping (Input Transform)
    //    This rotates the color primaries to avoid the "Notorious 6" issue
    //    (where bright colors unnaturally shift to Cyan/Magenta/Yellow).
    let val = AGX_INPUT_MAT * color;

    // 2. Log2 Space Encoding
    //    AgX operates on Log2 data. We clamp to a specific EV range.
    //    Min/Max EV values from the standard AgX config.
    const MIN_EV: f32 = -12.47393;
    const MAX_EV: f32 = 4.026069;

    //    Apply log2 to each component and clamp
    let val_log = val.map(|c| c.max(1e-10).log2().clamp(MIN_EV, MAX_EV));

    //    Normalize to 0.0 - 1.0 range
    let val_norm = (val_log - MIN_EV) / (MAX_EV - MIN_EV);

    // 3. Sigmoid Function (The "S-Curve")
    //    This polynomial approximates the AgX film response curve.
    let result = agx_default_contrast_approx(val_norm);

    // 4. Inverse Transform (Output)
    //    Convert back to linear space.
    let linear_result = AGX_OUTPUT_MAT * result;

    // 5. Final Gamma Correction (Optional but usually needed for display)
    //    AgX output is linear-ish. If you are saving to PNG/JPG, apply gamma 2.2.
    //    If saving to EXR, skip this.
    linear_result.clamp(Vec3::ZERO, Vec3::ONE).powf(1.0 / 2.2)
}

// Polynomial approximation for the AgX sigmoid curve
fn agx_default_contrast_approx(x: Vec3) -> Vec3 {
    let x2 = x * x;
    let x4 = x2 * x2;

    // Formula: + 15.5 * x^6 - 40.14 * x^5 + 31.96 * x^4 - 6.868 * x^3 + 0.4298 * x^2 + 0.1191 * x - 0.00232
    // We use Horner's method or direct expansion. Direct is clearer for this polynomial size.

    (Vec3::splat(15.5) * x4 * x2) - (Vec3::splat(40.14) * x4 * x) + (Vec3::splat(31.96) * x4)
        - (Vec3::splat(6.868) * x2 * x)
        + (Vec3::splat(0.4298) * x2)
        + (Vec3::splat(0.1191) * x)
        - Vec3::splat(0.00232)
}
