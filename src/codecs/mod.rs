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
    fn encode(&self, image: &Rgb32FImage, output_path: &Path) -> Result<(), String>;
}

pub fn get_encoder(format: OutputFormat, tonemap: ToneMapType) -> Box<dyn SkyboxEncoder> {
    match format {
        OutputFormat::Png => Box::new(png::PngEncoder { tonemap }),
        OutputFormat::Exr => Box::new(exr::ExrEncoder),
    }
}
