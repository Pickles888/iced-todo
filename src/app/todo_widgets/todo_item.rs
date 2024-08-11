use iced::{
    widget::{button, checkbox, horizontal_space, row, text, text_input},
    Command, Element,
};

use crate::app::Message;

use super::TodoMessage;

#[derive(Debug, Clone)]
pub enum ItemMessage {
    Edit(EditMessage),
    Regular(RegularMessage),
}

#[derive(Debug, Clone)]
pub enum EditMessage {
    Name(String),
    SubmitName,
    Delete,
    Done,
}

#[derive(Debug, Clone)]
pub enum RegularMessage {
    Completed(bool),
    StartEdit,
}

#[derive(Debug)]
pub struct TodoItemWidget {
    pub completed: bool,
    pub name: String,
    pub editing: bool,
}

impl TodoItemWidget {
    pub fn new(name: &str) -> Self {
        TodoItemWidget {
            completed: false,
            name: name.to_string(),
            editing: false,
        }
    }

    pub fn update(&mut self, message: ItemMessage) -> Command<Message> {
        match message {
            ItemMessage::Edit(edit_message) => match edit_message {
                EditMessage::Name(name) => {
                    self.name = name;

                    Command::none()
                }
                EditMessage::SubmitName => {
                    self.editing = false;

                    Command::none()
                }
                EditMessage::Done => todo!(),
                _ => Command::none(),
            },
            ItemMessage::Regular(regular_message) => match regular_message {
                RegularMessage::Completed(completed) => {
                    self.completed = completed;

                    Command::none()
                }
                RegularMessage::StartEdit => todo!(),
            },
        }
    }

    pub fn view(&self, index: usize) -> Element<TodoMessage> {
        if self.editing {
            self.view_edit()
                .map(move |message| TodoMessage::Item(index, ItemMessage::Edit(message)))
        } else {
            self.view_regular()
                .map(move |message| TodoMessage::Item(index, ItemMessage::Regular(message)))
        }
    }

    fn view_regular(&self) -> Element<RegularMessage> {
        row![
            checkbox("", self.completed)
                .on_toggle(move |toggled| RegularMessage::Completed(toggled)),
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
                .on_submit(EditMessage::SubmitName),
            horizontal_space(),
            button("Delete").on_press(EditMessage::Delete),
            button("Done").on_press(EditMessage::Done),
        ]
        .into()
    }
}
