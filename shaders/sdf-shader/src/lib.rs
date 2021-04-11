#![no_std]
#![cfg_attr(
    target_arch = "spirv",
    feature(register_attr, asm),
    register_attr(spirv)
)]

// #![deny(warnings)]

// mod arrayvec;
pub mod blit;
pub mod compute_renderer;
mod deriv;
mod extra;
mod interpreter;
mod sdf;
