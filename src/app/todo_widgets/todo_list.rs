use iced::{
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, text_input,
        vertical_space, Column,
    },
    Command, Element,
};

use crate::app::{
    todo_widgets::filter::{filter_button, Filter},
    Message,
};

use super::{
    todo_item::{EditMessage, TodoItemWidget},
    ItemMessage, TodoMessage,
};

#[derive(Debug)]
pub struct TodoListWidget {
    pub todo_items: Vec<TodoItemWidget>,
    pub name: String,
    pub input: String,
    pub filter: Filter,
}

impl TodoListWidget {
    pub fn new(name: &str) -> Self {
        TodoListWidget {
            todo_items: Vec::new(),
            name: name.to_string(),
            input: "".to_owned(),
            filter: Filter::All,
        }
    }

    pub fn add(&mut self, name: &str) {
        self.todo_items.push(TodoItemWidget::new(name));
    }

    pub fn update(&mut self, message: TodoMessage) -> Command<Message> {
        match message {
            TodoMessage::InputEdit(action) => {
                self.input = action;

                Command::none()
            }
            TodoMessage::NewSubmitted => {
                if !self.input.is_empty() {
                    let text = crate::utils::strip_trailing_newline(&self.input);
                    self.add(&text);
                    self.input = "".to_owned();
                }

                Command::none()
            }
            TodoMessage::SetFilter(filter) => {
                self.filter = filter;

                Command::none()
            }
            TodoMessage::Item(index, item_message) => match item_message {
                ItemMessage::Edit(EditMessage::Delete) => {
                    self.todo_items.remove(index);

                    Command::none()
                }
                _ => self.todo_items.get_mut(index).unwrap().update(item_message),
            },
        }
    }

    pub fn view(&self) -> Element<TodoMessage> {
        let title = text(&self.name).size(30);

        let new_todo = {
            let input = text_input("Input Todo", &self.input)
                .on_input(TodoMessage::InputEdit)
                .on_submit(TodoMessage::NewSubmitted);
            let new_button = button(container("+").center_x().center_y())
                .width(30)
                .on_press_maybe(if self.input.is_empty() {
                    None
                } else {
                    Some(TodoMessage::NewSubmitted)
                });

            row![input, new_button].spacing(10)
        };

        let todo_items: Element<_> = {
            let filtered = self
                .todo_items
                .iter()
                .filter(|item| self.filter.filter(item))
                .collect::<Vec<_>>();

            let items = filtered
                .iter()
                .enumerate()
                .map(|(index, item)| item.view(index))
                .collect::<Vec<_>>();

            if items.is_empty() {
                container(column![
                    vertical_space(),
                    text(match self.filter {
                        Filter::All => "Press + to add a new item todo",
                        Filter::Uncomplete => "Nothing Todo!",
                        Filter::Completed => "Nothing Completed...",
                    })
                    .size(30),
                    vertical_space(),
                ])
                .padding(10)
                .into()
            } else {
                scrollable(Column::with_children(items).spacing(10).padding(5)).into()
            }
        };

        let todo = column![title, new_todo, todo_items]
            .padding(15)
            .spacing(10)
            .align_items(iced::Alignment::Center);

        let status = {
            let filter = row![
                filter_button("All", &self.filter, Filter::All),
                filter_button("Uncomplete", &self.filter, Filter::Uncomplete),
                filter_button("Completed", &self.filter, Filter::Completed),
            ]
            .spacing(10);

            row![horizontal_space(), filter]
        };

        column![todo, vertical_space(), status]
            .spacing(10)
            .padding(10)
            .into()
    }
}
