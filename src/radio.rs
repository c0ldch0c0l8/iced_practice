use iced::{Sandbox, Radio, Column};

#[derive(Default)]
pub struct RadioApp {
    radio_val: i32
}

// make an enum for each choice, and then send it 
// (specific enum) + value of choice chosen previously. 
#[derive(Debug, Clone)]
pub enum Message {
    Radio1(i32),
    Radio2(i32),
    Radio2Extra(i32)
}

impl Sandbox for RadioApp {
    type Message = Message;

    fn new() -> Self {
        RadioApp::default()
    }

    fn title(&self) -> String {
        String::from("Radio App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .push(
            Radio::new(
                self.radio_val, 
                "radio label 1", 
                Some(1), 
                Message::Radio1
            )
        )
        .push(
            Radio::new(
                self.radio_val, 
                "radio label 2", 
                Some(2),
                Message::Radio2
            )
        )
        .push(
            Radio::new(
                self.radio_val, 
                "radio label 2 extra (both of the '2' radios will be activated, \
                but you can just toggle one of the 2 'selected' params off)", 
                Some(2),
                Message::Radio2Extra
            )
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Radio1(value) => {
                if value != 1 {
                    println!("value wasnt previously 1, act accordingly");
                }

                self.radio_val = 1;
            },
            Message::Radio2(value) => {
                if value != 2 {
                    println!("value wasnt previously 2, act accordingly");
                }

                self.radio_val = 2;
            },
            Message::Radio2Extra(value) => {
                println!("Do something different here but keep value 2");
                if value == 1 {
                    // thingy 1
                } else {
                    // thingy 2
                }

                self.radio_val = 2;
            }
        }
    }
}