use std::path::PathBuf;

use crate::utils::check_dirty;

use super::persistance::{self, PersistError, Persistance};
use super::widgets::filter::{filter_button, Filter};
use super::widgets::lists_bar::{ListsBar, ListsBarMessage};
use super::widgets::todo::todo_list::{TodoList, TodoListMessage};
use iced::{
    executor,
    widget::{column, horizontal_space, row, text},
    Application, Command, Element, Renderer, Theme,
};

pub struct Todo {
    pub todo_lists: Vec<TodoList>,
    is_dark: bool,
    pub is_dirty: bool,
    pub current_list: Option<usize>,
    status: Result<String, PersistError>,
    filter: Filter,
    pub lists_bar: ListsBar,
}

#[derive(Debug, Clone)]
pub enum Message {
    List(usize, TodoListMessage),
    SetFilter(Filter),
    Saved(Result<(), PersistError>),
    ListsBar(ListsBarMessage),
}

impl Persistance for Todo {
    fn config_path() -> Result<PathBuf, PersistError> {
        let mut path_buf = dirs::config_dir().ok_or(PersistError::Path)?;
        path_buf.push("todo_save.json");

        Ok(path_buf)
    }
}

impl Application for Todo {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        // loading is hacky
        let (todo_lists, error) = match Self::load::<Vec<TodoList>>() {
            Ok(item) => (item, Ok("Loaded".to_owned())),
            Err(error) => (Vec::new(), Err(error)),
        };

        (
            Self {
                todo_lists,
                status: error, // maybe update later on to be a message
                current_list: None,
                is_dark: true,
                is_dirty: false,
                filter: Filter::All,
                lists_bar: ListsBar::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let dirty_to_char = |dirty: bool| {
            if dirty {
                "*"
            } else {
                ""
            }
        };

        if let Some(i) = self.current_list {
            let list = self.todo_lists.get(i).unwrap(); // sometimes gets called while the

            format!(
                "Iced Todo{} - {}{}",
                dirty_to_char(self.is_dirty),
                list.name,
                dirty_to_char(list.is_dirty),
            )
        } else {
            format!("Iced Todo{}", dirty_to_char(self.is_dirty))
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        // saving is (kinda) hacky
        let command = match message {
            Message::SetFilter(filter) => {
                self.filter = filter;

                Command::none()
            }
            Message::Saved(result) => {
                let total_items = self.get_total_items();

                self.status = match result {
                    Ok(_) => Ok(format!(
                        "{} thing{} todo",
                        total_items,
                        if total_items != 1 { "s" } else { "" }
                    )),
                    Err(error) => Err(error),
                };

                Command::none()
            }
            Message::List(list_index, message) => {
                self.todo_lists.get_mut(list_index).unwrap().update(message)
            }
            Message::ListsBar(lists_bar_message) => self.update_lists_bar(lists_bar_message),
        };

        self.is_dirty = check_dirty(&self.is_dirty, &self.todo_lists, |list| list.is_dirty);

        if self.is_dirty {
            Command::perform(Self::save(self.todo_lists.clone()), Message::Saved)
        } else {
            command
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        let todo_lists_bar: Element<_> = self.lists_bar();

        let status = {
            // try to put this in update in the future
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

            row![persistance_status, horizontal_space(), filter]
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
            todo_lists_bar
        };

        column![main_view, status].into()
    }

    fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::CatppuccinFrappe
        } else {
            Theme::CatppuccinLatte
        }
    }
}

impl Todo {
    fn get_total_items(&self) -> u64 {
        let mut i = 0;

        for list in &self.todo_lists {
            i += list.todo_items.len();
        }

        i.try_into().unwrap()
    }
}
