use iced::Color;

use crate::utils::hex_to_rgb;

pub fn no_color() -> Color {
    Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    }
}

pub mod text {
    use iced::Color;

    use super::hex_to_color;

    pub fn primary() -> Color {
        hex_to_color("c6d0f5")
    }

    pub fn secondary() -> Color {
        hex_to_color("a5adce")
    }

    pub fn black() -> Color {
        hex_to_color("232634")
    }
}

pub mod accents {
    use iced::Color;

    use super::hex_to_color;

    pub fn primary() -> Color {
        hex_to_color("f4b8e4") // pink
    }

    pub fn secondary() -> Color {
        hex_to_color("ca9ee6") // mauve
    }

    pub fn danger() -> Color {
        hex_to_color("e78284") // red
    }

    pub fn bg() -> Color {
        hex_to_color("292c3c") // mantle
    }

    pub fn bg2() -> Color {
        hex_to_color("232634") // base
    }
}

pub mod container {
    pub mod with_background {
        use iced::Color;

        use crate::gui::styling::colors::hex_to_color;

        pub fn background() -> Color {
            hex_to_color("414559")
        }

        pub fn midground() -> Color {
            hex_to_color("51576d")
        }

        pub fn border() -> Color {
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }
        }
    }
}

pub fn hex_to_color(hex: &str) -> Color {
    let (r, g, b) = hex_to_rgb(hex);

    Color::from_rgb8(r, g, b)
}
