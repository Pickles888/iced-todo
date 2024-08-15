use iced::{widget::button, Background};

use crate::gui::styling::colors;

use super::{no_border, ROUNDING};

#[derive(Default)]
pub enum Button {
    #[default]
    Rounded,

    Text,
    DangerText,
    TextSelected,
    TextSecondary,
}

impl From<Button> for iced::theme::Button {
    fn from(val: Button) -> Self {
        iced::theme::Button::Custom(Box::new(val))
    }
}

impl button::StyleSheet for Button {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        match self {
            Button::Rounded => button::Appearance {
                background: Some(Background::Color(colors::accents::primary())),
                text_color: colors::text::black(),
                border: no_border(ROUNDING),
                ..Default::default()
            },
            Button::Text => button::Appearance {
                background: None,
                text_color: colors::accents::primary(),
                border: no_border(0),
                ..Default::default()
            },
            Button::TextSecondary => button::Appearance {
                background: None,
                text_color: colors::accents::secondary(),
                border: no_border(0),
                ..Default::default()
            },
            Button::TextSelected => button::Appearance {
                background: Some(Background::Color(
                    colors::container::with_background::midground(),
                )),
                text_color: colors::accents::primary(),
                border: no_border(ROUNDING),
                ..Default::default()
            },
            Button::DangerText => button::Appearance {
                background: None,
                text_color: colors::accents::danger(),
                border: no_border(0),
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: active.shadow_offset + iced::Vector::new(0.0, 1.0),
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        match self {
            Self::Text => active,
            _ => button::Appearance {
                shadow_offset: iced::Vector::default(),
                background: active.background.map(|background| match background {
                    iced::Background::Color(color) => iced::Background::Color(iced::Color {
                        a: color.a * 0.5,
                        ..color
                    }),
                    iced::Background::Gradient(gradient) => {
                        iced::Background::Gradient(gradient.mul_alpha(0.5))
                    }
                }),
                text_color: iced::Color {
                    a: active.text_color.a * 0.5,
                    ..active.text_color
                },
                ..active
            },
        }
    }
}
