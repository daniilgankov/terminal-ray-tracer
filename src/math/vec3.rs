use std::{
    iter::Sum,
    ops::{Add, Div, Mul, MulAssign, Neg, Sub},
};

use crate::util::random;

use super::traits::{Fract, Max, Min, One, Signum, Sqrt, Zero};

#[derive(Clone, Copy, Default)]
pub(crate) struct Vec3<T> {
    pub(crate) x: T,
    pub(crate) y: T,
    pub(crate) z: T,
}

pub(crate) type Vec3f = Vec3<f32>;

macro_rules! vec3 {
    ($x:expr) => {
        crate::math::vec3::Vec3 {
            x: $x,
            y: $x,
            z: $x,
        }
    };
    ($xy:expr,$z:expr) => {
        crate::math::vec3::Vec3 {
            x: $xy.x,
            y: $xy.y,
            z: $z,
        }
    };
    ($x:expr,$y:expr,$z:expr) => {
        crate::math::vec3::Vec3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}

pub(crate) use vec3;

impl<T> Vec3<T>
where
    T: Copy,
{
    pub(crate) fn cross(self, rhs: Self) -> Self
    where
        T: Mul<Output = T> + Sub<Output = T>,
    {
        vec3!(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x
        )
    }

    pub(crate) fn dot(self, rhs: Self) -> T
    where
        T: Mul<Output = T> + Add<Output = T>,
    {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub(crate) fn frac(self) -> Self
    where
        T: Fract,
    {
        vec3!(self.x.fract(), self.y.fract(), self.z.fract())
    }

    pub(crate) fn length(self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Sqrt,
    {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub(crate) fn max(self, rhs: Self) -> Self
    where
        T: Max,
    {
        vec3!(self.x.max(rhs.x), self.y.max(rhs.y), self.z.max(rhs.z))
    }

    pub(crate) fn max_component(self) -> T
    where
        T: Max,
    {
        self.x.max(self.y).max(self.z)
    }

    pub(crate) fn min(self, rhs: Self) -> Self
    where
        T: Min,
    {
        vec3!(self.x.min(rhs.x), self.y.min(rhs.y), self.z.min(rhs.z))
    }

    pub(crate) fn min_component(self) -> T
    where
        T: Min,
    {
        self.x.min(self.y).min(self.z)
    }

    pub(crate) fn normalize(self) -> Self
    where
        T: Mul<Output = T> + Add<Output = T> + Sqrt,
        Vec3<T>: Div<T, Output = Self>,
    {
        self / self.length()
    }

    pub(crate) fn signum(self) -> Self
    where
        T: Signum,
    {
        vec3!(self.x.signum(), self.y.signum(), self.z.signum())
    }

    pub(crate) fn step(self, edge: T) -> Self
    where
        T: PartialOrd + Zero + One,
    {
        vec3!(
            if self.x < edge { T::ZERO } else { T::ONE },
            if self.y < edge { T::ZERO } else { T::ONE },
            if self.z < edge { T::ZERO } else { T::ONE }
        )
    }
}

impl Vec3f {
    pub(crate) fn random_unit() -> Self {
        // NOTE: It is OK to use one "random()" inside macro here to generate multiple different
        // values because obviously the function call will be duplicated instead of its value
        let position = vec3!(random());
        let position = 2.0 * position - 1.0;
        position.normalize()
    }

    pub(crate) fn reflect(self, normal: Self) -> Self {
        self - normal * 2.0 * normal.dot(self)
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        vec3!(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Add<T> for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, rhs: T) -> Self::Output {
        vec3!(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        vec3!(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T> Div<Vec3<T>> for f32
where
    f32: Div<T, Output = T>,
{
    type Output = Vec3<T>;

    fn div(self, rhs: Vec3<T>) -> Self::Output {
        vec3!(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}

impl<T> Mul for Vec3<T>
where
    T: Mul<Output = T>,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        vec3!(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T> Mul<f32> for Vec3<T>
where
    T: Mul<f32, Output = T>,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: f32) -> Self::Output {
        vec3!(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> MulAssign<f32> for Vec3<T>
where
    T: MulAssign<f32>,
{
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T>,
{
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        vec3!(-self.x, -self.y, -self.z)
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3!(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Sub<T> for Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        vec3!(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl<T> Sum for Vec3<T>
where
    T: Zero + Add<Output = T>,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |sum, value| sum + value)
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;

    fn mul(self, rhs: Vec3f) -> Self::Output {
        vec3!(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
