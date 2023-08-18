<div align="center">

<h1><code>telers</code></h1>

<h3>
An asynchronous framework for Telegram Bot API written in Rust
</h3>

</div>

</p>

<b>Telers make it easy to create Telegram bots</b> in Rust.

Make sure you have a basic understanding of the [Telegram Bot API](https://core.telegram.org/bots/api) before you start, because **all types and methods in telers have the same fields and types as in Telegram Bot API**.

## Highlights
 - **Asynchronous**. Telers is built on top of [Tokio](https://tokio.rs/), a powerful asynchronous runtime for Rust.
 - **Easy to use**. Telers provides a simple and intuitive API to create Telegram bots.
 - **Based on** [aiogram](https://github.com/aiogram/aiogram). Telers is inspired by [aiogram](https://github.com/aiogram/aiogram), a Python framework for Telegram Bot API. Telers tries to provide the same API as aiogram, so if you know aiogram, you can easily start using this framework.
 - **Middlewares**, **Filters** and **Handlers**. Telers provides a powerful system of middlewares, filters and handlers. You can use middlewares to modify incoming/outgoing updates (logging, database connections, etc.), filters to filter incoming updates and handlers to handle incoming updates.
 - **Powerful extractors**. Telers provides a simple system of extractors. You can use extractors to extract data from incoming updates and context (middlewares, filters, etc.), and pass it to your handlers directly.
 - **Multiple bots**. Telers allows you to create multiple bots in one application without any problems.

## Examples
 - [Echo bot](examples/echo_bot). This example shows how to create a simple echo bot.
 - [Axum and echo bot](examples/axum_and_echo_bot). This example shows how to create a simple echo bot and run it parallel with polling axum server.
 - [Finite state machine](examples/finite_state_machine). This example shows how to implement a simple finite state machine (conversation).
 - [Bot http client](examples/bot_http_client). This example shows how to set a custom bot HTTP client.
 - [Uppercase filter](examples/uppercase_filter). This example shows how to create a simple uppercase filter.
 - [Stats incoming updates middleware](examples/stats_incoming_updates_middleware). This example shows how to create a simple middleware that counter incoming updates and processed handlers.

You may consider checking out [this directory](examples) for more examples.

## Community
### Telegram
- 🇺🇸 [@telers_en](https://t.me/telers_en)

## License
This project is licensed under either of the following licenses, at your option:
 - Apache License, Version 2.0
 - MIT License
