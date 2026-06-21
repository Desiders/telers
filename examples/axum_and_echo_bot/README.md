# axum_and_echo_bot

Runs a telers polling bot and an `axum` HTTP server side by side in the same process, sharing a single graceful-shutdown signal.

## What it does

- The Telegram bot echoes every message: when a user sends a message, the bot copies it back into the same chat with `CopyMessage`.
- Independently, an HTTP server listens on `0.0.0.0:3000` and answers `Hello, World!` to `GET /`.
- Both halves shut down cleanly together when the process receives a termination signal (Ctrl-C / SIGTERM).

## How it works

The Telegram side builds a telers `Router` named `"main"` whose `on_message` observer registers `echo_handler`, a `Handler` that calls `bot.send(CopyMessage::new(...))`. A `Dispatcher` is built from `router.configure_default()` with `allowed_update(UpdateType::Message)`.

The HTTP side uses `axum::Router` with one `GET /` route mapped to `hello_world_handler`. `main` spawns three Tokio tasks and `tokio::join!`s them: `run_server` (`axum::serve(...).with_graceful_shutdown(...)`), `run_dispatcher` (`dispatcher.run_polling().with_graceful_shutdown(...)`), and `handle_shutdown`, which awaits telers' `shutdown_signal()` (from the `signal` feature) and then fires a shared `tokio::sync::Notify` to stop both servers.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package axum_and_echo_bot
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
