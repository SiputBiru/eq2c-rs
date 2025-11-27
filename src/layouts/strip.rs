use super::{LayoutOutput, SkyboxLayout, render_all_faces};
use crate::math::CubeFace;
use image::{ImageBuffer, Rgb32FImage};

#[derive(Clone, Copy)]
pub enum StripDirection {
    Horizontal,
    Vertical,
}

pub struct StripLayout {
    pub direction: StripDirection,
}

impl SkyboxLayout for StripLayout {
    fn generate(&self, source: &Rgb32FImage, face_size: u32) -> LayoutOutput {
        let rendered_faces = render_all_faces(source, face_size);

        let (width, height) = match self.direction {
            StripDirection::Horizontal => (face_size * 6, face_size),
            StripDirection::Vertical => (face_size, face_size * 6),
        };

        let mut final_image = ImageBuffer::new(width, height);

        for (face, buffer) in rendered_faces {
            let index = match face {
                CubeFace::Right => 0,  // +X
                CubeFace::Left => 1,   // -X
                CubeFace::Top => 2,    // +Y
                CubeFace::Bottom => 3, // -Y
                CubeFace::Front => 4,  // +Z
                CubeFace::Back => 5,   // -Z
            };

            let (offset_x, offset_y) = match self.direction {
                StripDirection::Horizontal => (index * face_size, 0),
                StripDirection::Vertical => (0, index * face_size),
            };

            for (x, y, pixel) in buffer.enumerate_pixels() {
                final_image.put_pixel(offset_x + x, offset_y + y, *pixel);
            }
        }

        LayoutOutput::Single(final_image)
    }
}
