use iced::{Sandbox, Column, Row, Text, TextInput, text_input, Button, button, };

#[derive(Default)]
pub struct TodoApp {
    tasks: Vec<Task>,

    input_value: String,
    input_state: text_input::State,
    priority_value: usize,
    priority_state: text_input::State,
    add_button_state: button::State,

    show_todo: bool,
    todo_button_state: button::State,
    done_button_state: button::State
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    PriorityChanged(String),
    AddTask,
    ShowTodoTasks,
    ShowDoneTasks
}

impl Sandbox for TodoApp {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Todo App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let mut priority_display_value = self.priority_value.to_string();
        if self.priority_value == 0 {
            priority_display_value = "".to_string();
        }

        let mut content = Column::new()
        .push(
            Row::new()
            .push(
                TextInput::new(
                    &mut self.input_state,
                    "Enter task",
                    &self.input_value,
                    Message::InputChanged
                )
            )
            .push(
                TextInput::new(
                    &mut self.priority_state,
                    "0",
                    &priority_display_value,
                    Message::PriorityChanged
                )
            )
            .push(
                Button::new(
                    &mut self.add_button_state,
                    Text::new("Add task")
                )
                .on_press(Message::AddTask)
            )
        )
        .push(
            Row::new()
            .push(
                Button::new(
                    &mut self.todo_button_state,
                    Text::new("Todo")
                )
                .on_press(Message::ShowTodoTasks)
            )
            .push(
                Button::new(
                    &mut self.done_button_state,
                    Text::new("Done")
                )
                .on_press(Message::ShowDoneTasks)
            )
        );

        content.into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(input) => {
                self.input_value = input;
            },
            Message::PriorityChanged(priority) => {
                if priority == "".to_string() {
                    self.priority_value = 0;
                    return;
                }

                match priority.parse::<usize>() {
                    Ok(number) => {
                        self.priority_value = number;
                    },
                    Err(_) => {

                    }
                }
            },
            Message::AddTask => {

            },
            Message::ShowTodoTasks => {
                self.show_todo = true;
            },
            Message::ShowDoneTasks => {
                self.show_todo = false;
            }
        }   
    }
}

struct Task {
    name: String,
    priority: usize,
    date: bool, // get date var
    todo: bool // false == done
}
