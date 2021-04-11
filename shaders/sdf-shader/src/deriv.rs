//! An implementation of forward-mode automatic differentiation
//! for determining the normals of an sdf.

use core::ops::{Add, Div, Mul, Neg, Sub};
use glam::Vec3;
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float as _;

use crate::extra::VectorN;

/// A dual vector of rank 3, defined as "_v_ + _d_ε".
/// Use this in place of Vec3 to perform automatic
/// forward-mode differentiation.
///
/// This is automatically set up for cartesian coordinates.
#[derive(Default, Clone, Copy)]
pub struct Deriv3 {
    pub x: Deriv,
    pub y: Deriv,
    pub z: Deriv,
}

impl Deriv3 {
    pub const ZERO: Self = Self {
        x: Deriv::new(0.0),
        y: Deriv::new(0.0),
        z: Deriv::new(0.0),
    };

    pub fn new_xyz(v: Vec3) -> Self {
        let mut this = Self {
            x: Deriv::new(v.x),
            y: Deriv::new(v.y),
            z: Deriv::new(v.z),
        };
        // Set up for cartesian coordinates.
        this.x.d.x = 1.0;
        this.y.d.y = 1.0;
        this.z.d.z = 1.0;

        this
    }

    pub fn new(v: Vec3) -> Self {
        Self {
            x: Deriv::new(v.x),
            y: Deriv::new(v.y),
            z: Deriv::new(v.z),
        }
    }

    pub fn one() -> Self {
        Self::new(Vec3::ONE)
    }

    pub fn v(self) -> Vec3 {
        Vec3::new(self.x.v, self.y.v, self.z.v)
    }

    pub fn dot(self, other: Self) -> Deriv {
        self.x * other.x + self.y * other.y + self.z + other.z
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn length(self) -> Deriv {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }
}

impl VectorN for Deriv3 {
    fn sin(self) -> Self {
        Self {
            x: self.x.sin(),
            y: self.y.sin(),
            z: self.z.sin(),
        }
    }

    fn cos(self) -> Self {
        Self {
            x: self.x.cos(),
            y: self.y.cos(),
            z: self.z.cos(),
        }
    }
}

impl Add for Deriv3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vec3> for Deriv3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self {
        <Self as Add>::add(self, Self::new(rhs))
    }
}

impl Add<f32> for Deriv3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        <Self as Add>::add(self, Self::new(Vec3::splat(rhs)))
    }
}

impl Sub for Deriv3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vec3> for Deriv3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self {
        <Self as Sub>::sub(self, Self::new(rhs))
    }
}

impl Sub<f32> for Deriv3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        <Self as Sub>::sub(self, Self::new(Vec3::splat(rhs)))
    }
}

impl Mul for Deriv3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for Deriv3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        <Self as Mul>::mul(self, Self::new(rhs))
    }
}

impl Mul<f32> for Deriv3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        <Self as Mul>::mul(self, Self::new(Vec3::splat(rhs)))
    }
}

impl Div for Deriv3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<Vec3> for Deriv3 {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Self {
        <Self as Div>::div(self, Self::new(rhs))
    }
}

impl Div<f32> for Deriv3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        <Self as Div>::div(self, Self::new(Vec3::splat(rhs)))
    }
}

impl Neg for Deriv3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// A dual number, defined as v + _d_ε", but with a
/// derivative in ℝ³
#[derive(Default, Clone, Copy)]
pub struct Deriv {
    d: Vec3,
    v: f32,
}

impl Deriv {
    pub const ZERO: Self = Self::new(0.0);
    pub const ONE: Self = Self::new(1.0);

    pub const fn new(v: f32) -> Self {
        Self::new_with_deriv(v, Vec3::ZERO)
    }

    pub const fn new_with_deriv(v: f32, d: Vec3) -> Self {
        Deriv { v, d }
    }

    pub fn derivatives(self) -> Vec3 {
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

    pub fn clamp(self, min: Self, max: Self) -> Self {
        self.min(max).max(min)
    }

    pub fn lerp(self, other: Self, mix: Self) -> Self {
        self + (other - self) * mix
    }

    pub fn sqrt(self) -> Self {
        let a = self.v.sqrt();
        Self::new_with_deriv(a, self.d / (a * 2.0))
    }

    pub fn sin(self) -> Self {
        let a = self.v.cos();
        Self::new_with_deriv(self.v.sin(), self.d * a)
    }

    pub fn cos(self) -> Self {
        let a = -self.v.sin();
        Self::new_with_deriv(self.v.cos(), self.d * a)
    }
}

impl Add for Deriv {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            v: self.v + rhs.v,
            d: self.d + rhs.d,
        }
    }
}

impl Add<f32> for Deriv {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self {
            v: self.v + rhs,
            ..self
        }
    }
}

impl Sub for Deriv {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            v: self.v - rhs.v,
            d: self.d - rhs.d,
        }
    }
}

impl Sub<f32> for Deriv {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            v: self.v - rhs,
            ..self
        }
    }
}

impl Mul for Deriv {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            v: self.v * rhs.v,
            d: self.d * rhs.v + rhs.d * self.v,
        }
    }
}

impl Mul<f32> for Deriv {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            v: self.v * rhs,
            d: self.d * rhs,
        }
    }
}

impl Div for Deriv {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            v: self.v / rhs.v,
            d: (self.d * rhs.v - rhs.d * self.v) / (rhs.v * rhs.v),
        }
    }
}

impl Div<f32> for Deriv {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            v: self.v / rhs,
            d: self.d / rhs,
        }
    }
}

impl Neg for Deriv {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new_with_deriv(-self.v, -self.d)
    }
}
