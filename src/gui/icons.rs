use std::collections::HashMap;

use iced::{
    widget::{text, Text},
    Font,
};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum IconType {
    Edit,
    Delete,
}

impl IconType {
    pub fn get_char(&self) -> char {
        let icons: HashMap<IconType, char> =
            HashMap::from([(IconType::Edit, '\u{E801}'), (IconType::Delete, '\u{E800}')]);

        *icons
            .get(self)
            .unwrap_or_else(|| panic!("IconType: {:?} does not have coresponding character", &self))
    }

    pub fn get_text(&self) -> Text {
        text(self.get_char()).font(Font::with_name("todo-icons"))
    }
}
