# callback_data

Demonstrates telers' callback data support: packing a struct into the `callback_data` string of an inline keyboard button with the `CallbackData` derive macro, filtering callback queries by it with the `CallbackData` filter, and receiving the unpacked struct directly as a handler argument.

## What it does

Send `/start` and the bot replies with `Counter: 0` and two inline buttons, `-1` and `+1`. Pressing a button edits the message with the new value and rebuilds the keyboard, so the counter can be changed as many times as you like. Message and callback query updates are processed.

## How it works

- `Counter { value, step }` derives `CallbackData` with `#[callback_data(prefix = "counter")]`. The macro generates `pack`, which turns the struct into `counter:<value>:<step>` (e.g. `counter:5:-1`), and `unpack`, which parses it back. It also implements `Extractor`, so the struct can be used as a handler argument. The prefix identifies the type, so every callback data type needs its own.
- `counter_keyboard` packs the current value with steps `-1` and `1` into two `InlineKeyboardButton`s and returns an `InlineKeyboardMarkup`. `pack` returns `CallbackDataError` if a value contains the separator or the string is longer than 64 bytes; the error converts into `HandlerError` with `?`.
- `start_handler` (a `Handler` on the message observer, filtered with `CommandStart::default()`) sends the initial message with the keyboard.
- `counter_handler` is registered on the callback query observer with `CallbackDataFilter::<Counter>::new()` (the `CallbackData` filter from `telers::filters`). The filter passes only callback queries whose data unpacks into `Counter` and puts the result in the context, so the handler takes `Counter { value, step }: Counter` as an argument. It edits the message via `EditMessageText` with the new value and keyboard, and answers the query with `AnswerCallbackQuery` to stop the loading indicator. If the message is inaccessible (`callback_query.message` is `None`), editing is skipped.
- The `Dispatcher` is built with `allowed_updates([UpdateType::Message, UpdateType::CallbackQuery])` and run via `run_polling`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package callback_data
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
