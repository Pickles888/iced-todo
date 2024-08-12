use std::path::PathBuf;

use super::filter::{filter_button, Filter};
use super::persistance::{self, PersistError, Persistance};
use super::todo_widgets::todo_list::{TodoListMessage, TodoListWidget};
use iced::{
    executor,
    theme::Button as ButtonTheme,
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, text_input,
        vertical_space, Button, Column,
    },
    Application, Command, Element, Length, Renderer, Theme,
};

pub struct Todo {
    todo_lists: Vec<TodoListWidget>,
    current_list: Option<usize>,
    new_list_input: String,
    is_adding_list: bool,
    is_dark: bool,
    status: Result<String, PersistError>,
    filter: Filter,
}

#[derive(Debug, Clone)]
pub enum Message {
    List(usize, TodoListMessage),
    ListBarPressed(usize),
    NewListInput(String),
    SetFilter(Filter),
    NewListSubmit,
    AddingList,
    Saved(Result<(), PersistError>),
}

impl Persistance for Todo {
    fn path() -> Result<PathBuf, PersistError> {
        dirs::config_dir().ok_or(PersistError::Path)
    }
}

impl Todo {
    fn save_update<F>(&self, f: F) -> Command<Message>
    where
        F: FnOnce() -> Command<Message>,
    {
        // very hacky

        Command::perform(Self::save(self.todo_lists.clone()), Message::Saved)
    }
}

impl Application for Todo {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        // loading is hacky
        let (todo_lists, error) = match Self::load::<Vec<TodoListWidget>>() {
            Ok(item) => (item, Ok("Loaded".to_owned())),
            Err(error) => (Vec::new(), Err(error)),
        };

        (
            Self {
                todo_lists,
                status: error, // maybe update later on to be a message
                current_list: None,
                new_list_input: String::new(),
                is_adding_list: false,
                is_dark: true,
                filter: Filter::All,
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

                Command::perform(Self::save(self.todo_lists.clone()), Message::Saved)
            }
            Message::List(list_index, message) => {
                self.todo_lists.get_mut(list_index).unwrap().update(message)
            }
            Message::AddingList => {
                self.is_adding_list = true;

                Command::none()
            }
            Message::SetFilter(filter) => {
                self.filter = filter;

                Command::none()
            }
            Message::Saved(result) => {
                self.status = match result {
                    Ok(_) => Ok("Saved".to_owned()),
                    Err(error) => Err(error),
                };

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
                Button::new(
                    container(if self.todo_lists.is_empty() {
                        "Click me!"
                    } else {
                        "+"
                    })
                    .width(Length::Fill)
                    .center_x(),
                )
                .on_press(Message::AddingList)
                .into()
            };

            container(scrollable(
                column![add_new, lists].padding(10).spacing(15).width(150),
            ))
        };

        let status = {
            let persistance_status = text(match &self.status {
                Ok(message) => message,
                Err(error) => match error {
                    PersistError::Save(save_error) => match save_error {
                        persistance::SaveError::Write => "Failed to write to save file",
                        persistance::SaveError::Compose => "Failed to compose json data",
                    },
                    PersistError::Load(load_error) => match load_error {
                        persistance::LoadError::Read => "Failed to read config file",
                        persistance::LoadError::Parse => "Failed to parse config data",
                    },
                    PersistError::Path => "Could not get config directory",
                },
            })
            .size(20);

            let filter = row![
                filter_button("All", &self.filter, Filter::All),
                filter_button("Uncomplete", &self.filter, Filter::Uncomplete),
                filter_button("Completed", &self.filter, Filter::Completed),
            ]
            .spacing(10);

            row![horizontal_space(), persistance_status, filter]
                .align_items(iced::Alignment::Center)
                .spacing(10)
                .padding(10)
        };

        let main_view: Element<_> = if let Some(current_list) = self.current_list {
            let lists = self
                .todo_lists
                .get(current_list)
                .unwrap()
                .view(&self.filter)
                .map(move |message| Message::List(current_list, message));

            row![todo_lists_bar, lists].into()
        } else {
            todo_lists_bar.into()
        };

        column![main_view, vertical_space(), status].into()
    }

    fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::CatppuccinFrappe
        } else {
            Theme::CatppuccinLatte
        }
    }
}
