use super::eval::{Eval, Value, Value2, Value3};
use std::{
    borrow::Borrow,
    ops::{Add, Div, Mul, Neg, Sub},
};
use ultraviolet::{f32x8, Vec2x8, Vec3x8};

pub enum CpuEval {}

impl Eval for CpuEval {
    type V = CpuValue;
    type V2 = CpuValue2;
    type V3 = CpuValue3;
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CpuValue(pub f32x8);
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CpuValue2(pub Vec2x8);
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CpuValue3(pub Vec3x8);

impl Value<CpuEval> for CpuValue {
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

        CpuValue(start * (-self.0 + 1.0) + end * self.0)
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
impl From<f32> for CpuValue {
    fn from(v: f32) -> Self {
        Self(f32x8::splat(v))
    }
}
impl From<f32x8> for CpuValue {
    fn from(v: f32x8) -> Self {
        Self(v)
    }
}

impl Value2<CpuEval> for CpuValue2 {
    fn splat(v: impl Into<CpuValue>) -> Self {
        Self(Vec2x8::broadcast(v.into().0))
    }

    fn new(x: impl Into<CpuValue>, y: impl Into<CpuValue>) -> Self {
        Self(Vec2x8::new(x.into().0, y.into().0))
    }

    fn mag(&self) -> CpuValue {
        CpuValue(self.0.mag())
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

    fn dot(&self, other: impl Borrow<Self>) -> CpuValue {
        let other = other.borrow().0;
        CpuValue(self.0.dot(other))
    }

    fn max(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.max_by_component(other.borrow().0))
    }

    fn min(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.min_by_component(other.borrow().0))
    }

    fn x(&self) -> CpuValue {
        CpuValue(self.0.x)
    }

    fn y(&self) -> CpuValue {
        CpuValue(self.0.y)
    }
}

impl Value3<CpuEval> for CpuValue3 {
    fn splat(v: f32) -> Self {
        Self(Vec3x8::broadcast(v.into()))
    }

    fn new(x: impl Into<CpuValue>, y: impl Into<CpuValue>, z: impl Into<CpuValue>) -> Self {
        Self(Vec3x8::new(x.into().0, y.into().0, z.into().0))
    }

    fn mag(&self) -> CpuValue {
        CpuValue(self.0.mag())
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

    fn dot(&self, other: impl Borrow<Self>) -> CpuValue {
        let other = *other.borrow();
        CpuValue(self.0.dot(other.0))
    }

    fn max(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.max_by_component(other.borrow().0))
    }

    fn min(&self, other: impl Borrow<Self>) -> Self {
        Self(self.0.min_by_component(other.borrow().0))
    }

    fn x(&self) -> CpuValue {
        CpuValue(self.0.x)
    }

    fn y(&self) -> CpuValue {
        CpuValue(self.0.y)
    }

    fn z(&self) -> CpuValue {
        CpuValue(self.0.z)
    }

    fn xz(&self) -> CpuValue2 {
        CpuValue2(Vec2x8::new(self.0.x, self.0.z))
    }

    fn zxy(&self) -> CpuValue3 {
        CpuValue3(Vec3x8::new(self.0.z, self.0.x, self.0.y))
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
        impl_operators!($vec => (map field) Add::add[CpuValue]);
        impl_operators!($vec => (map field) Sub::sub[CpuValue]);
        impl_operators!($vec => (map field) Mul::mul[CpuValue]);
        impl_operators!($vec => (map field) Div::div[CpuValue]);

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

impl_operators!(value CpuValue);
impl_operators!(CpuValue2);
impl_operators!(CpuValue3);
