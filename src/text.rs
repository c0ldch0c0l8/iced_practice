use iced::{Sandbox, Column, Text, };

#[derive(Default)]
pub struct TextApp {

}

#[derive(Debug)]
pub enum Message {
    
}

impl Sandbox for TextApp {
    type Message = Message;

    fn new() -> Self {
        TextApp::default()
    }

    fn title(&self) -> String {
        String::from("Text App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        
        Column::new()
        .push(
            Text::new("Large Text")
            .size(100)
        )
        .push(
            Text::new("Medium Text")
            .size(50)
        )
        .push(
            Text::new("Small Text")
            .size(25)
        )
        .push(
            Text::new("Some Text")
            .size(50)
            .color(iced::Color::new(1.0, 0.941176, 0.2701, 1.0))
            .font(JETBRAINSMONO_EXTRABOLD_ITALIC)
        )   
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {

        }
    }
}


const JETBRAINSMONO_EXTRABOLD_ITALIC: iced::Font = iced::Font::External {
    name: "JetbrainsMono ExtraBold Italic",
    bytes: include_bytes!("A:\\Users\\c0ldch0c0l8\\Programming\\rust\\iced_practice\\resources\\JetBrainsMono\\JetBrainsMono-ExtraBoldItalic.ttf")
};