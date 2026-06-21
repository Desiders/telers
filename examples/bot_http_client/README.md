# bot_http_client

Shows how to plug a custom HTTP client into a `Bot` by implementing the telers `Session` trait, instead of relying on the default client.

## What it does

- Defines `CustomClient`, a type that implements `Session` and reports the production Telegram API server (`telegram::PRODUCTION`).
- Builds the bot with `Bot::with_client(token, CustomClient::default())` so it uses that client.
- Registers an echo handler that would copy each incoming message back with `CopyMessage`.

Important: this example is a structural skeleton — `CustomClient::send_request` is `unimplemented!()`, so the bot will panic the moment it actually tries to talk to Telegram. It demonstrates the wiring (the `Session` trait surface and how to inject a client), not a working network client. Use the default client or fill in `send_request` to make it functional.

## How it works

`CustomClient` implements `Session::api` (returning a borrowed `telegram::APIServer`) and `Session::send_request`. The token is read from `BOT_TOKEN` via `std::env::var` and passed to `Bot::with_client`. The handler is written generically as `echo_handler(bot: Bot<impl Session>, message: Message)`, so it works over any `Session` implementation. The `Router` registers it on `on_message`, and the `Dispatcher` runs with `allowed_update(UpdateType::Message)` via `run_polling()`.

A handler can take the client either generically (`Bot<impl Session>`) or as a concrete type (`Bot<CustomClient>`).

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package bot_http_client
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
