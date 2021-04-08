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
mod deriv;
mod sdf;

use glam::{Mat4, UVec2, Vec3};

#[repr(C)]
pub struct ViewParams {
    matrix: Mat4,
    eye: Vec3,
    light_pos: Vec3,
    resolution: UVec2,
    z_depth: f32,
}

static_assertions::assert_eq_size!(ViewParams, [u8; 112]);
