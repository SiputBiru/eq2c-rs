use super::SkyboxEncoder;
use crate::error::Result;
use image::Rgb32FImage;
use std::path::Path;

pub struct ExrEncoder;

impl SkyboxEncoder for ExrEncoder {
    /// Encodes the provided RGB 32-bit floating-point image and writes it to the specified file path in EXR format.
    ///
    /// Returns an error if the image cannot be written to the given path.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::path::Path;
    /// // create or load an `Rgb32FImage` here
    /// let image: image::Rgb32FImage = unimplemented!();
    /// let encoder = ExrEncoder;
    /// encoder.encode(&image, Path::new("skybox.exr")).unwrap();
    /// ````
    fn encode(&self, image: &Rgb32FImage, output_path: &Path) -> Result<()> {
        image.save(output_path)?;

        Ok(())
    }
}
