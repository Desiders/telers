# managed_bot

Demonstrates telers' support for *managed bots* (bots managed by your main bot), spinning up a separate `Dispatcher` at runtime for each managed bot that connects.

## What it does

- Run the main bot and connect it to a managed bot. When Telegram reports a new managed bot, the main bot fetches that bot's token and starts polling it in the background.
- Any message sent to a managed bot is echoed back (copied) to the same chat.
- When a managed bot is first created, the bot replies with `Managed bot created`.

> **Warning:** This example does not persist the tokens it retrieves for managed bots. Every managed bot's token is fetched live and used only for the lifetime of the process. It is for demonstration purposes only and is not suitable as-is for production; store tokens in persistent storage and run the bots in a separate process.

## How it works

The main `Router` named `main` registers two `on_message` handlers and one `on_managed_bot` handler:

- `managed_bot_created_handler` is filtered with `MessageType::one(enums::MessageType::ManagedBotCreated)` and answers via `SendMessage`.
- `echo_handler` copies the incoming `Message` back with `message.to_copy_message(...)`.
- `managed_bot` runs on the managed-bot observer. It calls the `GetManagedBotToken` method to obtain the new bot's token, builds a fresh `Bot`, and uses a `DispatcherBuilder` (shared through an `Extension`) to construct a new `Dispatcher` for it.

The new dispatcher is launched with `tokio::spawn` so the main dispatcher keeps polling. A shared `Arc<Notify>` (also passed via `Extension`) drives graceful shutdown: `shutdown_signal()` triggers `notify_waiters()`, and every dispatcher runs under `with_graceful_shutdown`, so the managed-bot dispatchers stop when the main bot stops. `resolve_used_update_types()` configures `allowed_updates` automatically.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package managed_bot
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
