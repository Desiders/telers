use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicU64, Ordering::Relaxed},
    time::{SystemTime, UNIX_EPOCH},
};

use super::AccessSettings;

pub type Data = serde_json::Value;
pub type DataMap = BTreeMap<String, Data>;

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

const ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE: u64 = ALPHABET.len() as u64;
const EMPTY_ENCODED_ID: &str = "0";

/// Generates a compact, URL-safe, roughly time-ordered ID.
/// Format: base-62 encoding of (`unix_ms` XOR monotonic counter).
#[must_use]
pub fn generate_id() -> String {
    let millis = u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
    )
    .unwrap_or(u64::MAX);

    let counter = ID_COUNTER.fetch_add(1, Relaxed);
    encode_base62(millis ^ counter)
}

#[must_use]
fn encode_base62(mut n: u64) -> String {
    if n == 0 {
        return EMPTY_ENCODED_ID.to_owned();
    }

    let mut buf = Vec::with_capacity(11); // ceil(log62(u64::MAX)) = 11
    while n > 0 {
        buf.push(ALPHABET[(n % BASE) as usize]);
        n /= BASE;
    }
    buf.reverse();

    // SAFETY: every byte comes from a hand-written ASCII literal.
    unsafe { String::from_utf8_unchecked(buf) }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub id: String,
    pub stack_id: String,
    pub state: String,
    pub start_data: Data,
    pub dialog_data: DataMap,
    pub widget_data: DataMap,
    pub access_settings: Option<AccessSettings>,
}

impl Context {
    #[must_use]
    pub fn new(stack_id: impl Into<String>, state: impl Into<String>, start_data: Data) -> Self {
        let stack_id = stack_id.into();
        let state = state.into();
        let id = generate_id();
        Self {
            id,
            stack_id,
            state,
            start_data,
            dialog_data: DataMap::new(),
            widget_data: DataMap::new(),
            access_settings: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn dialog_value(&self, key: &str) -> Option<&Data> {
        self.dialog_data.get(key)
    }

    #[inline]
    #[must_use]
    pub fn widget_value(&self, key: &str) -> Option<&Data> {
        self.widget_data.get(key)
    }

    #[must_use]
    pub fn dialog_value_as<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        self.dialog_value(key)
            .cloned()
            .and_then(|value| serde_json::from_value(value).ok())
    }

    #[must_use]
    pub fn widget_value_as<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        self.widget_value(key)
            .cloned()
            .and_then(|value| serde_json::from_value(value).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::Context;
    use serde_json::json;

    #[test]
    fn context_reads_dialog_and_widget_values() {
        let mut ctx = Context::new("stack", "state", serde_json::Value::Null);
        ctx.dialog_data.insert("count".into(), json!(3));
        ctx.widget_data.insert("selected".into(), json!("pear"));

        assert_eq!(ctx.dialog_value("count"), Some(&json!(3)));
        assert_eq!(ctx.widget_value("selected"), Some(&json!("pear")));
        assert_eq!(ctx.dialog_value_as::<u64>("count"), Some(3));
        assert_eq!(
            ctx.widget_value_as::<String>("selected"),
            Some("pear".to_owned())
        );
    }
}
