use iced::{
    widget::{button, checkbox, horizontal_space, row, text, text_input},
    Command, Element,
};

use serde::{Deserialize, Serialize};

use crate::gui::app::Message;

use super::todo_list::TodoListMessage;

#[derive(Debug, Clone)]
pub enum ItemMessage {
    Edit(EditMessage),
    Regular(RegularMessage),
}

#[derive(Debug, Clone)]
pub enum EditMessage {
    Name(String),
    Delete,
    Done,
}

#[derive(Debug, Clone)]
pub enum RegularMessage {
    Completed(bool),
    StartEdit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItemWidget {
    pub completed: bool,
    pub name: String,
    #[serde(skip)]
    pub editing: bool,
}

impl Default for TodoItemWidget {
    fn default() -> Self {
        Self {
            completed: false,
            name: "TodoItem".to_owned(),
            editing: false,
        }
    }
}

impl TodoItemWidget {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Self::default()
        }
    }

    pub fn update(&mut self, message: ItemMessage) -> Command<Message> {
        match message {
            ItemMessage::Edit(edit_message) => match edit_message {
                EditMessage::Name(name) => {
                    self.name = name;

                    Command::none()
                }
                EditMessage::Done => {
                    self.editing = false;

                    Command::none()
                }
                _ => Command::none(),
            },
            ItemMessage::Regular(regular_message) => match regular_message {
                RegularMessage::Completed(completed) => {
                    self.completed = completed;

                    Command::none()
                }
                RegularMessage::StartEdit => {
                    self.editing = true;

                    Command::none()
                }
            },
        }
    }

    pub fn view(&self, index: usize) -> Element<TodoListMessage> {
        if self.editing {
            self.view_edit()
                .map(move |message| TodoListMessage::Item(index, ItemMessage::Edit(message)))
        } else {
            self.view_regular()
                .map(move |message| TodoListMessage::Item(index, ItemMessage::Regular(message)))
        }
    }

    fn view_regular(&self) -> Element<RegularMessage> {
        row![
            checkbox("", self.completed).on_toggle(RegularMessage::Completed),
            text(&self.name),
            horizontal_space(),
            button("Edit").on_press(RegularMessage::StartEdit),
        ]
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn view_edit(&self) -> Element<EditMessage> {
        row![
            text_input("", &self.name)
                .on_input(EditMessage::Name)
                .on_submit(EditMessage::Done),
            button("Delete").on_press(EditMessage::Delete),
        ]
        .spacing(10)
        .into()
    }
}
