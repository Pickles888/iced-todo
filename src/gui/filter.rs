use iced::widget::{button, Button};

use super::{app::Message, todo_widgets::todo_item::TodoItemWidget};

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Uncomplete,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}

impl Filter {
    pub fn filter(&self, todoitem: &TodoItemWidget) -> bool {
        match *self {
            Filter::All => true,
            Filter::Uncomplete => !todoitem.completed,
            Filter::Completed => todoitem.completed,
        }
    }
}

pub fn filter_button<'a>(
    name: &'a str,
    current_filter: &'a Filter,
    filter: Filter,
) -> Button<'a, Message> {
    button(name).on_press_maybe(if *current_filter == filter {
        None
    } else {
        Some(Message::SetFilter(filter))
    })
}
