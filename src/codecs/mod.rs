// pub mod astc;
// pub mod png_ldr;
use image::Rgb32FImage;
use std::path::Path;

pub mod png_ldr;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Png,
    Exr,
}

pub trait SkyboxEncoder {
    fn encode(&self, image: &Rgb32FImage, output_path: &Path) -> Result<(), String>;
}

pub fn get_encoder(format: OutputFormat) -> Box<dyn SkyboxEncoder> {
    match format {
        OutputFormat::Png => Box::new(png_ldr::PngLdrEncoder),
        OutputFormat::Exr => panic!("EXR not implemented yet!"),
    }
}
