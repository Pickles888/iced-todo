use filter::Filter;
use iced::{
    widget::{
        button, column, horizontal_space, row, text, Button, Checkbox, Column, Text, TextInput,
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
    Filter(Filter),
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
                let text = self.input.clone();
                self.add(&text);
                self.input = "".to_owned();

                Command::none()
            }
            TodoMessage::Filter(_) => todo!(),
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

        let todo = {
            let filtered = self
                .todo_items
                .iter()
                .filter(|item| self.filter.filter(item.clone()))
                .collect::<Vec<_>>();

            let _items = filtered
                .iter()
                .enumerate()
                .map(|(index, item)| item.view(index))
                .collect::<Vec<_>>();

            if _items.is_empty() {
                let _items: Text<'_, Theme, iced::Renderer> = text(match self.filter {
                    Filter::All => "Press + to add a new ",
                    Filter::Uncomplete => "Nothing Todo!",
                    Filter::Completed => "Nothing Completed...",
                })
                .size(30);
            }

            Column::with_children(_items).spacing(10)
        };

        let status = {
            //let filter = row![button("All").]

            row![horizontal_space(),]
        };

        column![title, new_todo, todo, status]
            .padding(15)
            .spacing(10)
            .into()
    }
}
