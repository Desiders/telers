#![allow(clippy::module_name_repetitions)]

pub mod bases;
pub mod service;
pub mod simple;
pub mod telegram;

pub use bases::{cancel_event, finish_event, skip_event, EventReturn};
pub use service::ToServiceProvider;
