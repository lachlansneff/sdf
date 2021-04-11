use core::{convert::TryInto, mem};

#[repr(u32)]
pub enum Op {
    /// Return register at index in arg 0.
    Ret,

    // These have no arguments (except consuming the distances stored in registers 0 and 1).
    Union,
    Intersection,
    Subtraction,

    // These have one argument in arg 0 (besides registers 0 and 1).
    SmoothUnion,
    SmoothIntersection,
    SmoothSubtraction,

    // Shapes
    // Every shape has the index of an structure containing an inverse translate/rotate/scale 4x4 matrix in arg 0.
    /// The radius is stored in arg 1.
    Sphere,

    RectangularPrism, // store the side lengths somehow

                      // ...
}

pub trait InstData {
    const OP: Op;

    fn from_inst(inst: Inst) -> Self;
    fn to_inst(self, data: &mut [u32; 7]);
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Inst([u32; 8]);

impl Inst {
    pub fn reg(self) -> usize {
        (self.0[0] >> 30) as usize
    }

    pub fn op(self) -> Op {
        unsafe { mem::transmute(self.0[0] & 0x3fffffff) }
    }

    fn arg<const N: usize>(self) -> u32
// where
    //     [(); N - 6]:
    {
        self.0[N + 1]
    }

    /// Returns (output register index, T).
    pub fn extract<T: InstData>(self) -> T {
        T::from_inst(self)
    }

    #[cfg(not(target_arch = "spirv"))]
    pub fn make<T: InstData>(reg: usize, data: T) -> Self {
        assert!(reg < 2);
        let mut b = [0; 8];
        b[0] = (T::OP as u32) | ((reg as u32) << 30);
        T::to_inst(data, (&mut b[1..]).try_into().unwrap());
        Inst(b)
    }
}

macro_rules! declare_nonary {
    ($name:ident, $op:expr) => {
        pub struct $name;

        impl InstData for $name {
            const OP: Op = $op;
            fn from_inst(_: Inst) -> Self {
                Self
            }
            fn to_inst(self, _: &mut [u32; 7]) {}
        }
    };
}

declare_nonary!(Ret, Op::Ret);
declare_nonary!(Union, Op::Union);
declare_nonary!(Intersection, Op::Intersection);
declare_nonary!(Subtraction, Op::Subtraction);

macro_rules! declare_smooth_combine {
    ($name:ident, $op:expr) => {
        pub struct $name {
            pub k: f32,
        }

        impl InstData for $name {
            const OP: Op = $op;
            fn from_inst(inst: Inst) -> Self {
                Self {
                    k: f32::from_bits(inst.arg::<0>()),
                }
            }
            fn to_inst(self, data: &mut [u32; 7]) {
                data[0] = self.k.to_bits();
            }
        }
    };
}

declare_smooth_combine!(SmoothUnion, Op::SmoothUnion);
declare_smooth_combine!(SmoothIntersection, Op::SmoothSubtraction);
declare_smooth_combine!(SmoothSubtraction, Op::SmoothSubtraction);

pub struct Sphere {
    pub matrix_idx: usize,
    pub radius: f32,
}

impl InstData for Sphere {
    const OP: Op = Op::Sphere;
    fn from_inst(inst: Inst) -> Self {
        Self {
            matrix_idx: inst.arg::<0>() as usize,
            radius: f32::from_bits(inst.arg::<1>()),
        }
    }

    fn to_inst(self, data: &mut [u32; 7]) {
        data[0] = self.matrix_idx as u32;
        data[1] = self.radius.to_bits();
    }
}

pub struct RectangularPrism {
    pub matrix_idx: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl InstData for RectangularPrism {
    const OP: Op = Op::RectangularPrism;
    fn from_inst(inst: Inst) -> Self {
        Self {
            matrix_idx: inst.arg::<0>() as usize,
            x: f32::from_bits(inst.arg::<1>()),
            y: f32::from_bits(inst.arg::<2>()),
            z: f32::from_bits(inst.arg::<3>()),
        }
    }

    fn to_inst(self, data: &mut [u32; 7]) {
        data[0] = self.matrix_idx as u32;
        data[1] = self.x.to_bits();
        data[2] = self.y.to_bits();
        data[3] = self.z.to_bits();
    }
}
