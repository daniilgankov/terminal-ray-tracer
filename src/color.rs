use crate::math::vec3::{Vec3f, vec3};

#[derive(Clone, Copy)]
pub(crate) struct Color(pub(crate) Vec3f);

const MAX_VALUE: f32 = 255.0;
const MAX_VALUE_INVERTED: f32 = 1.0 / MAX_VALUE;

impl Color {
    pub(crate) const RED: Self = Self(vec3!(1.0, 0.0, 0.0));
    pub(crate) const GREEN: Self = Self(vec3!(0.0, 1.0, 0.0));
    pub(crate) const BLUE: Self = Self(vec3!(0.0, 0.0, 1.0));
    pub(crate) const BLACK: Self = Self(vec3!(0.0, 0.0, 0.0));

    pub(crate) const fn from_hex(value: u32) -> Color {
        let r = MAX_VALUE_INVERTED * (value >> 16 & 0xff) as f32;
        let g = MAX_VALUE_INVERTED * (value >> 8 & 0xff) as f32;
        let b = MAX_VALUE_INVERTED * (value & 0xff) as f32;
        Color(vec3!(r, g, b))
    }

    pub(crate) fn encode(&self) -> String {
        let r = (MAX_VALUE * self.0.x) as u8;
        let g = (MAX_VALUE * self.0.y) as u8;
        let b = (MAX_VALUE * self.0.z) as u8;
        format!(";2;{r};{g};{b}m")
    }

    pub(crate) fn inverted(&self) -> Self {
        Self((self.0 + 0.5).frac())
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}
