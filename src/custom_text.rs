use iced::{Color, Column, Font, Length, Sandbox, Size, alignment, Text};
use iced_graphics::{Renderer, Backend, Primitive, Defaults};
use iced_native::{Element, Layout, Widget, layout, mouse};

// i am faced with the choice of incorporating Text type into a CustomText
// type that I can customize,
// Rebuild Text from the ground up,
// Build on Text (This is == to above, do above)

pub struct CustomTextApp;

#[derive(Debug, Clone)]
pub enum Message {}

impl Sandbox for CustomTextApp {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Custom Text App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .push(CustomText { text: String::from("TEXT") })
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            
        }
    }
}


pub struct CustomText {
    text: String
}

impl<Message, B> Widget<Message, Renderer<B>> for CustomText where B: Backend {
    fn width(&self) -> iced::Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer<B>, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::new(100.0, 100.0))
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        use std::hash::Hash;

        self.text.hash(state)
    }

    fn draw(&self, 
        _renderer: &mut Renderer<B>, 
        _defaults: &Defaults, 
        layout: Layout<'_>, 
        _cursor_position: iced::Point, 
        _viewport: &iced::Rectangle
    ) -> (Primitive, mouse::Interaction) {
        (
            Primitive::Text {
                content: self.text.clone(),
                bounds: layout.bounds(),
                color: Color::BLACK,
                size: 80.0,
                font: Font::Default, 
                horizontal_alignment: alignment::Horizontal::Left,
                vertical_alignment: alignment::Vertical::Top
            },
            mouse::Interaction::Pointer
        )
    }
}

impl<'a, B> Into<Element<'a, Message, Renderer<B>>> for CustomText where B: Backend {
    fn into(self) -> Element<'a, Message, Renderer<B>> {
        Element::new(self)
    }
}