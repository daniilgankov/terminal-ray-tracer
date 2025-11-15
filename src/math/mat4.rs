use std::ops::{Add, Mul};

use crate::math::vec4::vec4;

use super::vec4::Vec4;

#[derive(Clone, Copy)]
pub(crate) struct Mat4<T> {
    pub(crate) x: Vec4<T>,
    pub(crate) y: Vec4<T>,
    pub(crate) z: Vec4<T>,
    pub(crate) w: Vec4<T>,
}

pub(crate) type Mat4f = Mat4<f32>;

impl<T> Mat4<T> {
    pub(crate) fn transpose(self) -> Self
    where
        T: Copy,
    {
        Mat4 {
            x: vec4!(self.x.x, self.y.x, self.z.x, self.w.x),
            y: vec4!(self.x.y, self.y.y, self.z.y, self.w.y),
            z: vec4!(self.x.z, self.y.z, self.z.z, self.w.z),
            w: vec4!(self.x.w, self.y.w, self.z.w, self.w.w),
        }
    }
}

impl<T> Mul<Vec4<T>> for Mat4<T>
where
    T: Mul<Output = T> + Add<Output = T>,
    Vec4<T>: Copy,
{
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        vec4!(
            self.x.hadamard(rhs).sum(),
            self.y.hadamard(rhs).sum(),
            self.z.hadamard(rhs).sum(),
            self.w.hadamard(rhs).sum()
        )
    }
}
