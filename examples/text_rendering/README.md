# text_rendering

The inverse of [`text_formatting`](../text_formatting): instead of building formatted text to send, it takes a message you *received* — its plain text plus the separate entity list Telegram delivers — and renders it back into a single HTML or MarkdownV2 string you could store and re-send later.

## What it does

Send the bot a message containing some formatting (bold, italic, a link, a custom emoji, …) and it replies with:

1. the **HTML source** of your message,
2. the **MarkdownV2 source** of your message, and
3. the message **re-rendered to HTML and sent back with HTML parse mode**, reproducing your original formatting (a round trip).

If you send a message with no text, it replies with a short hint instead.

## How it works

The single handler extracts `Bot` and `Message` and uses the rendering API three ways:

1. **`message.html_text()`** — a helper method on `Message` (generated alongside `to_copy_message` etc.) that renders the text and its entities to HTML, returning `None` when the message has no text. The result is sent **without** a parse mode, so the produced `<b>…</b>` markup is visible as plain text.
2. **`message.markdown_text()`** — the same helper for MarkdownV2.
3. **`Renderer` directly** — `Renderer::new(text, entities).as_html()`, the lower-level type that works on any text + entity slice rather than a whole `Message`. The output is re-sent with `.parse_mode(ParseMode::HTML)` to show it reproduces the original formatting.

The `Router` registers one handler on the message observer; the `Dispatcher` is built with `allowed_update(UpdateType::Message)` and runs via `run_polling()`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package text_rendering
```
