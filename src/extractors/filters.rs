use super::{from_context_impl, FromEventAndContext};

use crate::{
    client::Bot, context::Context, errors::ExtractionError, filters::CommandObject, types::Update,
};

use std::sync::Arc;

from_context_impl!([Client], CommandObject, "command");
