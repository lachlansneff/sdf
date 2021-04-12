//! An implementation of reduced affine arithmetic (AF1).
//! https://scihubtw.tw/10.1111/j.1467-8659.2008.01189.x
//! https://www.researchgate.net/publication/220348558_Extensions_of_Affine_Arithmetic_Application_to_Unconstrained_Global_Optimization
//! https://central.bac-lac.gc.ca/.item?id=MR43608&op=pdf&app=Library&oclc_number=689503128
//! https://affapy.readthedocs.io/_/downloads/en/latest/pdf/

use glam::Vec3;
use spirv_std::num_traits::Float as _;
use core::{ops::{Add, Div, Mul, Neg, Sub}};
use super::{Arithmetics, interval::{interval, Interval}};

pub enum Choice {
    Left,
    Right,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Affine3 {
    pub x: Affine,
    pub y: Affine,
    pub z: Affine,
}

impl Affine3 {
    pub fn new(p: Vec3) -> Self {
        Self {
            x: Affine::new(p.x),
            y: Affine::new(p.y),
            z: Affine::new(p.z),
        }
    }

    generate_component_wise!(Affine);
}

impl_component_wise3!(Affine3, Affine);

/// An implementation of AF1 Reduced Affine Arithmetic
/// where n = 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Affine {
    x0: f32,
    x1: f32,
    /// x_{n+1}
    x2: f32,
}

impl Affine {
    pub const ZERO: Self = Affine::new(0.0);

    fn rad(self) -> f32 {
        self.x1.abs() + self.x2
    }

    fn lower(self) -> f32 {
        self.x0 - self.rad()
    }

    fn higher(self) -> f32 {
        self.x0 + self.rad()
    }

    pub const fn new(x0: f32) -> Self {
        Self {
            x0,
            x1: 0.0,
            x2: 0.0,
        }
    }

    pub const fn from_interval(interval: Interval) -> Self {
        Affine {
            x0: (interval.high + interval.low) / 2.0,
            x1: (interval.high - interval.low) / 2.0,
            x2: 0.0,
        }
    }

    pub fn into_interval(self) -> Interval {
        let rad = self.rad();
        interval(self.x0 - rad, self.x0 + rad)
    }

    pub fn max_choice(self, rhs: Self) -> (Self, Choice) {
        let x: Interval = self.into();
        let y: Interval = rhs.into();

        if x.low > y.high {
            (self, Choice::Left)
        } else if y.low > x.high {
            (rhs, Choice::Right)
        } else {
            (interval(x.low.max(y.low), x.high.max(y.high)).into(), Choice::Both)
        }
    }

    pub fn min_choice(self, rhs: Self) -> (Self, Choice) {
        let x: Interval = self.into();
        let y: Interval = rhs.into();

        if x.high < y.low {
            (self, Choice::Left)
        } else if y.high < x.low {
            (rhs, Choice::Right)
        } else {
            (interval(x.low.min(y.low), x.high.min(y.high)).into(), Choice::Both)
        }
    }

    pub fn abs(self) -> Self {
        if self.higher() < 0.0 {
            -self
        } else if self.lower() > 0.0{
            self
        } else {
            self.into_interval().abs().into()
        }
    }

    pub fn sqrt(self) -> Self {
        // let Interval { low, high } = self.into();
        // let low_sqrt = low.sqrt();
        // let high_sqrt = high.sqrt();
        // let alpha = 1.0 / (low_sqrt + high_sqrt);
        // let zeta = ((low_sqrt + high_sqrt) / 8.0) + 0.5 * (low_sqrt * high_sqrt) / (low_sqrt + high_sqrt);
        // let delta = (1.0 / 8.0) * (high_sqrt - low_sqrt) * (high_sqrt - low_sqrt) / (low_sqrt + high_sqrt);

        // Self {
        //     x0: self.x0 * alpha + zeta,
        //     x1: self.x1 * alpha,
        //     x2: self.x2 * delta,
        // }
        self.into_interval().sqrt().into()
    }

    pub fn sin(self) -> Self {
        self.into_interval().sin().into()
    }

    pub fn cos(self) -> Self {
        self.into_interval().cos().into()
    }
}

impl Arithmetics for Affine {
    type Scalar = Self;
    fn max(self, rhs: Self) -> Self {
        let x: Interval = self.into();
        let y: Interval = rhs.into();

        if x.low > y.high {
            self
        } else if y.low > x.high {
            rhs
        } else {
            interval(x.low.max(y.low), x.high.max(y.high)).into()
        }
    }

    fn min(self, rhs: Self) -> Self {
        let x: Interval = self.into();
        let y: Interval = rhs.into();

        if x.high < y.low {
            self
        } else if y.high < x.low {
            rhs
        } else {
            interval(x.low.min(y.low), x.high.min(y.high)).into()
        }
    }

    fn clamp(self, low: Self, high: Self) -> Self {
        self.max(low).min(high)
    }

    fn lerp(self, rhs: Self, mix: Self) -> Self {
        self + (rhs - self) * mix
    }
}

impl Arithmetics<f32> for Affine {
    type Scalar = Self;
    fn min(self, rhs: f32) -> Self {
        let x: Interval = self.into();

        if x.high < rhs {
            self
        } else if x.low > rhs {
            interval(rhs, rhs).into()
        } else {
            interval(x.low.min(rhs), x.high.min(rhs)).into()
        }
    }

    fn max(self, rhs: f32) -> Self {
        let x: Interval = self.into();

        if x.low > rhs {
            self
        } else if x.high < rhs {
            interval(rhs, rhs).into()
        } else {
            interval(x.low.max(rhs), x.high.max(rhs)).into()
        }
    }

    fn clamp(self, low: f32, high: f32) -> Self {
        self.max(low).min(high)
    }

    fn lerp(self, rhs: Self, mix: f32) -> Self {
        self + (rhs - self) * mix
    }
}

impl Neg for Affine {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x0: -self.x0,
            x1: -self.x1,
            ..self
        }
    }
}

impl Add for Affine {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x0: self.x0 + rhs.x0,
            x1: self.x1 + rhs.x1,
            x2: self.x2 + rhs.x2,
        }
    }
}

impl Add<f32> for Affine {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x0: self.x0 + rhs,
            ..self
        }
    }
}

impl Sub for Affine {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x0: self.x0 - rhs.x0,
            x1: self.x1 - rhs.x1,
            x2: self.x2 + rhs.x2,
        }
    }
}

impl Sub<f32> for Affine {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x0: self.x0 - rhs,
            ..self
        }
    }
}

impl Sub<Affine> for f32 {
    type Output = Affine;

    fn sub(self, rhs: Affine) -> Self::Output {
        Affine {
            x0: self - rhs.x0,
            x1: -rhs.x1,
            ..rhs
        }
    }
}

impl Mul for Affine {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x0: self.x0 * rhs.x0,
            x1: self.x0 * rhs.x1 + rhs.x0 * self.x1,
            x2: (self.x0 * rhs.x2).abs() + (rhs.x0 * self.x2).abs() + self.rad() * rhs.rad(),
        }
    }
}

impl Mul<f32> for Affine {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x0: self.x0 * rhs,
            x1: self.x1 * rhs,
            x2: self.x2 * rhs.abs(),
        }
    }
}

impl Div for Affine {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let interval_rhs: Interval = rhs.into();
        let inverted_rhs: Affine = (1.0 / interval_rhs).into();
        self * inverted_rhs
    }
}

impl Div<f32> for Affine {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Into<Affine> for Interval {
    fn into(self) -> Affine {
        Affine::from_interval(self)
    }
}

impl Into<Interval> for Affine {
    fn into(self) -> Interval {
        self.into_interval()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affine_bounds() {
        let f1 = |x: Affine| x * (10.0 - x);
        assert_eq!(f1(interval(4.0, 6.0).into()).into_interval(), interval(24.0, 26.0));

        let f2 = |x: Affine| x * 10.0 - x * x;
        assert_eq!(f2(interval(4.0, 6.0).into()).into_interval(), interval(24.0, 26.0));
    }
}
