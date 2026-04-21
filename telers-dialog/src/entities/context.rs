use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicU64, Ordering::Relaxed},
    time::{SystemTime, UNIX_EPOCH},
};

use super::AccessSettings;

/// Untyped dialog payload stored in `start_data`, `dialog_data`, and `widget_data`.
pub type Data = serde_json::Value;
/// String-keyed map used by dialogs and widgets for persisted runtime state.
pub type DataMap = BTreeMap<String, Data>;

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

const ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE: u64 = ALPHABET.len() as u64;
const EMPTY_ENCODED_ID: &str = "0";
const COUNTER_BITS: u32 = 20;
const COUNTER_MASK: u64 = (1 << COUNTER_BITS) - 1;

/// Generates a compact, URL-safe, roughly time-ordered ID.
/// Format: base-62 encoding of (`unix_ms` << `COUNTER_BITS` | `counter_low_bits`).
#[must_use]
pub fn generate_id() -> String {
    let millis = u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
    )
    .unwrap_or(u64::MAX);

    let counter = ID_COUNTER.fetch_add(1, Relaxed) & COUNTER_MASK;
    encode_base62((millis << COUNTER_BITS) | counter)
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

/// Stored dialog context for one active intent on the stack.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    /// Unique intent id.
    pub id: String,
    /// Stack id that owns this context.
    pub stack_id: String,
    /// Current state id inside the dialog.
    pub state: String,
    /// Data provided when the dialog was started.
    pub start_data: Data,
    /// Dialog-level mutable state shared across windows.
    pub dialog_data: DataMap,
    /// Widget-level mutable state keyed by widget id.
    pub widget_data: DataMap,
    /// Optional access settings overriding stack-level defaults.
    pub access_settings: Option<AccessSettings>,
}

impl Context {
    /// Create a new dialog context with empty `dialog_data` and `widget_data`.
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

    /// Read a raw value from `widget_data`.
    #[inline]
    #[must_use]
    pub fn widget_value(&self, key: &str) -> Option<&Data> {
        self.widget_data.get(key)
    }

    /// Read and deserialize a typed value from `dialog_data`.
    #[must_use]
    pub fn dialog_value_as<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        self.dialog_value(key)
            .cloned()
            .and_then(|value| serde_json::from_value(value).ok())
    }

    /// Read and deserialize a typed value from `widget_data`.
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

    #[tokio::test]
    async fn context_reads_dialog_and_widget_values() {
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
