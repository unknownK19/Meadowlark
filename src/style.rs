// TODO Customize Layout

use iced::{
    border::Radius,
    theme::Palette,
    widget::button::{self, Appearance},
    Background, Border, Color, Theme,
};

use crate::Message;

pub struct DefaultButton(pub Palette, pub Message, pub Colorkey);

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
        Message::Hoveraction(1);
        // println!("Hovered me");
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

#[allow(dead_code)]
/// initial <-> now <-> final
#[derive(Clone, Copy)]
pub enum Colorkey {
    R(f32, f32, f32),
    G(f32, f32, f32),
    B(f32, f32, f32),
    A(f32, f32, f32),
}

#[allow(dead_code)]
/// initial <-> now <-> final
#[derive(Clone, Copy)]
pub enum Dimensionkey {
    Height(f32, f32, f32),
    Width(f32, f32, f32),
    BRadius(f32, f32, f32),
}

#[allow(dead_code)]
pub struct AnimationStyle {
    pub color: Vec<Colorkey>,
    pub dimension: Vec<Dimensionkey>,
}
impl AnimationStyle {
    pub fn get_width(&self, index: usize) -> f32 {
        match self.dimension[index] {
            Dimensionkey::Width(_, n, _) => n,
            _ => 0.0,
        }
    }
    pub fn get_color(&self, index: usize) -> Colorkey {
        match self.color[index] {
            Colorkey::A(x, y, z) => Colorkey::A(x, y, z),
            Colorkey::R(x, y, z) => Colorkey::R(x, y, z),
            Colorkey::G(x, y, z) => Colorkey::G(x, y, z),
            Colorkey::B(x, y, z) => Colorkey::B(x, y, z),
        }
    }
}
