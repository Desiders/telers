# skip_updates

Demonstrates dropping updates that piled up while the bot was offline, so polling starts from a clean slate.

## What it does

Before it begins polling, the bot discards any pending updates that Telegram queued while it was down. Once running, it simply logs every incoming `Update` at info level and does nothing else with it.

## How it works

In `main`, before building the dispatcher, the bot calls the `DeleteWebhook` method with `.drop_pending_updates(true)`. This both removes any configured webhook and tells Telegram to drop the backlog of pending updates, so long-polling resumes only with fresh updates.

A single `Router` named `main` registers one handler on the `on_update` observer that logs the received `Update` with `tracing::info!`. The `Dispatcher` is built with `allowed_updates(UpdateType::all())` so all update types are received.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package skip_updates
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
