#![allow(clippy::module_name_repetitions)]

mod bases;
mod handler;
mod telegram;

pub mod service;

pub use bases::{Action, EventReturn, PropagateEventResult};
pub use handler::{
    Handler, HandlerObject, HandlerObjectService, Request as HandlerRequest,
    Response as HandlerResponse,
};
pub use telegram::{
    EventObserver as TelegramEventObserver, ObserverService as TelegramEventObserverService,
    Request as TelegramObserverRequest, Response as TelegramObserverResponse,
};
