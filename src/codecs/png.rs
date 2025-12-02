use super::SkyboxEncoder;
use crate::codecs::tonemap::{self, ToneMapType};
use crate::error::{Eq2cError, Result};

use glam::Vec3;
use image::{ImageBuffer, Rgb, Rgb32FImage};
use rayon::prelude::*;
use std::path::Path;

pub struct PngEncoder {
    pub tonemap: ToneMapType,
    pub exposure: f32,
}

impl SkyboxEncoder for PngEncoder {
    /// Encodes an HDR RGB32F image to an 8-bit PNG file using the encoder's exposure and tonemap settings.
    ///
    /// On success, writes the resulting PNG to `output_path` and returns `Ok(())`.
    ///
    /// # Errors
    ///
    /// - Returns `Eq2cError::InvalidDimensions` if width*height or buffer size overflows, or if the source buffer is smaller than expected.
    /// - Returns `Eq2cError::Image` with `ParameterErrorKind::DimensionMismatch` if an output image buffer cannot be created from the processed data.
    /// - Propagates errors returned by the underlying image saving operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use image::Rgb32FImage;
    /// use crate::codecs::png::PngEncoder;
    /// use crate::tonemap::ToneMapType;
    ///
    /// // Create a 1x1 HDR image with a single white pixel.
    /// let mut img: Rgb32FImage = Rgb32FImage::new(1, 1);
    /// img.put_pixel(0, 0, image::Rgb([1.0f32, 1.0f32, 1.0f32]));
    ///
    /// let encoder = PngEncoder { tonemap: ToneMapType::Linear, exposure: 1.0 };
    /// let out_path = Path::new("test_out.png");
    ///
    /// // Writes a PNG file; returns Ok(()) on success.
    /// let result = encoder.encode(&img, out_path);
    /// assert!(result.is_ok());
    /// ```
    fn encode(&self, image: &Rgb32FImage, output_path: &Path) -> Result<()> {
        let width = image.width() as usize;
        let height = image.height() as usize;

        let num_pixels = width
            .checked_mul(height)
            .ok_or_else(|| Eq2cError::InvalidDimensions {
                expected: "valid dimensions".to_string(),
                found: "overflow".to_string(),
            })?;

        let src = image.as_raw();
        let expected = num_pixels
            .checked_mul(3)
            .ok_or_else(|| Eq2cError::InvalidDimensions {
                expected: "valid buffer size".to_string(),
                found: "overflow".to_string(),
            })?;

        if src.len() < expected {
            return Err(Eq2cError::InvalidDimensions {
                expected: format!("buffer size >= {}", expected),
                found: format!("{}", src.len()),
            });
        }

        let mut ldr_data = vec![0u8; num_pixels * 3];

        let exposure = self.exposure;
        let tonemap_type = self.tonemap;

        ldr_data
            .par_chunks_mut(3)
            .enumerate()
            .for_each(|(i, out_pixel)| {
                let base = i * 3;
                let r = src[base];
                let g = src[base + 1];
                let b = src[base + 2];

                let hdr = Vec3::new(r, g, b) * exposure;
                let mapped = tonemap::apply_tonemap(hdr, tonemap_type);

                let final_color = Vec3::new(
                    mapped.x.max(0.0).sqrt(),
                    mapped.y.max(0.0).sqrt(),
                    mapped.z.max(0.0).sqrt(),
                );

                out_pixel[0] = (final_color.x * 255.0).clamp(0.0, 255.0) as u8;
                out_pixel[1] = (final_color.y * 255.0).clamp(0.0, 255.0) as u8;
                out_pixel[2] = (final_color.z * 255.0).clamp(0.0, 255.0) as u8;
            });

        let out: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width as u32, height as u32, ldr_data).ok_or_else(|| {
                Eq2cError::Image(image::ImageError::Parameter(
                    image::error::ParameterError::from_kind(
                        image::error::ParameterErrorKind::DimensionMismatch,
                    ),
                ))
            })?;

        out.save(output_path)?;

        Ok(())
    }
}