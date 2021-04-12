#![no_std]
#![cfg_attr(
    target_arch = "spirv",
    feature(register_attr, asm),
    register_attr(spirv)
)]

#![feature(const_fn_floating_point_arithmetic)]

// #![deny(warnings)]

// mod arrayvec;
mod arithmetic;
pub mod blit;
pub mod compute_renderer;
mod extra;
mod interpreter;
mod sdf;

