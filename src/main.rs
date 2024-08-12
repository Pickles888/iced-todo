use gui::Todo;
use iced::{Application, Settings};

mod gui;
mod utils;

fn main() -> iced::Result {
    Todo::run(Settings::default())
}
