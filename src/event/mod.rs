mod buffer;
mod repo;

pub use repo::EventRepository;

use crate::graphics::window::{WindowPosition, WindowSize};

pub type Keycode = u32;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    WindowEvent(WindowEvent),
    InputEvent(InputEvent),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    WindowRedrawRequested,
    WindowClose,
    WindowResize(WindowSize),
    WindowFocused(bool),
    WindowMoved(WindowPosition),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    KeyPressed { keycode: Keycode },
    KeyReleased { keycode: Keycode },
    MouseButtonPressed { button: u8 },
    MouseButtonReleased { button: u8 },
    MouseMoved { x: f64, y: f64 },
}

#[derive(Debug, PartialEq)]
pub enum EventCategory {
    Window,
    Keyboard,
    Mouse,
}

impl Event {
    pub fn is_input(&self) -> bool {
        match self {
            Event::InputEvent(_) => true,
            _ => false,
        }
    }
}
