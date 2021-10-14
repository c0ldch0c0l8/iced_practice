use iced::{
    Sandbox, Image, Column, Text, 
    text_input, TextInput, Row, button, Button 
};

#[derive(Default)]
pub struct ImageApp {
    text_input_value: String,
    image_path: String,


    button_state: button::State,
    path_state: text_input::State
}

#[derive(Debug, Clone)]
pub enum Message {
    ShowImage,
    ImagePathChanged(String)
}

impl Sandbox for ImageApp {
    type Message = Message;

    fn new() -> Self {
        let path = String::from("A:\\Users\\c0ldch0c0l8\\Programming\\rust\\iced_practice\\resources\\images\\iced.png");

        ImageApp {
            text_input_value: path.clone(),
            image_path: path,
            
            ..Default::default()
        }
    }

    fn title(&self) -> String {
        String::from("Image App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .padding([20, 100])
        .push(
            Row::new()
            .push(
                Text::new("Enter the path for the image you want to display and press show image")
                .size(40)
            )
            .push(
                Button::new(
                    &mut self.button_state,
                    Text::new("show image")
                )
                .on_press(Message::ShowImage)
            )
        )
        .push(
            TextInput::new(
                &mut self.path_state,
                "path",
                &self.text_input_value,
                Message::ImagePathChanged
            )
        )
        .push(
            Image::new(&self.image_path)
            .width(iced::Length::Units(512))
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ShowImage => {
                self.image_path = self.text_input_value.clone();
            },
            Message::ImagePathChanged(path) => {
                self.text_input_value = path;
            }
        }
    }
}