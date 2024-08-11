use filter::{filter_button, Filter};
use iced::{
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, vertical_space, Button,
        Checkbox, Column, Text, TextInput,
    },
    Command, Element, Theme,
};

use super::Message;

mod filter;

#[derive(Debug)]
pub struct TodoItemWidget {
    pub completed: bool,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum TodoMessage {
    Completed(usize, bool),
    Edit(usize),
    InputEdit(String),
    SetFilter(Filter),
    NewSubmitted,
}

impl TodoItemWidget {
    pub fn new(name: &str) -> Self {
        TodoItemWidget {
            completed: false,
            name: name.to_string(),
        }
    }

    pub fn view(&self, index: usize) -> Element<TodoMessage> {
        row![
            Checkbox::new("", self.completed)
                .on_toggle(move |toggled| TodoMessage::Completed(index, toggled)),
            text(&self.name),
            horizontal_space(),
            Button::new("Edit").on_press(TodoMessage::Edit(index)),
        ]
        .align_items(iced::Alignment::Center)
        .into()
    }

    //pub fn view_edit(&self, index: usize) -> Element<TodoMessage> {}
}

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
            TodoMessage::Completed(item_index, completed) => {
                self.todo_items[item_index].completed = completed;

                Command::none()
            }
            TodoMessage::Edit(item_index) => Command::none(),
            TodoMessage::InputEdit(action) => {
                self.input = action;

                Command::none()
            }
            TodoMessage::NewSubmitted => {
                let text = crate::utils::strip_trailing_newline(&self.input);
                self.add(&text);
                self.input = "".to_owned();

                Command::none()
            }
            TodoMessage::SetFilter(filter) => {
                self.filter = filter;

                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<TodoMessage> {
        let title = text(&self.name).size(30);

        let new_todo = {
            let input = TextInput::new("Input Todo", &self.input)
                .on_input(TodoMessage::InputEdit)
                .on_submit(TodoMessage::NewSubmitted);
            let new_button = button("New").on_press(TodoMessage::NewSubmitted);

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
                container(
                    text(match self.filter {
                        Filter::All => "Press + to add a new item todo",
                        Filter::Uncomplete => "Nothing Todo!",
                        Filter::Completed => "Nothing Completed...",
                    })
                    .size(30),
                )
                .padding(10)
                .into()
            } else {
                scrollable(Column::with_children(items).spacing(10).padding(5)).into()
            }
        };

        let todo = column![title, new_todo, todo_items]
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
