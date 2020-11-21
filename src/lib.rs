#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

pub mod core;
pub mod event;
pub mod graphics;

pub use event::{Event, Events};
