use iced::widget::{button, container, Button};

use crate::gui::{app::Message, styling};

use super::todo::todo_item::TodoItem;

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
    pub fn filter(&self, todoitem: &TodoItem) -> bool {
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
    button(container(name).width(100).center_x())
        .on_press_maybe(if *current_filter == filter {
            None
        } else {
            Some(Message::SetFilter(filter))
        })
        .style(styling::button::Button::Rounded)
}
