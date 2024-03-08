mod style;
mod tools; // TODO Every Event Action to System Reaction

use std::f32::consts::PI;
use std::time::Duration;
use style::{AnimationStyle, Colorkey, DefaultButton, Dimensionkey};

use iced::theme;
use iced::widget::Button;
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

//#![allow(unused_variables)]
// #[allow(dead_code)]
// const ANIMATIONSPEED: f32 = 2.0; //unit second

pub struct Usernaut {
    // Every User is Astronaut! they want to Explore 😃
    //name: String,
    animatestyle: AnimationStyle,
    animmode: Vec<bool>,
    thememode: Theme,
}

/// Linear => slope tan f32 , delta change f32
#[allow(dead_code)]
#[derive(PartialEq)]
enum ChangeFn {
    Linear(f32, f32),
    // TODO Add another two function
    // Quadratic
    // Sine
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl ChangeFn {
    pub fn change(self, mut x: f32, y: f32) -> f32 {
        match self {
            Self::Linear(theta, delta) => {
                if theta != 0.0
                    && (x - y).abs() >= x * delta * theta.atan()
                    && x != y
                    && theta != PI / 2.0
                {
                    x = x + theta.tan() * delta;
                }
            }
        }
        x
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Animate,                           // Animation action
    Clickaction(usize),                // click action
    Fontload(Result<(), font::Error>), // Font Load
    //ChangeTheme(Theme),                //TODO: Theme changeble as settings (user config)
    Hoveraction(usize), // Hoveraction
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
        default_font: Font::with_name("Nova Square"),
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
                animatestyle: AnimationStyle {
                    color: [Colorkey::A(1.0, 1.0, 0.0); 4].to_vec(),
                    dimension: [Dimensionkey::Width(70.0, 70.0, 90.0); 4].to_vec(),
                },
                animmode: [false; 4].to_vec(),
                thememode: Theme::CatppuccinFrappe,
            },
            Command::batch(vec![
                font::load(
                    include_bytes!("../Assets/Font/DaddyTimeMonoNerdFont-Regular.ttf").as_slice(),
                )
                .map(Message::Fontload),
                font::load(include_bytes!("../Assets/Font/NovaSquare-Regular.ttf").as_slice())
                    .map(Message::Fontload),
            ]),
        )
    }
    fn title(&self) -> String {
        "Meadowlark".to_string()
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Animate => {
                for index in 0..=2 {
                    //let index = 0;
                    if let Colorkey::A(i, ref mut n, f) = self.animatestyle.color[index] {
                        match self.animmode[index] {
                            true => *n = ChangeFn::Linear(PI / 2.2, 0.1).change(*n, f),
                            false => *n = ChangeFn::Linear(PI / 2.2, 0.1).change(*n, i),
                        }
                    }

                    if let Dimensionkey::Width(i, ref mut n, f) = self.animatestyle.dimension[index]
                    {
                        match self.animmode[index] {
                            true => *n = ChangeFn::Linear(PI / 2.2, 0.1).change(*n, f),
                            false => *n = ChangeFn::Linear(-PI / 2.2, 0.1).change(*n, i),
                        }
                    }
                }
            }

            Message::Clickaction(i) => {
                if self.animmode[i - 1] {
                    self.animmode[i - 1] = false
                } else {
                    self.animmode[i - 1] = true
                }
                match i {
                    2 => self.thememode = Theme::Nord,
                    3 => self.thememode = Theme::TokyoNight,
                    _ => {}
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

        let button01 = |x| {
            Button::new(x)
                .width(Length::Shrink)
                .style(theme::Button::Custom(Box::new(DefaultButton(
                    self.thememode.palette(),
                    Message::Clickaction(1),
                    self.animatestyle.get_color(0),
                ))))
                .padding(10.0)
        };

        column![
            // top bar
            row![
                //action bar
                row![
                    // Width
                    button01("Width")
                        .width(Length::Fixed(self.animatestyle.get_width(0)))
                        .on_press(Message::Clickaction(1)),
                    // Nord Theme
                    button01("Nord")
                        .width(Length::Fixed(self.animatestyle.get_width(1)))
                        .on_press(Message::Clickaction(2)),
                    // TokyoNight Theme
                    button01("TN")
                        .width(Length::Fixed(self.animatestyle.get_width(2)))
                        .on_press(Message::Clickaction(3)),
                ]
                .spacing(4)
                .padding(10)
                .align_items(iced::Alignment::Center)
            ],
            row![file_open, file_save] // TODO Add Pane Grid
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
