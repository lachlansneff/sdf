use crate::op::{Op, Op2, Op3};

pub fn circle(radius: Op) -> Op {
    Op2::XY.mag() - radius
}

pub fn sphere(radius: Op) -> Op {
    Op3::XYZ.mag() - radius
}

// float sdf_box(vec3 p, vec3 b ) {
//     vec3 q = abs(p) - b;
//     return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
// }

/// A box.
pub fn rectangular_prism(sides: Op3) -> Op {
    let q = Op3::XYZ.abs() - sides;
    return q.clone().max(Op3::splat(0.0)).mag() + q.x.max(q.y.max(q.z)).min(Op::Const(0.0));
}

// float sdf_cylinder(vec3 p, float h, float r) {
//   vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(r,h);
//   return min(max(d.x,d.y),0.0) + length(max(d,0.0));
// }

pub fn cylinder(height: Op, radius: Op) -> Op {
    // Should this be `circle(radius).extrude(height)` instead?
    let p = Op3::XYZ;
    let d = Op2::new(p.clone().xz().mag(), p.y).abs() - Op2::new(radius, height);
    d.clone().max(Op2::splat(0.0)).mag() + d.x.max(d.y).min(Op::Const(0.0))
}

// float sdf_gyroid(vec3 p, float scale, float thickness) {
//     p *= scale;
//     return (abs(dot(sin(p), cos(p.zxy))) / scale - thickness) * 0.6;
// }

pub fn gyroid(scale: Op, thickness: Op) -> Op {
    let p = Op3::XYZ / scale.clone();
    (p.clone().sin().dot(p.zxy().cos()).abs() * scale - thickness) * 0.6
}

// float sdf_schwarzP(vec3 p, float scale, float thickness) {
//     p *= scale;
//     float implicit = abs(dot(cos(p), vec3(1.0)));
//     // float grad = length(sin(p));
//     return ((implicit) / scale - thickness) * 0.6;
// }

pub fn schwarz_p(scale: Op, thickness: Op) -> Op {
    let p = Op3::XYZ / scale.clone();
    (p.cos().dot(Op3::splat(1.0)).abs() / scale - thickness) * 0.6
}
