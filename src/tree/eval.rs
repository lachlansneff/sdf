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
    fn eval<E: Eval>(&self, p: E::V3) -> E::V;
}

pub trait Eval
where
    Self: Sized,
{
    type V: Value<Self>;
    type V2: Value2<Self>;
    type V3: Value3<Self>;
}

pub trait Value<E: Eval>:
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

pub trait Value2<E: Eval>:
    Add<Self, Output = Self>
    + Add<E::V, Output = Self>
    + Add<f32, Output = Self>
    + Sub<Self, Output = Self>
    + Sub<E::V, Output = Self>
    + Sub<f32, Output = Self>
    + Mul<Self, Output = Self>
    + Mul<E::V, Output = Self>
    + Mul<f32, Output = Self>
    + Div<Self, Output = Self>
    + Div<E::V, Output = Self>
    + Div<f32, Output = Self>
    + Neg<Output = Self>
    + Clone
where
    Self: Sized,
    Self: Borrow<Self>,
{
    fn splat(v: impl Into<E::V>) -> Self;
    fn new(x: impl Into<E::V>, y: impl Into<E::V>) -> Self;

    fn mag(&self) -> E::V;
    fn abs(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn dot(&self, other: impl Borrow<Self>) -> E::V;

    fn max(&self, other: impl Borrow<Self>) -> Self;
    fn min(&self, other: impl Borrow<Self>) -> Self;

    fn x(&self) -> E::V;
    fn y(&self) -> E::V;

    // Add more as necessary.
}

pub trait Value3<E: Eval>:
    Add<Self, Output = Self>
    + Add<E::V, Output = Self>
    + Add<f32, Output = Self>
    + Sub<Self, Output = Self>
    + Sub<E::V, Output = Self>
    + Sub<f32, Output = Self>
    + Mul<Self, Output = Self>
    + Mul<E::V, Output = Self>
    + Mul<f32, Output = Self>
    + Div<Self, Output = Self>
    + Div<E::V, Output = Self>
    + Div<f32, Output = Self>
    + Neg<Output = Self>
    + Clone
where
    Self: Sized,
    Self: Borrow<Self>,
{
    fn splat(v: f32) -> Self;
    fn new(x: impl Into<E::V>, y: impl Into<E::V>, z: impl Into<E::V>) -> Self;

    fn mag(&self) -> E::V;
    fn abs(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn dot(&self, other: impl Borrow<Self>) -> E::V;

    fn max(&self, other: impl Borrow<Self>) -> Self;
    fn min(&self, other: impl Borrow<Self>) -> Self;

    fn x(&self) -> E::V;
    fn y(&self) -> E::V;
    fn z(&self) -> E::V;

    fn xz(&self) -> E::V2;

    fn zxy(&self) -> E::V3;
    // Add more as necessary.
}
