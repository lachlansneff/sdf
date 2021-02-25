use ultraviolet::{f32x8, Vec2x8, Vec3x8};

// pub fn circle(point: impl Point, radius: Op) -> Op {
//     Op2::XY.mag() - radius
// }

pub fn sphere(p: Vec3x8, radius: f32x8) -> f32x8 {
    p.mag() - radius
}

// float sdf_box(vec3 p, vec3 b ) {
//     vec3 q = abs(p) - b;
//     return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
// }

/// A box.
pub fn rectangular_prism(p: Vec3x8, sides: f32x8) -> f32x8 {
    let q = p.abs().map(|c| c - sides);
    return q.max_by_component(Vec3x8::zero()).mag() + q.x.max(q.y.max(q.z).min(f32x8::splat(0.0)));
}

// float sdf_cylinder(vec3 p, float h, float r) {
//   vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(r,h);
//   return min(max(d.x,d.y),0.0) + length(max(d,0.0));
// }

pub fn cylinder(p: Vec3x8, height: f32x8, radius: f32x8) -> f32x8 {
    let d = Vec2x8::new(Vec2x8::new(p.x, p.z).mag(), p.y).abs() - Vec2x8::new(height, radius);
    d.x.max(d.y).min(f32x8::splat(0.0)) + d.max_by_component(Vec2x8::zero()).mag()
}
