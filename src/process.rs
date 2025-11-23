use crate::math::{self, CubeFace};
use glam::Vec3;
use image::{ImageBuffer, Rgb, Rgb32FImage};
use rayon::prelude::*;

pub struct ConvertOptions {
    pub face_size: u32,
}

pub fn generate_cross_layout(source: &Rgb32FImage, options: &ConvertOptions) -> Rgb32FImage {
    let faces = vec![
        CubeFace::Right,
        CubeFace::Left,
        CubeFace::Top,
        CubeFace::Bottom,
        CubeFace::Front,
        CubeFace::Back,
    ];

    let rendered_faces: Vec<(CubeFace, Rgb32FImage)> = faces
        .par_iter()
        .map(|&face| {
            let buffer = extract_face(source, face, options.face_size);
            (face, buffer)
        })
        .collect();

    let width = options.face_size * 4;
    let height = options.face_size * 3;
    let mut final_image = ImageBuffer::new(width, height);

    for (face, buffer) in rendered_faces {
        let (offset_x, offset_y) = get_cross_offset(face, options.face_size);

        for (x, y, pixel) in buffer.enumerate_pixels() {
            final_image.put_pixel(offset_x + x, offset_y + y, *pixel);
        }
    }

    final_image
}

fn extract_face(source: &Rgb32FImage, face: CubeFace, size: u32) -> Rgb32FImage {
    let mut buffer = ImageBuffer::new(size, size);

    buffer.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let u = (x as f32 + 0.5) / size as f32;
        let v = (y as f32 + 0.5) / size as f32;

        let source_uv = math::calculate_source_uv(face, u, v);

        let color = sample_bilinear(source, source_uv.x, source_uv.y);

        *pixel = color;
    });

    buffer
}

fn sample_bilinear(source: &Rgb32FImage, u: f32, v: f32) -> Rgb<f32> {
    let width = source.width() as f32;
    let height = source.height() as f32;

    let x = (u * width) - 0.5;
    let y = (v * height) - 0.5;

    let x0 = x.floor();
    let y0 = y.floor();

    let tx = x - x0;
    let ty = y - y0;

    let get_pixel = |ix: f32, iy: f32| -> Vec3 {
        // Wrap X (Longitude): If we go off the right edge, wrap to left
        let final_x = (ix as i32).rem_euclid(width as i32) as u32;

        // Clamp Y (Latitude): Don't wrap vertically, just clamp to top/bottom
        let final_y = (iy as i32).clamp(0, height as i32 - 1) as u32;

        let p = source.get_pixel(final_x, final_y);
        Vec3::new(p[0], p[1], p[2])
    };

    let c00 = get_pixel(x0, y0); // Top-Left
    let c10 = get_pixel(x0 + 1.0, y0); // Top-Right
    let c01 = get_pixel(x0, y0 + 1.0); // Bottom-Left
    let c11 = get_pixel(x0 + 1.0, y0 + 1.0); // Bottom-Right

    let top = c00.lerp(c10, tx);
    let bottom = c01.lerp(c11, tx);

    let final_color = top.lerp(bottom, ty);

    Rgb([final_color.x, final_color.y, final_color.z])
}

fn get_cross_offset(face: CubeFace, size: u32) -> (u32, u32) {
    // Layout Grid (4x3):
    //       [Top]
    // [Left][Front][Right][Back]
    //       [Bottom]

    let (col, row) = match face {
        CubeFace::Left => (0, 1),
        CubeFace::Front => (1, 1),
        CubeFace::Right => (2, 1),
        CubeFace::Back => (3, 1),
        CubeFace::Top => (1, 0),
        CubeFace::Bottom => (1, 2),
    };

    (col * size, row * size)
}
