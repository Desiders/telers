# random_sticker

Demonstrates working with stickers and sticker sets in telers: reading an incoming sticker, fetching its sticker set, and replying with a randomly chosen sticker from that set.

## What it does

- Send `/start` or `/help` and the bot greets you and asks for a sticker.
- Send any sticker that belongs to a sticker pack, and the bot picks a random sticker from that same pack and sends it back.
- Send a sticker that has no sticker set, and the bot replies that it cannot find a set and asks for another sticker.
- Send anything that is not a sticker, and the bot replies `Please, send me any sticker.`

## How it works

A single `Router` named `main` registers three message handlers on the `on_message` observer via `registers([...])`:

- `start_handler` is filtered with `MessageType::one(Text)` and `Command::many(["help", "start"])`, replying with `SendMessage`.
- `sticker_handler` is filtered with `MessageType::one(Sticker)`. It reads `message.sticker.set_name()`, calls the `GetStickerSet` method, picks a random index with `rand::rng().random_range(..)` (from the `rand` crate's `RngExt`), and sends the chosen sticker with `SendSticker` using `InputFile::id(...)` of its `file_id`.
- `wrong_message_handler` uses `MessageType::one(Sticker).invert()` to catch all non-sticker messages.

The `Dispatcher` is configured with `allowed_update(UpdateType::Message)` so only message updates are polled. Handler argument types like `MessageText`, `MessageSticker`, and `Message` are extracted automatically.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package random_sticker
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
