use crate::{
    core::{Engine, WgpuEngineBuilder},
    event::{Event, WindowEvent},
};
use crate::{event::InputEvent, graphics::window::WindowSize};
use anyhow::*;
use log::info;
use winit::{
    event::Event as WinitEvent,
    event::{ElementState, KeyboardInput, MouseButton, WindowEvent as WinitWindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use super::window::WindowPosition;

#[derive(Default)]
pub struct WgpuWindowBuilder {
    title: Option<String>,
    size: Option<WindowSize>,
}

impl WgpuWindowBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
    pub fn with_size(mut self, size: WindowSize) -> Self {
        self.size = Some(size);
        self
    }
    pub fn build(mut self) -> (winit::window::Window, winit::event_loop::EventLoop<Event>) {
        if self.title.is_none() {
            self.title = Some(String::from("Bark Engine"));
        }
        if self.size.is_none() {
            self.size = Some(WindowSize::new(1280, 720));
        }
        let title = self.title.unwrap();
        let size = self.size.unwrap();

        let event_loop: EventLoop<Event> = EventLoop::with_user_event();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(size.width, size.height))
            .build(&event_loop)
            .expect("Could not build Wgpu window");

        (window, event_loop)
    }
}

pub fn winit_run(engine_builder: WgpuEngineBuilder) -> Result<()> {
    let (window, event_loop) = engine_builder.window.unwrap().build();
    let proxy = event_loop.create_proxy();
    let mut engine = Engine::new(window, proxy);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            WinitEvent::WindowEvent {
                event: WinitWindowEvent::CloseRequested,
                ..
            } => {
                info!("Window was requested to close");
                *control_flow = ControlFlow::Exit;
                return;
            }
            WinitEvent::MainEventsCleared => {
                engine.window.request_redraw();
            }
            _ => (),
        }
        if let Some(event) = winit_event_parser(event) {
            match event {
                Event::WindowEvent(WindowEvent::WindowClose) => {
                    info!("Window was requested to close");
                    *control_flow = ControlFlow::Exit
                }
                event => engine.game_loop(event),
            }
        }
    })
}

pub fn winit_event_parser(event: WinitEvent<Event>) -> Option<Event> {
    let event = match event {
        WinitEvent::UserEvent(event) => event,
        WinitEvent::RedrawRequested(_) => Event::WindowEvent(WindowEvent::WindowRedrawRequested),
        WinitEvent::WindowEvent { ref event, .. } => match event {
            // window related
            WinitWindowEvent::CloseRequested => Event::WindowEvent(WindowEvent::WindowClose),
            WinitWindowEvent::Resized(size) => {
                Event::WindowEvent(WindowEvent::WindowResize(size.into()))
            }
            WinitWindowEvent::Focused(focused) => {
                Event::WindowEvent(WindowEvent::WindowFocused(*focused))
            }
            WinitWindowEvent::Moved(position) => {
                Event::WindowEvent(WindowEvent::WindowMoved(position.into()))
            }

            // devices inputs
            WinitWindowEvent::KeyboardInput {
                input: KeyboardInput {
                    scancode, state, ..
                },
                ..
            } => match state {
                ElementState::Pressed => {
                    Event::InputEvent(InputEvent::KeyPressed { keycode: *scancode })
                }
                ElementState::Released => {
                    Event::InputEvent(InputEvent::KeyReleased { keycode: *scancode })
                }
            },
            WinitWindowEvent::MouseInput { button, state, .. } => {
                let button = match button {
                    MouseButton::Left => 0,
                    MouseButton::Right => 1,
                    MouseButton::Middle => 2,
                    MouseButton::Other(button) => *button,
                };
                match state {
                    ElementState::Pressed => {
                        Event::InputEvent(InputEvent::MouseButtonPressed { button })
                    }
                    ElementState::Released => {
                        Event::InputEvent(InputEvent::MouseButtonReleased { button })
                    }
                }
            }
            WinitWindowEvent::CursorMoved { position, .. } => {
                Event::InputEvent(InputEvent::MouseMoved {
                    x: position.x,
                    y: position.y,
                })
            }
            _ => return None,
        },
        _ => return None,
    };
    Some(event)
}

impl<T> From<winit::dpi::PhysicalSize<T>> for WindowSize
where
    T: Into<u32>,
{
    fn from(size: winit::dpi::PhysicalSize<T>) -> Self {
        WindowSize::new(size.width.into(), size.height.into())
    }
}

impl<T> From<&winit::dpi::PhysicalSize<T>> for WindowSize
where
    T: Into<u32> + Clone,
{
    fn from(size: &winit::dpi::PhysicalSize<T>) -> Self {
        WindowSize::new(size.width.clone().into(), size.height.clone().into())
    }
}

impl<T> From<winit::dpi::PhysicalPosition<T>> for WindowPosition
where
    T: Into<i32>,
{
    fn from(size: winit::dpi::PhysicalPosition<T>) -> Self {
        WindowPosition::new(size.x.into(), size.y.into())
    }
}

impl<T> From<&winit::dpi::PhysicalPosition<T>> for WindowPosition
where
    T: Into<i32> + Clone,
{
    fn from(size: &winit::dpi::PhysicalPosition<T>) -> Self {
        WindowPosition::new(size.x.clone().into(), size.y.clone().into())
    }
}
