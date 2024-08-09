use app::Todo;
use iced::{Application, Settings};

mod app;
mod icon;
mod todo;
mod utils;

fn main() -> iced::Result {
    Todo::run(Settings::default())
}
