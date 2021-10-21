use iced::{Button, Column, Row, Sandbox, Text, TextInput, button, text_input};
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

            content = content.push(task_row);
        }

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
