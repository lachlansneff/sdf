use std::{cell::RefCell, fmt, rc::Rc};

use crate::op::{Op, Op3};

pub trait Shape: fmt::Debug {
    fn generate_ops(&self) -> Op;
}

#[derive(Debug)]
pub struct Sphere {
    radius: Op,
}

impl Shape for Sphere {
    fn generate_ops(&self) -> Op {
        Op3::XYZ.mag() - self.radius.clone()
    }
}

// TODO: get rid of this
#[derive(Debug)]
pub enum TempShape {
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

impl fmt::Display for TempShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TempShape::Sphere { radius } => write!(f, "sphere, r = {}", radius),
            TempShape::Box {
                side_x,
                side_y,
                side_z,
            } => write!(f, "box, sides = ⟨{}, {}, {}⟩", side_x, side_y, side_z),
        }
    }
}

#[derive(Debug)]
pub enum Fill {
    Solid,
    Gyroid {
        scale: ConstantOrExpr,
        thickness: ConstantOrExpr,
    },
    // ...
}

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fill::Solid => write!(f, "solid"),
            Fill::Gyroid { scale, thickness } => {
                write!(f, "gyroid(s = {}, t = {})", scale, thickness)
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
    Shape(TempShape, Fill),
    Union {
        lhs: Rc<RefCell<CsgNode>>,
        rhs: Rc<RefCell<CsgNode>>,
    },
    SmoothUnion {
        lhs: Rc<RefCell<CsgNode>>,
        rhs: Rc<RefCell<CsgNode>>,
        k: ConstantOrExpr,
    },
    Intersection {
        lhs: Rc<RefCell<CsgNode>>,
        rhs: Rc<RefCell<CsgNode>>,
    },
    SmoothIntersection {
        lhs: Rc<RefCell<CsgNode>>,
        rhs: Rc<RefCell<CsgNode>>,
        k: ConstantOrExpr,
    },
    Subtraction {
        lhs: Rc<RefCell<CsgNode>>,
        rhs: Rc<RefCell<CsgNode>>,
    },
    SmoothSubtraction {
        lhs: Rc<RefCell<CsgNode>>,
        rhs: Rc<RefCell<CsgNode>>,
        k: ConstantOrExpr,
    },
    Translate {
        x: ConstantOrExpr,
        y: ConstantOrExpr,
        z: ConstantOrExpr,
        node: Rc<RefCell<CsgNode>>,
    },
    Rotate {
        roll: ConstantOrExpr,
        pitch: ConstantOrExpr,
        yaw: ConstantOrExpr,
        node: Rc<RefCell<CsgNode>>,
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
                lhs: Rc::new(RefCell::new(CsgNode::SmoothUnion {
                    lhs: Rc::new(RefCell::new(CsgNode::Shape(
                        TempShape::Sphere {
                            radius: ConstantOrExpr::Constant(1.0),
                        },
                        Fill::Solid,
                    ))),
                    rhs: Rc::new(RefCell::new(CsgNode::Shape(
                        TempShape::Box {
                            side_x: ConstantOrExpr::Constant(1.0),
                            side_y: ConstantOrExpr::Constant(1.0),
                            side_z: ConstantOrExpr::Constant(1.0),
                        },
                        Fill::Gyroid {
                            scale: ConstantOrExpr::Constant(10.0),
                            thickness: ConstantOrExpr::Constant(0.02),
                        },
                    ))),
                    k: ConstantOrExpr::Constant(0.4),
                })),
                rhs: Rc::new(RefCell::new(CsgNode::Shape(
                    TempShape::Box {
                        side_x: ConstantOrExpr::Constant(1.0),
                        side_y: ConstantOrExpr::Constant(1.0),
                        side_z: ConstantOrExpr::Constant(1.0),
                    },
                    Fill::Solid,
                ))),
            }),
        }
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
                CsgNode::Shape(shape, fill) => writeln!(f, "{}, fill = {}", shape, fill)?,
                CsgNode::Union { lhs, rhs } => {
                    writeln!(f, "union")?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                CsgNode::SmoothUnion { lhs, rhs, k } => {
                    writeln!(f, "smooth union, k = {}", k)?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                CsgNode::Intersection { lhs, rhs } => {
                    writeln!(f, "intersection")?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                CsgNode::SmoothIntersection { lhs, rhs, k } => {
                    writeln!(f, "smooth intersection, k = {}", k)?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                CsgNode::Subtraction { lhs, rhs } => {
                    writeln!(f, "subtraction")?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                CsgNode::SmoothSubtraction { lhs, rhs, k } => {
                    writeln!(f, "smooth subtraction, k = {}", k)?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                CsgNode::Translate { x, y, z, node } => {
                    writeln!(f, "translate by ⟨{}, {}, {}⟩", x, y, z)?;
                    recurse(f, &node.borrow(), indent, true, false)?;
                }
                CsgNode::Rotate {
                    roll,
                    pitch,
                    yaw,
                    node,
                } => {
                    writeln!(f, "rotate by ⟨{}, {}, {}⟩", roll, pitch, yaw)?;
                    recurse(f, &node.borrow(), indent, true, false)?;
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
