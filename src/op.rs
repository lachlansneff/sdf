use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    sync::Arc,
};

use ultraviolet::{Vec2, Vec3};

// #[derive(Debug, Clone)]
// pub struct Op2 {
//     pub x: Op,
//     pub y: Op,
// }
// #[derive(Debug, Clone)]
// pub struct Op3 {
//     pub x: Op,
//     pub y: Op,
//     pub z: Op,
// }

// macro_rules! impl_op_vec {
//     ($vec:ty [$($component:ident),*]) => {
//         impl_op_vec!($vec [$($component),*] => Add::add);
//         impl_op_vec!($vec [$($component),*] => Add::add[f32]);
//         impl_op_vec!($vec [$($component),*] => Add::add[Op]);

//         impl_op_vec!($vec [$($component),*] => Sub::sub);
//         impl_op_vec!($vec [$($component),*] => Sub::sub[f32]);
//         impl_op_vec!($vec [$($component),*] => Sub::sub[Op]);

//         impl_op_vec!($vec [$($component),*] => Mul::mul);
//         impl_op_vec!($vec [$($component),*] => Mul::mul[f32]);
//         impl_op_vec!($vec [$($component),*] => Mul::mul[Op]);

//         impl_op_vec!($vec [$($component),*] => Div::div);
//         impl_op_vec!($vec [$($component),*] => Div::div[f32]);
//         impl_op_vec!($vec [$($component),*] => Div::div[Op]);

//         impl Neg for $vec {
//             type Output = Self;

//             fn neg(self) -> Self {
//                 Self {
//                     $($component: Neg::neg(self.$component),)*
//                 }
//             }
//         }

//         impl $vec {
//             pub fn new($($component: Op,)*) -> Self {
//                 Self { $($component,)* }
//             }
//             pub fn splat(v: f32) -> Self {
//                 Self { $($component: Op::Const(v),)* }
//             }
//             pub fn abs(self) -> Self {
//                 Self { $($component: self.$component.abs(),)* }
//             }
//             pub fn max(self, rhs: Self) -> Self {
//                 Self { $($component: self.$component.max(rhs.$component),)* }
//             }
//             pub fn sin(self) -> Self {
//                 Self { $($component: self.$component.sin(),)* }
//             }
//             pub fn cos(self) -> Self {
//                 Self { $($component: self.$component.cos(),)* }
//             }
//         }
//     };
//     ($vec:ty [$($component:ident),*] => $trait:ident::$trait_fn:ident) => {
//         impl $trait<Self> for $vec {
//             type Output = Self;

//             fn $trait_fn(self, other: Self) -> Self {
//                 Self {
//                     $($component: $trait::$trait_fn(self.$component, other.$component),)*
//                 }
//             }
//         }
//     };
//     ($vec:ty [$($component:ident),*] => $trait:ident::$trait_fn:ident[$rhs:ty]) => {
//         impl $trait<$rhs> for $vec {
//             type Output = Self;

//             fn $trait_fn(self, other: $rhs) -> Self {
//                 Self {
//                     $($component: $trait::$trait_fn(self.$component, other.clone()),)*
//                 }
//             }
//         }
//     };
// }

// impl_op_vec!(Op2 [x, y]);
// impl_op_vec!(Op3 [x, y, z]);

// #[derive(Debug, Clone)]
// pub struct Binary<T = Op> {
//     pub lhs: T,
//     pub rhs: T,
// }

macro_rules! generate_op {
    ($name:ident, const: Const($c:tt)) => {
        pub enum $name {
            Const($c),
            Add {
                lhs: Arc<Self>,
                rhs: Arc<Self>,
            },
            Sub {
                lhs: Arc<Self>,
                rhs: Arc<Self>,
            },
            Mul {
                lhs: Arc<Self>,
                rhs: Arc<Self>,
            },
            Div {
                lhs: Arc<Self>,
                rhs: Arc<Self>,
            },
            Abs(Arc<Self>),
            Neg(Arc<Self>),
            Sqrt(Arc<Self>),
            Squared(Arc<Self>),
            Pow(Arc<Self>, i32),
            Sin(Arc<Self>),
            Cos(Arc<Self>),

            Max {
                lhs: Arc<Self>,
                rhs: Arc<Self>,
            },
            Min {
                lhs: Arc<Self>,
                rhs: Arc<Self>,
            },
        }


    };
}

#[derive(Debug, Clone)]
pub enum Op {
    Const(f32),
    Add {
        lhs: Arc<Op>,
        rhs: Arc<Op>,
    },
    Sub {
        lhs: Arc<Op>,
        rhs: Arc<Op>,
    },
    Mul {
        lhs: Arc<Op>,
        rhs: Arc<Op>,
    },
    Div {
        lhs: Arc<Op>,
        rhs: Arc<Op>,
    },
    Abs(Arc<Op>),
    Neg(Arc<Op>),
    Sqrt(Arc<Op>),
    Squared(Arc<Op>),
    Pow(Arc<Op>, i32),
    // Mag2(Arc<Op2>),
    // Mag3(Arc<Op3>),
    // Dot2(Arc<Binary<Op2>>),
    // Dot3(Arc<Binary<Op3>>),
    Sin(Arc<Op>),
    Cos(Arc<Op>),

    Max {
        lhs: Arc<Op>,
        rhs: Arc<Op>,
    },
    Min {
        lhs: Arc<Op>,
        rhs: Arc<Op>,
    },
}

pub fn slow_sdf(p: (f32, f32, f32), op: &Op) -> f32 {
    // let op2_vec2 = |op2: &Op2| Vec2::new(slow_sdf(p, &op2.x), slow_sdf(p, &op2.y));
    // let op3_vec3 = |op3: &Op3| {
    //     Vec3::new(
    //         slow_sdf(p, &op3.x),
    //         slow_sdf(p, &op3.y),
    //         slow_sdf(p, &op3.z),
    //     )
    // };

    match op {
        Op::Const(x) => *x,
        Op::X => p.0,
        Op::Y => p.1,
        Op::Z => p.2,
        Op::Add(b) => slow_sdf(p, &b.lhs) + slow_sdf(p, &b.rhs),
        Op::Sub(b) => slow_sdf(p, &b.lhs) - slow_sdf(p, &b.rhs),
        Op::Mul(b) => slow_sdf(p, &b.lhs) * slow_sdf(p, &b.rhs),
        Op::Div(b) => slow_sdf(p, &b.lhs) / slow_sdf(p, &b.rhs),
        Op::Abs(x) => slow_sdf(p, x).abs(),
        Op::Neg(x) => -slow_sdf(p, x),
        Op::Sqrt(x) => slow_sdf(p, x).sqrt(),
        Op::Squared(x) => slow_sdf(p, x).powi(2),
        Op::Pow(x, exp) => slow_sdf(p, x).powi(*exp),
        // Op::Mag2(b) => op2_vec2(&b).mag(),
        // Op::Mag3(b) => op3_vec3(&b).mag(),
        // Op::Dot2(b) => op2_vec2(&b.lhs).dot(op2_vec2(&b.rhs)),
        // Op::Dot3(b) => op3_vec3(&b.lhs).dot(op3_vec3(&b.rhs)),
        Op::Sin(op) => slow_sdf(p, &op).sin(),
        Op::Cos(op) => slow_sdf(p, &op).cos(),

        Op::Max(b) => slow_sdf(p, &b.lhs).max(slow_sdf(p, &b.rhs)),
        Op::Min(b) => slow_sdf(p, &b.lhs).min(slow_sdf(p, &b.rhs)),
        Op::Remap { x, y, z, op } => {
            slow_sdf((slow_sdf(p, &x), slow_sdf(p, &y), slow_sdf(p, &z)), &op)
        }
    }
}

impl Op {
    pub fn abs(self) -> Self {
        match self {
            Op::Abs(x) => Op::Abs(x),
            Op::Squared(x) => Op::Squared(x),
            x => Op::Abs(Arc::new(x)),
        }
    }

    pub fn sqrt(self) -> Self {
        Op::Sqrt(Arc::new(self))
    }

    pub fn pow(self, exp: i32) -> Self {
        if exp == 1 {
            self
        } else if exp == 2 {
            Self::Squared(Arc::new(self))
        } else {
            Self::Pow(Arc::new(self), exp)
        }
    }

    pub fn sin(self) -> Self {
        Op::Sin(Arc::new(self))
    }

    pub fn cos(self) -> Self {
        Op::Cos(Arc::new(self))
    }

    pub fn max(self, rhs: Self) -> Self {
        Op::Max(Arc::new(Binary { lhs: self, rhs }))
    }

    pub fn min(self, rhs: Self) -> Self {
        Op::Min(Arc::new(Binary { lhs: self, rhs }))
    }

    pub fn remap(self, x: Self, y: Self, z: Self) -> Self {
        Op::Remap {
            x: Arc::new(x),
            y: Arc::new(y),
            z: Arc::new(z),
            op: Arc::new(self),
        }
    }
}
// impl Op2 {
//     pub const XY: Self = Op2 { x: Op::X, y: Op::Y };

//     pub fn mag(self) -> Op {
//         Op::Mag2(Arc::new(self))
//     }
//     pub fn dot(self, rhs: Self) -> Op {
//         Op::Dot2(Arc::new(Binary { lhs: self, rhs }))
//     }
// }
// impl Op3 {
//     pub const XYZ: Self = Op3 {
//         x: Op::X,
//         y: Op::Y,
//         z: Op::Z,
//     };

//     pub fn mag(self) -> Op {
//         Op::Mag3(Arc::new(self))
//     }
//     pub fn dot(self, rhs: Self) -> Op {
//         Op::Dot3(Arc::new(Binary { lhs: self, rhs }))
//     }
// }
// impl Op3 {
//     pub fn xy(self) -> Op2 {
//         Op2 {
//             x: self.x,
//             y: self.y,
//         }
//     }
//     pub fn yz(self) -> Op2 {
//         Op2 {
//             x: self.y,
//             y: self.z,
//         }
//     }
//     pub fn xz(self) -> Op2 {
//         Op2 {
//             x: self.x,
//             y: self.z,
//         }
//     }

//     pub fn zxy(self) -> Op3 {
//         Op3 {
//             x: self.z,
//             y: self.x,
//             z: self.y,
//         }
//     }
// }

impl Add<Self> for Op {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Op::Const(a), Op::Const(b)) => Op::Const(a + b),
            (x, Op::Const(y)) if y == 0.0 => x,
            (lhs, Op::Neg(rhs)) => Op::Sub(Arc::new(Binary {
                lhs,
                rhs: (*rhs).clone(),
            })),
            (lhs, rhs) => Op::Add(Arc::new(Binary { lhs, rhs })),
        }
    }
}
impl Add<f32> for Op {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        match self {
            Op::Const(x) => Op::Const(x + other),
            x if other == 0.0 => x,
            lhs => Op::Add(Arc::new(Binary {
                lhs,
                rhs: Op::Const(other),
            })),
        }
    }
}
impl Sub<Self> for Op {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Op::Const(a), Op::Const(b)) => Op::Const(a - b),
            (x, Op::Const(y)) if y == 0.0 => x,
            (lhs, Op::Neg(rhs)) => Op::Add(Arc::new(Binary {
                lhs,
                rhs: (*rhs).clone(),
            })),
            (lhs, rhs) => Op::Sub(Arc::new(Binary { lhs, rhs })),
        }
    }
}
impl Sub<f32> for Op {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        match self {
            Op::Const(x) => Op::Const(x - other),
            x if other == 0.0 => x,
            lhs => Op::Sub(Arc::new(Binary {
                lhs,
                rhs: Op::Const(other),
            })),
        }
    }
}
impl Mul<Self> for Op {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Op::Const(a), Op::Const(b)) => Op::Const(a * b),
            (x, Op::Const(y)) if y == 1.0 => x,
            (lhs, rhs) => Op::Mul(Arc::new(Binary { lhs, rhs })),
        }
    }
}
impl Mul<f32> for Op {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        match self {
            Op::Const(x) => Op::Const(x * other),
            x if other == 1.0 => x,
            lhs => Op::Mul(Arc::new(Binary {
                lhs,
                rhs: Op::Const(other),
            })),
        }
    }
}
impl Div<Self> for Op {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Op::Const(a), Op::Const(b)) => Op::Const(a / b),
            (x, Op::Const(y)) if y == 1.0 => x,
            (lhs, rhs) => Op::Div(Arc::new(Binary { lhs, rhs })),
        }
    }
}
impl Div<f32> for Op {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        match self {
            Op::Const(x) => Op::Const(x / other),
            x if other == 1.0 => x,
            lhs => Op::Div(Arc::new(Binary {
                lhs,
                rhs: Op::Const(other),
            })),
        }
    }
}
impl Neg for Op {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Op::Neg(x) => (*x).clone(),
            x => Op::Neg(Arc::new(x)),
        }
    }
}
