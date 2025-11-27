use image::Rgb32FImage;
pub mod cross;
pub mod separate;
pub mod strip;

pub enum LayoutOutput {
    Single(Rgb32FImage),
    Frames(Vec<(CubeFace, Rgb32FImage)>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutType {
    Cross,
    StripHorizontal,
    StripVertical,
    Separate,
}

pub trait SkyboxLayout {
    fn generate(&self, source: &Rgb32FImage, face_size: u32) -> LayoutOutput;
}

pub fn generate_layout(layout: LayoutType, source: &Rgb32FImage, face_size: u32) -> LayoutOutput {
    let processor: Box<dyn SkyboxLayout> = match layout {
        LayoutType::Cross => Box::new(cross::CrossLayout),

        LayoutType::StripHorizontal => Box::new(strip::StripLayout {
            direction: strip::StripDirection::Horizontal,
        }),

        LayoutType::StripVertical => Box::new(strip::StripLayout {
            direction: strip::StripDirection::Vertical,
        }),

        LayoutType::Separate => Box::new(separate::SeparateLayout),
    };

    processor.generate(source, face_size)
}

// --- SHARED HELPERS ---
use crate::math::{self, CubeFace};
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

pub fn render_all_faces(source: &Rgb32FImage, face_size: u32) -> Vec<(CubeFace, Rgb32FImage)> {
    let faces = vec![
        CubeFace::Right,
        CubeFace::Left,
        CubeFace::Top,
        CubeFace::Bottom,
        CubeFace::Front,
        CubeFace::Back,
    ];

    faces
        .par_iter()
        .map(|&face| {
            let buffer = extract_single_face(source, face, face_size);
            (face, buffer)
        })
        .collect()
}

fn extract_single_face(source: &Rgb32FImage, face: CubeFace, size: u32) -> Rgb32FImage {
    let mut buffer = ImageBuffer::new(size, size);

    buffer.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let u = (x as f32 + 0.5) / size as f32;
        let v = (y as f32 + 0.5) / size as f32;

        let source_uv = math::calculate_source_uv(face, u, v);

        // Re-using the bilinear sampler logic
        // For production, this should be shared in a utils module
        *pixel = sample_bilinear(source, source_uv.x, source_uv.y);
    });

    buffer
}

fn sample_bilinear(source: &Rgb32FImage, u: f32, v: f32) -> Rgb<f32> {
    use glam::Vec3;
    let width = source.width() as f32;
    let height = source.height() as f32;

    let x = (u * width) - 0.5;
    let y = (v * height) - 0.5;

    let x0 = x.floor();
    let y0 = y.floor();
    let tx = x - x0;
    let ty = y - y0;

    let get_pixel = |ix: f32, iy: f32| -> Vec3 {
        let final_x = (ix as i32).rem_euclid(width as i32) as u32;
        let final_y = (iy as i32).clamp(0, height as i32 - 1) as u32;
        let p = source.get_pixel(final_x, final_y);
        Vec3::new(p[0], p[1], p[2])
    };

    let c00 = get_pixel(x0, y0);
    let c10 = get_pixel(x0 + 1.0, y0);
    let c01 = get_pixel(x0, y0 + 1.0);
    let c11 = get_pixel(x0 + 1.0, y0 + 1.0);

    let top = c00.lerp(c10, tx);
    let bottom = c01.lerp(c11, tx);
    let final_color = top.lerp(bottom, ty);

    Rgb([final_color.x, final_color.y, final_color.z])
}
