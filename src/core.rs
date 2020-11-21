use std::time;

use crate::{event::EventRepository, event::WindowEvent, graphics::WgpuWindowBuilder, Event};

use anyhow::*;
use log::trace;

#[allow(dead_code)]
pub struct Engine {
    pub running: bool,
    pub window: winit::window::Window,
    pub event_repository: EventRepository,

    winit_proxy: winit::event_loop::EventLoopProxy<Event>,
    engine_time: time::Duration,
    delta_time: time::Duration,
    current_time: time::Instant,
    accumulator: time::Duration,
}

impl Engine {
    pub fn new(
        window: winit::window::Window,
        winit_proxy: winit::event_loop::EventLoopProxy<Event>,
    ) -> Self {
        Engine {
            running: true,
            window,
            event_repository: EventRepository::new(),

            winit_proxy,
            engine_time: time::Duration::from_secs(0),
            delta_time: time::Duration::from_secs_f64(0.01),
            current_time: time::Instant::now(),
            accumulator: time::Duration::from_secs(0),
        }
    }

    pub fn update(&mut self) {
        let events = self.event_repository.drain();

        for event in events {
            trace!("Event: {:?}", event)
        }
    }

    pub fn render(&mut self) {}

    pub(crate) fn game_loop(&mut self, event: Event) {
        match event {
            event @ Event::WindowEvent(WindowEvent::WindowClose) => {
                // TODO: handle result
                self.winit_proxy.send_event(event).unwrap();
            }
            Event::WindowEvent(WindowEvent::WindowRedrawRequested) => {
                let new_time = time::Instant::now();
                let frame_time = new_time - self.current_time;
                self.current_time = new_time;

                self.accumulator += frame_time;

                while self.accumulator >= self.delta_time {
                    self.update();
                    self.accumulator -= self.delta_time;
                    self.engine_time += self.delta_time;
                }
                self.render();
            }
            // accumulate other events that are happening
            event => {
                self.event_repository.push(event);
            }
        }
    }
}

#[derive(Default)]
pub struct WgpuEngineBuilder {
    pub(crate) window: Option<WgpuWindowBuilder>,
}

impl WgpuEngineBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn with_window(mut self, window_builder: WgpuWindowBuilder) -> Self {
        self.window = Some(window_builder);
        self
    }

    pub fn build_and_run(self, run: impl Fn(WgpuEngineBuilder) -> Result<()>) -> Result<()> {
        run(self)
    }
}
