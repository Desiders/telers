# extractor

Demonstrates telers' `Extractor` trait — how handler arguments get built from an incoming `Request`. It covers deriving extractors with the `FromEvent` and `FromContext` macros, implementing `Extractor` by hand, and pulling values out of the `Context`, request extensions, and the `Bot` itself.

## What it does

- `/data` — the bot replies with a message combining several extracted values, e.g. `NumData: 1. StrData: "1". BoolData: true. BotId: <id>`.
- `/update_id` — the bot replies with the current update's id (`Update id: <n>`); if the update has no chat it logs a warning instead of replying.

Only message updates are processed.

## How it works

- `UpdateId` derives `FromEvent` with `#[event(from = Update)]` and an infallible `From<Update>` impl, so it can be used directly as a handler argument.
- `UpdateChatId` derives `FromEvent` with `#[event(try_from = Update)]` and a `TryFrom<Update>` impl returning `ConvertToTypeError`; because `Extractor` is also implemented for `Option<T>`, the handler takes `Option<UpdateChatId>` to handle chat-less updates gracefully.
- `NumData` and `StrData` derive `FromContext` with `#[context(key = "...")]`, so they are read from the `Context` populated via the builder's `context_extend(...)`.
- `DataCombined` implements `Extractor` manually by composing `NumData::extract` and `StrData::extract`; `BotId` implements `Extractor` (with `Infallible` error) to read `request.bot.id`.
- `BoolData` is injected as a global extension through the builder's `.extension(...)` and consumed via the `Extension<BoolData>` extractor — no trait impl of its own needed.
- Handlers are registered on the message observer with `registers([...])`, gated by `Command::one("data")` and `Command::one("update_id")`. The `Dispatcher` is built with `allowed_update(UpdateType::Message)` and run via `run_polling`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package extractor
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
