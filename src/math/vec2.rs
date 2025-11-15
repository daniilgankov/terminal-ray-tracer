use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Clone, Copy)]
pub(crate) struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

pub(crate) type Vec2f = Vec2<f32>;
pub(crate) type Vec2u = Vec2<usize>;

macro_rules! vec2 {
    ($x:expr) => {
        crate::math::vec2::Vec2 { x: $x, y: $x }
    };
    ($x:expr,$y:expr) => {
        crate::math::vec2::Vec2 { x: $x, y: $y }
    };
}

pub(crate) use vec2;

impl<T> Vec2<T>
where
    T: Copy,
{
    pub(crate) fn area(&self) -> T
    where
        T: Mul<Output = T> + Copy,
    {
        self.x * self.y
    }

    pub(crate) fn hadamard(self, rhs: Self) -> Self
    where
        T: Mul<Output = T>,
    {
        vec2!(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        vec2!(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Add<T> for Vec2<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn add(self, rhs: T) -> Self::Output {
        vec2!(self.x + rhs, self.y + rhs)
    }
}

impl<T> AddAssign for Vec2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> AddAssign<T> for Vec2<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl<T> Div for Vec2<T>
where
    T: Div<Output = T>,
{
    type Output = Vec2<T>;

    fn div(self, rhs: Self) -> Self::Output {
        vec2!(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Self::Output {
        vec2!(self.x / rhs, self.y / rhs)
    }
}

// TODO: Reimplement with macroses; use TryFrom
impl From<Vec2u> for Vec2f {
    fn from(value: Vec2u) -> Self {
        vec2!(value.x as f32, value.y as f32)
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        vec2!(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vec2f> for f32 {
    type Output = Vec2f;

    fn mul(self, rhs: Vec2f) -> Self::Output {
        vec2!(self * rhs.x, self * rhs.y)
    }
}

impl<T> Sub<T> for Vec2<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn sub(self, rhs: T) -> Self::Output {
        vec2!(self.x - rhs, self.y - rhs)
    }
}

impl<T> SubAssign for Vec2<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> SubAssign<T> for Vec2<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}
