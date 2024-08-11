use iced::Color;

use crate::utils::hex_to_rgb;

pub mod text {
    use iced::Color;

    use super::hex_to_color;

    pub fn primary() -> iced::Color {
        iced::Color {
            r: 198.0,
            g: 208.0,
            b: 245.0,
            a: 255.0,
        }
    }

    pub fn secondary() -> Color {
        hex_to_color("a5adce")
    }
}

pub fn hex_to_color(hex: &str) -> Color {
    let (r, g, b) = hex_to_rgb(hex);

    Color::from_rgb8(r, g, b)
}
