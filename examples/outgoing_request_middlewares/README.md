# outgoing_request_middlewares

Shows how to make middlewares run around every request a `Bot` sends to the Telegram API.

## What it does

- Registers `Logging` on the bot, ignoring `getUpdates`, because polling would otherwise report every incoming update.
- Builds the bot with `Bot::new(token).on_middleware(..)` and runs a normal echo bot.

## How it works

`Bot` holds a `SessionManager`, which wraps the inner `Session` and the registered middlewares. Every request goes through the chain before it is sent and after the response comes back.

A middleware is a named type implementing `Middleware`; it cannot be a closure, because the trait takes `self` by `Arc`.

A middleware is called with the bot id and the name of the called method, for example `sendMessage`. It does not see the parameters of the method, and it does not see the parsed result: it sees the response before it is parsed, so a Telegram error arrives as a successful result with an error status code in it.

Downloading a file is not covered by the chain: `getFile` goes through it, but the bytes are streamed by `Session::stream_content` straight from the inner session.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package outgoing_request_middlewares
```

The `Logging` middleware reports at the `DEBUG` level, so the example sets `info,telers=debug` as the default filter. Override it with `RUST_LOG` if you want to see something else.
