use iced::{Application, Settings};

// mod hello;
// use hello::Hello;
// mod counter;
// use counter::Counter;
// mod notes;
// use notes::Notes;
// mod pick_list;
// use pick_list::PickList;
// mod quiz;
// use quiz::{Quiz, Question};
// mod pages;
// use pages::PagesApp;
// mod slider;
// use slider::SliderApp;
// mod image;
// use image::ImageApp;
// mod radio;
// use radio::RadioApp;
// mod quiz;
// use quiz::{QuizApp, Question};
// mod space;
// use space::SpaceApp;
// mod toggle_checkbox;
// use toggle_checkbox::ToggleCheckApp;
// mod style;
// use style::StyleApp;
// mod todo;
// use todo::TodoApp;
// mod widget;
// use widget::WidgetApp;
mod widget_text_color;
use widget_text_color::WidgetTextColor;

fn main() -> iced::Result {
    // Hello::run(Settings::default())
    // Counter::run(Settings::default())
    // Notes::run(Settings::default())
    // PickList::run(Settings::default())
    // let questions = vec![
    //     Question::new("What is my name?", "c0ldch0c0l8", ("A", "B")),
    //     Question::new("What is the color of this question?", "Black", ("Red", "White")),
    // ];

    // let mut settings = iced::Settings::default();
    // settings.flags = questions;

    // Quiz::run(settings)

    // let mut settings = Settings::default();
    // settings.flags = (
    //     pages::Page::FirstPage,
    //     "3 May. Bistritz.—Left Munich at 8:35 P. M., on 1st May, arriving at Vienna early next morning; should have arrived at 6:46, but train was an hour late. Buda-Pesth seems a wonderful place, from the glimpse which I got of it from the train and the little I could walk through the streets. I feared to go very far from the station, as we had arrived late and would start as near the correct time as possible. The impression I had was that we were leaving the West and entering the East; the most western of splendid bridges over the Danube, which is here of noble width and depth, took us among the traditions of Turkish rule.".to_string(),
    //     "We left in pretty good time, and came after nightfall to Klausenburgh. Here I stopped for the night at the Hotel Royale. I had for dinner, or rather supper, a chicken done up some way with red pepper, which was very good but thirsty. (Mem., get recipe for Mina.) I asked the waiter, and he said it was called “paprika hendl,” and that, as it was a national dish, I should be able to get it anywhere along the Carpathians. I found my smattering of German very useful here; indeed, I don’t know how I should be able to get on without it.".to_string(),
    //     "Having had some time at my disposal when in London, I had visited the British Museum, and made search among the books and maps in the library regarding Transylvania; it had struck me that some foreknowledge of the country could hardly fail to have some importance in dealing with a nobleman of that country. I find that the district he named is in the extreme east of the country, just on the borders of three states, Transylvania, Moldavia and Bukovina, in the midst of the Carpathian mountains; one of the wildest and least known portions of Europe. I was not able to light on any map or work giving the exact locality of the Castle Dracula, as there are no maps of this country as yet to compare with our own Ordnance Survey maps; but I found that Bistritz, the post town named by Count Dracula, is a fairly well-known place. I shall enter here some of my notes, as they may refresh my memory when I talk over my travels with Mina.".to_string(),
    // );

    // PagesApp::run(settings)

    // SliderApp::run(Settings::default())

    // ImageApp::run(Settings::default())

    // RadioApp::run(Settings::default())

    // let mut settings = Settings::default();
    // settings.flags = vec![
    //     Question::new("What is your name?", vec!["Faith", "Coeur"], 0, Some(1)),
    //     Question::new("What is your favorite letter?", vec!["A", "B", "C"], 1, Some(0))
    // ];

    // QuizApp::run(settings)

    // SpaceApp::run(Settings::default())

    // ToggleCheckApp::run(Settings::default())

    // TODO: style, window settings, creating widgets, maybe radioagain? 

    // let mut settings = Settings::default();
    // settings.default_text_size = 50;
    // settings.window.size = (400, 400);
    // settings.window.resizable = false;
    // settings.window.position = iced::window::Position::Centered;
    // settings.window.decorations = false;


    // let icon = image::open("resources/images/iced.png").unwrap();
    // settings.window.icon = iced::window::Icon::
    // from_rgba(icon.to_rgba8().to_vec(), 68, 68).ok();
    
    // Hello::run(settings)

    // StyleApp::run(Settings::default())
    // let mut settings = Settings::default();
    // settings.window.min_size = Some((750, 400));
    // settings.default_font = Some(include_bytes!("../resources/Roboto_Mono/static/RobotoMono-Regular.ttf"));

    // TodoApp::run(settings)

    // WidgetApp::run(Settings::default())
    
    WidgetTextColor::run(Settings::default())
}
