mod state;

use super::window::{Window, WindowSize};
use log::error;
use state::State;
use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct DefaultWindow {
    title: String,
    width: WindowSize,
    height: WindowSize,
}

impl Window for DefaultWindow {
    fn create(title: &str, width: WindowSize, height: WindowSize) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
        }
    }

    fn run(&self) {
        let event_loop = EventLoop::new();
        let size = PhysicalSize::new(self.width, self.height);
        let window = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap();
        run(event_loop, window);
    }
    fn width(&self) -> u32 {
        todo!()
    }
    fn height(&self) -> u32 {
        todo!()
    }
    #[allow(unused_variables)]
    fn set_vsync(&mut self, enabled: bool) {
        todo!();
    }
}

fn run(event_loop: EventLoop<()>, window: winit::window::Window) {
    use futures::executor::block_on; // TODO: add in imports?

    // Since main can't be async, we're going to need to block
    let mut state = block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        },
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => (),
                    }
                }
            }
            Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) | Err(wgpu::SwapChainError::Outdated) => {
                        state.resize(state.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(error) => error!("{:?}", error),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    })
}
