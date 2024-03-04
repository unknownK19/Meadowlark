mod tools; // TODO Every Event Action to System Reaction

use std::time::Duration;

use iced::theme;
use iced::widget::button;
use iced::{
    border::Radius,
    executor,
    font,
    //theme::Palette,
    widget::{
        button::{Appearance, StyleSheet},
        column, row,
    },
    window,
    Application,
    Border,
    Color,
    Command,
    Font,
    Length,
    Point,
    Settings,
    Theme,
};

struct Usernaut {
    // Every User is Astronaut! they want to Explore 😃
    //name: String,
    width: f32,
    animmode: Vec<bool>,
    thememode: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Animate,                           // Animation action
    Clickaction(u8),                   // click action
    Fontload(Result<(), font::Error>), // Font Load
}

fn main() -> iced::Result {
    Usernaut::run(Settings {
        window: window::Settings {
            size: iced::Size {
                width: 300.,
                height: 250.,
            },
            position: window::Position::Specific(Point { x: 0., y: 0. }),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        antialiasing: true,
        default_font: Font::with_name("DaddyTimeMono Nerd Font"),
        ..Default::default()
    })
}

impl Application for Usernaut {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                //name: "user".to_string(),
                width: 55.,
                animmode: vec![false],
                thememode: Theme::CatppuccinFrappe,
            },
            Command::batch(vec![font::load(
                include_bytes!("../Assets/DaddyTimeMonoNerdFont-Regular.ttf").as_slice(),
            )
            .map(Message::Fontload)]),
        )
    }
    fn title(&self) -> String {
        "MarkDown Carbide".to_string()
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Animate => {
                if self.animmode[0] {
                    if self.width <= 80. {
                        self.width += (80. / self.width).sin();
                    } else {
                    }
                } else {
                    if self.width >= 55. {
                        self.width -= (55. / self.width).sin();
                    } else {
                    }
                }
            }
            Message::Clickaction(1) => {
                if self.animmode[0] {
                    self.animmode[0] = false
                } else {
                    self.animmode[0] = true
                }
            }
            _ => {}
        }
        Command::none()
    }
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(Duration::from_millis(16)).map(|_| Message::Animate)
    }
    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        // What we view.
        column![
            // top bar
            row![
                //action bar
                row![
                    // File
                    button("File")
                        .width(Length::Fixed(self.width))
                        .on_press(Message::Clickaction(1))
                        .style(theme::Button::Destructive),
                    // Edit
                    button("Edit").width(Length::Fill)
                ]
                .padding(10)
                .align_items(iced::Alignment::Center)
            ],
        ]
        .into()
    }
    // Theme
    fn theme(&self) -> Self::Theme {
        // TODO Add Light theme as Defaut
        // But currently we use Dark theme
        self.thememode.clone()

        // Theme::custom(
        //     "Rose-pine".to_string(), // Theme Name
        //     Palette {
        //         // Color Ingredient
        //         background: Color::from_rgb8(25, 23, 36),
        //         text: Color::from_rgb8(224, 222, 244),
        //         primary: Color::from_rgb8(156, 207, 216),
        //         success: Color::from_rgb8(196, 167, 231),
        //         danger: Color::from_rgb8(246, 193, 119),
        //     },
        // )
    }
}

// TODO Customize Widget
struct Buttonstyle;

impl StyleSheet for Buttonstyle {
    type Style = ();
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        Appearance {
            border: Border {
                color: Color::from_rgb8(5, 222, 222),
                width: 2.0,
                radius: Radius::default(),
            },
            ..Default::default()
        }
    }
}
