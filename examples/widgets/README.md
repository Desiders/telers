# widgets

A minimal HTML widget bot using `sendWidget` and `answerWidgetQuery`.

## What it does

Send `/start` in a private chat and the bot replies with a widget containing a **Say hello** button. Press it and the widget displays a greeting with your first name. While the request is pending, the button is disabled; a failed request displays its error code.

Use a Telegram client that supports HTML widgets. In production, widgets can currently be sent only to private chats and are automatically protected from forwarding.

## How it works

- `start_handler` is filtered with `CommandStart` and `ChatType::one(Private)`. It sends `SendWidget` with `.allow_queries(true)` and the HTML embedded from `src/widget.html` via `include_str!`.
- The client provides the `TelegramWidget` global. The button calls `TelegramWidget.request('hello')`, which sends a `widget_query` update to the bot.
- `widget_handler` extracts `WidgetQuery`, reads its `data`, and sends `AnswerWidgetQuery` with the greeting. The widget displays the answer's `result` using `textContent`.
- The `Dispatcher` receives `Message` and `WidgetQuery` updates through polling. Widget queries must be answered within 15 seconds.

See Telegram's [widget SDK](https://github.com/TelegramMessenger/widget-sandbox/blob/main/docs/sdk-v1.md#214-requests-to-the-bot) for the client API and the Bot API documentation for [`sendWidget`](https://core.telegram.org/bots/api#sendwidget) and [`answerWidgetQuery`](https://core.telegram.org/bots/api#answerwidgetquery).

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package widgets
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
