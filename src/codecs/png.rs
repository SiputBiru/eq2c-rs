use super::SkyboxEncoder;
use crate::codecs::tonemap::{self, ToneMapType};

use glam::Vec3;
use image::{ImageBuffer, Rgb, Rgb32FImage};
use rayon::prelude::*;
use std::path::Path;

pub struct PngEncoder {
    pub tonemap: ToneMapType,
    pub exposure: f32,
}

impl SkyboxEncoder for PngEncoder {
    fn encode(&self, image: &Rgb32FImage, output_path: &Path) -> Result<(), String> {
        let width = image.width() as usize;
        let height = image.height() as usize;

        let num_pixels = width
            .checked_mul(height)
            .ok_or_else(|| "image dimensions too large".to_string())?;

        let src = image.as_raw();
        let expected = num_pixels
            .checked_mul(3)
            .ok_or_else(|| "image too large".to_string())?;
        if src.len() < expected {
            return Err("unexpected HDR buffer size".into());
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
            ImageBuffer::from_raw(width as u32, height as u32, ldr_data)
                .ok_or_else(|| "failed to create output image buffer".to_string())?;

        out.save(output_path).map_err(|e| e.to_string())
    }
}
