# context

Shows how to pass data into handlers through the telers `Context` — both by inserting it from a middleware and from the dispatcher builder — and how to auto-extract it with `FromContext`.

## What it does

- A user sends the `/data` command, and the bot replies with `Data1: 1. Data2: 2`.
- `Data1` is injected per-message by an outer middleware; `Data2` is injected once at dispatcher build time.

## How it works

Two newtypes, `Data1(i64)` and `Data2(i64)`, derive `FromContext` with `#[context(key = "...")]`, so telers can extract them from the request context straight into handler arguments by their keys.

`to_context_middleware` is an outer middleware registered on the message observer; it calls `request.context.insert("data1", Data1(1))` and returns the request with `EventReturn::default()`. `Data2(2)` is seeded globally via the dispatcher builder's `.context("data2", Data2(2))`.

`send_data_handler` is registered with `.filter(Command::one("data"))` so it only fires on `/data`. It receives `bot`, `message`, the auto-extracted `data1`/`data2`, and the raw `Context`; it asserts the extracted values match `context.get::<...>(...)` and then sends them back with `SendMessage`. The dispatcher runs with `allowed_update(UpdateType::Message)` via `run_polling()`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package context
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
