# text_case_filters

Demonstrates writing custom handler filters in telers two different ways — as a type implementing the `Filter` trait and as a plain async function — and composing them with `.invert()`.

## What it does

Send the bot a text message and it classifies the casing:

- All-uppercase text gets "Uppercase message!".
- All-lowercase text gets "Lowercase message!".
- Anything else (mixed case) gets "Any case message!".

## How it works

Two filters are defined in different styles, then attached to handlers via `Handler::new(...).filter(...)`:

- `UppercaseFilter` is a struct implementing the `Filter` trait. Its `check` reads `request.update.text()` and returns whether the text equals its uppercased form. `type Error = Infallible`, so the check never fails.
- `lowercase_filter` is a free function returning a future of `FilterResult<Infallible>`, showing that a filter can just be an async closure/function rather than a type.

The third handler combines both: `UppercaseFilter.invert()` and `lowercase_filter.invert()` chained as two `.filter(...)` calls, so it matches only text that is neither all-uppercase nor all-lowercase. All three are registered together with `observer.registers([...])` on the message observer; each handler replies with `SendMessage`. The `Dispatcher` is built with `allowed_update(UpdateType::Message)` and runs via `run_polling()`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package text_case_filters
```
