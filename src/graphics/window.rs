use crate::core::Engine;

#[derive(Debug, Clone, PartialEq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}
pub type WindowPosition = i32;
pub type WindowID = u64;

pub trait Window {
    fn run(&mut self, engine: Engine);
    fn width(&self) -> WindowSize;
    fn height(&self) -> WindowSize;
}

impl WindowSize {
    pub fn new(width: u32, height: u32) -> Self {
        WindowSize { width, height }
    }
}
