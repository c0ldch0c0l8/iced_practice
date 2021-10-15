use iced::{Sandbox, Space, Column, Row, Text, Length::Units};

#[derive(Default)]
pub struct SpaceApp;

impl Sandbox for SpaceApp {
    type Message = ();

    fn new() -> Self {
        SpaceApp::default()
    }

    fn title(&self) -> String {
        String::from("Space (as used in layouting) App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .push(
            Row::new()
            .push(
                Space::new(Units(512), Units(4))
            )
            .push(
                Text::new("Heyy right")
            )
        )
        .push(
            Space::new(Units(4), Units(512))
        )
        .push(
            Text::new("Heyy Down")
        )
        .push(
            Text::new("Heyy Down")
        )
        .into()
    }

    fn update(&mut self, _message: Self::Message) {
        
    }
}