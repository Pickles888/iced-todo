use iced::widget::{button, Button};

use super::{TodoItemWidget, TodoMessage};

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Uncomplete,
    Completed,
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
) -> Button<'a, TodoMessage> {
    button(name).on_press_maybe(if *current_filter == filter {
        None
    } else {
        Some(TodoMessage::SetFilter(filter))
    })
}
