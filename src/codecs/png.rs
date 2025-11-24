use super::SkyboxEncoder;
use crate::codecs::tonemap::{self, ToneMapType};
use glam::Vec3;
use image::{ImageBuffer, Rgb, Rgb32FImage};
use std::path::Path;

pub struct PngEncoder {
    pub tonemap: ToneMapType,
}

impl SkyboxEncoder for PngEncoder {
    fn encode(&self, image: &Rgb32FImage, output_path: &Path) -> Result<(), String> {
        let width = image.width();
        let height = image.height();
        let mut ldr_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

        for (x, y, pixel) in image.enumerate_pixels() {
            let hdr = Vec3::new(pixel.0[0], pixel.0[1], pixel.0[2]);

            // 1. Delegate math to the tonemap module
            let mapped = tonemap::apply_tonemap(hdr, self.tonemap);

            // 2. Apply Gamma Correction (Linear -> sRGB)
            let gamma = 1.0 / 2.2;
            let final_color = mapped.powf(gamma);

            // 3. Convert to u8
            let r = (final_color.x * 255.0).clamp(0.0, 255.0) as u8;
            let g = (final_color.y * 255.0).clamp(0.0, 255.0) as u8;
            let b = (final_color.z * 255.0).clamp(0.0, 255.0) as u8;

            ldr_buffer.put_pixel(x, y, Rgb([r, g, b]));
        }

        ldr_buffer.save(output_path).map_err(|e| e.to_string())
    }
}
