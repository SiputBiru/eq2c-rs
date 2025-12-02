use crate::error::Result;
use image::Rgb32FImage;
use std::path::Path;

pub mod exr;
pub mod png;
pub mod tonemap;

pub use tonemap::ToneMapType;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Png,
    Exr,
}

pub trait SkyboxEncoder {
    fn encode(&self, image: &Rgb32FImage, output_path: &Path) -> Result<()>;
}

/// Selects and returns a boxed skybox encoder for the requested output format.
///
/// The returned encoder implements `SkyboxEncoder`. For `OutputFormat::Png` the encoder
/// is configured with the provided `tonemap` and `exposure`; for `OutputFormat::Exr`
/// an EXR encoder is returned.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use crate::codecs::{get_encoder, OutputFormat};
/// use crate::tonemap::ToneMapType;
///
/// let encoder = get_encoder(OutputFormat::Png, ToneMapType::Reinhard, 1.0);
/// // encoder.encode(&image, Path::new("out.png")).unwrap();
/// ```
pub fn get_encoder(
    format: OutputFormat,
    tonemap: ToneMapType,
    exposure: f32,
) -> Box<dyn SkyboxEncoder> {
    match format {
        OutputFormat::Png => Box::new(png::PngEncoder { tonemap, exposure }),
        OutputFormat::Exr => Box::new(exr::ExrEncoder),
    }
}
