use iced::{Sandbox, Settings};

// mod hello;
// use hello::Hello;
// mod counter;
// use counter::Counter;
mod notes;
use notes::Notes;

fn main() -> iced::Result {
    // Hello::run(Settings::default())
    // Counter::run(Settings::default())
    Notes::run(Settings::default())
}
