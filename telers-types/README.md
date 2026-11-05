# telers-types

Telegram Bot API objects, builders, discriminator enums and JSON serialization without
the `telers` bot framework or an HTTP client. This crate has its own version so API
types can be updated independently of routing and dispatching.

```toml
[dependencies]
telers-types = "0.1"
```

```rust
use telers_types::{serialization, types::User};

let user: User = serialization::from_str(
    r#"{"id": 1, "is_bot": false, "first_name": "Ada"}"#,
)?;
assert_eq!(&*user.first_name, "Ada");
# Ok::<(), telers_types::serialization::Error>(())
```

The default `serde` feature uses Serde. To select Deser instead, use
`default-features = false, features = ["deser"]`. As in `telers`, Deser takes
precedence when both features are enabled.

If another dependency enables `telers-types/deser`, also enable `telers/deser`
when using the framework. Cargo unifies features, so the data and framework
must use the same backend. Enabling `telers/deser` forwards the feature to
`telers-types` automatically.

Upload descriptors (`InputFile`, `FSFile`, `BufferedFile`, `StreamFile`) retain
their existing APIs and use bytes, futures and Tokio for local file streams;
they do not depend on Reqwest, TLS, or networking. This is not a `no_std` crate.

Existing bot code can continue using `telers::types`, `telers::enums` and
`telers::serialization`: those paths re-export the same types. Request shortcuts
and handler extraction remain in `telers`. Import
`telers::types::CallbackQueryShortcuts as _` to call `query.answer()`;
message shortcuts continue to use `MessageShortcuts`.

From the workspace root, validate this crate independently with:

```sh
cargo test -p telers-types --locked
cargo test -p telers-types --no-default-features --features deser --locked
cargo tree -p telers-types --edges normal --locked
```

Generated data lives under `src/types` and `src/enums`. Run `just codegen` from
the workspace root to regenerate the data alongside framework methods and
extractors. See the generator's CLI for separate output directories.
