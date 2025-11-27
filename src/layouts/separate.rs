use super::{LayoutOutput, SkyboxLayout, render_all_faces};
use image::Rgb32FImage;

pub struct SeparateLayout;

impl SkyboxLayout for SeparateLayout {
    fn generate(&self, source: &Rgb32FImage, face_size: u32) -> LayoutOutput {
        let faces = render_all_faces(source, face_size);

        LayoutOutput::Frames(faces)
    }
}
