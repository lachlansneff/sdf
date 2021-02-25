use std::{fmt, mem, ops::Range, rc::Rc};

use ultraviolet::{f32x8, Vec3};

use self::{eval::Evaluate, eval_cpu::CpuEval};

pub mod eval;
pub mod eval_cpu;
// pub mod eval_glsl;
pub mod fills;
pub mod operations;
pub mod shapes;

// TODO: get rid of this
#[derive(Debug)]
pub enum Shape {
    Sphere {
        radius: ConstantOrExpr,
    },
    Box {
        side_x: ConstantOrExpr,
        side_y: ConstantOrExpr,
        side_z: ConstantOrExpr,
    },
    // ...
}

impl Evaluate for Shape {
    fn eval<E: eval::Eval>(&self, p: E::R3) -> E::R1 {
        use eval::{Real1, Real3};
        match self {
            Shape::Sphere { radius } => shapes::sphere::<E>(p, E::R1::new(radius.get())),
            Shape::Box {
                side_x,
                side_y,
                side_z,
            } => shapes::rectangular_prism::<E>(
                p,
                E::R3::new(side_x.get(), side_y.get(), side_z.get()),
            ),
        }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Sphere { radius } => write!(f, "sphere, r = {}", radius),
            Shape::Box {
                side_x,
                side_y,
                side_z,
            } => write!(f, "box, sides = ⟨{}, {}, {}⟩", side_x, side_y, side_z),
        }
    }
}

#[derive(Debug)]
pub enum Fill {
    Gyroid {
        scale: ConstantOrExpr,
        thickness: ConstantOrExpr,
    },
    SchwarzP {
        scale: ConstantOrExpr,
        thickness: ConstantOrExpr,
    },
    // ...
}

impl Evaluate for Fill {
    fn eval<E: eval::Eval>(&self, p: E::R3) -> E::R1 {
        use eval::Real1;
        match self {
            Fill::Gyroid { scale, thickness } => {
                fills::gyroid::<E>(p, E::R1::new(scale.get()), E::R1::new(thickness.get()))
            }
            Fill::SchwarzP { scale, thickness } => {
                fills::schwarz_p::<E>(p, E::R1::new(scale.get()), E::R1::new(thickness.get()))
            }
        }
    }
}

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fill::Gyroid { scale, thickness } => {
                write!(f, "gyroid(s = {}, t = {})", scale, thickness)
            }
            Fill::SchwarzP { scale, thickness } => {
                write!(f, "schwarz primitive(s = {}, t = {})", scale, thickness)
            }
        }
    }
}

/// TODO: Needs Units
#[derive(Debug)]
pub enum ConstantOrExpr {
    Constant(f32),
    /// TODO: Write a math expression parser that can convert to shader code.
    ///     - https://en.wikipedia.org/wiki/Shunting-yard_algorithm
    Expr(),
}

impl ConstantOrExpr {
    pub fn get(&self) -> f32 {
        match *self {
            ConstantOrExpr::Constant(x) => x,
            ConstantOrExpr::Expr() => panic!(),
        }
    }
}

impl fmt::Display for ConstantOrExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantOrExpr::Constant(x) => write!(f, "{}", x),
            ConstantOrExpr::Expr() => write!(f, "no expr specified"),
        }
    }
}

#[derive(Debug)]
pub enum CsgNode {
    Shape(Shape, Option<Fill>),
    Union {
        lhs: Rc<CsgNode>,
        rhs: Rc<CsgNode>,
    },
    SmoothUnion {
        lhs: Rc<CsgNode>,
        rhs: Rc<CsgNode>,
        k: ConstantOrExpr,
    },
    Intersection {
        lhs: Rc<CsgNode>,
        rhs: Rc<CsgNode>,
    },
    SmoothIntersection {
        lhs: Rc<CsgNode>,
        rhs: Rc<CsgNode>,
        k: ConstantOrExpr,
    },
    Subtraction {
        lhs: Rc<CsgNode>,
        rhs: Rc<CsgNode>,
    },
    SmoothSubtraction {
        lhs: Rc<CsgNode>,
        rhs: Rc<CsgNode>,
        k: ConstantOrExpr,
    },
    Translate {
        x: ConstantOrExpr,
        y: ConstantOrExpr,
        z: ConstantOrExpr,
        node: Rc<CsgNode>,
    },
    Rotate {
        roll: ConstantOrExpr,
        pitch: ConstantOrExpr,
        yaw: ConstantOrExpr,
        node: Rc<CsgNode>,
    },
}

/// A Constructive Solid Geometry Tree.
pub struct CsgTree {
    root: Option<CsgNode>,
}

impl CsgTree {
    pub fn new_example() -> Self {
        Self {
            root: Some(CsgNode::Union {
                lhs: Rc::new(CsgNode::SmoothUnion {
                    lhs: Rc::new(CsgNode::Shape(
                        Shape::Sphere {
                            radius: ConstantOrExpr::Constant(1.0),
                        },
                        None,
                    )),
                    rhs: Rc::new(CsgNode::Shape(
                        Shape::Box {
                            side_x: ConstantOrExpr::Constant(1.0),
                            side_y: ConstantOrExpr::Constant(1.0),
                            side_z: ConstantOrExpr::Constant(1.0),
                        },
                        Some(Fill::Gyroid {
                            scale: ConstantOrExpr::Constant(10.0),
                            thickness: ConstantOrExpr::Constant(0.02),
                        }),
                    )),
                    k: ConstantOrExpr::Constant(0.4),
                }),
                rhs: Rc::new(CsgNode::Shape(
                    Shape::Box {
                        side_x: ConstantOrExpr::Constant(1.0),
                        side_y: ConstantOrExpr::Constant(1.0),
                        side_z: ConstantOrExpr::Constant(1.0),
                    },
                    None,
                )),
            }),
        }
    }

    /// Automatically evalutes using simd.
    pub fn eval_points_on_cpu<'a>(
        &'a self,
        points: impl IntoIterator<Item = Vec3> + 'a,
    ) -> impl Iterator<Item = f32> + 'a {
        use eval::{Real1, Real3};
        use eval_cpu::{CpuReal1, CpuReal3};

        pub struct Chunk8Iter<I>(I);

        impl<I: Iterator<Item = Vec3>> Iterator for Chunk8Iter<I> {
            type Item = ([Vec3; 8], usize);
            fn next(&mut self) -> Option<Self::Item> {
                let mut array: [Vec3; 8] = Default::default();
                let iter = (&mut self.0).take(8);

                let mut count = 0;
                for (a, b) in array.iter_mut().zip(iter) {
                    count += 1;
                    *a = b;
                }

                if count != 0 {
                    Some((array, count))
                } else {
                    None
                }
            }
        }

        pub struct F32x8Iter {
            data: [f32; 8],
            alive: Range<usize>,
        }

        impl F32x8Iter {
            fn new(v: f32x8, count: usize) -> Self {
                Self {
                    data: unsafe { mem::transmute(v) },
                    alive: 0..count,
                }
            }
        }

        impl Iterator for F32x8Iter {
            type Item = f32;
            fn next(&mut self) -> Option<f32> {
                self.alive
                    .next()
                    .map(|idx| unsafe { *self.data.get_unchecked(idx) })
            }
        }

        fn recurse(node: &CsgNode, p: CpuReal3) -> CpuReal1 {
            match node {
                CsgNode::Shape(shape, fill) => {
                    if let Some(fill) = fill {
                        operations::intersection::<CpuEval>(
                            shape.eval::<CpuEval>(p),
                            fill.eval::<CpuEval>(p),
                        )
                    } else {
                        shape.eval::<CpuEval>(p)
                    }
                }
                CsgNode::Union { lhs, rhs } => {
                    operations::union::<CpuEval>(recurse(&lhs, p), recurse(&rhs, p))
                }
                CsgNode::SmoothUnion { lhs, rhs, k } => operations::smooth_union::<CpuEval>(
                    recurse(&lhs, p),
                    recurse(&rhs, p),
                    CpuReal1::new(k.get()),
                ),
                CsgNode::Intersection { lhs, rhs } => {
                    operations::intersection::<CpuEval>(recurse(&lhs, p), recurse(&rhs, p))
                }
                CsgNode::SmoothIntersection { lhs, rhs, k } => {
                    operations::smooth_intersection::<CpuEval>(
                        recurse(&lhs, p),
                        recurse(&rhs, p),
                        CpuReal1::new(k.get()),
                    )
                }
                CsgNode::Subtraction { lhs, rhs } => {
                    operations::subtraction::<CpuEval>(recurse(&lhs, p), recurse(&rhs, p))
                }
                CsgNode::SmoothSubtraction { lhs, rhs, k } => {
                    operations::smooth_subtraction::<CpuEval>(
                        recurse(&lhs, p),
                        recurse(&rhs, p),
                        CpuReal1::new(k.get()),
                    )
                }
                CsgNode::Translate { x, y, z, node } => {
                    recurse(&node, p + CpuReal3::new(x.get(), y.get(), z.get()))
                }
                CsgNode::Rotate { .. } => todo!(),
            }
        }

        let root = self.root.as_ref().unwrap();

        Chunk8Iter(points.into_iter())
            .map(move |(array, count)| {
                F32x8Iter::new(recurse(root, CpuReal3(array.into())).0, count)
            })
            .flatten()
    }
}

impl fmt::Display for CsgTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const CROSS: &str = " ├─";
        const CORNER: &str = " └─";
        const VERTICAL: &str = " │ ";
        const SPACE: &str = "   ";

        fn recurse(
            f: &mut fmt::Formatter,
            node: &CsgNode,
            mut indent: String,
            last_node: bool,
            top_node: bool,
        ) -> fmt::Result {
            if !top_node {
                write!(f, "{}", indent)?;
                if last_node {
                    write!(f, "{}", CORNER)?;
                    indent += SPACE;
                } else {
                    // write!(f, "{}", " | ".repeat(indents))?;
                    write!(f, "{}", CROSS)?;
                    indent += VERTICAL;
                }
            }

            match node {
                CsgNode::Shape(shape, fill) => {
                    if let Some(fill) = fill {
                        writeln!(f, "{}, fill = {}", shape, fill)?
                    } else {
                        writeln!(f, "{}", shape)?
                    }
                }
                CsgNode::Union { lhs, rhs } => {
                    writeln!(f, "union")?;
                    recurse(f, &lhs, indent.clone(), false, false)?;
                    recurse(f, &rhs, indent, true, false)?;
                }
                CsgNode::SmoothUnion { lhs, rhs, k } => {
                    writeln!(f, "smooth union, k = {}", k)?;
                    recurse(f, &lhs, indent.clone(), false, false)?;
                    recurse(f, &rhs, indent, true, false)?;
                }
                CsgNode::Intersection { lhs, rhs } => {
                    writeln!(f, "intersection")?;
                    recurse(f, &lhs, indent.clone(), false, false)?;
                    recurse(f, &rhs, indent, true, false)?;
                }
                CsgNode::SmoothIntersection { lhs, rhs, k } => {
                    writeln!(f, "smooth intersection, k = {}", k)?;
                    recurse(f, &lhs, indent.clone(), false, false)?;
                    recurse(f, &rhs, indent, true, false)?;
                }
                CsgNode::Subtraction { lhs, rhs } => {
                    writeln!(f, "subtraction")?;
                    recurse(f, &lhs, indent.clone(), false, false)?;
                    recurse(f, &rhs, indent, true, false)?;
                }
                CsgNode::SmoothSubtraction { lhs, rhs, k } => {
                    writeln!(f, "smooth subtraction, k = {}", k)?;
                    recurse(f, &lhs, indent.clone(), false, false)?;
                    recurse(f, &rhs, indent, true, false)?;
                }
                CsgNode::Translate { x, y, z, node } => {
                    writeln!(f, "translate by ⟨{}, {}, {}⟩", x, y, z)?;
                    recurse(f, &node, indent, true, false)?;
                }
                CsgNode::Rotate {
                    roll,
                    pitch,
                    yaw,
                    node,
                } => {
                    writeln!(f, "rotate by ⟨{}, {}, {}⟩", roll, pitch, yaw)?;
                    recurse(f, &node, indent, true, false)?;
                }
            };
            Ok(())
        }

        if let Some(root) = &self.root {
            recurse(f, root, "".to_string(), true, true)
        } else {
            write!(f, "empty CSG tree")
        }
    }
}
