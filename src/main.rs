use gui::app::Todo;
use iced::{Application, Font, Settings};

mod gui;
mod utils;

fn main() -> iced::Result {
    Todo::run(Settings {
        default_font: Font::with_name("Montserrat"),
        fonts: vec![include_bytes!("../fonts/Montserrat-SemiBold.ttf")
            .as_slice()
            .into()],
        ..Settings::default()
    })
}
