use iced::{border::Radius, Border, Shadow, Vector};

use super::colors;

fn no_shadow() -> Shadow {
    Shadow {
        color: colors::no_color(),
        offset: Vector { x: 0.0, y: 0.0 },
        blur_radius: 0.0,
    }
}

fn no_offset() -> Vector {
    Vector { x: 0.0, y: 0.0 }
}

fn no_border(radius: i32) -> Border {
    Border {
        color: colors::no_color(),
        width: 0.0,
        radius: Radius::from(radius),
    }
}

pub const ROUNDING: i32 = 20;

pub mod container {
    use iced::{
        border::Radius,
        widget::container::{self},
        Background, Border,
    };

    use crate::gui::colors::{self, container::with_background};

    use super::ROUNDING;

    #[derive(Default)]
    pub struct Container;

    impl From<Container> for iced::theme::Container {
        fn from(val: Container) -> Self {
            iced::theme::Container::Custom(Box::new(val))
        }
    }

    impl container::StyleSheet for Container {
        type Style = iced::Theme;

        fn appearance(&self, style: &Self::Style) -> container::Appearance {
            let _palette = style.palette();

            container::Appearance {
                background: Some(Background::Color(with_background::background())),
                border: Border {
                    color: colors::container::with_background::border(),
                    radius: Radius::from(ROUNDING),
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }
}

pub mod button {
    use iced::{widget::button, Background};

    use crate::gui::colors;

    use super::{no_border, ROUNDING};

    #[derive(Default)]
    pub enum Button {
        #[default]
        Rounded,

        Text,
        DangerText,
        TextSelected,
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

            button::Appearance {
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
            }
        }
    }
}
