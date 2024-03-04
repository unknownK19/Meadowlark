// TODO Customize Layout

use iced::{
    theme::Palette,
    widget::button::{self, Appearance},
    Background, Border, Color, Theme,
};

pub struct DefaultButton(pub Palette);

impl button::StyleSheet for DefaultButton {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        Appearance {
            background: Some(Background::Color(Color::from_rgb(
                self.0.primary.r,
                self.0.primary.g,
                self.0.primary.b, //0.6, 0.6, 0.6,
            ))),
            border: Border::with_radius(5),
            ..Default::default()
        }
    }
    fn hovered(&self, style: &Self::Style) -> Appearance {
        button::Appearance {
            border: Border::with_radius(8),
            ..self.active(style)
        }
    }
    fn disabled(&self, style: &Self::Style) -> Appearance {
        button::Appearance {
            border: Border::with_radius(8),
            ..self.active(style)
        }
    }
    fn pressed(&self, style: &Self::Style) -> Appearance {
        button::Appearance {
            border: Border::with_radius(8),
            ..self.active(style)
        }
    }
}
