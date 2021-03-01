//! An implementation of forward-mode automatic differentiation
//! for determining the normals of an sdf.

use spirv_std::{
    num_traits::Float as _,
    glam::Vec3,
};
use core::ops::{Neg, Add, Sub, Mul, Div};

use crate::extra::ExtraVecMethods;

/// A dual vector of rank 3, defined as "_v_ + _d_ε".
/// Use this in place of Vec3 to perform automatic
/// forward-mode differentiation.
#[derive(Default, Clone, Copy)]
pub struct DualVec3 {
    v: Vec3,
    d: Vec3,
}

impl DualVec3 {
    pub fn new(v: Vec3) -> Self {
        Self::new_d(v, Vec3::zero())
    }

    fn new_d(v: Vec3, d: Vec3) -> Self {
        Self { v, d }
    }

    pub fn zero() -> Self {
        Self::new(Vec3::zero())
    }

    pub fn one() -> Self {
        Self::new(Vec3::one())
    }

    pub fn v(self) -> Vec3 {
        self.v
    }

    pub fn d(self) -> Vec3 {
        self.d
    }

    pub fn x(self) -> Dual {
        Dual::new_d(self.v.x, self.d)
    }

    pub fn y(self) -> Dual {
        Dual::new_d(self.v.y, self.d)
    }

    pub fn z(self) -> Dual {
        Dual::new_d(self.v.z, self.d)
    }

    pub fn dot(self, rhs: Self) -> Dual {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() + rhs.z()
    }

    pub fn max(self, rhs: Self) -> Self {
        if self.v >= rhs.v {
            self
        } else {
            rhs
        }
    }

    pub fn min(self, rhs: Self) -> Self {
        if self.v < rhs.v {
            self
        } else {
            rhs
        }
    }

    pub fn abs(self) -> Self {
        if self.v > Vec3::zero() {
            self
        } else {
            -self
        }
    }

    pub fn length(self) -> Dual {
        let x = self.x();
        let y = self.y();
        let z = self.z();

        ((x * x) + (y * y) + (z * z)).sqrt()
    }
}

impl ExtraVecMethods for DualVec3 {
    fn sin(self) -> Self {
        let a = self.v.cos();
        Self::new_d(self.v.sin(), self.d * a)
    }

    fn cos(self) -> Self {
        let a = -self.v.sin();
        Self::new_d(self.v.cos(), self.d * a)
    }
}

impl Add for DualVec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new_d(self.v + rhs.v, self.d + rhs.d)
    }
}

impl Add<Vec3> for DualVec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self {
        <Self as Add>::add(self, Self::new_d(rhs, Vec3::zero()))
    }
}

impl Add<f32> for DualVec3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        <Self as Add>::add(self, Self::new_d(Vec3::splat(rhs), Vec3::zero()))
    }
}

impl Sub for DualVec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new_d(self.v - rhs.v, self.d - rhs.d)
    }
}

impl Sub<Vec3> for DualVec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self {
        <Self as Sub>::sub(self, Self::new_d(rhs, Vec3::zero()))
    }
}

impl Sub<f32> for DualVec3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        <Self as Sub>::sub(self, Self::new_d(Vec3::splat(rhs), Vec3::zero()))
    }
}

impl Mul for DualVec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new_d(self.v * rhs.v, self.d * rhs.v + rhs.d * self.v)
    }
}

impl Mul<Vec3> for DualVec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        <Self as Mul>::mul(self, Self::new_d(rhs, Vec3::zero()))
    }
}

impl Mul<f32> for DualVec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        <Self as Mul>::mul(self, Self::new_d(Vec3::splat(rhs), Vec3::zero()))
    }
}

impl Div for DualVec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new_d(self.v / rhs.v, (self.d * rhs.v - rhs.d * self.v) / (rhs.v * rhs.v))
    }
}

impl Div<Vec3> for DualVec3 {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Self {
        <Self as Div>::div(self, Self::new_d(rhs, Vec3::zero()))
    }
}

impl Div<f32> for DualVec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        <Self as Div>::div(self, Self::new_d(Vec3::splat(rhs), Vec3::zero()))
    }
}

impl Neg for DualVec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new_d(-self.v, -self.d)
    }
}

/// A dual number, defined as v + _d_ε", but with a
/// derivative in ℝ³
#[derive(Default, Clone, Copy)]
pub struct Dual {
    d: Vec3,
    v: f32,
}

impl Dual {
    pub fn new(v: f32) -> Self {
        Self::new_d(v, Vec3::zero())
    }

    fn new_d(v: f32, d: Vec3) -> Self {
        Dual { v, d }
    }

    pub fn v(self) -> f32 {
        self.v
    }

    pub fn d(self) -> Vec3 {
        self.d
    }

    pub fn max(self, rhs: Self) -> Self {
        if self.v >= rhs.v {
            self
        } else {
            rhs
        }
    }

    pub fn min(self, rhs: Self) -> Self {
        if self.v < rhs.v {
            self
        } else {
            rhs
        }
    }

    pub fn abs(self) -> Self {
        if self.v > 0.0 {
            self
        } else {
            -self
        }
    }

    pub fn sqrt(self) -> Self {
        let v_sqrt = self.v.sqrt();
        Self::new_d(v_sqrt, self.d / (v_sqrt * 2.0))
    }

    pub fn sin(self) -> Self {
        let a = self.v.cos();
        Self::new_d(self.v.sin(), self.d * a)
    }

    pub fn cos(self) -> Self {
        let a = -self.v.sin();
        Self::new_d(self.v.cos(), self.d * a)
    }
}

impl Add for Dual {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new_d(self.v + rhs.v, self.d + rhs.d)
    }
}

impl Add<f32> for Dual {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        <Self as Add>::add(self, Self::new_d(rhs, Vec3::zero()))
    }
}

impl Sub for Dual {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new_d(self.v - rhs.v, self.d - rhs.d)
    }
}

impl Sub<f32> for Dual {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        <Self as Sub>::sub(self, Self::new_d(rhs, Vec3::zero()))
    }
}

impl Mul for Dual {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new_d(self.v * rhs.v, self.d * rhs.v + rhs.d * self.v)
    }
}

impl Mul<f32> for Dual {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        <Self as Mul>::mul(self, Self::new_d(rhs, Vec3::zero()))
    }
}

impl Div for Dual {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new_d(self.v / rhs.v, (self.d * rhs.v - rhs.d * self.v) / (rhs.v * rhs.v))
    }
}

impl Div<f32> for Dual {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        <Self as Div>::div(self, Self::new_d(rhs, Vec3::zero()))
    }
}

impl Neg for Dual {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new_d(-self.v, -self.d)
    }
}