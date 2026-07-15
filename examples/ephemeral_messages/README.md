# ephemeral_messages

Demonstrates telers' support for *ephemeral messages* — messages a bot sends in a group or supergroup that only a specific user and the bot can see.

## What it does

- `/whisper` sends the sender an ephemeral message only they can see, then edits it.
- `/cleanup` sends the sender an ephemeral message, then deletes it.

## How it works

A message becomes ephemeral when `SendMessage` is given `.receiver_user_id(user_id)`: it is then shown only to that user, and only in group and supergroup chats. The returned `Message` carries an `ephemeral_message_id`, which `EditEphemeralMessageText` and `DeleteEphemeralMessage` need to edit or remove the message afterwards.

`whisper_handler` and `cleanup_handler` are registered behind `Command::one(...)` on the `main` router's `on_message` observer; each reads the sender from `message.from()` and the chat from `message.chat().id()`. The `Dispatcher` is built with `allowed_update(UpdateType::Message)` and runs via `run_polling()`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package ephemeral_messages
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
