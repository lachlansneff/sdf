use super::eval::{Eval, Real1 as _, Real2 as _, Real3 as _};

// pub fn circle(point: impl Point, radius: Op) -> Op {
//     Op2::XY.mag() - radius
// }

pub fn sphere<E: Eval>(p: E::R3, radius: E::R1) -> E::R1 {
    p.mag() - radius
}

// float sdf_box(vec3 p, vec3 b ) {
//     vec3 q = abs(p) - b;
//     return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
// }

/// A box.
pub fn rectangular_prism<E: Eval>(p: E::R3, sides: E::R3) -> E::R1 {
    let q = p.abs() - sides;
    return q.max(E::R3::splat(0.0)).mag() + q.x().max(q.y().max(q.z())).min(E::R1::new(0.0));
}

// float sdf_cylinder(vec3 p, float h, float r) {
//   vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(r,h);
//   return min(max(d.x,d.y),0.0) + length(max(d,0.0));
// }

pub fn cylinder<E: Eval>(p: E::R3, height: E::R1, radius: E::R1) -> E::R1 {
    // Should this be `circle(radius).extrude(height)` instead?
    let d = E::R2::new(p.xz().mag(), p.y()).abs() - E::R2::new(height, radius);
    d.max(E::R2::splat(0.0)).mag() + d.x().max(d.y()).min(E::R1::new(0.0))
}
