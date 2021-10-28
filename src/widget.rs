use iced_graphics::{Backend, Defaults, Renderer, Primitive};
use iced_native::{Widget, layout, mouse, Element};
use iced::{Background, Color, Column, Length, Sandbox, Size, Slider, Space, slider, Rectangle};

#[derive(Default)]
pub struct WidgetApp {
    value: u16,
    slider_state: slider::State
}

#[derive(Debug, Clone)]
pub enum Message {
    ValueChanged(u16)
}

impl Sandbox for WidgetApp {
    type Message = Message;
    
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Widget App")
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .push( Circle { radius: self.value } )
        .push( Square { half_length: self.value } )
        .push( Space::with_height(Length::Fill) )
        .push(
            Slider::new(&mut self.slider_state, 0..=256, self.value, Message::ValueChanged)
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ValueChanged(value) => {
                self.value = value;
            }
        }
    }
}


pub struct Circle {
    radius: u16
}

impl<Message, B> Widget<Message, Renderer<B>> for Circle where B: Backend {
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer<B>, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::new(self.radius as f32 * 2.0, self.radius as f32 * 2.0))
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        use std::hash::Hash;

        f32::to_bits(self.radius as f32).hash(state)
    }

    fn draw(
        &self, 
        _renderer: &mut Renderer<B>, 
        _defaults: &Defaults, 
        layout: iced_native::Layout<'_>, 
        _cursor_position: iced::Point, 
        _viewport: &iced::Rectangle
    ) -> (Primitive, mouse::Interaction) {
        (
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Background::Color(Color::BLACK),
                border_radius: self.radius as f32,
                border_width: 0.0,
                border_color: Color::TRANSPARENT
            },
            mouse::Interaction::default()
        )
    }
}

impl<'a, B> Into<Element<'a, Message, Renderer<B>>> for Circle where B: Backend {
    fn into(self) -> Element<'a, Message, Renderer<B>> {
        Element::new(self)
    }
}


pub struct Square {
    half_length: u16
}

impl<Message, B> Widget<Message, Renderer<B>> for Square where B: Backend {
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer<B>, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::new(self.half_length as f32 * 2.0, self.half_length as f32 * 2.0))
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        use std::hash::Hash;

        f32::to_bits(self.half_length as f32).hash(state)
    }

    fn draw(&self, 
        _renderer: &mut Renderer<B>, 
        _defaults: &Defaults, 
        layout: iced_native::Layout<'_>, 
        _cursor_position: iced::Point, 
        _viewport: &iced::Rectangle
    ) -> (Primitive, mouse::Interaction) {
        let bounds = Rectangle {
            width: self.half_length as f32 * 2.0,
            height: self.half_length as f32 * 2.0,
            ..layout.bounds()
        };
        
        (
            Primitive::Quad {
                bounds,
                background: Background::Color(Color::BLACK),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            mouse::Interaction::default()
        )
    }
}

impl<'a, B> Into<Element<'a, Message, Renderer<B>>> for Square where B: Backend {
    fn into(self) -> Element<'a, Message, Renderer<B>> {
        Element::new(self)
    }
}