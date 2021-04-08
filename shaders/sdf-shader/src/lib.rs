#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr, asm),
    register_attr(spirv)
)]

// #![deny(warnings)]

mod extra;
mod inst;
// mod interpreter;
pub mod blit;
pub mod compute_renderer;
mod diff;
mod sdf;

use glam::{Mat4, Vec2, Vec3};

#[repr(C)]
pub struct ViewParams {
    matrix: Mat4,
    eye: Vec3,
    light_pos: Vec3,
    resolution: Vec2,
    z_depth: f32,
}

macro_rules! assert_eq_size {
    ($x:ty, $($xs:ty),+ $(,)?) => {
        const _: fn() = || {
            $(let _ = core::mem::transmute::<$x, $xs>;)+
        };
    };
}

assert_eq_size!(ViewParams, [u8; 112]);

// #[allow(unused_attributes)]
// #[spirv(vertex)]
// pub fn main_vs(
//     #[spirv(vertex_index)] vertex_id: i32,
//     #[spirv(position)] output: &mut Vec4,
// ) {
//     let uv = vec2(
//         ((vertex_id << 1) & 2) as f32,
//         (vertex_id & 2) as f32
//     );
//     *output = vec4(uv.x * 2.0 - 1.0, -uv.y * 2.0 + 1.0, 0.0, 1.0);
// }

// const MAX_STEPS: usize = 64;
// const EPSILON: f32 = 0.001;

// struct Intersection {
//     dist: f32,
//     steps: usize,
// }

// fn estimate_normals(p: Vec3, sdf: impl Fn(Vec3) -> f32) -> Vec3 {
//     let k = vec2(1.0, -1.0);

//     (k.xyy() * sdf(p + k.xyy() * EPSILON) +
//     k.yyx() * sdf(p + k.yyx() * EPSILON) +
//     k.yxy() * sdf(p + k.yxy() * EPSILON) +
//     k.xxx() * sdf(p + k.xxx() * EPSILON)).normalize()
// }

// /// Change this to return an option maybe, when it's supported?
// fn sphere_march(
//     origin: Vec3,
//     ray_dir: Vec3,
//     sdf: impl Fn(Vec3) -> f32,
// ) -> Intersection {
//     let mut t = 0.0;
//     let mut i = 0;

//     while i < MAX_STEPS {
//         let r = sdf(origin + ray_dir * t);

//         if r < EPSILON * t {
//             return Intersection {
//                 dist: t,
//                 steps: i
//             };
//         }

//         t += r;
//         i += 1;
//     }

//     Intersection {
//         dist: -1.0,
//         steps: MAX_STEPS
//     }
// }

// fn compute_ray_direction(data: &ShaderData, frag_coord: Vec4) -> Vec3 {
//     let xy = frag_coord.xy() - data.resolution / 2.0;
//     (data.matrix * xy.extend(-data.z_depth).normalize().extend(0.0)).xyz()
// }

// #[allow(unused_attributes)]
// #[spirv(fragment)]
// pub fn main_fs(
//     #[spirv(frag_coord)] frag_coord: Vec4,
//     #[spirv(uniform, descriptor_set = 0, binding = 0)] data: &ShaderData,
//     output: &mut Vec4,
// ) {
//     let ray_dir = compute_ray_direction(&data, frag_coord);
//     let intersection = sphere_march(
//         data.eye,
//         ray_dir,
//         |p| {
//             // let s = sdf::sphere(p + vec3(1.0, 0.0, 0.0));

//             sdf::intersect(
//                 sdf::schwarz_p(p, 10.0, 0.03),
//                 sdf::rectangular_prism(p, vec3(1.0, 1.0, 1.0)),
//             )
//         },
//     );

//     if intersection.dist == -1.0 {
//         *output = vec4(140.0/255.0, 156.0/255.0, 161.0/255.0, 0.0);
//         return;
//     }

//     let color = vec3(171.0/255.0, 146.0/255.0, 103.0/255.0);
//     let shade = vec3(99.0/255.0, 84.0/255.0, 59.0/255.0);
//     let ao = 1.0 - (intersection.steps as f32) / (MAX_STEPS - 1) as f32;

//     // let dif = intersection.normals.dot(data.light_pos.normalize());

//     // let color = color.lerp(shade, dif) * ao;

//     *output = color.extend(1.0);
// }
