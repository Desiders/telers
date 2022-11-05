#![allow(clippy::module_name_repetitions)]

mod bases;
mod event_handler;
mod event_observer;
mod telegram_handler;
mod telegram_observer;

pub mod service;

pub use bases::{Action, EventReturn, PropagateEventResult};
pub use event_handler::{
    Handler as EventHandler, HandlerObject as EventHandlerObject,
    HandlerObjectService as EventHandlerObjectService,
};
pub use event_observer::{EventObserver, ObserverService as EventObserverService};
pub use telegram_handler::{
    Handler as TelegramHandler, HandlerObject as TelegramHandlerObject,
    HandlerObjectService as TelegramHandlerObjectService, Request as TelegramHandlerRequest,
    Response as TelegramHandlerResponse,
};
pub use telegram_observer::{
    EventObserver as TelegramEventObserver, ObserverService as TelegramEventObserverService,
    Request as TelegramObserverRequest, Response as TelegramObserverResponse,
};
