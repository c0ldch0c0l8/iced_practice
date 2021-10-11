#[derive(PartialEq, Eq)]
pub enum Page {
    FirstPage,
    MiddlePage,
    LastPage
}

impl Default for Page {
    fn default() -> Self {
        Page::FirstPage
    }
}


#[derive(Default)]
pub struct PagesApp {
    current_page: Page,
    first_page_text: String,
    middle_page_text: String, 
    last_page_text: String,

    previous_button_state: iced::button::State,
    next_button_state: iced::button::State,
}

#[derive(Debug, Clone)]
pub enum GoTo {
    NextPage,
    PreviousPage
}

impl iced::Application for PagesApp {
    type Executor = iced::executor::Default;
    type Flags = (Page, String, String, String);
    type Message = GoTo;

    // flags are starting page, and strings for first middle and last pages
    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = PagesApp {
            current_page: flags.0,
            first_page_text: flags.1,
            middle_page_text: flags.2,
            last_page_text: flags.3,

            ..Default::default()
        };

        (app, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("PagesApp")
    }

    
    fn view(&mut self) -> iced::Element<'_, Self::Message> {

        // in an actual multi page app, each match pattern here
        // will return a column of its own, with its own elements
        // but lets just say what i am going over are _book_ pages,
        // and you know, mostly only text changes with that
        let current_page_text = match self.current_page {
            Page::FirstPage => { &self.first_page_text },
            Page::MiddlePage => { &self.middle_page_text },
            Page::LastPage => { &self.last_page_text },
        };

        iced::Column::new()
        .padding([25, 200, 80, 200])
        .push(
            iced::Text::new(current_page_text)
        )
        .push(
            iced::Row::new()
            .push(
                iced::Button::new(
                    &mut self.previous_button_state,
                    iced::Text::new("Previous Page")
                )
                .on_press(GoTo::PreviousPage)
            )
            .push(
                iced::Button::new(
                    &mut self.next_button_state,
                    iced::Text::new("Next Page")
                )
                .on_press(GoTo::NextPage)
            )
        )
        .into()
    }
    
    // dumbest design of them all lol. too tired to change stuff
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            GoTo::PreviousPage => {
                if self.current_page != Page::FirstPage {
                    if self.current_page == Page::LastPage {
                        self.current_page = Page::MiddlePage
                    } else {
                        self.current_page = Page::FirstPage
                    }
                }

                iced::Command::none()
            },
            GoTo::NextPage => {
                if self.current_page != Page::LastPage {
                    if self.current_page == Page::FirstPage {
                        self.current_page = Page::MiddlePage
                    } else {
                        self.current_page = Page::LastPage
                    }
                }

                iced::Command::none()
            }
        }
    }
}