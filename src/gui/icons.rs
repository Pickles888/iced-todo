use std::collections::HashMap;

pub struct Icon {
    char: char,
    icon: IconType,
}

impl Icon {
    pub fn new(icon_type: IconType) -> Self {
        Icon {
            char: icon_type.get_char(),
            icon: icon_type,
        }
    }

    pub fn get(&self) -> char {
        self.char
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum IconType {
    Star,
    StarEmpty,
}

impl IconType {
    fn get_char(&self) -> char {
        let icons: HashMap<IconType, char> = HashMap::from([
            (IconType::Star, '\u{E800}'),
            (IconType::StarEmpty, '\u{E801}'),
        ]);

        *icons
            .get(&self)
            .expect(format!("IconType: {:?} does not have coresponding character", &self).as_str())
    }
}
