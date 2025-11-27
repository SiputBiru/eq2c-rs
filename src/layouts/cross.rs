use super::{LayoutOutput, SkyboxLayout, render_all_faces};
use crate::math::CubeFace;
use image::{ImageBuffer, Rgb32FImage};

pub struct CrossLayout;

impl SkyboxLayout for CrossLayout {
    fn generate(&self, source: &Rgb32FImage, face_size: u32) -> LayoutOutput {
        let rendered_faces = render_all_faces(source, face_size);

        let width = face_size * 4;
        let height = face_size * 3;
        let mut final_image = ImageBuffer::new(width, height);

        for (face, buffer) in rendered_faces {
            let (col, row) = match face {
                CubeFace::Left => (0, 1),
                CubeFace::Front => (1, 1),
                CubeFace::Right => (2, 1),
                CubeFace::Back => (3, 1),
                CubeFace::Top => (1, 0),
                CubeFace::Bottom => (1, 2),
            };

            let offset_x = col * face_size;
            let offset_y = row * face_size;

            for (x, y, pixel) in buffer.enumerate_pixels() {
                final_image.put_pixel(offset_x + x, offset_y + y, *pixel);
            }
        }

        LayoutOutput::Single(final_image)
    }
}
