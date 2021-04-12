

pub use self::affine::*;
pub use self::interval::*;
pub use self::deriv::*;

#[macro_use]
macro_rules! generate_component_wise {
    ($scalar:ty) => {
        pub fn dot(self, other: Self) -> $scalar {
            self.x * other.x + self.y * other.y + self.z + other.z
        }
    
        pub fn abs(self) -> Self {
            Self {
                x: self.x.abs(),
                y: self.y.abs(),
                z: self.z.abs(),
            }
        }
    
        pub fn length(self) -> $scalar {
            ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
        }
    
        pub fn sin(self) -> Self {
            Self {
                x: self.x.sin(),
                y: self.y.sin(),
                z: self.z.sin(),
            }
        }
    
        pub fn cos(self) -> Self {
            Self {
                x: self.x.cos(),
                y: self.y.cos(),
                z: self.z.cos(),
            }
        }
    };
}

macro_rules! impl_component_wise3 {
    ($v:ty, $scalar:ty) => {
        impl<T: Copy> Arithmetics<T> for $v where $scalar: Arithmetics<T, Scalar = $scalar> {
            type Scalar = $scalar;
            fn min(self, rhs: T) -> Self {
                Self {
                    x: self.x.min(rhs),
                    y: self.y.min(rhs),
                    z: self.z.min(rhs),
                }
            }
            fn max(self, rhs: T) -> Self {
                Self {
                    x: self.x.max(rhs),
                    y: self.y.max(rhs),
                    z: self.z.max(rhs),
                }
            }
            fn clamp(self, low: T, high: T) -> Self {
                Self {
                    x: self.x.clamp(low, high),
                    y: self.y.clamp(low, high),
                    z: self.z.clamp(low, high),
                }
            }
            fn lerp(self, rhs: Self::Scalar, mix: T) -> Self {
                Self {
                    x: self.x.lerp(rhs, mix),
                    y: self.y.lerp(rhs, mix),
                    z: self.z.lerp(rhs, mix),
                }
            }
        }

        impl Add<Vec3> for $v {
            type Output = Self;
        
            fn add(self, rhs: Vec3) -> Self {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }

        impl Sub<Vec3> for $v {
            type Output = Self;
        
            fn sub(self, rhs: Vec3) -> Self {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                }
            }
        }
    };
}

mod affine;
mod interval;
mod deriv;

pub trait Arithmetics<Rhs = Self> {
    type Scalar;
    fn min(self, rhs: Rhs) -> Self;
    fn max(self, rhs: Rhs) -> Self;
    fn clamp(self, low: Rhs, high: Rhs) -> Self;
    fn lerp(self, rhs: Self::Scalar, mix: Rhs) -> Self;
}