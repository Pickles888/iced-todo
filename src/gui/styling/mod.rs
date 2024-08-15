use iced::{border::Radius, Border};

pub mod button;
pub mod checkbox;
pub mod colors;
pub mod container;
pub mod text_input;

fn no_border(radius: i32) -> Border {
    Border {
        color: colors::no_color(),
        width: 0.0,
        radius: Radius::from(radius),
    }
}

pub const ROUNDING: i32 = 20;
