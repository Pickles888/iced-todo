use iced::{
    executor,
    theme::{Button as ButtonTheme, Text},
    widget::{button, column, container, row, scrollable, text, text_input, Button, Column},
    Application, Command, Element, Length, Renderer, Theme,
};
use todo_widgets::{todo_list::TodoListWidget, TodoMessage};

mod colors;
mod icons;
mod persistance;
mod todo_widgets;

#[derive(Default)]
pub struct Todo {
    todo_lists: Vec<TodoListWidget>,
    current_list: Option<usize>,
    new_list_input: String,
    is_adding_list: bool,
    is_dark: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    List(usize, TodoMessage),
    ListBarPressed(usize),
    NewListInput(String),
    NewListSubmit,
    AddingList,
}

impl Application for Todo {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                todo_lists: Vec::new(),
                current_list: None,
                is_dark: true,
                is_adding_list: false,
                new_list_input: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Todo List")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ListBarPressed(list_index) => {
                self.current_list = Some(list_index);

                Command::none()
            }
            Message::NewListInput(input) => {
                self.new_list_input = input;

                Command::none()
            }
            Message::NewListSubmit => {
                self.todo_lists
                    .push(TodoListWidget::new(&self.new_list_input));

                self.new_list_input = String::new();
                self.is_adding_list = false;

                Command::none()
            }
            Message::List(list_index, message) => {
                self.todo_lists.get_mut(list_index).unwrap().update(message)
            }
            Message::AddingList => {
                self.is_adding_list = true;

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        let todo_lists_bar = {
            let lists: Column<_> = Column::with_children(
                self.todo_lists
                    .iter()
                    .enumerate()
                    .map(|(index, list)| {
                        button(&*list.name)
                            .on_press(Message::ListBarPressed(index))
                            .style(ButtonTheme::Text)
                            .into()
                    })
                    .collect::<Vec<_>>(),
            );

            let add_new: Element<_> = if self.is_adding_list {
                text_input("Add a todo list", &self.new_list_input)
                    .on_input(Message::NewListInput)
                    .on_submit(Message::NewListSubmit)
                    .width(Length::Fill)
                    .into()
            } else {
                Button::new(container("+").width(Length::Fill).center_x())
                    .on_press(Message::AddingList)
                    .into()
            };

            container(scrollable(
                column![add_new, lists].padding(10).spacing(15).width(150),
            ))
        };

        let todo_list: Element<_> = if let Some(current_list) = self.current_list {
            self.todo_lists
                .get(current_list)
                .unwrap()
                .view()
                .map(move |message| Message::List(current_list, message))
        } else {
            text("Begin by pressing + to add a list")
                .size(40)
                .style(Text::Color(colors::text::secondary()))
                .into()
        };

        row![todo_lists_bar, todo_list].into()
    }

    fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::CatppuccinFrappe
        } else {
            Theme::CatppuccinLatte
        }
    }
}
