use crate::{
    event::{EventRepository, EventSender},
    graphics::WgpuWindowBuilder,
    Event,
};

use anyhow::*;

#[allow(dead_code)]
pub struct Engine {
    pub running: bool,
    pub window: winit::window::Window,
    pub event_repository: EventRepository,
}

impl Engine {
    pub fn new(window: winit::window::Window) -> Self {
        Engine {
            running: true,
            window,
            event_repository: EventRepository::new(),
        }
    }

    pub(crate) fn game_loop(&mut self, event: Event, event_sender: &mut EventSender) {
        match event {
            Event::WindowRedrawRequested => {
                // render *should* be fine, but update shouldn't be tied
                // to the redraw event
                self.update();
                self.render();
            }
            Event::CoreEventsClear => {
                self.window.request_redraw();
            }
            // accumulate other events that are happening
            event => event_sender.send(event),
        }
    }

    pub fn update(&mut self) {
        // **********
        self.event_repository.update();
        for event in self.event_repository.drain() {
            dbg!(event);
        }

        // **********
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    pub fn render(&mut self) {}
}

#[derive(Default)]
pub struct EngineBuilder {
    pub(crate) window: Option<WgpuWindowBuilder>,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn with_window(mut self, window_builder: WgpuWindowBuilder) -> Self {
        self.window = Some(window_builder);
        self
    }

    pub fn build_and_run(self, main_loop: impl Fn(EngineBuilder)) -> Result<()> {
        main_loop(self);
        Ok(())
    }
}
