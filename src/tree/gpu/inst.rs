//! This module used to generate bytecode that is interpreted the sosm.wgsl shader.
//! Instead of computing the signed distance functions through interpretation, like
//! the MPR technique by Matt Ketter, the shader contains functions for all basic shapes
//! and the shader combines them through things like unions, offsets, etc.

pub enum Reg {
    A = 0,
    B = 1,
}

#[repr(u16)]
pub enum Combination {
    // These have no arguments (except consuming the distances stored in registers A and B).
    Union = 1,
    Intersection = 2,
    Subtraction = 3,

    // These have one argument (besides registers A and B).
    // At the moment, it is a constant that is stored in the second
    // 32 bits of the instruction, but that may change in the future.
    SmoothUnion = 4,
    SmoothIntersection = 5,
    SmoothSubtraction = 6,
}

// Shapes
// Every shape has the index of an structure containing an inverse translate/rotate 3x4 matrix
// and the scale in the second 32 bits.
pub enum Shape {
    Sphere = 7, // The radius is stored in the 3rd 32 bits

    RectangularPrism = 8, // store the side lengths somehow
    // ...
}

pub fn shape_unary(shape: Shape, out: Reg, transform_idx: u32, arg0: u32) -> [u32; 4] {
    [(out as u32) << 30 | shape as u32, transform_idx, arg0, 0]
}

pub fn shape_binary(shape: Shape, out: Reg, transform_idx: u32, arg0: u32, arg1: u32) -> [u32; 4] {
    [(out as u32) << 30 | shape as u32, transform_idx, arg0, arg1]
}

pub fn combination_nullary(comb: Combination, out: Reg) -> [u32; 4] {
    [(out as u32) << 30 | comb as u32, 0, 0, 0]
}

pub fn combination_unary(comb: Combination, out: Reg, arg0: u32) -> [u32; 4] {
    [(out as u32) << 30 | comb as u32, arg0, 0, 0]
}
