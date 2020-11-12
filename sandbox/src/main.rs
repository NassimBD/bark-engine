use bark_engine::{
    core::Engine,
    graphics::{DefaultWindow, Window},
};
use env_logger::{self, WriteStyle};
use log::info;

fn main() {
    init_logger();

    // let mut engine = Engine::new();
    /* ... */
    info!("Starting engine");
    // engine.run();
    let window = DefaultWindow::create("Bark Engine", 1280, 720);
    // DefaultWindow::main();
    window.run();
}

fn init_logger() {
    env_logger::builder()
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .write_style(WriteStyle::Always) // might want to removes this if artifacts on output
        .init();
}
