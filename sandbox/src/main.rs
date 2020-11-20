use anyhow::*;
use bark_engine::{
    core::EngineBuilder,
    graphics::{winit_loop, WgpuWindowBuilder},
};
use env_logger::{self, WriteStyle};
use log::info;

fn main() -> Result<()> {
    init_logger();

    let window = WgpuWindowBuilder::new();
    info!("Starting engine");
    EngineBuilder::new()
        .with_window(window)
        .build_and_run(winit_loop)?;

    Ok(())
}

fn init_logger() {
    env_logger::builder()
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .write_style(WriteStyle::Always) // might want to removes this if artifacts on output
        .init();
}
