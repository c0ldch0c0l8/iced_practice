use iced::{Align, Button, Column, Sandbox, Text, button};

#[derive(Default)]
pub struct Counter {
    value: i32,

    increment_pressed: button::State,
    decrement_pressed: button::State
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed
}


impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .padding(20)
        .align_items(Align::Center)
        .push(
            Button::new(
                &mut self.increment_pressed,
                Text::new("+").size(50)
            ).on_press(Message::IncrementPressed)
        )
        .push(
            Text::new(self.value.to_string()).size(100)
        )
        .push(
            Button::new(
                &mut self.decrement_pressed,
                Text::new("-").size(50)
            ).on_press(Message::DecrementPressed)
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            },
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}
