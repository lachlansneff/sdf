use super::eval::{Eval, Value as _, Value2 as _, Value3 as _};

// float sdf_gyroid(vec3 p, float scale, float thickness) {
//     p *= scale;
//     return (abs(dot(sin(p), cos(p.zxy))) / scale - thickness) * 0.6;
// }

pub fn gyroid<E: Eval>(p: E::V3, scale: E::V, thickness: E::V) -> E::V {
    let p = p / scale.clone();
    (p.sin().dot(p.zxy().cos()).abs() * scale - thickness) * 0.6
}

// float sdf_schwarzP(vec3 p, float scale, float thickness) {
//     p *= scale;
//     float implicit = abs(dot(cos(p), vec3(1.0)));
//     // float grad = length(sin(p));
//     return ((implicit) / scale - thickness) * 0.6;
// }

pub fn schwarz_p<E: Eval>(p: E::V3, scale: E::V, thickness: E::V) -> E::V {
    let p = p / scale.clone();
    (p.cos().dot(E::V3::splat(1.0)).abs() * scale - thickness) * 0.6
}
