use std::collections::HashMap;

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
            .get(self)
            .unwrap_or_else(|| panic!("IconType: {:?} does not have coresponding character", &self))
    }
}
