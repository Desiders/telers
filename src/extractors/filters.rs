use super::{from_context, FromEventAndContext};

use crate::{
    client::Bot, context::Context, errors::ExtractionError, filters::CommandObject, types::Update,
};

use std::sync::Arc;

from_context!([Client], CommandObject, "command");
