use iced::{Application, executor, Command, Text,
    TextInput, text_input, Radio, radio, PickList, pick_list, Column, Row};



#[derive(Clone)]
pub struct Question {
    q: String,
    a: Vec<String>
}

impl Question {
    pub fn new(q: &str, a: Vec<&str>) -> Question {
        Question {
            q: q.to_string(),
            a: a.iter().map(|answer| {answer.to_string()}).collect()
        }
    }
}


#[derive(Default)]
pub struct QuizApp {
    quetions: Vec<Question>,
    answers_active: Vec<String> 
}

impl Application for QuizApp {
    type Executor = executor::Default;
    type Flags = Vec<Question>;
    type Message = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            QuizApp {
                quetions: flags.clone(),
                answers_active: flags.iter().map(|q| {
                    q.a[0].clone()
                }).collect()
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Quiz App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let mut column = Column::new();
        
        for (question_index, question) in self.quetions.iter().enumerate() {
            let mut answer_row = Row::new();
            for a in &question.a {

                // self.answers_active[question_index], should be val and others should be a
                answer_row = answer_row
                .push(
                    Radio::new(
                        self.answers_active[question_index],
                                 
                    )
                )
            } 

            column = column
            .push(
                Text::new(&question.q)
                .size(35)
            )
            .push(
                answer_row
            )
        }

        column.into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}




// pub struct Question {
//     question: String,
//     answer: String,
//     wrong_answers: (String, String),
//     // order in which answers appear
//     shuffled_answers_indices: [usize; 3]
// }

// impl Question {
//     // question, answer, and wrong answers
//     pub fn new(q: &str, a: &str, wa: (&str, &str)) -> Self {
//         let mut ind = [0, 1, 2]; // ordered 
//         fastrand::shuffle(&mut ind);

//         Question {
//             question: q.to_string(),
//             answer: a.to_string(),
//             wrong_answers: (wa.0.to_string(), wa.1.to_string()),
//             shuffled_answers_indices: ind
//         }
//     }
// }



// #[derive(Debug, Clone)]
// pub enum QxAy {
//     Q1A1(i32),
//     Q1A2(i32),
//     Q1A3(i32),
//     Q2A1(i32),
//     Q2A2(i32),
//     Q2A3(i32),
//     Q3A1(i32),
//     Q3A2(i32),
//     Q3A3(i32),
// }




// #[derive(Default)]
// struct State {
// }

// #[derive(Debug, Clone)]
// pub enum Message {
//     RadioPressed(usize), // (question, answer)
//     // TextInputted(String), 
// }

// #[derive(Default)]
// pub struct Quiz {
//     questions: Vec<Question>,

//     // state: State
// }

// impl Application for Quiz {
//     type Executor = executor::Default;
//     type Flags = Vec<Question>;
//     type Message = Message;

//     fn new(questions: Self::Flags) -> (Self, Command<Self::Message>) {
//         (
//             Quiz {
//                 questions,
//                 ..Default::default()
//             },
//             Command::none()
//         )
//     }

//     fn title(&self) -> String {
//         String::from("Quiz")
//     }

//     fn view(&mut self) -> iced::Element<'_, Self::Message> {
//         let mut content = Column::new();

//         for q in &self.questions {
//             let mut answer_row = Row::new()
//             .align_items(iced::Alignment::Center);

//             for a_ind in &q.shuffled_answers_indices {
//                 let answer_string = match a_ind {
//                     0 => { &q.answer },
//                     1 => { &q.wrong_answers.0 },
//                     2 => { &q.wrong_answers.1 },
//                     _ => { panic!("Unreachable answer index") }
//                 };

//                 let answer = Radio::new(
//                     a_ind, answer_string, Some(0), Message::RadioPressed
//                 )
//                 .spacing(4);

//                 answer_row = answer_row.push(answer);
//             }

//             content = content
//             .push(Text::new(&q.question))
//             .push(answer_row);
//         }

//         content.into()
//     }

//     fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
//         match message {
//             Message::RadioPressed(answer_index) => {
//                 Command::none()
//             },
//             // Message::TextInputted(text) => {
//             //     Command::none()
//             // }
//         }
//     }
// }

// pub struct Question {
//     question: String,
//     answer: String,
//     wrong_answers: (String, String),
//     shuffled_answers_indices: [usize; 3]
// }

// impl Question {
//     // question, answer, and wrong answers
//     pub fn new(q: &str, a: &str, wa: (&str, &str)) -> Self {
//         let mut ind = [0, 1, 2]; // ordered 
//         fastrand::shuffle(&mut ind);

//         Question {
//             question: q.to_string(),
//             answer: a.to_string(),
//             wrong_answers: (wa.0.to_string(), wa.1.to_string()),
//             shuffled_answers_indices: ind
//         }
//     }
// }