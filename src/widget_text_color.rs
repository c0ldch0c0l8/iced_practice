use iced::{Background, Checkbox, Color, Column, Sandbox, checkbox};


// This was a supposed test file to check if my impl of text_color for
// radio and checkbox worked. it did, but it turned out stuff was already
// implemented. so yeah

pub struct WidgetTextColor {
    is_checked: bool
}

#[derive(Debug, Clone)]
pub enum Message {
    Checked(bool)
}

impl Sandbox for WidgetTextColor {
    type Message = Message;

    fn new() -> Self {
        Self {
            is_checked: false
        }
    }

    fn title(&self) -> String {
        String::from("Checkbox Text Color App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .push(
            Checkbox::new(
                self.is_checked,
                "label",
                Message::Checked
            )
            .style(CheckboxStyle)
            .text_color(Color::from_rgb8(0, 0, 255))
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Checked(value) => {
                self.is_checked = value;

                println!("Checked: {}", self.is_checked)
            }
        }
    }
}

struct CheckboxStyle;

impl checkbox::StyleSheet for CheckboxStyle {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            checkmark_color: Color::from_rgba8(255, 0, 0, 1.0),
            background: Background::Color(Color::TRANSPARENT),
            border_color: Color::TRANSPARENT,
            border_radius: 0.0,
            border_width: 0.0,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(Color::from_rgba8(0, 255, 0, 1.0)),
            ..self.active(is_checked)
        }
    }
}