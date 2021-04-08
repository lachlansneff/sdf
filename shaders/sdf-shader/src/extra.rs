use core::ops::{Add, Div, Mul, Sub};

use glam::{vec2, vec3, Vec2, Vec3};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

pub trait VectorN: Sized {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

pub trait Scalar: Sized + Copy + Add + Sub + Mul + Div + PartialEq + PartialOrd {
    fn lerp(self, other: Self, mix: Self) -> Self;
}

impl VectorN for Vec2 {
    fn sin(self) -> Self {
        vec2(self.x.sin(), self.y.sin())
    }

    fn cos(self) -> Self {
        vec2(self.x.cos(), self.y.cos())
    }
}

impl VectorN for Vec3 {
    fn sin(self) -> Self {
        vec3(self.x.sin(), self.y.sin(), self.z.sin())
    }
    fn cos(self) -> Self {
        vec3(self.x.cos(), self.y.cos(), self.z.cos())
    }
}

impl Scalar for f32 {
    fn lerp(self, other: Self, mix: Self) -> Self {
        self + (other - self) * mix
    }
}
