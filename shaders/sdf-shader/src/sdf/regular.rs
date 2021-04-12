#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float as _;

use crate::extra::Scalar;
use glam::Vec3;

pub fn sphere(p: Vec3, r: f32) -> f32 {
    p.length() - r
}

pub fn rectangular_prism(p: Vec3, sides: Vec3) -> f32 {
    let q = p.abs() - sides;
    q.max(Vec3::ZERO).length() + q.y.max(q.z).max(q.x).min(0.0)
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

pub fn union(lhs: f32, rhs: f32) -> f32 {
    lhs.min(rhs)
}

pub fn intersect(lhs: f32, rhs: f32) -> f32 {
    lhs.max(rhs)
}

pub fn subtract(lhs: f32, rhs: f32) -> f32 {
    (-lhs).max(rhs)
}

pub fn smooth_union(lhs: f32, rhs: f32, k: f32) -> f32 {
    let h = ((rhs - lhs) * 0.5 / k + 0.5).clamp(0.0, 1.0);
    rhs.lerp(lhs, h) - k * h * (1.0 - h)
}
