use glam::{vec2, vec4, UVec2, Vec4, Vec4Swizzles};
#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;
use spirv_std::{Image2d, Sampler};

#[spirv(vertex)]
pub fn vertex(#[spirv(vertex_index)] vertex_id: i32, #[spirv(position)] output: &mut Vec4) {
    let uv = vec2(((vertex_id << 1) & 2) as f32, (vertex_id & 2) as f32);
    *output = vec4(uv.x * 2.0 - 1.0, -uv.y * 2.0 + 1.0, 0.0, 1.0);
}

#[spirv(fragment)]
pub fn fragment(
    #[spirv(frag_coord)] frag_coord: Vec4,

    #[spirv(push_constant)] resolution: &UVec2,
    #[spirv(descriptor_set = 0, binding = 0)] input_texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,

    output: &mut Vec4,
) {
    *output = input_texture.sample(*sampler, frag_coord.xy() / resolution.as_f32());
}
