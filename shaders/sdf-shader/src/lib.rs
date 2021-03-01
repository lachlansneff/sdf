#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]

#[cfg(not(target_arch = "spirv"))]
#[macro_use]
pub extern crate spirv_std_macros;

mod extra;
mod sdf;
// mod inst;
// mod interpreter;
mod diff;

// #![deny(warnings)]

use diff::{Dual, DualVec3};
use spirv_std::glam::{vec2, vec3, vec4, Vec2, Vec3, Vec4, Mat4, Vec2Swizzles as _, Vec4Swizzles as _};
use spirv_std::storage_class::{Input, Output, Uniform};

#[allow(unused_attributes)]
#[spirv(block)]
#[repr(C)]
pub struct ShaderData {
    matrix: Mat4,
    eye: Vec3,
    resolution: Vec2,
    z_depth: f32,
    light_pos: Vec3,
}

#[allow(unused_attributes)]
#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vertex_id: Input<i32>,
    #[spirv(position)] mut position: Output<Vec4>,
) {
    let vertex_id = *vertex_id;

    let uv = vec2(
        ((vertex_id << 1) & 2) as f32,
        (vertex_id & 2) as f32
    );
    *position = vec4(uv.x * 2.0 - 1.0, -uv.y * 2.0 + 1.0, 0.0, 1.0);
}

const MAX_STEPS: usize = 64;
const EPSILON: f32 = 0.001;

// fn sdf(p: Vec3) -> f32 {
    
// }

struct Foo {
    a: Vec3,
    b: Vec3,
}

struct Intersection {
    // normals: Vec3,
    dist: f32,
    steps: usize,
}

fn estimate_normals(p: Vec3, sdf: impl Fn(Vec3) -> f32) -> Vec3 {
    let k = vec2(1.0, -1.0);

    (k.xyy() * sdf(p + k.xyy() * EPSILON) +
    k.yyx() * sdf(p + k.yyx() * EPSILON) +
    k.yxy() * sdf(p + k.yxy() * EPSILON) +
    k.xxx() * sdf(p + k.xxx() * EPSILON)).normalize()
}

/// Change this to return an option maybe, when it's supported?
fn sphere_march(
    origin: Vec3,
    ray_dir: Vec3,
    sdf: impl Fn(Vec3) -> f32,
    // sdf_diff: impl Fn(Foo) -> f32,
) -> Intersection {
    let mut t = 0.0;
    let mut i = 0;

    while i < MAX_STEPS {
        let r = sdf(origin + ray_dir * t);

        if r < EPSILON * t {
            return Intersection {
                // normals: Vec3::new(sdf_diff(Foo { a: origin + ray_dir * t, b: Vec3::zero() }), 0.0, 0.0),
                dist: t,
                steps: i
            };
        }

        t += r;
        i += 1;
    }

    Intersection {
        // normals: Vec3::zero(),
        dist: -1.0,
        steps: MAX_STEPS
    }
}

fn compute_ray_direction(data: &ShaderData, frag_coord: Vec4) -> Vec3 {
    let xy = frag_coord.xy() - data.resolution / 2.0;
    (data.matrix * xy.extend(-data.z_depth).normalize().extend(0.0)).xyz()
}

// fn bar(f: impl Fn(Foo)) {
//     f(Foo { a: Vec3::zero(), b: Vec3::zero() })
// }

#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Input<Vec4>,
    #[spirv(descriptor_set = 0, binding = 0)] data: Uniform<ShaderData>,
    mut out_color: Output<Vec4>,
) {
    // bar(|_| {});
    let ray_dir = compute_ray_direction(&data, *frag_coord);
    let intersection = sphere_march(
        data.eye,
        ray_dir,
        |p| {
            // let s = sdf::sphere(p + vec3(1.0, 0.0, 0.0));

            sdf::intersect(
                sdf::schwarz_p(p, 10.0, 0.03),
                sdf::rectangular_prism(p, vec3(1.0, 1.0, 1.0)),
            )
        },
        // |p| {
        //     sdf::intersect_diff(
        //         sdf::schwarz_p_diff(p, 10.0, 0.03),
        //         sdf::rectangular_prism_diff(p, vec3(1.0, 1.0, 1.0)),
        //     )
        // }
    );

    if intersection.dist == -1.0 {
        *out_color = vec4(140.0/255.0, 156.0/255.0, 161.0/255.0, 0.0);
        return;
    }

    let color = vec3(171.0/255.0, 146.0/255.0, 103.0/255.0);
    let shade = vec3(99.0/255.0, 84.0/255.0, 59.0/255.0);
    let ao = 1.0 - (intersection.steps as f32) / (MAX_STEPS - 1) as f32;

    // let dif = intersection.normals.dot(data.light_pos.normalize());

    // let color = color.lerp(shade, dif) * ao;

    *out_color = color.extend(1.0);
}
