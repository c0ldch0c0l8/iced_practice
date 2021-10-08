use iced::{Sandbox, Align, Column, Row, Text, TextInput, 
    text_input, Button, button};

#[derive(Default)]
pub struct State {
    text_edited: text_input::State,
    text_value: String,
    add_note_button_pressed: button::State,
    remove_note_buttons_pressed: Vec<button::State>
}

pub struct Notes {
    notes: Vec<String>,
    state: State
}

#[derive(Debug, Clone)]
pub enum Message {
    TextEdited(String),
    NoteAdded,
    NoteRemoved(usize)
}

impl Sandbox for Notes {
    type Message = Message;
    
    fn new() -> Self {
        Notes {
            notes: Vec::<String>::new(),
            state: State::default()
        }
    }

    fn title(&self) -> String {
        String::from("Notes")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        
        let mut column = Column::new()
        .align_items(Align::Center)
        .padding(20)
        .push(
            Row::new()
            .padding(50)
            .push(
                TextInput::new(
                    &mut self.state.text_edited,
                    "Enter note here",
                    &self.state.text_value,
                    Message::TextEdited
                )
                .on_submit(Message::NoteAdded)
            )
            .push(
                Button::new(
                    &mut self.state.add_note_button_pressed,
                    Text::new("+")
                )
                .on_press(Message::NoteAdded)
            )
        );

        // holy shit this method is a nice just works (tm) solution.
        let mut remove_note_buttons_pressed_iter = self.state.remove_note_buttons_pressed.iter_mut().rev();

        for (index, note) in self.notes.iter().enumerate().rev() {
            column = column.push(
                Row::new()
                .push(
                    Text::new(note)
                )
                .push(
                    Button::new(
                        remove_note_buttons_pressed_iter.next().unwrap(),
                        Text::new("-")
                    )
                    .width(iced::Length::Units(50))
                    .height(iced::Length::Units(50))
                    .on_press(Message::NoteRemoved(index))
                )
            )
        }

        column.into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextEdited(text) => {
                self.state.text_value = text;
            },
            Message::NoteAdded => {
                self.notes.push(self.state.text_value.clone());
                self.state.remove_note_buttons_pressed.push(Default::default());
                self.state.text_value = "".to_string();
            },
            Message::NoteRemoved(index) => {
                self.notes.remove(index);
                self.state.remove_note_buttons_pressed.remove(index);
            }
        }
    }
    
}