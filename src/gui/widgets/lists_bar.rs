use iced::{
    alignment,
    theme::Button as ButtonTheme,
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, text_input, Button,
        Column,
    },
    Alignment, Command, Element, Length,
};

use crate::{
    gui::{
        app::{Message, Todo},
        icons::IconType,
        styling,
    },
    SIDEBAR_WIDTH,
};

use super::todo::todo_list::TodoList;

pub struct ListsBar {
    pub new_list_input: String,
    pub is_adding_list: bool,
}

#[derive(Debug, Clone)]
pub enum ListsBarMessage {
    Regular(usize, RegularMessage),
    Edit(usize, EditMessage),
    NewList(NewListMessage),
    AddingList,
    Select(usize),
}

#[derive(Debug, Clone)]
pub enum NewListMessage {
    Input(String),
    Submit,
}

#[derive(Debug, Clone)]
pub enum RegularMessage {
    StartEdit,
}

#[derive(Debug, Clone)]
pub enum EditMessage {
    Name(String),
    Delete,
    Done,
}

impl ListsBar {
    pub fn new() -> Self {
        Self {
            new_list_input: String::new(),
            is_adding_list: false,
        }
    }
}

impl Todo {
    pub fn lists_bar(&self) -> Element<Message> {
        let add_new: Element<_> = if self.lists_bar.is_adding_list {
            text_input("Add a todo list", &self.lists_bar.new_list_input)
                .on_input(|input| {
                    Message::ListsBar(ListsBarMessage::NewList(NewListMessage::Input(input)))
                })
                .on_submit(Message::ListsBar(ListsBarMessage::NewList(
                    NewListMessage::Submit,
                )))
                .width(Length::Fill)
                .style(styling::text_input::TextInput)
                .into()
        } else {
            Button::new(
                container(if self.todo_lists.is_empty() {
                    "Click me!"
                } else {
                    "+"
                })
                .width(Length::Fill)
                .center_x(),
            )
            .style(styling::button::Button::Rounded)
            .on_press(Message::ListsBar(ListsBarMessage::AddingList))
            .into()
        };

        let lists =
            Column::with_children(self.todo_lists.iter().enumerate().map(|(index, list)| {
                if Some(index) == self.current_list {
                    list.view_bar_current(list.is_editing, index)
                        .map(Message::ListsBar)
                } else {
                    list.view_bar(index).map(Message::ListsBar)
                }
            }));

        container(
            container(scrollable(
                column![add_new, lists]
                    .padding(15)
                    .spacing(15)
                    .width(SIDEBAR_WIDTH),
            ))
            .style(styling::container::Container::ListsBar)
            .height(Length::Fill),
        )
        .padding(10)
        .into()
    }
}

impl TodoList {
    pub fn view_bar_current(&self, is_editing: bool, index: usize) -> Element<ListsBarMessage> {
        if is_editing {
            self.view_edit()
                .map(move |message| ListsBarMessage::Edit(index, message))
        } else {
            self.view_regular()
                .map(move |message| ListsBarMessage::Regular(index, message))
        }
    }

    pub fn view_bar(&self, index: usize) -> Element<ListsBarMessage> {
        button(&*self.name)
            .on_press(ListsBarMessage::Select(index))
            .style(ButtonTheme::Text)
            .into()
    }

    fn view_regular(&self) -> Element<RegularMessage> {
        let name = button(&*self.name).style(styling::button::Button::Text);
        let edit_button = button(IconType::Edit.get_text())
            .on_press(RegularMessage::StartEdit)
            .style(styling::button::Button::TextSecondary);

        row![name, horizontal_space(), edit_button]
            .align_items(Alignment::Center)
            .into()
    }

    fn view_edit(&self) -> Element<EditMessage> {
        let name_edit = text_input("", &self.name)
            .on_input(EditMessage::Name)
            .on_submit(EditMessage::Done)
            .style(styling::text_input::TextInput);
        let delete = button(IconType::Delete.get_text())
            .on_press(EditMessage::Delete)
            .style(styling::button::Button::DangerText);

        row![name_edit, delete].into()
    }
}
