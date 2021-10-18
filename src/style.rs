use iced::{Column, Container, Radio, Sandbox, Text, container, radio};

pub struct StyleApp {
    dark_mode: bool,

    light_theme: Theme,
    dark_theme: Theme,
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

            light_theme: Theme::new(false),
            dark_theme: Theme::new(true),
        }
    }

    fn title(&self) -> String {
        String::from("Style App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let current_theme = Theme::new(self.dark_mode);

        let content = Column::new()
        .push(
            Text::new("Choose a Theme")
            .size(50)
            .color(current_theme.text_color)
        )
        .push(
            Radio::new(
                self.dark_mode,
                "light",
                Some(false),
                DarkMode::Off
            )
            .style(current_theme.radio.clone())
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
    dark_mode: bool,

    container: ContainerStyle,
    text_color: Color,
    radio: RadioStyle
}

impl Theme {
    fn new(dark_mode: bool) -> Theme {
        Theme {
            dark_mode,
            container: ContainerStyle::new(dark_mode),
            text_color: if dark_mode { Color::WHITE } else { Color::BLACK },
            radio: RadioStyle::new(dark_mode)
        }
    }

    fn is_dark_mode(&self) -> bool {
        self.dark_mode
    }

    fn set_mode(&mut self, dark_mode: bool) {
        *self = Theme::new(dark_mode);
    }
}

struct ContainerStyle {
    dark_mode: bool
}

impl ContainerStyle {
    fn new(dark_mode: bool) -> ContainerStyle {
        ContainerStyle {
            dark_mode
        }
    }
}

impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
        if self.dark_mode {
            container::Style {
                background: Some(Background::Color(Color::BLACK)),
                text_color: Some(Color::WHITE),
                border_color: Color::WHITE,
                border_radius: 0.0,
                border_width: 0.0
            }
        } else {
            container::Style {
                background: Some(Background::Color(Color::WHITE)),
                text_color: Some(Color::BLACK),
                border_color: Color::BLACK,
                border_radius: 0.0,
                border_width: 0.0
            }
        }
    }
}


#[derive(Clone)]
struct RadioStyle {
    dark_mode: bool
}

impl RadioStyle {
    fn new(dark_mode: bool) -> RadioStyle {
        RadioStyle {
            dark_mode
        }
    }
}

impl radio::StyleSheet for RadioStyle {
    fn active(&self) -> radio::Style {
        if self.dark_mode {
            radio::Style {
                background: Background::Color(Color::BLACK),
                dot_color: Color::WHITE,
                border_color: Color::WHITE,
                border_width: 0.0
            }
        } else {
            radio::Style {
                background: Background::Color(Color::WHITE),
                dot_color: Color::BLACK,
                border_color: Color::BLACK,
                border_width: 0.0
            }
        }
    }

    fn hovered(&self) -> radio::Style {
        if self.dark_mode {
            radio::Style {
                background: Background::Color(Color::BLACK),
                dot_color: Color::TRANSPARENT,
                border_color: Color::WHITE,
                border_width: 0.0
            }
        } else {
            radio::Style {
                background: Background::Color(Color::WHITE),
                dot_color: Color::TRANSPARENT,
                border_color: Color::BLACK,
                border_width: 0.0
            }
        }
    }
}

