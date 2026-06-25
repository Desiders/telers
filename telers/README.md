<div align="center">

<h1><code>telers</code></h1>

<a href="https://docs.rs/telers">
<img src="https://img.shields.io/docsrs/telers?style=flat-square"/>
</a>
<a href="https://crates.io/crates/telers">
<img src="https://img.shields.io/crates/v/telers?style=flat-square"/>
</a>
<a href="https://core.telegram.org/bots/api">
<img src="https://img.shields.io/badge/Telegram%20Bot%20API-10.1-blue?style=flat-square&logo=telegram&label=Telegram%20Bot%20API"/>
</a>

<h3>
An asynchronous framework for Telegram Bot API written in Rust
</h3>

</div>

</p>

<b>Telers make it easy to create Telegram bots</b> in Rust.

Before you start, make sure that you have a basic understanding of the [Telegram Bot API](https://core.telegram.org/bots/api), because types and methods in the library have the same fields and types as in the documentation.

More information about this crate can be found in the [crate documentation][docs].

## Highlights
 - **Asynchronous**. Built on top of [Tokio](https://tokio.rs/), a powerful asynchronous runtime.
 - **Easy to use**. Provides a simple and intuitive API for creating bots.
 - **Based on** [aiogram](https://github.com/aiogram/aiogram/). Inspired by the framework written in Python and tries to provide a similar functionality.
 - **Routers**, **Middlewares**, **Filters** and **Handlers**. Provides a powerful system of routers, middlewares, filters and handlers to make your code more readable and maintainable, and simplify the creation of bots.
 - **Extractors**. Have similar system of extractors as in [axum](https://docs.rs/axum/latest/axum/extract/) and [actix](https://actix.rs/docs/extractors/).
 - **Smart filters**. Analog of "magic" filters in `aiogram`.

## Examples
 - [Echo bot][examples/echo_bot]. This example shows how to create an echo bot.
 - [Context][examples/context]. This example shows how to use context to save data.
 - [Extensions][examples/extensions]. This example shows how to use extensions to save data.
 - [Extractor][examples/extractor]. This example shows how to use extractor to extract data.
 - [Text formatting][examples/text_formatting]. This example shows how to format text.
 - [Text rendering][examples/text_rendering]. This example shows how to render a message's text and entities back to HTML or MarkdownV2.
 - [Smart filter][examples/smart_filter]. This example show how to use smart filter.
 - [Text case filters][examples/text_case_filters]. This example shows how to create text case filters.
 - [Stats updates middleware][examples/stats_incoming_updates_middleware]. This example shows how to create a middleware that count incoming updates.
 - [Input file][examples/input_file]. This example shows how to send files by the bot.
 - [Finite state machine][examples/fsm]. This example shows how to use a finite state machine (conversation).
 - [Router tree][examples/router_tree]. This example shows how to create a router tree.
 - [Bot http client][examples/bot_http_client]. This example shows how to set a custom bot HTTP client.
 - [Axum and echo bot][examples/axum_and_echo_bot]. This example shows how to create an echo bot and run it concurrently with polling `axum` server.
 - [Axum webhook][examples/axum_webhook]. This example shows how to setup webhooks for a bot using `axum` server.
 - [Managed bot][examples/managed_bot]. This example shows how to create a managed bot that echoes messages and handles managed bot creation events by spawning separate dispatchers with a shared shutdown signal.

You may consider checking out [this directory][examples] for more examples.

## Community
### Telegram
- 🇺🇸 [@telers_en](https://t.me/telers_en)
- 🇷🇺 [@telers_ru](https://t.me/telers_ru)

## License
This project is licensed under either of the following licenses, at your option:
 - [Apache License, Version 2.0][licence_apache]
 - [MIT License][licence_mit]

[examples]: https://github.com/Desiders/telers/tree/dev-1.x/examples
[examples/axum_and_echo_bot]: https://github.com/Desiders/telers/tree/dev-1.x/examples/axum_and_echo_bot
[examples/axum_webhook]: https://github.com/Desiders/telers/tree/dev-1.x/examples/axum_webhook
[examples/bot_http_client]: https://github.com/Desiders/telers/tree/dev-1.x/examples/bot_http_client
[examples/router_tree]: https://github.com/Desiders/telers/tree/dev-1.x/examples/router_tree
[examples/fsm]: https://github.com/Desiders/telers/tree/dev-1.x/examples/fsm
[examples/input_file]: https://github.com/Desiders/telers/tree/dev-1.x/examples/input_file
[examples/stats_incoming_updates_middleware]: https://github.com/Desiders/telers/tree/dev-1.x/examples/stats_incoming_updates_middleware
[examples/smart_filter]: https://github.com/Desiders/telers/tree/dev-1.x/examples/smart_filter
[examples/text_case_filters]: https://github.com/Desiders/telers/tree/dev-1.x/examples/text_case_filters
[examples/text_formatting]: https://github.com/Desiders/telers/tree/dev-1.x/examples/text_formatting
[examples/text_rendering]: https://github.com/Desiders/telers/tree/dev-1.x/examples/text_rendering
[examples/echo_bot]: https://github.com/Desiders/telers/tree/dev-1.x/examples/echo_bot
[examples/extractor]: https://github.com/Desiders/telers/tree/dev-1.x/examples/extractor
[examples/context]: https://github.com/Desiders/telers/tree/dev-1.x/examples/context
[examples/extensions]: https://github.com/Desiders/telers/tree/dev-1.x/examples/extensions
[examples/managed_bot]: https://github.com/Desiders/telers/tree/dev-1.x/examples/managed_bot
[licence_apache]: https://github.com/Desiders/telers/blob/dev-1.x/telers/LICENSE-APACHE
[licence_mit]: https://github.com/Desiders/telers/blob/dev-1.x/telers/LICENSE-MIT
[docs]: https://docs.rs/telers
