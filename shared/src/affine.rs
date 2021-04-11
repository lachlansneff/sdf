//! An implementation of reduced affine arithmetic (AF1).
//! https://scihubtw.tw/10.1111/j.1467-8659.2008.01189.x
//! https://www.researchgate.net/publication/220348558_Extensions_of_Affine_Arithmetic_Application_to_Unconstrained_Global_Optimization

use num_traits::Float as _;
use core::ops::{Add, Div, Mul, Sub};
use crate::interval::{interval, Interval};

pub enum Choice {
    Left,
    Right,
    Both,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Affine3 {
    pub x: Affine,
    pub y: Affine,
    pub z: Affine,
}

/// An implementation of AF1 Reduced Affine Arithmetic
/// where n = 0.
#[derive(Clone, Copy, PartialEq)]
pub struct Affine {
    x0: f32,
    x1: f32,
    e1: f32,
}

impl Affine {
    fn rad(self) -> f32 {
        self.x1.abs() + self.e1
    }

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

    fn max_choice(self, rhs: Self) -> (Self, Choice) {
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

    fn min_choice(self, rhs: Self) -> (Self, Choice) {
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
}

impl Add for Affine {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x0: self.x0 + rhs.x0,
            x1: self.x1 + rhs.x1,
            e1: self.e1 + rhs.e1,
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
            x1: self.x1 + rhs.x1,
            e1: self.e1 + rhs.e1,
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

impl Mul for Affine {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x0: self.x0 * rhs.x0,
            x1: self.x0 * rhs.x1 + rhs.x0 * self.x1,
            e1: (self.x0 * rhs.e1).abs() + (rhs.x0 * self.e1).abs() + self.rad() * rhs.rad(),
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

impl Into<Affine> for Interval {
    fn into(self) -> Affine {
        Affine {
            x0: (self.high + self.low) / 2.0,
            x1: (self.high - self.low) / 2.0,
            e1: 0.0,
        }
    }
}

impl Into<Interval> for Affine {
    fn into(self) -> Interval {
        let rad = self.rad();
        interval(self.x0 - rad, self.x0 + rad)
    }
}
