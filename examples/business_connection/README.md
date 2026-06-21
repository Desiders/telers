# business_connection

Demonstrates handling Telegram Business Account updates — the events delivered when a business account connects your bot and messages flow through it.

## What it does

- When a user connects (or disconnects) the bot to their business account, the bot logs the `BusinessConnection` at debug level.
- For each incoming business message, the bot logs it and replies `Hello world!` in the same chat, sending the reply through the originating business connection.
- Edited business messages and deleted-message notifications are also received and logged.

Replies and logs are visible at `RUST_LOG=debug`, since the handlers use `tracing::debug!`.

## How it works

The `Router` wires four business observers to dedicated handlers:

- `on_business_connection` -> `connection` (takes a `BusinessConnection`)
- `on_business_message` -> `message` (takes `Bot` and `Message`; replies with `SendMessage::new(...).business_connection_id(message.business_connection_id().unwrap())`)
- `on_edited_business_message` -> `message_edited`
- `on_deleted_business_messages` -> `messages_deleted` (takes a `BusinessMessagesDeleted`)

The dispatcher derives the update subscription from the router itself via `allowed_updates(router.resolve_used_update_types())` rather than listing them manually, then runs with `run_polling()`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package business_connection
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=debug`).
