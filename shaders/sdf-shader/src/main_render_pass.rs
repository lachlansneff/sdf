#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

use spirv_std::StorageImage2d;
use glam::{Vec3, Vec4, Vec4Swizzles, vec2, vec3, vec4};

use crate::{ViewParams, sdf};


#[spirv(vertex)]
pub fn vertex_shader(
    #[spirv(vertex_index)] vertex_id: i32,
    #[spirv(position)] output: &mut Vec4,
) {
    let uv = vec2(
        ((vertex_id << 1) & 2) as f32,
        (vertex_id & 2) as f32
    );
    *output = vec4(uv.x * 2.0 - 1.0, -uv.y * 2.0 + 1.0, 0.0, 1.0);
}

#[spirv(fragment)]
pub fn fragment_shader(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] view_params: &ViewParams,
    output: &mut Vec4,
) {
    let ray_dir = compute_ray_direction(&view_params, frag_coord);
    let intersection = sphere_march(
        view_params.eye,
        ray_dir,
        |p| {
            // let s = sdf::sphere(p + vec3(1.0, 0.0, 0.0));

            sdf::intersect(
                sdf::schwarz_p(p, 10.0, 0.03),
                sdf::rectangular_prism(p, vec3(1.0, 1.0, 1.0)),
            )
        },
    );

    if intersection.steps == 0 {
        *output = vec4(140.0/255.0, 156.0/255.0, 161.0/255.0, 0.0);
        return;
    }

    let color = vec3(171.0/255.0, 146.0/255.0, 103.0/255.0);
    let shade = vec3(99.0/255.0, 84.0/255.0, 59.0/255.0);
    let ao = 1.0 - (intersection.steps as f32) / (MAX_STEPS - 1) as f32;

    // let dif = intersection.normals.dot(data.light_pos.normalize());

    // let color = color.lerp(shade, dif) * ao;

    *output = color.extend(1.0);
}


