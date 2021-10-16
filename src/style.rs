use iced::{Column, Container, Radio, Sandbox, Text, container, radio};

// #[derive(Default)]
pub struct StyleApp {
    dark_mode: bool,

    light_theme: Theme,
    dark_theme: Theme,
}

impl StyleApp {
    fn get_current_theme(&self) -> &Theme {
        match self.dark_mode {
            false => {
                &self.light_theme
            },
            true => {
                &self.dark_theme
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum DarkMode {
    Off(bool),
    On(bool)
}

impl Sandbox for StyleApp {
    type Message = DarkMode;

    fn new() -> Self {
        Self {
            dark_mode: false,

            light_theme: Theme::light(),
            dark_theme: Theme::dark(),
        }
    }

    fn title(&self) -> String {
        String::from("Style App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let content = Column::new()
        .push(
            Text::new("Choose a Theme")
            .size(50)
            .color(self.get_current_theme().text_color)
        )
        .push(
            Radio::new(
                self.dark_mode,
                "light",
                Some(false),
                DarkMode::Off
            )
            .style(self.get_current_theme().radio)
        )
        .push(
            Radio::new(
                self.dark_mode,
                "dark".to_string(),
                Some(true),
                DarkMode::On
            )
            .style(current_theme.radio)
        );

        Container::new(content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x()
        .center_y()
        .style(current_theme.container)
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            // I dont really need to know whether darkmode was previously on
            // also it appears in the styling example that there exists a 
            // RadioChanged(value) enum element... 
            DarkMode::Off(_on) => {
                self.dark_mode = false;
            },
            DarkMode::On(_on) => {
                self.dark_mode = true;
            }
        }
    }
}


use iced::{Background, Color};

struct Theme {
    container: container::Style,
    text_color: Color,
    radio: radio::Style
}

impl Theme {
    fn light() -> Self {
        Self {
            container: container::Style {
                background: Some(Background::Color(Color::WHITE)),
                text_color: Some(Color::BLACK),
                border_color: Color::BLACK,
                border_radius: 0.0,
                border_width: 0.0
            },
            text_color: Color::BLACK,
            radio: radio::Style {
                background: Background::Color(Color::WHITE),
                dot_color: Color::BLACK,
                border_color: Color::BLACK,
                border_width: 0.0
            }
        }
    }

    fn dark() -> Self {
        Self {
            container: container::Style {
                background: Some(Background::Color(Color::BLACK)),
                text_color: Some(Color::WHITE),
                border_color: Color::WHITE,
                border_radius: 0.0,
                border_width: 0.0
            },
            text_color: Color::WHITE,
            radio: radio::Style {
                background: Background::Color(Color::BLACK),
                dot_color: Color::WHITE,
                border_color: Color::WHITE,
                border_width: 0.0
            }
        }
    }
}

