use std::{cell::RefCell, fmt, rc::Rc};

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
pub enum Node {
    Shape(Shape, Fill),
    Union {
        lhs: Rc<RefCell<Node>>,
        rhs: Rc<RefCell<Node>>,
    },
    SmoothUnion {
        lhs: Rc<RefCell<Node>>,
        rhs: Rc<RefCell<Node>>,
        k: ConstantOrExpr,
    },
    Intersection {
        lhs: Rc<RefCell<Node>>,
        rhs: Rc<RefCell<Node>>,
    },
    SmoothIntersection {
        lhs: Rc<RefCell<Node>>,
        rhs: Rc<RefCell<Node>>,
        k: ConstantOrExpr,
    },
    Subtraction {
        lhs: Rc<RefCell<Node>>,
        rhs: Rc<RefCell<Node>>,
    },
    SmoothSubtraction {
        lhs: Rc<RefCell<Node>>,
        rhs: Rc<RefCell<Node>>,
        k: ConstantOrExpr,
    },
    Translate {
        x: ConstantOrExpr,
        y: ConstantOrExpr,
        z: ConstantOrExpr,
        node: Rc<RefCell<Node>>,
    },
    Rotate {
        roll: ConstantOrExpr,
        pitch: ConstantOrExpr,
        yaw: ConstantOrExpr,
        node: Rc<RefCell<Node>>,
    },
}

/// A Constructive Solid Geometry Tree.
pub struct CSG {
    root: Option<Node>,
}

impl CSG {
    pub fn new_example() -> Self {
        Self {
            root: Some(Node::Union {
                lhs: Rc::new(RefCell::new(Node::SmoothUnion {
                    lhs: Rc::new(RefCell::new(Node::Shape(
                        Shape::Sphere {
                            radius: ConstantOrExpr::Constant(1.0),
                        },
                        Fill::Solid,
                    ))),
                    rhs: Rc::new(RefCell::new(Node::Shape(
                        Shape::Box {
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
                rhs: Rc::new(RefCell::new(Node::Shape(
                    Shape::Box {
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

impl fmt::Display for CSG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const CROSS: &str = " ├─";
        const CORNER: &str = " └─";
        const VERTICAL: &str = " │ ";
        const SPACE: &str = "   ";

        fn recurse(
            f: &mut fmt::Formatter,
            node: &Node,
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
                Node::Shape(shape, fill) => writeln!(f, "{}, fill = {}", shape, fill)?,
                Node::Union { lhs, rhs } => {
                    writeln!(f, "union")?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                Node::SmoothUnion { lhs, rhs, k } => {
                    writeln!(f, "smooth union, k = {}", k)?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                Node::Intersection { lhs, rhs } => {
                    writeln!(f, "intersection")?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                Node::SmoothIntersection { lhs, rhs, k } => {
                    writeln!(f, "smooth intersection, k = {}", k)?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                Node::Subtraction { lhs, rhs } => {
                    writeln!(f, "subtraction")?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                Node::SmoothSubtraction { lhs, rhs, k } => {
                    writeln!(f, "smooth subtraction, k = {}", k)?;
                    recurse(f, &lhs.borrow(), indent.clone(), false, false)?;
                    recurse(f, &rhs.borrow(), indent, true, false)?;
                }
                Node::Translate { x, y, z, node } => {
                    writeln!(f, "translate by ⟨{}, {}, {}⟩", x, y, z)?;
                    recurse(f, &node.borrow(), indent, true, false)?;
                }
                Node::Rotate {
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
