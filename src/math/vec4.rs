use std::ops::{Add, Mul};

use crate::math::vec3::vec3;

use super::vec3::Vec3;

#[derive(Clone, Copy)]
pub(crate) struct Vec4<T> {
    pub(crate) x: T,
    pub(crate) y: T,
    pub(crate) z: T,
    pub(crate) w: T,
}

macro_rules! vec4 {
    ($xyz:expr,$w:expr) => {
        crate::math::vec4::Vec4 {
            x: $xyz.x,
            y: $xyz.y,
            z: $xyz.z,
            w: $w,
        }
    };
    ($x:expr,$y:expr,$z:expr,$w:expr) => {
        crate::math::vec4::Vec4 {
            x: $x,
            y: $y,
            z: $z,
            w: $w,
        }
    };
}

pub(crate) use vec4;

impl<T> Vec4<T> {
    pub(crate) fn hadamard(self, rhs: Self) -> Self
    where
        T: Mul<Output = T>,
    {
        vec4!(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w
        )
    }

    pub(crate) fn sum(self) -> T
    where
        T: Add<Output = T>,
    {
        self.x + self.y + self.z + self.w
    }

    pub(crate) fn xyz(self) -> Vec3<T> {
        vec3!(self.x, self.y, self.z)
    }
}
