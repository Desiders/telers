# echo_bot

A minimal telers bot that echoes every message back — the smallest end-to-end example of a router, a handler, and a polling dispatcher.

## What it does

When a user sends any message, the bot copies it straight back into the same chat, so it appears as if the message was sent again by the bot.

## How it works

`echo_handler(bot: Bot, message: Message)` builds the reply with the `Message::to_copy_message(chat_id)` convenience method (which produces a `CopyMessage` targeting the same chat) and dispatches it with `bot.send(...)`.

`main` reads the token via `Bot::from_env`, creates a `Router` named `"main"` registering the handler on its `on_message` observer, and builds a `Dispatcher` from `router.configure_default()` with `allowed_update(UpdateType::Message)`. The bot then runs with `run_polling()`, logging on stop.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package echo_bot
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
