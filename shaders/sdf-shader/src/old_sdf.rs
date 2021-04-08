use spirv_std::{
    num_traits::Float as _,
};
use glam::{vec2, Vec2, Vec3, Vec3Swizzles as _};
use crate::{diff::{Dual, DualVec3}, extra::{VectorN as _, Scalar as _}};

pub fn sphere(p: Vec3, r: f32) -> f32 {
    p.length() - r
}

// pub fn sphere_diff(p: DualVec3, r: f32) -> Dual {
//     p.length() - r
// }

// float box(vec3 p, vec3 b ) {
//     vec3 q = abs(p) - b;
//     return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
// }

pub fn rectangular_prism(p: Vec3, sides: Vec3) -> f32 {
    let q = p.abs() - sides;
    q.max(Vec3::ZERO).length() + q.y.max(q.z).max(q.x).min(0.0)
}

pub fn rectangular_prism_diff(p: DualVec3, sides: Vec3) -> Dual {
    let q = p.abs() - sides;
    q.max(DualVec3::zero()).length() + q.y.max(q.z.max(q.x.min(Dual::new(0.0))))
}

// float cylinder(vec3 p, float h, float r) {
//   vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(r,h);
//   return min(max(d.x,d.y),0.0) + length(max(d,0.0));
// }

pub fn cylinder(p: Vec3, h: f32, r: f32) -> f32 {
    let d = vec2(p.xz().length(), p.y).abs() - vec2(r, h);
    d.x.max(d.y).min(0.0) + d.max(Vec2::ZERO).length()
}

// float gyroid(vec3 p, float scale, float thickness) {
//     p *= scale;
//     return (abs(dot(sin(p), cos(p.zxy))) / scale - thickness) * 0.6;
// }

pub fn gyroid(p: Vec3, scale: f32, thickness: f32) -> f32 {
    let p = p * scale;
    (p.sin().dot(p.zxy().cos()) / scale - thickness) * 0.6
}

pub fn schwarz_p(p: Vec3, scale: f32, thickness: f32) -> f32 {
    let p = p * scale;
    (p.cos().dot(Vec3::ONE).abs() / scale - thickness) * 0.6
}

pub fn schwarz_p_diff(p: DualVec3, scale: f32, thickness: f32) -> Dual {
    let p = p * scale;
    (p.cos().dot(DualVec3::one()).abs() / scale - thickness) * 0.6
}

pub fn union(lhs: f32, rhs: f32) -> f32 {
    lhs.min(rhs)
}

// pub fn union_diff(lhs: Dual, rhs: Dual) -> Dual {
//     lhs.min(rhs)
// }

pub fn intersect(lhs: f32, rhs: f32) -> f32 {
    lhs.max(rhs)
}

pub fn intersect_diff(lhs: Dual, rhs: Dual) -> Dual {
    lhs.max(rhs)
}

pub fn subtract(lhs: f32, rhs: f32) -> f32 {
    (-lhs).max(rhs)
}

pub fn union_smooth(lhs: f32, rhs: f32, k: f32) -> f32 {
    let h = ((rhs - lhs) * 0.5 / k + 0.5).clamp(0.0, 1.0);
    rhs.lerp(lhs, h) - k * h * (1.0 - h)
}
