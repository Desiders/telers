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

#[cfg(test)]
mod tests {
    use super::{format_callback_data, parse_callback_data};
    use crate::entities::Context;
    use serde_json::Value;

    fn ctx() -> Context {
        Context::new("", "state", Value::Null)
    }

    #[test]
    fn format_with_payload() {
        let ctx = ctx();
        let data = format_callback_data(&ctx, "wid", Some("pl"));
        assert_eq!(data, format!("td:{}:wid:pl", ctx.id));
    }

    #[test]
    fn format_without_payload() {
        let ctx = ctx();
        let data = format_callback_data(&ctx, "wid", None);
        assert_eq!(data, format!("td:{}:wid", ctx.id));
    }

    #[test]
    fn format_with_numeric_display_id() {
        let ctx = ctx();

        let data_i32 = format_callback_data(&ctx, 42_i32, Some("pl"));
        assert_eq!(data_i32, format!("td:{}:42:pl", ctx.id));

        let data_usize = format_callback_data(&ctx, 7_usize, None);
        assert_eq!(data_usize, format!("td:{}:7", ctx.id));
    }

    #[test]
    fn round_trip_with_payload() {
        let ctx = ctx();
        let data = format_callback_data(&ctx, "wid", Some("pl"));
        let parsed = parse_callback_data(&ctx, &data).expect("should parse");
        assert_eq!(parsed.target_id, "wid");
        assert_eq!(parsed.payload, Some("pl"));
    }

    #[test]
    fn round_trip_without_payload() {
        let ctx = ctx();
        let data = format_callback_data(&ctx, "wid", None);
        let parsed = parse_callback_data(&ctx, &data).expect("should parse");
        assert_eq!(parsed.target_id, "wid");
        assert_eq!(parsed.payload, None);
    }

    #[test]
    fn round_trip_numeric_id() {
        let ctx = ctx();
        let data = format_callback_data(&ctx, 42_i32, Some("x"));
        let parsed = parse_callback_data(&ctx, &data).expect("should parse");
        assert_eq!(parsed.target_id, "42");
        assert_eq!(parsed.payload, Some("x"));
    }

    #[test]
    fn parse_wrong_prefix() {
        let ctx = ctx();
        let data = format!("x:{}:wid:pl", ctx.id);
        assert!(parse_callback_data(&ctx, &data).is_none());
    }

    #[test]
    fn parse_wrong_intent() {
        let one = ctx();
        let other = ctx();
        assert_ne!(one.id, other.id);

        let data = format_callback_data(&other, "wid", Some("pl"));
        assert!(parse_callback_data(&one, &data).is_none());
    }

    #[test]
    fn parse_truncated_data() {
        let ctx = ctx();
        assert!(parse_callback_data(&ctx, "td").is_none());
        assert!(parse_callback_data(&ctx, &format!("td:{}", ctx.id)).is_none());
    }

    #[test]
    fn parse_empty_string() {
        let ctx = ctx();
        assert!(parse_callback_data(&ctx, "").is_none());
    }

    #[test]
    fn parse_payload_with_colons() {
        let ctx = ctx();
        let data = format!("td:{}:wid:a:b:c", ctx.id);
        let parsed = parse_callback_data(&ctx, &data).expect("should parse");
        assert_eq!(parsed.target_id, "wid");
        assert_eq!(parsed.payload, Some("a:b:c"));
    }

    #[test]
    fn parse_empty_payload() {
        let ctx = ctx();
        let data = format!("td:{}:wid:", ctx.id);
        let parsed = parse_callback_data(&ctx, &data).expect("should parse");
        assert_eq!(parsed.target_id, "wid");
        assert_eq!(parsed.payload, Some(""));
    }

    #[test]
    fn parse_empty_target_id_is_accepted() {
        let ctx = ctx();
        let data = format!("td:{}::pl", ctx.id);
        let parsed = parse_callback_data(&ctx, &data).expect("should parse");
        assert_eq!(parsed.target_id, "");
        assert_eq!(parsed.payload, Some("pl"));
    }
}
