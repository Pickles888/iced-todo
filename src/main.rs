use gui::Todo;
use iced::{Application, Settings};

mod gui;
mod todo_api;

fn main() -> iced::Result {
    Todo::run(Settings::default())
}
