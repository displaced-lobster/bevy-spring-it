use bevy::prelude::{Vec2, Vec3};
use std::ops::{Add, Mul, Sub};

// Hack: since the compiler won't allow a generic implementation since upstream may break it in the future...
pub trait ComputeSpringReady {}

impl ComputeSpringReady for f32 {}
impl ComputeSpringReady for Vec2 {}
impl ComputeSpringReady for Vec3 {}

pub trait ComputeSpring {
    fn add(&self, other: &Self) -> Self;
    fn add_f32(&self, other: f32) -> Self;
    fn mul_f32(&self, other: f32) -> Self;
    fn sub(&self, other: &Self) -> Self;
}

impl<T> ComputeSpring for T
where
    T: Add<Output = T>
        + Add<f32, Output = T>
        + Mul<f32, Output = T>
        + Sub<Output = T>
        + Clone
        + Copy
        + ComputeSpringReady,
{
    fn add(&self, other: &Self) -> Self {
        *self + *other
    }

    fn add_f32(&self, v: f32) -> Self {
        *self + v
    }

    fn mul_f32(&self, v: f32) -> Self {
        *self * v
    }

    fn sub(&self, other: &Self) -> Self {
        *self - *other
    }
}
