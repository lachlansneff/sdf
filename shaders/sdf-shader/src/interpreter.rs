use glam::Vec3;
use crate::{
    inst::{Inst, Op},
    sdf,
};

pub fn interpret_sdf(bytecode: &[Inst], p: Vec3) -> f32 {
    let mut i = 0;
    let mut regs = [0.0; 2];

    loop {
        let inst = bytecode[i];
        match inst.op() {
            Op::Ret => {
                return regs[inst.reg()];
            }

            // Combinations
            Op::Union => {
                regs[inst.reg()] = sdf::union(regs[0], regs[1]);
            }
            Op::Intersection => {
                regs[inst.reg()] = sdf::intersect(regs[0], regs[1]);
            }
            Op::Subtraction => {
                regs[inst.reg()] = sdf::subtract(regs[0], regs[1]);
            }
            Op::SmoothUnion => {}
            Op::SmoothIntersection => {}
            Op::SmoothSubtraction => {}

            // Shapes
            Op::Sphere => {
                let radius = f32::from_bits(inst.arg0());
                // TODO: retrieve transformation data.
                regs[inst.reg()] = sdf::sphere(p, radius);
            }
            Op::RectangularPrism => {}
        }

        i += 1;
    }
}
