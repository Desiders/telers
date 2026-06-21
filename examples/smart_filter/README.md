# smart_filter

Demonstrates telers' built-in `SmartFilter`, a composable, declarative filter builder that inspects fields of an incoming `Message` without writing a custom filter type.

## What it does

The bot listens for messages and replies depending on what each message contains. Several handlers are registered on the same message observer, each guarded by a different `SmartFilter`:

- Text longer than 100 characters gets "Long message detected!".
- A message containing a bold entity gets "Message has bold entities!".
- A sticker gets "Sticker message detected!".
- All-uppercase text gets "Uppercase message!"; all-lowercase text gets "Lowercase message!".
- Text that is neither fully uppercase nor fully lowercase gets "Any case message!".

The long-message and bold-entity handlers call `skip_event()` so propagation continues to the next matching handler instead of stopping after the first match (the default).

## How it works

Handlers are wrapped with `Handler::new(...)` and registered together via `observer.registers([...])` on the message observer. Each one attaches a `SmartFilter`:

- `SmartFilter::text().len().gt(100)` — numeric comparison on text length.
- `SmartFilter::message().entities().matches(|entities| ...)` — a custom predicate over the message entities, here checking for `MessageEntityType::Bold`.
- `SmartFilter::sticker().is_some()` — presence of a sticker field.
- `SmartFilter::text().is_uppercase()` / `.is_lowercase()` — case checks.
- `SmartFilter::text().all().branch(...).branch(...)` with `.invert()` — combines two inverted case checks into a single "neither" filter.

The sticker handler uses a `MessageSticker` extractor directly; the others take `Message`. Each handler extracts `Bot` and replies with `SendMessage`. The `Dispatcher` is built with `allowed_update(UpdateType::Message)` and runs via `run_polling()`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package smart_filter
```
