use app::Todo;
use iced::{Application, Settings};

mod app;
mod utils;

fn main() -> iced::Result {
    Todo::run(Settings::default())
}
