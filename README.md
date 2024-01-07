<div align="center">

<h1><code>telers</code></h1>

<a href="https://docs.rs/telers">
<img src="https://img.shields.io/docsrs/telers?style=flat-square"/>
</a>
<a href="https://crates.io/crates/telers">
<img src="https://img.shields.io/crates/v/telers?style=flat-square"/>
</a>
<a href="https://core.telegram.org/bots/api">
<img src="https://img.shields.io/badge/Telegram%20Bot%20API-7.0-blue?style=flat-square&logo=telegram&label=Telegram%20Bot%20API"/>
</a>

<h3>
An asynchronous framework for Telegram Bot API written in Rust
</h3>

</div>

</p>

<b>Telers make it easy to create Telegram bots</b> in Rust.

Before you start, make sure that you have a basic understanding of the [Telegram Bot API](https://core.telegram.org/bots/api), because types and methods in the library have the same fields and types as in the documentation.

## Highlights
 - **Asynchronous**. Built on top of [Tokio](https://tokio.rs/), a powerful asynchronous runtime.
 - **Easy to use**. Provides a simple and intuitive API for creating bots.
 - **Based on** [aiogram](https://github.com/aiogram/aiogram/). Inspired by the framework written in Python and tries to provide a similar functionality.
 - **Routers**, **Middlewares**, **Filters** and **Handlers**. Provides a powerful system of routers, middlewares, filters and handlers to make your code more readable and maintainable, and simplify the creation of bots.
 - **Extractors**. Have similar system of extractors as in [axum](https://docs.rs/axum/latest/axum/extract/) and [actix](https://actix.rs/docs/extractors/).

## Examples
 - [Echo bot](examples/echo_bot). This example shows how to create an echo bot.
 - [Text formatting](examples/text_formatting). This example shows how to format text.
 - [Text case filters](examples/text_case_filters). This example shows how to create text case filters.
 - [Stats updates middleware](examples/stats_incoming_updates_middleware). This example shows how to create a middleware that count incoming updates.
 - [Input file](examples/input_file). This example shows how to send files by the bot.
 - [Finite state machine](examples/finite_state_machine). This example shows how to use a finite state machine (conversation).
 - [Router tree](examples/router_tree). This example shows how to create a router tree.
 - [Bot http client](examples/bot_http_client). This example shows how to set a custom bot HTTP client.
 - [Axum and echo bot](examples/axum_and_echo_bot). This example shows how to create an echo bot and run it concurrently with polling `axum` server.

You may consider checking out [this directory](examples) for more examples.

## Community
### Telegram
- 🇺🇸 [@telers_en](https://t.me/telers_en)

## License
This project is licensed under either of the following licenses, at your option:
 - Apache License, Version 2.0
 - MIT License
