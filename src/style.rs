// TODO Customize More Layout

use iced::{
    border::Radius,
    theme::Palette,
    widget::button::{self, Appearance},
    Background, Border, Color, Theme,
};

use crate::Message;

pub struct DefaultButton(pub Palette, pub Message);

impl button::StyleSheet for DefaultButton {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        Appearance {
            background: Some(Background::Color(Color::from_rgb(
                self.0.primary.r,
                self.0.primary.g,
                self.0.primary.b,
            ))),
            border: Border::with_radius(10),
            ..Default::default()
        }
    }
    fn hovered(&self, style: &Self::Style) -> Appearance {
        Message::Clickaction(2);
        println!("Hovered me");
        button::Appearance {
            background: Some(Background::Color(Color {
                a: 1.0,
                ..self.0.background
            })),
            border: Border {
                color: self.0.primary,
                width: 2.0,
                radius: Radius::from(10.0),
            },
            text_color: Color { ..self.0.primary },
            ..self.active(style)
        }
    }
    fn disabled(&self, style: &Self::Style) -> Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }
    fn pressed(&self, style: &Self::Style) -> Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }
}
