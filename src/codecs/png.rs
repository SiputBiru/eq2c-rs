use super::SkyboxEncoder;
use image::{ImageBuffer, Rgb, Rgb32FImage};
use std::path::Path;

pub struct PngEncoder;

impl SkyboxEncoder for PngEncoder {
    fn encode(&self, image: &Rgb32FImage, output_path: &Path) -> Result<(), String> {
        let width = image.width();
        let height = image.height();

        let mut ldr_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

        for (x, y, pixel) in image.enumerate_pixels() {
            let hdr = pixel.0; // [r, g, b]

            let tm_r = hdr[0] / (hdr[0] + 1.0);
            let tm_g = hdr[1] / (hdr[1] + 1.0);
            let tm_b = hdr[2] / (hdr[2] + 1.0);

            let gamma = 1.0 / 2.2;
            let final_r = tm_r.powf(gamma);
            let final_g = tm_g.powf(gamma);
            let final_b = tm_b.powf(gamma);

            let r = (final_r * 255.0).clamp(0.0, 255.0) as u8;
            let g = (final_g * 255.0).clamp(0.0, 255.0) as u8;
            let b = (final_b * 255.0).clamp(0.0, 255.0) as u8;

            ldr_buffer.put_pixel(x, y, Rgb([r, g, b]));
        }

        ldr_buffer
            .save(output_path)
            .map_err(|e| format!("Failed to save PNG: {}", e))
    }
}
