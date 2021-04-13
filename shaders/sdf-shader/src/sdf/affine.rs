#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float as _;
use crate::{arithmetic::{Affine, Affine3, Arithmetics, Choice}};
use glam::Vec3;

pub fn sphere(p: Affine3, r: f32) -> Affine {
    p.length() - r
}

pub fn rectangular_prism(p: Affine3, sides: Vec3) -> Affine {
    let q = p.abs() - sides;
    q.max(0.0).length() + q.y.max(q.z).max(q.x).min(0.0)
}

// pub fn cylinder(p: Vec3, h: f32, r: f32) -> f32 {
//     let d = vec2(p.xz().length(), p.y).abs() - vec2(r, h);
//     d.x.max(d.y).min(0.0) + d.max(Vec2::ZERO).length()
// }

// pub fn gyroid(p: Vec3, scale: f32, thickness: f32) -> f32 {
//     let p = p * scale;
//     (p.sin().dot(p.zxy().cos()) / scale - thickness) * 0.6
// }

// pub fn schwarz_p(p: Vec3, scale: f32, thickness: f32) -> f32 {
//     let p = p * scale;
//     (p.cos().dot(Vec3::ONE).abs() / scale - thickness) * 0.6
// }

pub fn union(lhs: Affine, rhs: Affine) -> (Affine, Choice) {
    lhs.min_choice(rhs)
}

pub fn intersect(lhs: Affine, rhs: Affine) -> (Affine, Choice) {
    lhs.max_choice(rhs)
}

pub fn subtract(lhs: Affine, rhs: Affine) -> (Affine, Choice) {
    (-lhs).max_choice(rhs)
}

pub fn smooth_union(lhs: Affine, rhs: Affine, k: f32) -> Affine {
    let h = ((rhs - lhs) * 0.5 / k + 0.5).clamp(0.0, 1.0);
    rhs.lerp(lhs, h) - h * (1.0 - h) * k
}
