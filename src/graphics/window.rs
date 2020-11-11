

pub type WindowSize = u32;

pub trait Window {
    fn create(title: &str, width: WindowSize, height: WindowSize) -> Self;
    fn run(&self);
    fn width(&self) -> WindowSize;
    fn height(&self) -> WindowSize;
    fn set_vsync(&mut self, enabled: bool);
}
