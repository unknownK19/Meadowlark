mod tools; // TODO Every Event Action to System Reaction

use std::time::Duration;

use iced::theme;
use iced::widget::button;
use iced::{
    executor,
    font,
    //theme::Palette,
    widget::{column, row, svg},
    window,
    Application,
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
    ChangeTheme(Theme),
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
                include_bytes!("../Assets/Font//DaddyTimeMonoNerdFont-Regular.ttf").as_slice(),
            )
            .map(Message::Fontload)]),
        )
    }
    fn title(&self) -> String {
        "Meadowlark".to_string()
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
            Message::ChangeTheme(x) => self.thememode = x,
            _ => {}
        }
        Command::none()
    }
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(Duration::from_millis(16)).map(|_| Message::Animate)
    }
    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        // What we view. currently this is just test.
        let file_open = svg(svg::Handle::from_path(format!(
            "{}/Assets/theme/icons/Default-Light/file_open.svg",
            env!("CARGO_MANIFEST_DIR")
        )))
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(theme::Svg::Default);

        let file_save = svg(svg::Handle::from_path(format!(
            "{}/Assets/theme/icons/Default-Light/file_save.svg",
            env!("CARGO_MANIFEST_DIR")
        )))
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(theme::Svg::Default);

        column![
            // top bar
            row![
                //action bar
                row![
                    // Width
                    button("Width")
                        .width(Length::Fixed(self.width))
                        .on_press(Message::Clickaction(1)),
                    // Nord Theme
                    button("Nord")
                        .width(Length::Shrink)
                        .on_press(Message::ChangeTheme(Theme::Nord)),
                    // TokyoNight Theme
                    button("TokyoNight")
                        .width(Length::Shrink)
                        .on_press(Message::ChangeTheme(Theme::TokyoNight)),
                ]
                .spacing(4)
                .padding(10)
                .align_items(iced::Alignment::Center)
            ],
            row![file_open, file_save] // Alternately add list view

                                       // Add Pane Grid
                                       //PaneGrid::new(&state, view)
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
