# axum_webhook

Receives Telegram updates via a webhook served by `axum` instead of long polling, registering the webhook automatically on startup.

## What it does

- On startup, the bot calls `setWebhook` to point Telegram at `https://example.com/`, restricted to message updates and protected by a secret token.
- Incoming updates arrive as HTTP POSTs to the `axum` server on `0.0.0.0:3000` at path `/`, which verifies the secret token and feeds them to the dispatcher.
- For each message, the bot echoes it back to the same chat with `CopyMessage`.
- A termination signal shuts down the HTTP server and dispatcher together.

Note: `WEBHOOK_URL` (`https://example.com`), `HANDLER_PATH` (`/`), and `SECRET_TOKEN` (`123`) are hardcoded constants — edit them to point at your own publicly reachable HTTPS endpoint.

## How it works

The `Router` registers `echo_handler` on `on_message` and, on `on_startup`, a `simple::Handler` wrapping `set_webhook` (which sends `SetWebhook::new(...).allowed_update(...).secret_token_option(...)`). The webhook endpoint is built with `webhooks::axum::get_updates_router(UpdatesHandler::new(bot, dispatcher).secret_token(...))` (from the `webhooks-axum` feature) and mounted on an `axum::Router`.

Because updates are pushed, the dispatcher runs via `run_no_polling()` instead of polling. `main` spawns three tasks — `run_server`, `run_dispatcher`, and `handle_shutdown` — coordinated through a `tokio::sync::broadcast` channel; `handle_shutdown` awaits `shutdown_signal()` and broadcasts to stop both.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package axum_webhook
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
