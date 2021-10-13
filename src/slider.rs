use iced::{Sandbox, Column, Slider, slider, Text};

#[derive(Default)]
pub struct SliderApp {
    value: i32,
    
    state: slider::State
}

#[derive(Debug, Clone)]
pub enum Message {
    SliderValueChanged(i32)
} 

impl Sandbox for SliderApp {
    type Message = Message;

    fn new() -> Self {
        SliderApp::default()
    }

    fn title(&self) -> String {
        String::from("Slider App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .padding([20, 300])
        .push(
            Text::new("How old are you?")
            .size(70)
        )
        .push(
            Slider::new(
                &mut self.state,
                0..=100,
                self.value,
                Message::SliderValueChanged
            )
        )
        .push(
            Text::new(format!("You are {} years old!", self.value))
            .size(70)
        )
        .push(
            Text::new("This text's size is controlled by the slider!")
            .size(self.value as u16) // value is always + and in range of u16 bounds
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SliderValueChanged(value) => {
                self.value = value;
            }
        }
    }
}