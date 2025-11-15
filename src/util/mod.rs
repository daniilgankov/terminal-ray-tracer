pub(crate) mod average_sum;
pub(crate) mod timer;

use std::ops::{Add, Mul, Sub};

use libc::{RAND_MAX, rand};

#[cfg(test)]
use crate::consts::EPSILON;

pub(crate) fn mix<T>(a: T, b: T, ratio: f32) -> T
where
    T: Sub<Output = T> + Mul<f32, Output = T> + Add<Output = T> + Copy,
{
    a + (b - a) * ratio
}

pub(crate) fn random() -> f32 {
    let rand = unsafe { rand() };
    rand as f32 / RAND_MAX as f32
}

#[cfg(test)]
pub(crate) fn assert_nearly_eq_f32(a: f32, b: f32) -> bool {
    let delta = (a - b).abs();
    delta < EPSILON
}
