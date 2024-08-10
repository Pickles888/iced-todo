use iced::{
    widget::{button, column, horizontal_space, row, text, Button, Checkbox, Column, TextInput},
    Command, Element,
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
}

impl TodoListWidget {
    pub fn new(name: &str) -> Self {
        TodoListWidget {
            todo_items: Vec::new(),
            name: name.to_string(),
            input: "".to_owned(),
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

        let todo = Column::with_children(
            self.todo_items
                .iter()
                .enumerate()
                .map(|(index, item)| item.view(index))
                .collect::<Vec<_>>(),
        )
        .spacing(10);

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
