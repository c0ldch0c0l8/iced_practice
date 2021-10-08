use iced::{Sandbox, Text};

pub struct Hello;

impl Sandbox for Hello {
    type Message = ();

    fn new() -> Self {
        Hello
    }

    fn title(&self) -> String {
        String::from("Hello World!")
    }

    fn update(&mut self, _message: Self::Message) {
        
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Text::new("Hello World!").into()
    }
}
