use crate::{
    arithmetic::{Deriv, Deriv3, Affine, Affine3},
    sdf,
};
use core::convert::identity;
use glam::{vec3, Mat4, Vec3};
use shared::inst::{Inst, Op, RectangularPrism, SmoothUnion, Sphere};

fn transform_deriv3_by_mat4(mat: &Mat4, a: Deriv3) -> Deriv3 {
    // Hopefully this works.
    // First, rotate the real vector, and then rotate each of the 3D derivatives.
    let v = mat.transform_point3(a.v());
    Deriv3 {
        x: Deriv::new_with_deriv(v.x, mat.transform_point3(a.x.derivatives())),
        y: Deriv::new_with_deriv(v.y, mat.transform_point3(a.y.derivatives())),
        z: Deriv::new_with_deriv(v.z, mat.transform_point3(a.z.derivatives())),
    }
}

macro_rules! generate_interpreter {
    ($name:ident<$ty:ty>, $sdf_path:path, $p:expr, $reg_init:expr, $mat_transform:expr) => {
        #[inline(always)]
        pub fn $name(tape: &[Inst], matrices: &[Mat4], p: Vec3) -> $ty {
            use $sdf_path as s;
            const REG_INIT: [$ty; 2] = $reg_init;

            let mut i = 0;
            let mut regs: [$ty; 2] = REG_INIT;
            let p = $p(p);

            loop {
                let inst = tape[i];
                match inst.op() {
                    Op::Ret => {
                        return regs[inst.reg()];
                    }

                    // Combinations
                    Op::Union => {
                        regs[inst.reg()] = s::union(regs[0], regs[1]);
                    }
                    Op::Intersection => {
                        regs[inst.reg()] = s::intersect(regs[0], regs[1]);
                    }
                    Op::Subtraction => {
                        regs[inst.reg()] = s::subtract(regs[0], regs[1]);
                    }
                    Op::SmoothUnion => {
                        let su = inst.extract::<SmoothUnion>();
                        regs[inst.reg()] = s::smooth_union(regs[0], regs[1], su.k);
                    }
                    Op::SmoothIntersection => {}
                    Op::SmoothSubtraction => {}

                    // Shapes
                    Op::Sphere => {
                        let sphere = inst.extract::<Sphere>();
                        // let mat = &matrices[0];
                        regs[inst.reg()] = s::sphere(p, sphere.radius);
                    }
                    Op::RectangularPrism => {
                        let prism = inst.extract::<RectangularPrism>();
                        // let mat = &matrices[0];
                        regs[inst.reg()] = s::rectangular_prism(p, vec3(prism.x, prism.y, prism.z))
                    }
                }

                i += 1;
            }
        }
    };
}

generate_interpreter!(sdf<f32>, sdf, identity, [0.0, 0.0], Mat4::transform_point3);
generate_interpreter!(
    sdf_deriv<Deriv>,
    sdf::deriv,
    Deriv3::new_xyz,
    [Deriv::ZERO, Deriv::ZERO],
    transform_deriv3_by_mat4
);


#[inline(always)]
pub fn sdf_affine(tape: &[Inst], matrices: &[Mat4], p: Affine3) -> Affine {
    const REG_INIT: [Affine; 2] = [Affine::ZERO, Affine::ZERO];

    let mut i = 0;
    let mut regs = REG_INIT;

    loop {
        let inst = tape[i];
        match inst.op() {
            Op::Ret => {
                return regs[inst.reg()];
            }

            // Combinations
            Op::Union => {
                let (d, choice) = sdf::affine::union(regs[0], regs[1]);
                regs[inst.reg()] = d;
            }
            Op::Intersection => {
                let (d, choice) = sdf::affine::intersect(regs[0], regs[1]);;
                regs[inst.reg()] = d;
            }
            Op::Subtraction => {
                let (d, choice) = sdf::affine::subtract(regs[0], regs[1]);
                regs[inst.reg()] = d;
            }
            Op::SmoothUnion => {
                let su = inst.extract::<SmoothUnion>();
                regs[inst.reg()] = sdf::affine::smooth_union(regs[0], regs[1], su.k);
            }
            Op::SmoothIntersection => {}
            Op::SmoothSubtraction => {}

            // Shapes
            Op::Sphere => {
                let sphere = inst.extract::<Sphere>();
                // let mat = &matrices[0];
                regs[inst.reg()] = sdf::affine::sphere(p, sphere.radius);
            }
            Op::RectangularPrism => {
                let prism = inst.extract::<RectangularPrism>();
                // let mat = &matrices[0];
                regs[inst.reg()] = sdf::affine::rectangular_prism(p, vec3(prism.x, prism.y, prism.z))
            }
        }

        i += 1;
    }
}