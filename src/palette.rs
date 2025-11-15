use crate::{color::Color, consts::EPSILON, util::mix};

pub(crate) struct Palette<const S: usize> {
    pub(crate) colors: [Color; S],
}

impl Palette<4> {
    pub(crate) const SKY: Self = Palette {
        colors: [
            // https://colorhunt.co/palette/eef5ffb4d4ff86b6f6176b87
            Color::from_hex(0xeef5ff), // #eef5ff
            Color::from_hex(0xb4d4ff), // #b4d4ff
            Color::from_hex(0x86b6f6), // #86b6f6
            Color::from_hex(0x176b87), // #176b87
        ],
    };
}

impl Palette<5> {
    pub(crate) const TEMPERATURE: Self = Palette {
        colors: [
            Color::from_hex(0x000000), // #000000
            Color::from_hex(0x0000ff), // #0000ff
            Color::from_hex(0x7f7fff), // #7f7fff
            Color::from_hex(0xff7f7f), // #ff7f7f
            Color::from_hex(0xff0000), // #ff0000
        ],
    };
}

impl<const S: usize> Palette<S> {
    pub(crate) fn get_color(&self, mut position: f32) -> Color {
        assert!(!self.colors.is_empty());
        if self.colors.len() == 1 {
            return self.colors[0];
        }
        if (position - 1.0).abs() <= EPSILON {
            position -= EPSILON;
        }
        assert!((0.0..1.0).contains(&position));
        position *= (self.colors.len() - 1) as f32;
        let index = position as usize;
        let ratio = position.fract();
        Color(mix(self.colors[index].0, self.colors[index + 1].0, ratio))
    }
}
