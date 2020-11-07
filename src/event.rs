use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Events {
    events: VecDeque<Event>,
}

impl Events {
    pub fn new() -> Self {
        Events {
            ..Default::default()
        }
    }
    pub fn push(&mut self, event: &Event) {
        self.events.push_back(event.clone())
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_front()
    }
}

pub type Keycode = i32;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    KeyPressed(Keycode),
    KeyReleased(Keycode),
    KeyRepeated(Keycode),
    MouseButtonPressed { x: i32, y: i32 },
    MouseButtonReleased { x: i32, y: i32 },
    MouseMoved { x: i32, y: i32 },
}

#[derive(Debug, PartialEq)]
pub enum EventCategory {
    Window,
    Keyboard,
    Mouse,
}

impl Event {
    pub fn category(&self) -> EventCategory {
        match self {
            // Window events
            Event::WindowClose
            | Event::WindowResize
            | Event::WindowFocus
            | Event::WindowLostFocus
            | Event::WindowMoved => EventCategory::Window,
            // Keyboard events
            Event::KeyPressed(_) | Event::KeyReleased(_) | Event::KeyRepeated(_) => 
                EventCategory::Keyboard
            ,
            // Mouse events
            Event::MouseButtonPressed { .. }
            | Event::MouseButtonReleased { .. }
            | Event::MouseMoved { .. } => EventCategory::Mouse,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_event() {
        let mut events = Events::new();
        assert_eq!(events.pop(), None);
        events.push(&Event::WindowClose);
        events.push(&Event::WindowFocus);
        assert_eq!(events.pop(), Some(Event::WindowClose));
        assert_eq!(events.pop(), Some(Event::WindowFocus));
        assert_eq!(events.pop(), None);
    }
    #[test]
    fn event_category() {
        let event: Event = Event::WindowClose;
        assert_eq!(EventCategory::Window, event.category());
    }
}
