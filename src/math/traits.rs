use super::vec3::{Vec3, vec3};

macro_rules! impl_trait {
    ($trait:tt, $fn:ident, $type:ty) => {
        pub(crate) trait $trait {
            fn $fn(self) -> Self;
        }

        impl $trait for $type {
            fn $fn(self) -> Self {
                self.$fn()
            }
        }
    };
    ($trait:tt, $fn:ident rhs, $type:ty) => {
        pub(crate) trait $trait {
            fn $fn(self, rhs: Self) -> Self;
        }

        impl $trait for $type {
            fn $fn(self, rhs: Self) -> Self {
                self.$fn(rhs)
            }
        }
    };
}

impl_trait!(Fract, fract, f32);
impl_trait!(Max, max rhs, f32);
impl_trait!(Min, min rhs, f32);
impl_trait!(Signum, signum, f32);
impl_trait!(Sqrt, sqrt, f32);

pub(crate) trait Zero {
    const ZERO: Self;
}

impl Zero for f32 {
    const ZERO: Self = 0.0;
}

impl<T> Zero for Vec3<T>
where
    T: Zero,
{
    const ZERO: Self = vec3!(T::ZERO);
}

pub(crate) trait One {
    const ONE: Self;
}

impl One for f32 {
    const ONE: Self = 1.0;
}
