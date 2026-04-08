use std::fmt::Display;

use crate::entities::Context;

const CALLBACK_PREFIX: &str = "td";

#[inline]
#[must_use]
pub(crate) fn format_callback_data(
    ctx: &Context,
    target_id: impl Display,
    payload: Option<&str>,
) -> String {
    match payload {
        Some(payload) => format!("{CALLBACK_PREFIX}:{}:{target_id}:{payload}", ctx.id),
        None => format!("{CALLBACK_PREFIX}:{}:{target_id}", ctx.id),
    }
}

pub(crate) struct ParsedCallbackData<'a> {
    pub(crate) target_id: &'a str,
    pub(crate) payload: Option<&'a str>,
}

pub(crate) fn parse_callback_data<'a>(
    ctx: &Context,
    callback_data: &'a str,
) -> Option<ParsedCallbackData<'a>> {
    let mut parts = callback_data.splitn(4, ':');
    if parts.next()? != CALLBACK_PREFIX {
        return None;
    }
    if parts.next()? != ctx.id {
        return None;
    }
    Some(ParsedCallbackData {
        target_id: parts.next()?,
        payload: parts.next(),
    })
}
