use iced::{Checkbox, Column, Row, Sandbox, Text, 
    TextInput, Toggler, alignment, text_input};

#[derive(Default)]
pub struct ToggleCheckApp {
    input: String,
    shown_text: String,

    hide_caps: bool,
    hide_small: bool,
    display_text: bool,

    text_input_state: text_input::State
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    HideCaps(bool),
    HideSmall(bool),
    ShowText(bool)
}

impl Sandbox for ToggleCheckApp {
    type Message = Message;
    
    fn new() -> Self {
        Self {
            display_text: true,
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Toggle Checkbox App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        self.shown_text = "".to_string();

        for ch in self.input.chars() {
            if self.hide_caps && ch.is_uppercase() {
                continue;
            }

            if self.hide_small && ch.is_lowercase() {
                continue;
            }

            self.shown_text.push(ch);
        }

        let mut content = Column::new()
        .push(
            Text::new("Enter text and toggle and check like your heart desires")
            .size(30)
        )
        .push(
            TextInput::new(
                &mut self.text_input_state,
                "Enter text here...",
                &self.input,
                Message::InputChanged
            )
        )
        .push(
            Row::new()
            .push(
                Checkbox::new(
                    self.hide_caps,
                    "Hide caps",
                    Message::HideCaps
                )
            )
            .push(
                Checkbox::new(
                    self.hide_small,
                    "Hide small",
                    Message::HideSmall
                )
            )
            .push(
                Toggler::new(
                    self.display_text,
                    Some("show text".to_string()),
                    Message::ShowText
                )
                .text_alignment(
                    alignment::Horizontal::Right
                )
            )
        );

        if self.display_text {
            content = content.push(
                Text::new(&self.shown_text)
            );
        }

        content.into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(input) => {
                self.input = input;
            },
            Message::HideCaps(hide) => {
                self.hide_caps = hide;
            },
            Message::HideSmall(hide) => {
                self.hide_small = hide;
            },
            Message::ShowText(show) => {
                self.display_text = show;
            }
        }
    }
}