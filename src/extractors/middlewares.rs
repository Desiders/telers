use super::{from_context_impl, FromEventAndContext};

use crate::{
    client::Bot,
    context::Context,
    errors::ExtractionError,
    fsm::Context as FSMContext,
    types::{Chat, Update, User},
};

use std::sync::Arc;

from_context_impl!([Client], User, "event_user");
from_context_impl!([Client], Chat, "event_chat");
from_context_impl!([Client, S: Clone], FSMContext<S>, "fsm_context");
