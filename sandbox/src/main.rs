use anyhow::*;
use bark_engine::{
    core::WgpuEngineBuilder,
    graphics::{winit_run, WgpuWindowBuilder},
};
use env_logger::{self, WriteStyle};
use log::info;

fn main() -> Result<()> {
    init_logger();

    info!("Starting engine");
    WgpuEngineBuilder::new()
        .with_window(WgpuWindowBuilder::new())
        .build_and_run(winit_run)?;

    Ok(())
}

fn init_logger() {
    env_logger::builder()
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .write_style(WriteStyle::Always) // might want to removes this if artifacts on output
        .init();
}
