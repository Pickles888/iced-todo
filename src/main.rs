use app::Todo;
use iced::{Application, Settings};

mod app;

fn main() -> iced::Result {
    Todo::run(Settings::default())
}
