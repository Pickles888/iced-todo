use iced::{
    executor,
    widget::{column, container, row, scrollable, text, Column},
    Application, Command, Element, Renderer, Theme,
};

pub struct Todo;

#[derive(Debug, Clone)]
pub enum Message {}

impl Application for Todo {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("Todo List")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        let side_bar = scrollable(
            Column::from_vec(Vec::from())
                .padding(10)
                .max_width(500)
                .spacing(5),
        );

        row![side_bar].into()
    }
}
