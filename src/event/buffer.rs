use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use crate::Event;

type EventBuffer = VecDeque<Event>;

const NB_BUFFERS: usize = 2;

#[derive(Debug, Default)]
pub(crate) struct Events {
    event_buffers: [EventBuffer; NB_BUFFERS],
    current_buffer_index: usize,
}

impl Events {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn push(&mut self, event: &Event) {
        self.event_buffers[self.current_buffer_index].push_back(event.clone());
    }

    pub fn drain<'a>(&'a mut self) -> impl Iterator<Item = Event> + 'a {
        let current_buffer = self.current_buffer_index;
        self.next_buffer();
        self.event_buffers[current_buffer].drain(..)
    }

    fn next_buffer(&mut self) {
        self.current_buffer_index = (self.current_buffer_index + 1) % self.event_buffers.len();
    }
}

impl Index<usize> for Events {
    type Output = EventBuffer;
    fn index(&self, index: usize) -> &Self::Output {
        &self.event_buffers[index]
    }
}
impl IndexMut<usize> for Events {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.event_buffers[index]
    }
}
