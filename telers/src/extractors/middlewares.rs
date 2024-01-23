use super::from_context;

use crate::{
    fsm::Context as FSMContext,
    types::{Chat, User},
};

from_context!([Client], User, "event_user");
from_context!([Client], Chat, "event_chat");
from_context!([Client, S: Clone], FSMContext<S>, "fsm_context");
