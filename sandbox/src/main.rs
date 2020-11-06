use bark_engine::core::Engine;
use env_logger;
use log::info;

fn main() {
    init_logger();

    let mut engine = Engine::new();
    /* ... */
    info!("Starting engine");
    engine.run();
}

fn init_logger() {
    env_logger::builder()
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();
}
