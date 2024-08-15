use iced::{
    border::Radius,
    widget::container::{self},
    Background, Border,
};

use crate::gui::styling::colors::{self, container::with_background};

use super::ROUNDING;

#[derive(Default)]
pub enum Container {
    #[default]
    ListsBar,

    CurrentItem,
}

impl From<Container> for iced::theme::Container {
    fn from(val: Container) -> Self {
        iced::theme::Container::Custom(Box::new(val))
    }
}

impl container::StyleSheet for Container {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let _palette = style.palette();

        match self {
            Container::ListsBar => container::Appearance {
                background: Some(Background::Color(with_background::background())),
                border: Border {
                    color: colors::container::with_background::border(),
                    radius: Radius::from(ROUNDING),
                    ..Default::default()
                },
                ..Default::default()
            },
            Container::CurrentItem => container::Appearance {
                background: Some(Background::Color(with_background::midground())),
                border: Border {
                    color: colors::container::with_background::border(),
                    radius: Radius::from(ROUNDING),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
