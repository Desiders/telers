# serialize

Demonstrates that telers' `Update` (and the rest of the Telegram Bot API types) are `serde`-serializable, by echoing each raw update back to the chat as formatted JSON.

## What it does

For every update that has an associated chat, the bot serializes the whole `Update` to pretty-printed JSON and sends it back to that chat as an HTML-formatted code block. If serialization fails, it sends a `Serialize error :(` message and returns the error.

## How it works

A single `Router` named `main` registers `serialize_handler` on the `on_update` observer, so it sees every update type. The handler:

- Resolves the chat with `update.chat()` and skips updates without one.
- Calls `serde_json::to_string_pretty(&update)` to serialize the `Update`.
- Wraps the result with `html_quote` and `html_pre_language(..., "json")` from `telers::utils::text` and sends it via `SendMessage` with `parse_mode(ParseMode::HTML)`.
- On error, reports it to the chat and returns `HandlerError::new(err)`.

The `Dispatcher` is built with `allowed_updates(UpdateType::all())` so every update type is polled and serialized.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package serialize
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
