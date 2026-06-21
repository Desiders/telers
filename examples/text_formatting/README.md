# text_formatting

Demonstrates the three ways telers lets you build formatted message text, all using HTML parse mode (Markdown is also supported, but the two can't be mixed in one message).

## What it does

Send the bot any message and it replies with three messages that all render the same formatted content — bold text, italic text, and a link to `example.com` — each produced by a different technique.

## How it works

The single handler extracts `Bot` and `Message` and sends three `SendMessage` requests, each with `.parse_mode(ParseMode::HTML)`:

1. **Raw markup** — a string literal containing `<b>`, `<i>`, and `<a href>` tags written by hand.
2. **`Builder` (`TextBuilder`)** — a fluent builder constructed over `HTMLFormatter`, chaining `.text(...)`, `.bold(...)`, `.italic(...)`, and `.text_link(...)`, then producing the final string with `.get_text()`.
3. **`HTMLFormatter` directly** — calling `html.bold(...)` / `html.italic(...)` and the standalone helper `html_text_link(...)` inside a `format!`, showing the short free functions mirror the formatter methods.

The `Router` registers one handler on the message observer; the `Dispatcher` is built with `allowed_update(UpdateType::Message)` and runs via `run_polling()`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package text_formatting
```
