use crate::{
    color::Color,
    math::vec3::{Vec3f, vec3},
    palette::Palette,
};

pub(crate) struct Sky {
    pub(crate) sun_light_direction: Vec3f,
}

impl Sky {
    pub(crate) fn get_color(&self, direction: Vec3f) -> Color {
        let ratio = 0.5 * self.sun_light_direction.dot(-direction) + 0.5;
        Palette::SKY.get_color(ratio)
    }
}

impl Default for Sky {
    fn default() -> Self {
        Self {
            sun_light_direction: vec3!(-0.5).normalize(),
        }
    }
}
