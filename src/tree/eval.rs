use std::{
    borrow::Borrow,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// Used to allow a single sdf function to
///  1. evaluate a CSG tree on CPU
///  2. generate shader code
///     a. for regular point evaluation
///     b. for interval/affine arithmetic evaluation
pub trait Evaluate {
    fn eval<E: Eval>(&self, p: E::R3) -> E::R1;
}

pub trait Eval
where
    Self: Sized,
{
    type R1: Real1<Self>;
    type R2: Real2<Self>;
    type R3: Real3<Self>;
}

pub trait Real1<E: Eval>:
    Add<Self, Output = Self>
    + Add<f32, Output = Self>
    + Sub<Self, Output = Self>
    + Sub<f32, Output = Self>
    + Mul<Self, Output = Self>
    + Mul<f32, Output = Self>
    + Div<Self, Output = Self>
    + Div<f32, Output = Self>
    + Neg<Output = Self>
    + From<f32>
    + Clone
where
    Self: Sized,
    Self: Borrow<Self>,
{
    fn new(v: f32) -> Self;

    fn max(&self, other: impl Borrow<Self>) -> Self;
    fn min(&self, other: impl Borrow<Self>) -> Self;
    fn clamp(&self, min: impl Borrow<Self>, max: impl Borrow<Self>) -> Self;
    fn mix(&self, start: impl Borrow<Self>, end: impl Borrow<Self>) -> Self;

    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn abs(&self) -> Self;
}

pub trait Real2<E: Eval>:
    Add<Self, Output = Self>
    + Add<E::R1, Output = Self>
    + Add<f32, Output = Self>
    + Sub<Self, Output = Self>
    + Sub<E::R1, Output = Self>
    + Sub<f32, Output = Self>
    + Mul<Self, Output = Self>
    + Mul<E::R1, Output = Self>
    + Mul<f32, Output = Self>
    + Div<Self, Output = Self>
    + Div<E::R1, Output = Self>
    + Div<f32, Output = Self>
    + Neg<Output = Self>
    + Clone
where
    Self: Sized,
    Self: Borrow<Self>,
{
    fn splat(v: impl Into<E::R1>) -> Self;
    fn new(x: impl Into<E::R1>, y: impl Into<E::R1>) -> Self;

    fn mag(&self) -> E::R1;
    fn abs(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn dot(&self, other: impl Borrow<Self>) -> E::R1;

    fn max(&self, other: impl Borrow<Self>) -> Self;
    fn min(&self, other: impl Borrow<Self>) -> Self;

    fn x(&self) -> E::R1;
    fn y(&self) -> E::R1;

    // Add more as necessary.
}

pub trait Real3<E: Eval>:
    Add<Self, Output = Self>
    + Add<E::R1, Output = Self>
    + Add<f32, Output = Self>
    + Sub<Self, Output = Self>
    + Sub<E::R1, Output = Self>
    + Sub<f32, Output = Self>
    + Mul<Self, Output = Self>
    + Mul<E::R1, Output = Self>
    + Mul<f32, Output = Self>
    + Div<Self, Output = Self>
    + Div<E::R1, Output = Self>
    + Div<f32, Output = Self>
    + Neg<Output = Self>
    + Clone
where
    Self: Sized,
    Self: Borrow<Self>,
{
    fn splat(v: f32) -> Self;
    fn new(x: impl Into<E::R1>, y: impl Into<E::R1>, z: impl Into<E::R1>) -> Self;

    fn mag(&self) -> E::R1;
    fn abs(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn dot(&self, other: impl Borrow<Self>) -> E::R1;

    fn max(&self, other: impl Borrow<Self>) -> Self;
    fn min(&self, other: impl Borrow<Self>) -> Self;

    fn x(&self) -> E::R1;
    fn y(&self) -> E::R1;
    fn z(&self) -> E::R1;

    fn xz(&self) -> E::R2;

    fn zxy(&self) -> E::R3;
    // Add more as necessary.
}
