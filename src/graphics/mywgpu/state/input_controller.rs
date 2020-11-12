use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

#[derive(Default)]
pub struct InputController {
    pub speed: f32,
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub is_space_pressed: bool,
}

impl InputController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            ..Default::default()
        }
    }

    pub fn process_input(&mut self, input: &KeyboardInput) -> bool {
        if let Some(key) = input.virtual_keycode {
            let is_pressed = input.state == ElementState::Pressed;
            match key {
                VirtualKeyCode::Space => {
                    self.is_up_pressed = is_pressed;
                    self.is_space_pressed = is_pressed;
                    true
                }
                VirtualKeyCode::LShift => {
                    self.is_down_pressed = is_pressed;
                    true
                }
                VirtualKeyCode::W | VirtualKeyCode::Up => {
                    self.is_forward_pressed = is_pressed;
                    true
                }
                VirtualKeyCode::A | VirtualKeyCode::Left => {
                    self.is_left_pressed = is_pressed;
                    true
                }
                VirtualKeyCode::S | VirtualKeyCode::Down => {
                    self.is_backward_pressed = is_pressed;
                    true
                }
                VirtualKeyCode::D | VirtualKeyCode::Right => {
                    self.is_right_pressed = is_pressed;
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }
}
