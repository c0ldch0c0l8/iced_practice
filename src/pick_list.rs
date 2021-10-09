use iced::{Sandbox, pick_list, Column};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Choice1,
    Choice2,
    Choice3
}

impl Choice {
    const ALL: [Choice; 3] = [
        Choice::Choice1,
        Choice::Choice2,
        Choice::Choice3
    ];
}

impl Default for Choice {
    fn default() -> Self {
        Choice::Choice1
    }
}

impl std::fmt::Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Choice::Choice1 => {
                write!(f, "Choice1")
            },
            Choice::Choice2 => {
                write!(f, "Choice2")
            },
            Choice::Choice3 => {
                write!(f, "Choice3")
            },
        }
    }
}


#[derive(Debug)]
pub enum Message {
    Selected(Choice)
}

#[derive(Default)]
pub struct PickList {
    selected: Choice,
    pick_list_state: pick_list::State<Choice>
}


impl Sandbox for PickList {
    type Message = Message;

    fn new() -> Self {
        PickList::default()
    }

    fn title(&self) -> String {
        "Pick List".to_string()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .push(
            iced::PickList::new(
                &mut self.pick_list_state,
                &Choice::ALL[..],
                Some(self.selected),
                Message::Selected
            )
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Selected(choice) => {
                self.selected = choice;
            }
        }
    }
} 