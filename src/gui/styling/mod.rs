use iced::{border::Radius, Border, Shadow, Vector};

pub mod button;
pub mod checkbox;
pub mod colors;
pub mod container;
pub mod text_input;

fn no_shadow() -> Shadow {
    Shadow {
        color: colors::no_color(),
        offset: Vector { x: 0.0, y: 0.0 },
        blur_radius: 0.0,
    }
}

fn no_offset() -> Vector {
    Vector { x: 0.0, y: 0.0 }
}

fn no_border(radius: i32) -> Border {
    Border {
        color: colors::no_color(),
        width: 0.0,
        radius: Radius::from(radius),
    }
}

pub const ROUNDING: i32 = 20;
