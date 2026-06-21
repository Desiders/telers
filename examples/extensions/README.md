# extensions

Demonstrates request *extensions* in telers: attaching arbitrary typed data to a `Request` from middlewares and filters, then pulling it back out inside a handler — either automatically via the `Extension<T>` extractor or manually via the `Extensions` map.

## What it does

Send the `/data` command to the bot and it replies with a single message echoing the extension values that were injected upstream, e.g. `NumData: 1. StrData: "1"`. Only message updates are processed.

## How it works

- A `Router` named `main` registers everything on its message observer.
- `to_extensions_filter` is a `filter` that inserts `StrData("1")` into `request.extensions` and returns `true` so the handler runs.
- `to_extensions_middleware` is an outer middleware (`register_outer_middleware`) that inserts `NumData(1)`. It is registered on the message observer, so it only fires for messages — register on the update observer to cover every update.
- `EmptyData` is supplied globally through the `Dispatcher` builder's `.extension(...)` method.
- `send_data_handler` is gated by `Command::one("data")`. It receives `Extension<NumData>`, `Extension<StrData>`, and `Extension<EmptyData>` extracted automatically, plus the raw `Extensions` map, and asserts that both access paths return the same values before replying with `SendMessage`.
- The `Dispatcher` is built with `allowed_update(UpdateType::Message)` and run via `run_polling`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package extensions
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
