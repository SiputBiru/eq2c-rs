use glam::{Vec2, Vec3};
use std::f32::consts::{PI, TAU};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CubeFace {
    Right,  // +X
    Left,   // -X
    Top,    // +Y
    Bottom, // -Y
    Front,  // +Z
    Back,   // -Z
}

pub fn calculate_source_uv(face: CubeFace, u: f32, v: f32) -> Vec2 {
    let direction = face_uv_to_dir(face, u, v);
    dir_to_equirect_uv(direction)
}

fn face_uv_to_dir(face: CubeFace, u: f32, v: f32) -> Vec3 {
    let uc = 2.0 * u - 1.0;
    let vc = 2.0 * v - 1.0;

    let dir = match face {
        CubeFace::Right => Vec3::new(1.0, -vc, -uc),
        CubeFace::Left => Vec3::new(-1.0, -vc, uc),
        CubeFace::Top => Vec3::new(uc, 1.0, vc),
        CubeFace::Bottom => Vec3::new(uc, -1.0, -vc),
        CubeFace::Front => Vec3::new(uc, -vc, 1.0),
        CubeFace::Back => Vec3::new(-uc, -vc, -1.0),
    };

    dir.normalize()
}

fn dir_to_equirect_uv(dir: Vec3) -> Vec2 {
    let phi = dir.z.atan2(dir.x);

    let theta = dir.y.clamp(-1.0, 1.0).asin();

    let u = (phi / TAU) + 0.5;

    let v = (theta / PI) + 0.5;

    Vec2::new(u, v)
}

pub fn bilerp(v00: Vec3, v10: Vec3, v01: Vec3, v11: Vec3, tx: f32, ty: f32) -> Vec3 {
    let top = v00.lerp(v10, tx);

    let bottom = v01.lerp(v11, tx);

    top.lerp(bottom, ty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_uv_to_direction_front() {
        let face = CubeFace::Front;
        let dir = face_uv_to_dir(face, 0.5, 0.5);

        assert!((dir.z - 1.0).abs() < 0.0001);
        assert!(dir.x.abs() < 0.0001);
        assert!(dir.y.abs() < 0.0001);
    }

    #[test]
    fn test_face_uv_to_direction_right() {
        let face = CubeFace::Right;
        let dir = face_uv_to_dir(face, 0.5, 0.5);

        assert!((dir.x - 1.0).abs() < 0.0001);
    }
}
