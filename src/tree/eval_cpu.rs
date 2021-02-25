use super::eval::{Eval, Real1, Real2, Real3};
use std::{
    borrow::Borrow,
    ops::{Add, Div, Mul, Neg, Sub},
};
use ultraviolet::{f32x8, Vec2x8, Vec3x8};

pub enum CpuEval {}

impl Eval for CpuEval {
    type R1 = CpuReal1;
    type R2 = CpuReal2;
    type R3 = CpuReal3;
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CpuReal1(pub f32x8);
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CpuReal2(pub Vec2x8);
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CpuReal3(pub Vec3x8);

impl Real1<CpuEval> for CpuReal1 {
    fn new(v: f32) -> Self {
        Self(f32x8::splat(v))
    }

    fn max(&self, other: impl Borrow<Self>) -> Self {
        Self(f32x8::max(self.0, other.borrow().0))
    }

    fn min(&self, other: impl Borrow<Self>) -> Self {
        Self(f32x8::min(self.0, other.borrow().0))
    }

    fn clamp(&self, min: impl Borrow<Self>, max: impl Borrow<Self>) -> Self {
        let min = min.borrow().0;
        let max = max.borrow().0;
        Self(self.0.max(min).min(max))
    }

    fn mix(&self, start: impl Borrow<Self>, end: impl Borrow<Self>) -> Self {
        // x×(1−a)+y×a
        let start = start.borrow().0;
        let end = end.borrow().0;

        CpuReal1(start * (-self.0 + 1.0) + end * self.0)
    }

    fn sin(&self) -> Self {
        Self(self.0.sin())
    }

    fn cos(&self) -> Self {
        Self(self.0.cos())
    }

    fn abs(&self) -> Self {
        Self(self.0.abs())
    }
}
impl From<f32> for CpuReal1 {
    fn from(v: f32) -> Self {
        Self(f32x8::splat(v))
    }
}
impl From<f32x8> for CpuReal1 {
    fn from(v: f32x8) -> Self {
        Self(v)
    }
}

impl Real2<CpuEval> for CpuReal2 {
    fn splat(v: impl Into<CpuReal1>) -> Self {
        Self(Vec2x8::broadcast(v.into().0))
    }

    fn new(x: impl Into<CpuReal1>, y: impl Into<CpuReal1>) -> Self {
        Self(Vec2x8::new(x.into().0, y.into().0))
    }

    fn mag(&self) -> CpuReal1 {
        CpuReal1(self.0.mag())
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

    fn dot(&self, other: impl Borrow<Self>) -> CpuReal1 {
        let other = other.borrow().0;
        CpuReal1(self.0.dot(other))
    }

    fn max(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.max_by_component(other.borrow().0))
    }

    fn min(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.min_by_component(other.borrow().0))
    }

    fn x(&self) -> CpuReal1 {
        CpuReal1(self.0.x)
    }

    fn y(&self) -> CpuReal1 {
        CpuReal1(self.0.y)
    }
}

impl Real3<CpuEval> for CpuReal3 {
    fn splat(v: f32) -> Self {
        Self(Vec3x8::broadcast(v.into()))
    }

    fn new(x: impl Into<CpuReal1>, y: impl Into<CpuReal1>, z: impl Into<CpuReal1>) -> Self {
        Self(Vec3x8::new(x.into().0, y.into().0, z.into().0))
    }

    fn mag(&self) -> CpuReal1 {
        CpuReal1(self.0.mag())
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

    fn dot(&self, other: impl Borrow<Self>) -> CpuReal1 {
        let other = *other.borrow();
        CpuReal1(self.0.dot(other.0))
    }

    fn max(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.max_by_component(other.borrow().0))
    }

    fn min(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.min_by_component(other.borrow().0))
    }

    fn x(&self) -> CpuReal1 {
        CpuReal1(self.0.x)
    }

    fn y(&self) -> CpuReal1 {
        CpuReal1(self.0.y)
    }

    fn z(&self) -> CpuReal1 {
        CpuReal1(self.0.z)
    }

    fn xz(&self) -> CpuReal2 {
        CpuReal2(Vec2x8::new(self.0.x, self.0.z))
    }

    fn zxy(&self) -> CpuReal3 {
        CpuReal3(Vec3x8::new(self.0.z, self.0.x, self.0.y))
    }
}

macro_rules! impl_operators {
    (value $value:ty) => {
        impl_operators!($value => Add::add[Self]);
        impl_operators!($value => Add::add[f32]);

        impl_operators!($value => Sub::sub[Self]);
        impl_operators!($value => Sub::sub[f32]);

        impl_operators!($value => Mul::mul[Self]);
        impl_operators!($value => Mul::mul[f32]);

        impl_operators!($value => Div::div[Self]);
        impl_operators!($value => Div::div[f32]);

        impl Neg for $value {
            type Output = Self;
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }
    };
    ($vec:ty) => {
        impl_operators!($vec => (map field) Add::add[CpuReal1]);
        impl_operators!($vec => (map field) Sub::sub[CpuReal1]);
        impl_operators!($vec => (map field) Mul::mul[CpuReal1]);
        impl_operators!($vec => (map field) Div::div[CpuReal1]);

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
    ($vec:ty => $trait:ident::$traitfn:ident[$t:ty]) => {
        impl $trait<$t> for $vec {
            type Output = Self;
            fn $traitfn(self, rhs: $t) -> Self {
                Self($trait::$traitfn(self.0, rhs))
            }
        }
    };
    ($vec:ty => (map field) $trait:ident::$traitfn:ident[$t:ty]) => {
        impl $trait<$t> for $vec {
            type Output = Self;
            fn $traitfn(self, rhs: $t) -> Self {
                Self(self.0.map(|v| $trait::$traitfn(v, rhs.0)))
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

impl_operators!(value CpuReal1);
impl_operators!(CpuReal2);
impl_operators!(CpuReal3);
