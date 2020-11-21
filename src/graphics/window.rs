#[derive(Debug, Clone, PartialEq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
}
pub type WindowID = u64;

impl WindowSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
impl WindowPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
