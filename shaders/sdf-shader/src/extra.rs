use spirv_std::glam::{vec2, vec3, Vec2, Vec3};
use spirv_std::num_traits::Float;

pub trait ExtraVecMethods where Self: Sized {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

pub trait ExtraF32Methods where Self: Sized {
    fn lerp(self, other: Self, mix: Self) -> Self;
}

impl ExtraVecMethods for Vec2 {
    fn sin(self) -> Self {
        vec2(self.x.sin(), self.y.sin())
    }
    fn cos(self) -> Self {
        vec2(self.x.cos(), self.y.cos())
    }
}

impl ExtraVecMethods for Vec3 {
    fn sin(self) -> Self {
        vec3(self.x.sin(), self.y.sin(), self.z.sin())
    }
    fn cos(self) -> Self {
        vec3(self.x.cos(), self.y.cos(), self.z.cos())
    }
}

impl ExtraF32Methods for f32 {
    fn lerp(self, other: Self, mix: Self) -> Self {
        self + (other - self) * mix
    }
}
