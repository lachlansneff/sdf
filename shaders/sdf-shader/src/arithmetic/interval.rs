//! This is an implementation of interval arithmetic.

use core::ops::{Add, Div, Mul, Neg, Sub};

use spirv_std::num_traits::Float as _;

pub const fn interval(low: f32, high: f32) -> Interval {
    Interval { low, high }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub low: f32,
    pub high: f32,
}

impl Interval {
    pub fn abs(self) -> Self {
        if self.high < 0.0 {
            -self
        } else if self.low > 0.0 {
            self
        } else {
            interval(0.0, (-self.low).max(self.high))
        }
    }

    pub fn sqrt(self) -> Self {
        if self.low < 0.0 {
            interval(0.0, self.high.sqrt())
        } else {
            interval(self.low.sqrt(), self.high.sqrt())
        }
    }

    pub fn sin(self) -> Self {
        // TODO: Refine this later.
        interval(-1.0, 1.0)
    }

    pub fn cos(self) -> Self {
        // TODO: Refine this later.
        interval(-1.0, 1.0)
    }
}

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        interval(-self.high, -self.low)
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            low: self.low + rhs.low,
            high: self.high + rhs.high,
        }
    }
}

impl Add<f32> for Interval {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            low: self.low + rhs,
            high: self.high + rhs,
        }
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            low: self.low - rhs.high,
            high: self.high - rhs.low,
        }
    }
}

impl Sub<f32> for Interval {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            low: self.low - rhs,
            high: self.high - rhs,
        }
    }
}

impl Sub<Interval> for f32 {
    type Output = Interval;

    fn sub(self, rhs: Interval) -> Self::Output {
        interval(self - rhs.low, self - rhs.high)
    }
}

impl Mul for Interval {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x1y1 = self.low * rhs.low;
        let x1y2 = self.low * rhs.high;
        let x2y1 = self.high * rhs.low;
        let x2y2 = self.high * rhs.high;

        Self {
            low: x1y1.min(x1y2).min(x2y1).min(x2y2),
            high: x1y1.max(x1y2).max(x2y1).max(x2y2),
        }
    }
}

impl Mul<f32> for Interval {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        if rhs < 0.0 {
            Self {
                low: self.high * rhs,
                high: self.low * rhs,
            }
        } else {
            Self {
                low: self.low * rhs,
                high: self.high * rhs,
            }
        }
    }
}

impl Div for Interval {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let x1_y1 = self.low / rhs.low;
        let x1_y2 = self.low / rhs.high;
        let x2_y1 = self.high / rhs.low;
        let x2_y2 = self.high / rhs.high;

        Self {
            low: x1_y1.min(x1_y2).min(x2_y1).min(x2_y2),
            high: x1_y1.max(x1_y2).max(x2_y1).max(x2_y2),
        }
    }
}

impl Div<Interval> for f32 {
    type Output = Interval;

    fn div(self, rhs: Interval) -> Self::Output {
        interval(self, self) / rhs
    }
}

impl Div<f32> for Interval {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let xa = self.low / rhs;
        let ya = self.high / rhs;

        Self {
            low: xa.min(ya),
            high: xa.max(ya),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_bounds() {
        let f1 = |x| x * (10.0 - x);
        assert_eq!(f1(interval(4.0, 6.0)), interval(16.0, 36.0));

        let f2 = |x| x * 10.0 - x * x;
        assert_eq!(f2(interval(4.0, 6.0)), interval(4.0, 44.0));
    }
}