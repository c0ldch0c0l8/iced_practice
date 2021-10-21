use iced::{Container, Button, Column, Row, Sandbox, Text, TextInput, button, text_input, Svg};
use chrono::{DateTime, Local};

#[derive(Default)]
pub struct TodoApp {
    tasks: Vec<Task>,

    input_value: String,
    input_state: text_input::State,
    priority_value: usize,
    priority_state: text_input::State,
    add_button_state: button::State,

    show_todo: bool,
    show_todo_button_state: button::State,
    show_done_button_state: button::State,

    done_buttons: Vec<button::State>
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    PriorityChanged(String),
    AddTask,
    ShowTodoTasks,
    ShowDoneTasks,
    TaskDone(usize)
}

impl Sandbox for TodoApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            show_todo: true,
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Todo App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let mut priority_display_value = self.priority_value.to_string();
        if self.priority_value == 0 {
            priority_display_value = "".to_string();
        }

        let mut column = Column::new()
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
                .style(style::PriorityInputStyle)
            )
            .push(
                Button::new(
                    &mut self.add_button_state,
                    // Text::new("Add task")
                    Svg::from_path("/home/c0ldch0c0l8/Programming/iced_practice/resources/material icons/add_task_white.svg")
                )
                .on_press(Message::AddTask)
                .style(style::AddTaskStyle)
            )
        )
        .push(
            Row::new()
            .push(
                Button::new(
                    &mut self.show_todo_button_state,
                    Text::new("Todo")
                )
                .on_press(Message::ShowTodoTasks)
            )
            .push(
                Button::new(
                    &mut self.show_done_button_state,
                    Text::new("Done")
                )
                .on_press(Message::ShowDoneTasks)
            )
        );

        let mut tasks_to_render = Vec::<&Task>::new();
        for task in &self.tasks {
            if self.show_todo == task.todo {
                tasks_to_render.push(task);
            }
        }
        tasks_to_render.sort();

        let mut done_buttons_mut_iter = self.done_buttons.iter_mut();

        for task in tasks_to_render {

            let mut task_row = Row::new()
            .push(
                Text::new(&task.name)
            )
            .push(
                Text::new(&task.priority.to_string())
            )
            .push(
                Text::new(task.date.format("%M:%S").to_string())
            );

            // make sure nth() is what i am aiming for and that this isnt buggy
            if task.todo {
                task_row = task_row.push(
                    Button::new(
                        done_buttons_mut_iter.nth(0).unwrap(),
                        Text::new("done")
                    )
                    .on_press(Message::TaskDone(task.index))
                );       
            }

            column = column.push(task_row);
        }

        let container = Container::new(column)
        .style(style::ContainerStyle);

        container.into()
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
                if !self.input_value.is_empty() {
                    self.tasks.push(Task::new(
                        &self.input_value, 
                        self.priority_value, 
                        self.tasks.len()
                    ));

                    self.done_buttons.push(button::State::default());

                    self.input_value = "".to_string();
                    self.priority_value = 0;
                }
            },
            Message::ShowTodoTasks => {
                self.show_todo = true;
            },
            Message::ShowDoneTasks => {
                self.show_todo = false;
            },
            Message::TaskDone(index) => {
                self.tasks[index].todo = false;
            }
        }   
    }
}

#[derive(PartialEq, Eq, Ord)]
struct Task {
    name: String,
    priority: usize,
    date: DateTime<Local>, 
    todo: bool, 

    index: usize // in vec
}

impl Task {
    fn new(name: &str, priority: usize, index: usize) -> Task {
        Task {
            name: name.to_string(),
            priority,
            date: Local::now(),
            todo: true,
            index
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // if one is a task not done yet and the other is a done task => None
        // todo tasks are ordered according to priority while
        // done tasks are ordered accroding to date. this impl is limiting methinks
        
        use std::cmp::Ordering;

        if self.todo != other.todo {
            None
        } else {
            if self.todo {
                // ordered in descending priority
                if self.priority > other.priority {
                    Some(Ordering::Less)
                } else if self.priority < other.priority {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            } else {
                // make sure this works to have old appear last 
                if self.date > other.date {
                    Some(Ordering::Less)
                } else if self.date < other.date {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
        }
    }
}


mod style {
    use iced::{Background, Color, Length::{self, Units}, Space, button, container, text_input};

    lazy_static::lazy_static! {
        static ref DARK_BG_COLOR: Color = Color::new(18.0/255.0, 18.0/255.0, 18.0/255.0, 1.0);

        static ref HIGH_EMPHASIS_TEXT_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.87); 
        static ref MEDIUM_EMPHASIS_TEXT_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.6);
        static ref DISABLED_TEXT_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.38); 

        static ref TEXT_SELECTION_COLOR: Color = Color::new(52.0/255.0, 152.0/255.0, 219.0/255.0, 1.0);
    }


    pub struct ContainerStyle;

    impl container::StyleSheet for ContainerStyle {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(*DARK_BG_COLOR)),
                text_color: Some(Color::WHITE),
                border_color: Color::WHITE,
                border_radius: 0.0,
                border_width: 0.0
            }
        }
    }

    pub struct PriorityInputStyle;

    impl text_input::StyleSheet for PriorityInputStyle {
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: Background::Color(*DARK_BG_COLOR),
                border_color: Color::WHITE,
                border_radius: 0.0,
                border_width: 0.0
            }
        }

        fn focused(&self) -> text_input::Style {
            text_input::Style {
                background: Background::Color(*DARK_BG_COLOR),
                border_color: Color::WHITE,
                border_radius: 0.0,
                border_width: 0.0
            }
        }

        fn value_color(&self) -> Color {
            *HIGH_EMPHASIS_TEXT_COLOR
        }

        fn placeholder_color(&self) -> Color {
            *MEDIUM_EMPHASIS_TEXT_COLOR
        }

        fn selection_color(&self) -> Color {
            *TEXT_SELECTION_COLOR
        }
    }

    pub struct AddTaskStyle;

    impl button::StyleSheet for AddTaskStyle {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(*DARK_BG_COLOR)),
                border_color: Color::WHITE,
                border_radius: 0.0,
                border_width: 0.0,
                text_color: *HIGH_EMPHASIS_TEXT_COLOR,
                ..Default::default()
            }

            // other methods available
        }
    }
}