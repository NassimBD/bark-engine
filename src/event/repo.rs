use crate::Event;

use super::buffer::Events;

pub struct EventRepository {
    events: Events,
}

impl EventRepository {
    pub fn new() -> Self {
        Self {
            events: Events::new(),
        }
    }

    pub fn drain<'a>(&'a mut self) -> impl Iterator<Item = Event> + 'a {
        self.events.drain()
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(&event);
    }
}
