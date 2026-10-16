//! Mounting a bot as an `axum` route that receives the updates pushed by Telegram,
//! instead of polling them with the dispatcher.
//!
//! [`UpdatesHandler`] is the route handler that checks the secret token of the request, parses
//! the update and feeds it to the dispatcher, and [`get_updates_router`] turns it into a
//! `MethodRouter` to mount on any path of your server.

mod handler;

pub use handler::{get_updates_router, UpdatesHandler};
