use iced::{Application, executor, Command, Clipboard, Align, Text,
    TextInput, text_input, Radio, radio, PickList, pick_list, Column, Row};


#[derive(Default)]
struct State {
}

#[derive(Debug, Clone)]
pub enum Message {
    RadioPressed(usize), // (question, answer)
    // TextInputted(String), 
}

#[derive(Default)]
pub struct Quiz {
    questions: Vec<Question>,

    // state: State
}

impl Application for Quiz {
    type Executor = executor::Default;
    type Flags = Vec<Question>;
    type Message = Message;

    fn new(questions: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Quiz {
                questions,
                ..Default::default()
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Quiz")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let mut content = Column::new();

        for q in &self.questions {
            let mut answer_row = Row::new()
            .align_items(Align::Center);

            for a_ind in q.shuffled_answers_indices {
                let answer_string = match a_ind {
                    0 => { &q.answer },
                    1 => { &q.wrong_answers.0 },
                    2 => { &q.wrong_answers.1 },
                    _ => { panic!("Unreachable answer index") }
                };

                let answer = Radio::new(
                    a_ind, answer_string, Some(0), Message::RadioPressed
                )
                .spacing(4);

                answer_row = answer_row.push(answer);
            }

            content = content
            .push(Text::new(&q.question))
            .push(answer_row);
        }

        content.into()
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) 
      -> Command<Self::Message> {
        match message {
            Message::RadioPressed(answer_index) => {
                Command::none()
            },
            // Message::TextInputted(text) => {
            //     Command::none()
            // }
        }
    }
}

pub struct Question {
    question: String,
    answer: String,
    wrong_answers: (String, String),
    shuffled_answers_indices: [usize; 3]
}

impl Question {
    // question, answer, and wrong answers
    pub fn new(q: &str, a: &str, wa: (&str, &str)) -> Self {
        let mut ind = [0, 1, 2]; // ordered 
        fastrand::shuffle(&mut ind);

        Question {
            question: q.to_string(),
            answer: a.to_string(),
            wrong_answers: (wa.0.to_string(), wa.1.to_string()),
            shuffled_answers_indices: ind
        }
    }
}