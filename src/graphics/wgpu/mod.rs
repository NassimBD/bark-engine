use crate::graphics::window::WindowSize;
use crate::{
    core::{Engine, EngineBuilder},
    event::Event,
};
use log::trace;
use winit::{
    event::Event as WinitEvent,
    event::MouseButton,
    event::{ElementState, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
};

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
    pub fn build(mut self) -> (winit::window::Window, winit::event_loop::EventLoop<()>) {
        if self.title.is_none() {
            self.title = Some(String::from("Bark Engine"));
        }
        if self.size.is_none() {
            self.size = Some(WindowSize::new(1280, 720));
        }

        let build = |title, size: WindowSize| {
            let event_loop = EventLoop::new();
            let window = winit::window::WindowBuilder::new()
                .with_title(title)
                .with_inner_size(winit::dpi::PhysicalSize::new(size.width, size.height))
                .build(&event_loop)
                .expect("Could not build Wgpu window");
            (window, event_loop)
        };
        build(&self.title.unwrap(), self.size.unwrap())
    }
}

pub fn winit_loop(engine_builder: EngineBuilder) {
    let (window, event_loop) = engine_builder.window.unwrap().build();
    let mut engine = Engine::new(window);
    let mut event_sender = engine.event_repository.clone_sender();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        if let Some(event) = winit_event_parser(event) {
            trace!("Event: {:?}", event);
            match event {
                Event::WindowCloseRequested => *control_flow = ControlFlow::Exit,
                event => engine.engine_loop(event, &mut event_sender),
            }
        }
    })
}

pub fn winit_event_parser<T>(event: WinitEvent<T>) -> Option<Event> {
    let event = match event {
        WinitEvent::MainEventsCleared => Event::CoreEventsClear,
        WinitEvent::RedrawRequested(_) => Event::WindowRedrawRequested,
        WinitEvent::WindowEvent { ref event, .. } => match event {
            // window related
            WindowEvent::CloseRequested => Event::WindowCloseRequested,
            WindowEvent::Resized(size) => Event::WindowResize(size.into()),
            WindowEvent::Focused(focused) => Event::WindowFocused(*focused),
            WindowEvent::Moved(position) => Event::WindowMoved(position.x, position.y),

            // devices inputs
            WindowEvent::KeyboardInput { input, .. } => match input.state {
                ElementState::Pressed => Event::KeyPressed {
                    keycode: input.scancode,
                },
                ElementState::Released => Event::KeyReleased {
                    keycode: input.scancode,
                },
            },
            WindowEvent::MouseInput { button, state, .. } => {
                let button = match button {
                    MouseButton::Left => 0,
                    MouseButton::Right => 1,
                    MouseButton::Middle => 2,
                    MouseButton::Other(button) => *button,
                };
                match state {
                    ElementState::Pressed => Event::MouseButtonPressed { button },
                    ElementState::Released => Event::MouseButtonReleased { button },
                }
            }
            WindowEvent::CursorMoved { position, .. } => Event::MouseMoved {
                x: position.x,
                y: position.y,
            },
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
