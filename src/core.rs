use std::time;

use crate::{
    event::EventRepository, event::WindowEvent, graphics::WgpuWindowBuilder, layer::Layer,
    layer::Layers, Event,
};

use anyhow::*;
use log::trace;

const FIXED_TIME_STEP: f64 = 0.01;

#[allow(dead_code)]
pub struct Engine {
    pub running: bool,
    pub window: winit::window::Window,
    pub event_repository: EventRepository,

    winit_proxy: winit::event_loop::EventLoopProxy<Event>,
    // Time stuff
    engine_time: time::Duration,
    time_step: time::Duration,
    current_time: time::Instant,
    time_accumulator: time::Duration,

    layers: Layers,
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
            time_step: time::Duration::from_secs_f64(FIXED_TIME_STEP),
            current_time: time::Instant::now(),
            time_accumulator: time::Duration::from_secs(0),
            layers: Layers::new(),
        }
    }

    pub fn update(&mut self) {
        let events = self.event_repository.drain();

        for event in events {
            trace!("Event: {:?}", event)
        }
    }

    pub fn render(&mut self) {}

    pub fn add_layer(&mut self, layer: impl Layer + 'static) {
        self.layers.push(layer)
    }
}

impl Engine {
    pub(crate) fn game_loop(&mut self, event: Event) {
        match event {
            event @ Event::WindowEvent(WindowEvent::WindowClose) => {
                // TODO: handle result
                self.winit_proxy.send_event(event).unwrap();
            }
            Event::WindowEvent(WindowEvent::WindowRedrawRequested) => {
                // TODO: watchZout for the death spiral, make sure
                // the time_accumulator doesn't accumulate more and more time
                // otherwise might have to add a maximum of time that it can
                // accumulate
                let new_time = time::Instant::now();
                let elapsed_time = new_time - self.current_time;
                self.current_time = new_time;

                self.time_accumulator += elapsed_time;

                while self.time_accumulator > self.time_step {
                    self.update();
                    self.time_accumulator -= self.time_step;
                    self.engine_time += self.time_step;
                }
                // TODO: use blending_factor for rendering
                // let blending_factor = self.time_accumulator.div_f64(self.time_step.as_secs_f64());
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
