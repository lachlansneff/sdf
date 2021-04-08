use glam::Vec3;

use crate::{
    diff::{Dual, DualVec3},
    extra::VectorN as _,
};

pub fn sphere(p: DualVec3, r: f32) -> Dual {
    p.length() - r
}

pub fn rectangular_prism(p: DualVec3, sides: Vec3) -> Dual {
    let q = p.abs() - sides;
    q.max(DualVec3::zero()).length() + q.y.max(q.z.max(q.x.min(Dual::new(0.0))))
}

// pub fn cylinder(p: DualVec3, h: f32, r: f32) -> f32 {
//     let d = vec2(p.xz().length(), p.y).abs() - vec2(r, h);
//     d.x.max(d.y).min(0.0) + d.max(Vec2::ZERO).length()
// }

pub fn schwarz_p(p: DualVec3, scale: f32, thickness: f32) -> Dual {
    let p = p * scale;
    (p.cos().dot(DualVec3::one()).abs() / scale - thickness) * 0.6
}

pub fn union(lhs: Dual, rhs: Dual) -> Dual {
    lhs.min(rhs)
}

pub fn intersect(lhs: Dual, rhs: Dual) -> Dual {
    lhs.max(rhs)
}
