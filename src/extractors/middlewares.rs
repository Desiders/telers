use super::{from_context, FromEventAndContext};

use crate::{
    client::Bot,
    context::Context,
    errors::ExtractionError,
    fsm::Context as FSMContext,
    types::{Chat, Update, User},
};

use std::sync::Arc;

from_context!([Client], User, "event_user");
from_context!([Client], Chat, "event_chat");
from_context!([Client, S: Clone], FSMContext<S>, "fsm_context");
