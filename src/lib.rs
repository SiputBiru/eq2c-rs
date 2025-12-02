pub mod codecs;
pub mod error;
pub mod image_utils;
pub mod layouts;
pub mod math;
mod paths;

pub use codecs::{OutputFormat, ToneMapType, get_encoder};
pub use error::{Eq2cError, Result};
pub use layouts::{LayoutType, generate_layout};
pub use math::CubeFace;

use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub input: PathBuf,
    pub output: PathBuf,
    pub format: OutputFormat,
    pub layout: LayoutType,
    pub tonemap: ToneMapType,
    pub exposure: f32,
    pub size: u32,
}

/// Runs the export pipeline: loads the input image, optionally reports very high brightness,
/// generates the chosen layout, encodes the resulting buffer(s) with the selected tone-mapping
/// and exposure, and writes output files.
///
/// On success, the function completes after writing one or more output files. On failure, an error
/// is returned (for example, if the image cannot be opened or encoding fails).
///
/// # Examples
///
/// ```ignore
/// // Build a Config for your run (fill fields appropriately)
/// let config = Config {
///     input: std::path::PathBuf::from("input.hdr"),
///     output: std::path::PathBuf::from("output.png"),
///     format: OutputFormat::Png,
///     layout: LayoutType::Equirectangular,
///     tonemap: ToneMapType::Reinhard,
///     exposure: 1.0,
///     size: 1024,
/// };
///
/// // Execute the pipeline
/// run(config).unwrap();
/// ```
pub fn run(config: Config) -> Result<()> {
    use rayon::prelude::*;

    println!("Loading {}...", config.input.display());
    let img = image::open(&config.input)?.into_rgb32f();

    let raw_pixels = img.as_raw();
    if !raw_pixels.is_empty() {
        let max_brightness = raw_pixels
            .par_iter()
            .cloned()
            .reduce(|| 0.0f32, |a, b| a.max(b));

        if max_brightness > 10.0 {
            let suggested = 1.0 / (max_brightness * 0.1);
            println!(
                "Note: Max Brightness = {:.2}. Recommended exposure: ~{:.4}",
                max_brightness, suggested
            );
        }
    }

    println!("Generating layout...");

    let layout_output = generate_layout(config.layout, &img, config.size);

    println!(
        "Encoding to output (Tone Map: {:?}, Exposure: {})...",
        config.tonemap, config.exposure
    );

    let encoder = get_encoder(config.format, config.tonemap, config.exposure);

    match layout_output {
        layouts::LayoutOutput::Single(buffer) => {
            encoder.encode(&buffer, &config.output)?;
            println!("Success! Saved to {}", config.output.display());
        }

        layouts::LayoutOutput::Frames(faces) => {
            for (face, buffer) in faces {
                let suffix = paths::face_suffix(face);
                let new_path = paths::append_suffix(&config.output, suffix);

                encoder.encode(&buffer, &new_path)?;
                println!("Saved {}", new_path.display());
            }
        }
    }

    Ok(())
}
