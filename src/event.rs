use anyhow::*;
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::graphics::window::{WindowID, WindowPosition, WindowSize};
type EventBuffer = VecDeque<Event>;

#[derive(Debug, Copy, Clone)]
enum CurrentEventBuffer {
    First,
    Second,
}
#[derive(Debug, Default)]
pub struct Events {
    events: (EventBuffer, EventBuffer),
    current_event_buffer: CurrentEventBuffer,
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
    event_sender: Sender<Event>,
    event_receiver: Receiver<Event>,
    events: Events,
}

pub struct EventSender {
    sender: Sender<Event>,
}

impl EventRepository {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            event_sender: sender,
            event_receiver: receiver,
            events: Events::new(),
        }
    }

    pub fn clone_sender(&mut self) -> EventSender {
        EventSender {
            sender: self.event_sender.clone(),
        }
    }

    pub fn update(&mut self) {
        self.events
            .buffer(self.events.current_event_buffer)
            .extend(self.event_receiver.try_iter());
    }

    pub fn drain<'a>(&'a mut self) -> impl Iterator<Item = Event> + 'a {
        let current_buffer = self.events.current_event_buffer;
        self.events.current_event_buffer = !self.events.current_event_buffer;
        let events = self.events.buffer(current_buffer).drain(..);
        events
    }
}

impl EventSender {
    pub fn send(&self, event: Event) {
        self.sender
            .send(event)
            .expect("Could not send an event");
    }
}

impl Events {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn push(&mut self, event: &Event) {
        self.buffer(self.current_event_buffer)
            .push_back(event.clone());
    }

    fn buffer(&mut self, buffer: CurrentEventBuffer) -> &mut EventBuffer {
        match buffer {
            CurrentEventBuffer::First => &mut self.events.0,
            CurrentEventBuffer::Second => &mut self.events.1,
        }
    }
}

pub type Keycode = u32;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    CoreEventsClear,
    WindowRedrawRequested,
    WindowCloseRequested,
    WindowResize(WindowSize),
    WindowFocused(bool),
    WindowMoved(WindowPosition, WindowPosition),
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

// impl Event {
//     pub fn category(&self) -> EventCategory {
//         match self {
//             // Window events
//             Event::WindowClose
//             | Event::WindowResize
//             | Event::WindowFocus
//             | Event::WindowLostFocus
//             | Event::WindowMoved => EventCategory::Window,
//             // Keyboard events
//             Event::KeyPressed(_) | Event::KeyReleased(_) | Event::KeyRepeated(_) =>
//                 EventCategory::Keyboard
//             ,
//             // Mouse events
//             Event::MouseButtonPressed { .. }
//             | Event::MouseButtonReleased { .. }
//             | Event::MouseMoved { .. } => EventCategory::Mouse,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn push_pop_event() {
//         let mut events = Events::new();
//         assert_eq!(events.pop(), None);
//         events.push(&Event::WindowClose);
//         events.push(&Event::WindowFocus);
//         assert_eq!(events.pop(), Some(Event::WindowClose));
//         assert_eq!(events.pop(), Some(Event::WindowFocus));
//         assert_eq!(events.pop(), None);
//     }
//     #[test]
//     fn event_category() {
//         let event: Event = Event::WindowClose;
//         assert_eq!(EventCategory::Window, event.category());
//     }
// }
