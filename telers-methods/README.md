# telers-methods

Telegram Bot API request builders, file collection and response parsing without
an HTTP client or the `telers` bot framework. Applications can use these methods
with their own transport.

```toml
[dependencies]
telers-methods = "0.1"
```

```rust
use telers_methods::{serialization, SendMessage, TelegramMethod as _};

let request = SendMessage::new(42, "Hello").build_request();
assert_eq!(request.method_name, "sendMessage");
let body = serialization::to_string(&request.data)?;
assert!(body.contains("Hello"));
# Ok::<(), telers_methods::serialization::Error>(())
```

`Request` contains the method name, serializable parameters and optional upload
parts. Building a request moves upload contents into the parts and keeps their
`attach://` references in the serializable parameters. It does not read files or make network requests.
`TelegramMethod::build_response` parses Telegram's response envelope, including
the result and error parameters. HTTP status checks, retries and sending remain
the transport's responsibility.

The dependency graph is:

```text
telers -> telers-methods -> telers-types
telers -----------------> telers-types
```

The methods crate uses `telers-types` for Telegram objects and upload descriptors.
It owns `Either`, which represents method results with multiple possible types.
Neither standalone crate depends on Reqwest, TLS, routing or dispatching.

The default `serde` feature uses Serde. To select Deser, use
`default-features = false, features = ["deser"]`. Deser takes precedence when
both features are enabled. Methods and types must use the same backend; enable
`telers-methods/deser` when another dependency enables `telers-types/deser`.
When using the framework too, enable `telers/deser`, which forwards the feature
to both standalone crates.

Existing framework imports continue to work: `telers::methods` re-exports the
same method structs, requests and responses, and `telers::Either` is the same
type. `Bot::send` accepts methods from this crate, including custom implementations
of its `TelegramMethod` trait. Message and callback query shortcuts remain in
`telers`.

The framework retains its `telers::methods::TelegramMethod` compatibility trait
with `build_request(self, &Bot)`, so existing custom framework methods keep
working. The standalone trait has `build_request(self)` and needs no bot. A
blanket implementation connects standalone methods to the framework trait.
Implement the standalone trait for new transport-independent methods; use the
framework trait when request preparation needs a bot. If importing both traits,
call their methods with a fully qualified path to avoid ambiguity.

From the workspace root, validate the crate independently with:

```sh
cargo test -p telers-methods --locked
cargo test -p telers-methods --no-default-features --features deser --locked
cargo tree -p telers-methods --edges normal --locked
```

Generated methods live under `src/methods`. Run `just codegen` from the workspace
root to regenerate them alongside types and framework helpers. See
[`telers-codegen`](../telers-codegen/README.md) for the output directory options.
