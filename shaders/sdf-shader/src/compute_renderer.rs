use core::sync::atomic::AtomicUsize;

use glam::{Mat4, UVec2, UVec3, Vec3, Vec3Swizzles, Vec4Swizzles, uvec2, vec3};

use shared::inst::Inst;
#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

use crate::{arithmetic::{Affine3, interval}, interpreter};

// #[repr(C)]
// pub struct ConeTracingParams {
//     view_mat: Mat4,
//     eye: Vec3,
//     resolution: UVec2,
//     grid_size: UVec2,
//     neg_z_depth: f32,
//     cone_multiplier: f32,
// }

// /// Runs on 64 pixel x 64 pixel tiles and uses the initial tape.
// /// 
// #[spirv(compute(threads(8, 8, 1)))]
// pub fn cone_push_64x64(
//     #[spirv(global_invocation_id)] global_invocation_id: UVec3,
//     #[spirv(num_workgroups)] num_workgroups: UVec3,
//     #[spirv(push_constant)] params: &ConeTracingParams,
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] initial_tape: &[Inst],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] starting_depths: &mut [f32],
// ) {
//     let tile_coords = global_invocation_id.xy();

//     if tile_coords.x >= params.grid_size.x || tile_coords.y >= params.grid_size.y {
//         // If we're off the edge, just return.
//         // This will happen when the resolution width or height aren't multiples
//         // of 64 * 8.
//         return;
//     }

//     let tile_center = tile_coords * 64 + uvec2(32, 32);

//     let ray_dir = compute_ray_direction(
//         params.resolution,
//         params.neg_z_depth,
//         params.view_mat,
//         tile_center,
//     );

//     let idx = tile_coords.y as usize * params.grid_size.x as usize + tile_coords.x as usize;
//     starting_depths[idx] = cone_march(params.eye, ray_dir, params.cone_multiplier, sdf);
// }

// fn cone_march(origin: Vec3, ray_dir: Vec3, cone_multiplier: f32, sdf: impl Fn(Vec3) -> f32) -> f32 {
//     const MAX_STEPS: usize = 64;
//     const EPSILON: f32 = 0.001;

//     let mut t = 0.0;

//     for _ in 0..MAX_STEPS {
//         let p = origin + ray_dir * t;
//         let d = sdf(p);

//         let x = (t + d) * cone_multiplier;
//         if x - t <= EPSILON {
//             return t;
//         }

//         t = x;
//     }

//     0.0
// }

#[repr(C)]
pub struct RenderParams {
    view_mat: Mat4,
    eye: Vec3,
    light_pos: Vec3,
    resolution: UVec2,
    grid_size: UVec2,
    neg_z_depth: f32,
    /// The size of the initial instruction tape.
    initial_tape_len: usize,
    /// The starting index of the space available for
    /// the 8x8 tile tape optimizer.
    tile8x8_tape_start: usize,
}

#[cfg(target_arch = "spirv")]
static_assertions::assert_eq_size!(RenderParams, [u8; 128]);

/// The initial tape is stored at 0..params.initial_tape_len.
/// This runs on a 64x64 pixel tile.
///
/// Because parts of the model could be at any depth within the tile, this evaluates
/// the initial tape on the entire beam volume with the x and y intervals
/// corresponding to the tiles bounds in world coordinates and the z interval stretching
/// from the near plane to the far plane.
#[spirv(compute(threads(8, 8, 1)))]
pub fn evaluate_ray_volume_64x64_tiles(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(push_constant)] params: &RenderParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global_tapes: &mut [Inst],
    // #[spirv(storage_buffer, descriptor_set = 0, binding = 1)]
) {
    let tile_coords = global_invocation_id.xy();
    
    if tile_coords.x >= params.grid_size.x || tile_coords.y >= params.grid_size.y {
        return;
    }

    let tile_center = tile_coords * 64 + uvec2(32, 32);

    let ro = params.eye;
    let rd = compute_ray_direction(
        params.resolution,
        params.neg_z_depth,
        params.view_mat,
        tile_center,
    );
    // Is this the near and far clip distances?
    let t = interval(0.0, 1.0);

    // This *should* be a skewed bounding box.
    let ray_bounds = Affine3 {
        x: (t * rd.x + ro.x).into(),
        y: (t * rd.y + ro.y).into(),
        z: (t * rd.z + ro.z).into(),
    };


}

#[spirv(compute(threads(8, 8, 1)))]
pub fn render_sdf_final(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(push_constant)] params: &RenderParams,
    #[spirv(descriptor_set = 0, binding = 0)] output_texture: &CustomStorageImage2d,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] tape: &[Inst],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] matrices: &[Mat4],
) {
    let texture_coords = global_invocation_id.xy();

    if texture_coords.x >= params.resolution.x || texture_coords.y >= params.resolution.y {
        // We're off the edge, so just return early.
        // This might result in some iffy performance around the edges, but there's
        // nothing else to do.
        return;
    }

    let ray_dir = compute_ray_direction(
        params.resolution,
        params.neg_z_depth,
        params.view_mat,
        texture_coords,
    );

    let intersection = sphere_march(params.eye, ray_dir, |p| interpreter::sdf(tape, matrices, p));

    let color = if intersection.depth_ratio > 0.0 {
        let color = vec3(171.0 / 255.0, 146.0 / 255.0, 103.0 / 255.0);
        let shade = vec3(99.0 / 255.0, 84.0 / 255.0, 59.0 / 255.0);
        let ao = 1.0 - intersection.depth_ratio;

        let normals = interpreter::sdf_deriv(tape, matrices, intersection.hit)
            .derivatives()
            .normalize()
            * 0.5
            + vec3(0.5, 0.5, 0.5);
        let dif = normals.dot(params.light_pos.normalize());
        let color = color.lerp(shade, dif) * ao;

        color.lerp(vec3(1.0, 0.0, 0.0), intersection.depth_ratio)
    } else {
        // Background color
        vec3(140.0 / 255.0, 156.0 / 255.0, 161.0 / 255.0)
    };

    unsafe {
        output_texture.write(texture_coords, color.extend(1.0));
    }
}

fn compute_ray_direction(
    resolution: UVec2,
    neg_z_depth: f32,
    view_mat: Mat4,
    texture_coords: UVec2,
) -> Vec3 {
    let xy = texture_coords.as_f32() - resolution.as_f32() / 2.0;
    (view_mat * xy.extend(neg_z_depth).normalize().extend(0.0)).xyz()
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

#[spirv(image_type(
    // sampled_type is hardcoded to f32 for now
    dim = "Dim2D",
    depth = 0,
    arrayed = 0,
    multisampled = 0,
    sampled = 2,
    image_format = "R32f"
))]
#[derive(Copy, Clone)]
pub struct CustomStorageImage2d {
    _x: u32,
}

impl CustomStorageImage2d {
    /// Write a texel to an image without a sampler.
    #[spirv_std_macros::gpu_only]
    pub unsafe fn write<I, const N: usize>(
        &self,
        coordinate: impl spirv_std::vector::Vector<I, 2>,
        texels: impl spirv_std::vector::Vector<f32, N>,
    ) where
        I: spirv_std::integer::Integer,
    {
        asm! {
            "%image = OpLoad _ {this}",
            "%coordinate = OpLoad _ {coordinate}",
            "%texels = OpLoad _ {texels}",
            "OpImageWrite %image %coordinate %texels",
            this = in(reg) self,
            coordinate = in(reg) &coordinate,
            texels = in(reg) &texels,
        }
    }
}
