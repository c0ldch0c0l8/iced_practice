use iced::{Sandbox, Settings};

// mod hello;
// use hello::Hello;
// mod counter;
// use counter::Counter;
// mod notes;
// use notes::Notes;
mod pick_list;
use pick_list::PickList;

fn main() -> iced::Result {
    // Hello::run(Settings::default())
    // Counter::run(Settings::default())
    // Notes::run(Settings::default())
    PickList::run(Settings::default())
}
