use glam::{vec3, UVec2, UVec3, Vec3, Vec3Swizzles, Vec4Swizzles};
use spirv_std::StorageImage2d;

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

use crate::{
    deriv::{Deriv, Deriv3},
    grid::Grid,
    sdf, ViewParams,
};

#[spirv(compute(threads(8, 8, 1)))]
pub fn sphere_push(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(num_workgroups)] num_workgroups: UVec3,
    #[spirv(push_constant)] view_params: &ViewParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] farthest_grid_data: &mut [f32],
) {
    let grid_size = num_workgroups.xy() * 8;
    let mut grid = Grid::new(grid_size.x as usize, farthest_grid_data);
}

#[spirv(compute(threads(8, 8, 1)))]
pub fn render_sdf_final(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(push_constant)] view_params: &ViewParams,
    // #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] tape: &[Inst],
    #[spirv(descriptor_set = 0, binding = 1)] output_texture: &StorageImage2d,
) {
    let texture_coords = global_invocation_id.xy();

    if texture_coords.x >= view_params.resolution.x || texture_coords.y >= view_params.resolution.y
    {
        // We're off the edge, so just return early.
        // This might result in some iffy performance around the edges, but there's
        // nothing else to do.
        return;
    }

    let ray_dir = compute_ray_direction(&view_params, texture_coords);
    let intersection = sphere_march(view_params.eye, ray_dir, sdf);

    let color = if intersection.depth_ratio > 0.0 {
        let color = vec3(171.0 / 255.0, 146.0 / 255.0, 103.0 / 255.0);
        let shade = vec3(99.0 / 255.0, 84.0 / 255.0, 59.0 / 255.0);
        let ao = 1.0 - intersection.depth_ratio;

        let normals = sdf_diff(intersection.hit).derivatives();
        let dif = normals.dot(view_params.light_pos.normalize());
        color.lerp(shade, dif) * ao
    } else {
        // Background color
        vec3(140.0 / 255.0, 156.0 / 255.0, 161.0 / 255.0)
    };

    unsafe {
        output_texture.write(texture_coords, color.extend(1.0));
    }
}

fn sdf(p: Vec3) -> f32 {
    sdf::intersect(
        sdf::schwarz_p(p, 10.0, 0.03),
        sdf::rectangular_prism(p, vec3(1.0, 1.0, 1.0)),
    )
}

fn sdf_diff(p: Vec3) -> Deriv {
    let p = Deriv3::new(p);
    sdf::deriv::intersect(
        sdf::deriv::schwarz_p(p, 10.0, 0.03),
        sdf::deriv::rectangular_prism(p, vec3(1.0, 1.0, 1.0)),
    )
}

fn compute_ray_direction(params: &ViewParams, texture_coords: UVec2) -> Vec3 {
    let xy = texture_coords.as_f32() - params.resolution.as_f32() / 2.0;
    (params.matrix * xy.extend(-params.z_depth).normalize().extend(0.0)).xyz()
}

struct Intersection {
    hit: Vec3,
    /// a percentage from 0 to 1
    /// e.g. the number of steps divided by MAX_STEPS
    depth_ratio: f32,
}

/// Change this to return an option maybe, when it's supported?
fn sphere_march(origin: Vec3, ray_dir: Vec3, sdf: impl Fn(Vec3) -> f32) -> Intersection {
    const MAX_STEPS: usize = 64;
    const EPSILON: f32 = 0.001;

    let mut t = 0.0;

    for i in 0..MAX_STEPS {
        let p = origin + ray_dir * t;
        let r = sdf(p);

        if r < EPSILON * t {
            return Intersection {
                hit: p,
                depth_ratio: i as f32 / (MAX_STEPS - 1) as f32,
            };
        }

        t += r;
    }

    Intersection {
        hit: Vec3::ZERO,
        depth_ratio: -1.0,
    }
}
