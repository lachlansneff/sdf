use core::mem;
use glam::UVec4;

#[repr(u32)]
pub enum Op {
    Ret = 0,
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

    // Shapes
    // Every shape has the index of an structure containing an inverse translate/rotate 3x4 matrix
    // and the scale in the second 32 bits.
    Sphere = 7, // The radius is stored in the 3rd 32 bits

    RectangularPrism = 8, // store the side lengths somehow
                          // ...
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Inst(UVec4);

impl Inst {
    pub fn reg(self) -> usize {
        (self.0.x >> 30) as usize
    }

    pub fn op(self) -> Op {
        unsafe { mem::transmute(self.0.x & 0x3fffffff) }
    }

    pub fn transform_index(self) -> usize {
        self.0.y as usize
    }

    pub fn arg0(self) -> u32 {
        self.0.z
    }

    pub fn arg1(self) -> u32 {
        self.0.w
    }
}
