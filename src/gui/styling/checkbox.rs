use iced::{widget::checkbox, Border};

use super::{colors, no_border, ROUNDING};

pub struct Checkbox;

impl From<Checkbox> for iced::theme::Checkbox {
    fn from(val: Checkbox) -> Self {
        iced::theme::Checkbox::Custom(Box::new(val))
    }
}

impl checkbox::StyleSheet for Checkbox {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        let _palette = style.palette();

        if is_checked {
            checkbox::Appearance {
                background: iced::Background::Color(colors::accents::primary()),
                icon_color: colors::text::black(),
                border: no_border(ROUNDING),
                text_color: None,
            }
        } else {
            checkbox::Appearance {
                background: iced::Background::Color(colors::accents::bg()),
                icon_color: colors::no_color(),
                border: Border {
                    color: colors::text::primary(),
                    width: 2.0,
                    radius: ROUNDING.into(),
                },
                text_color: None,
            }
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        if is_checked {
            self.active(style, is_checked)
        } else {
            checkbox::Appearance {
                background: iced::Background::Color(colors::accents::bg()),
                icon_color: colors::no_color(),
                border: Border {
                    color: colors::accents::primary(),
                    width: 2.0,
                    radius: ROUNDING.into(),
                },
                text_color: None,
            }
        }
    }

    fn disabled(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        let active = self.active(style, is_checked);

        checkbox::Appearance {
            background: match active.background {
                iced::Background::Color(color) => iced::Background::Color(iced::Color {
                    a: color.a * 0.5,
                    ..color
                }),
                iced::Background::Gradient(gradient) => {
                    iced::Background::Gradient(gradient.mul_alpha(0.5))
                }
            },
            ..active
        }
    }
}
