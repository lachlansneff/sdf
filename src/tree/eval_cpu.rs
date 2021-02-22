use super::eval::{Eval, Value, Value2, Value3};
use std::{
    borrow::Borrow,
    ops::{Add, Div, Mul, Neg, Sub},
};
use ultraviolet::{Vec2, Vec3};

pub enum CpuEval {}

impl Eval for CpuEval {
    type V = f32;
    type V2 = CpuValue2;
    type V3 = CpuValue3;
}

#[derive(Clone, Copy)]
pub struct CpuValue2(pub Vec2);
#[derive(Clone, Copy)]
pub struct CpuValue3(pub Vec3);

impl Value<CpuEval> for f32 {
    fn new(v: f32) -> Self {
        v
    }

    fn max(&self, other: impl Borrow<Self>) -> Self {
        f32::max(*self, *other.borrow())
    }

    fn min(&self, other: impl Borrow<Self>) -> Self {
        f32::min(*self, *other.borrow())
    }

    fn clamp(&self, min: impl Borrow<Self>, max: impl Borrow<Self>) -> Self {
        f32::clamp(*self, *min.borrow(), *max.borrow())
    }

    fn mix(&self, start: impl Borrow<Self>, end: impl Borrow<Self>) -> Self {
        // x×(1−a)+y×a
        let start = *start.borrow();
        let end = *end.borrow();

        start * (1.0 - *self) + end * *self
    }

    fn sin(&self) -> Self {
        f32::sin(*self)
    }

    fn cos(&self) -> Self {
        f32::cos(*self)
    }

    fn abs(&self) -> Self {
        f32::abs(*self)
    }
}

impl Value2<CpuEval> for CpuValue2 {
    fn splat(v: impl Into<f32>) -> Self {
        Self(Vec2::broadcast(v.into()))
    }

    fn new(x: impl Into<f32>, y: impl Into<f32>) -> Self {
        Self(Vec2::new(x.into(), y.into()))
    }

    fn mag(&self) -> f32 {
        self.0.mag()
    }

    fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    fn sin(&self) -> Self {
        Self(self.0.map(|v| v.sin()))
    }

    fn cos(&self) -> Self {
        Self(self.0.map(|v| v.cos()))
    }

    fn dot(&self, other: impl Borrow<Self>) -> f32 {
        let other = *other.borrow();
        self.0.dot(other.0)
    }

    fn max(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.max_by_component(other.borrow().0))
    }

    fn min(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.min_by_component(other.borrow().0))
    }

    fn x(&self) -> f32 {
        self.0.x
    }

    fn y(&self) -> f32 {
        self.0.y
    }
}

impl Value3<CpuEval> for CpuValue3 {
    fn splat(v: f32) -> Self {
        Self(Vec3::broadcast(v.into()))
    }

    fn new(x: impl Into<f32>, y: impl Into<f32>, z: impl Into<f32>) -> Self {
        Self(Vec3::new(x.into(), y.into(), z.into()))
    }

    fn mag(&self) -> f32 {
        self.0.mag()
    }

    fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    fn sin(&self) -> Self {
        Self(self.0.map(|v| v.sin()))
    }

    fn cos(&self) -> Self {
        Self(self.0.map(|v| v.cos()))
    }

    fn dot(&self, other: impl Borrow<Self>) -> f32 {
        let other = *other.borrow();
        self.0.dot(other.0)
    }

    fn max(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.max_by_component(other.borrow().0))
    }

    fn min(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.min_by_component(other.borrow().0))
    }

    fn x(&self) -> f32 {
        self.0.x
    }

    fn y(&self) -> f32 {
        self.0.y
    }

    fn z(&self) -> f32 {
        self.0.z
    }

    fn xz(&self) -> CpuValue2 {
        CpuValue2::new(self.0.x, self.0.z)
    }

    fn zxy(&self) -> CpuValue3 {
        CpuValue3::new(self.0.z, self.0.x, self.0.y)
    }
}

macro_rules! impl_operators {
    ($vec:ty) => {
        impl_operators!($vec => Add::add[Self]);
        impl_operators!($vec => (map) Add::add[f32]);

        impl_operators!($vec => Sub::sub[Self]);
        impl_operators!($vec => (map) Sub::sub[f32]);

        impl_operators!($vec => Mul::mul[Self]);
        impl_operators!($vec => (map) Mul::mul[f32]);

        impl_operators!($vec => Div::div[Self]);
        impl_operators!($vec => (map) Div::div[f32]);

        impl Neg for $vec {
            type Output = Self;
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }
    };
    ($vec:ty => $trait:ident::$traitfn:ident[Self]) => {
        impl $trait<Self> for $vec {
            type Output = Self;
            fn $traitfn(self, rhs: Self) -> Self {
                Self($trait::$traitfn(self.0, rhs.0))
            }
        }
    };
    ($vec:ty => (map) $trait:ident::$traitfn:ident[$t:ty]) => {
        impl $trait<$t> for $vec {
            type Output = Self;
            fn $traitfn(self, rhs: $t) -> Self {
                Self(self.0.map(|v| $trait::$traitfn(v, rhs)))
            }
        }
    };
}

impl_operators!(CpuValue2);
impl_operators!(CpuValue3);
