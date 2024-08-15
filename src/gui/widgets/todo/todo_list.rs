use iced::{
    theme::Text,
    widget::{column, container, scrollable, text, text_input, vertical_space, Column},
    Command, Element,
};
use serde::{Deserialize, Serialize};

use crate::{
    gui::{
        app::Message,
        styling::{self, colors},
        widgets::filter::Filter,
    },
    utils::{check_dirty, strip_trailing_newline},
};

use super::todo_item::{EditMessage, ItemMessage, TodoItem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    pub todo_items: Vec<TodoItem>,
    pub name: String,

    #[serde(skip)]
    pub input: String,

    #[serde(skip)]
    pub is_dirty: bool,

    #[serde(skip)]
    pub is_editing: bool,
}

#[derive(Debug, Clone)]
pub enum TodoListMessage {
    Item(usize, ItemMessage),
    InputEdit(String),
    NewSubmitted,
}

impl Default for TodoList {
    fn default() -> Self {
        Self {
            todo_items: Vec::new(),
            name: "TodoList".to_owned(),
            input: String::new(),
            is_dirty: false,
            is_editing: false,
        }
    }
}

impl TodoList {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Self::default()
        }
    }

    pub fn add(&mut self, name: &str) {
        self.todo_items.push(TodoItem::new(name));
    }

    pub fn update(&mut self, message: TodoListMessage) -> Command<Message> {
        let command = match message {
            TodoListMessage::InputEdit(action) => {
                self.input = action;

                Command::none()
            }
            TodoListMessage::NewSubmitted => {
                if !self.input.is_empty() {
                    let text = strip_trailing_newline(&self.input);
                    self.add(&text);
                    self.input = "".to_owned();
                }

                self.is_dirty = true;

                Command::none()
            }
            TodoListMessage::Item(index, item_message) => match item_message {
                ItemMessage::Edit(EditMessage::Delete) => {
                    self.todo_items.remove(index);

                    self.is_dirty = true;

                    Command::none()
                }
                _ => self.todo_items.get_mut(index).unwrap().update(item_message),
            },
        };

        self.is_dirty = check_dirty(&self.is_dirty, &self.todo_items, |item| item.is_dirty);

        command
    }

    pub fn view(&self, filter: &Filter) -> Element<TodoListMessage> {
        let title = text(&self.name).size(50);

        let new_todo = text_input("Input Todo", &self.input)
            .on_input(TodoListMessage::InputEdit)
            .on_submit(TodoListMessage::NewSubmitted)
            .style(styling::text_input::TextInput);

        let todo_items: Element<_> = {
            let filtered = self
                .todo_items
                .iter()
                .filter(|item| filter.filter(item))
                .collect::<Vec<_>>();

            let items = filtered
                .iter()
                .enumerate()
                .map(|(index, item)| item.view(index))
                .collect::<Vec<_>>();

            if items.is_empty() {
                container(column![
                    vertical_space(),
                    text(match filter {
                        Filter::All => "Add a new item todo",
                        Filter::Uncomplete => "Nothing Todo!",
                        Filter::Completed => "Nothing Completed...",
                    })
                    .size(30)
                    .style(Text::Color(colors::text::secondary())),
                    vertical_space(),
                ])
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center)
                .padding(10)
                .into()
            } else {
                scrollable(Column::with_children(items).spacing(10).padding(10)).into()
            }
        };

        column![title, new_todo, todo_items]
            .padding(15)
            .spacing(15)
            .align_items(iced::Alignment::Center)
            .into()
    }
}
