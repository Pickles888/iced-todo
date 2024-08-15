use iced::{border::Radius, widget::text_input, Background, Border, Theme};

use crate::gui::styling;

use super::{no_border, ROUNDING};

pub struct TextInput;

impl From<TextInput> for iced::theme::TextInput {
    fn from(val: TextInput) -> Self {
        iced::theme::TextInput::Custom(Box::new(val))
    }
}

impl text_input::StyleSheet for TextInput {
    type Style = Theme;

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let _palette = style.palette();

        text_input::Appearance {
            background: Background::Color(styling::colors::accents::bg()),
            border: no_border(ROUNDING),
            icon_color: styling::colors::accents::primary(),
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let _palette = style.palette();

        text_input::Appearance {
            background: Background::Color(styling::colors::accents::bg()),
            border: Border {
                color: styling::colors::accents::primary(),
                width: 2.0,
                radius: Radius::from(ROUNDING),
            },
            icon_color: styling::colors::accents::primary(),
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        styling::colors::accents::bg()
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        styling::colors::text::primary()
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        styling::colors::accents::bg2()
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        styling::colors::accents::primary()
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(styling::colors::accents::bg2()),
            border: Border {
                color: styling::colors::accents::danger(),
                width: 2.0,
                radius: Radius::from(ROUNDING),
            },
            icon_color: styling::colors::accents::primary(),
        }
    }
}
