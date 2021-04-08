use ultraviolet::{f32x8, Vec3x8};

// float sdf_gyroid(vec3 p, float scale, float thickness) {
//     p *= scale;
//     return (abs(dot(sin(p), cos(p.zxy))) / scale - thickness) * 0.6;
// }

pub fn gyroid(p: Vec3x8, scale: f32x8, thickness: f32x8) -> f32x8 {
    let p = p / scale;
    (p.map(|c| c.sin())
        .dot(Vec3x8::new(p.z, p.x, p.y).map(|c| c.cos()))
        .abs()
        * scale
        - thickness)
        * 0.6
}

// float sdf_schwarzP(vec3 p, float scale, float thickness) {
//     p *= scale;
//     float implicit = abs(dot(cos(p), vec3(1.0)));
//     // float grad = length(sin(p));
//     return ((implicit) / scale - thickness) * 0.6;
// }

pub fn schwarz_p(p: Vec3x8, scale: f32x8, thickness: f32x8) -> f32x8 {
    let p = p / scale;
    (p.map(|c| c.cos()).dot(Vec3x8::one()).abs() * scale - thickness) * 0.6
}
