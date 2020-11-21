use crate::graphics::window::{WindowPosition, WindowSize};
use std::collections::VecDeque;

type EventBuffer = VecDeque<Event>;

#[derive(Debug, Copy, Clone)]
pub(crate) enum CurrentEventBuffer {
    First,
    Second,
}
#[derive(Debug, Default)]
pub struct Events {
    events: (EventBuffer, EventBuffer),
    pub(crate) current_event_buffer: CurrentEventBuffer,
}

impl Default for CurrentEventBuffer {
    fn default() -> Self {
        Self::First
    }
}
impl std::ops::Not for CurrentEventBuffer {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}
pub struct EventRepository {
    pub(crate) events: Events,
}

impl EventRepository {
    pub fn new() -> Self {
        Self {
            events: Events::new(),
        }
    }

    pub fn drain<'a>(&'a mut self) -> impl Iterator<Item = Event> + 'a {
        let current_buffer = self.events.current_event_buffer;
        self.switch_buffer();
        self.events.current(current_buffer).drain(..)
    }

    fn switch_buffer(&mut self) {
        self.events.current_event_buffer = !self.events.current_event_buffer;
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(&event);
    }
}

impl Events {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn push(&mut self, event: &Event) {
        self.current(self.current_event_buffer)
            .push_back(event.clone());
    }

    pub(crate) fn current(&mut self, buffer: CurrentEventBuffer) -> &mut EventBuffer {
        match buffer {
            CurrentEventBuffer::First => &mut self.events.0,
            CurrentEventBuffer::Second => &mut self.events.1,
        }
    }
}

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
