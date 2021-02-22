use super::eval::{Eval, Value as _, Value2 as _, Value3 as _};

// pub fn circle(point: impl Point, radius: Op) -> Op {
//     Op2::XY.mag() - radius
// }

pub fn sphere<E: Eval>(p: E::V3, radius: E::V) -> E::V {
    p.mag() - radius
}

// float sdf_box(vec3 p, vec3 b ) {
//     vec3 q = abs(p) - b;
//     return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
// }

/// A box.
pub fn rectangular_prism<E: Eval>(p: E::V3, sides: E::V3) -> E::V {
    let q = p.abs() - sides;
    return q.max(E::V3::splat(0.0)).mag() + q.x().max(q.y().max(q.z())).min(E::V::new(0.0));
}

// float sdf_cylinder(vec3 p, float h, float r) {
//   vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(r,h);
//   return min(max(d.x,d.y),0.0) + length(max(d,0.0));
// }

pub fn cylinder<E: Eval>(p: E::V3, height: E::V, radius: E::V) -> E::V {
    // Should this be `circle(radius).extrude(height)` instead?
    let d = E::V2::new(p.xz().mag(), p.y());
    d.max(E::V2::splat(0.0)).mag() + d.x().max(d.y()).min(E::V::new(0.0))
}
