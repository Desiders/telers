# download_file

Demonstrates how telers downloads files sent to the bot with `Bot::download`: into memory via `FileDownload::bytes` and onto the file system via `FileDownload::to_path`.

## What it does

Send a photo to the bot and it replies with the size of the downloaded original in bytes. Send a document and the bot saves it to the current directory under its original name (or its unique file id when there is no name) and replies with the path. Any other message gets a hint to send a photo or a document. Only message updates are processed.

## How it works

- `photo_handler` (a `Handler` on the message observer, filtered with `MessageType::one(Photo)` and taking `MessagePhoto`) picks the last, largest `PhotoSize`, passes it to `bot.download(...)` and reads the whole file into memory with `.bytes()`. Any object with a `file_id` implements `FileIdGetter`, so the `PhotoSize` is passed directly.
- `document_handler` (filtered with `MessageType::one(Document)`, taking `MessageDocument`) derives the destination name from `Document::file_name`, falling back to `file_unique_id`, and streams the download straight to disk with `.to_path(...)`.
- `bot.download` calls `getFile` to resolve the file path and then streams the content from the Telegram file server; the result is a `FileDownload` stream, so nothing is buffered unless `.bytes()` asks for it. Errors are `DownloadErrorKind` and convert into `HandlerError` with `?`.
- `fallback_handler` answers everything else, using the inverted `MessageType::many([Photo, Document])` filter. The `Dispatcher` is built with `allowed_update(UpdateType::Message)` and run via `run_polling`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package download_file
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
