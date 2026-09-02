# throttling

A telers bot that demonstrates per-chat-user throttling with the `Throttling` inner middleware.

## What it does

When a user sends messages faster than the configured rate (one message per 5 seconds), the bot skips the extra messages and sends "Too many requests!" — but only for the first two throttled requests in a row, so the user isn't spammed with warnings.

## How it works

`main` registers the `Throttling` inner middleware on the `on_message` observer with a 5-second rate and the `Strategy::UserInChat` key (each user is throttled separately in each chat). The `on_throttled` callback receives the request and `ThrottledInfo` (how many times the rate was exceeded and how long until the request would be allowed), and sends the notification message. The middleware itself logs every throttled request on the `info` level.

`echo_handler` copies every non-throttled message back into the same chat.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package throttling
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
