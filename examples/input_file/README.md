# input_file

Demonstrates every way telers can supply a file to the Telegram Bot API via the `InputFile` type: by URL, by local filesystem path, by in-memory bytes, by an async byte stream, and by a previously-uploaded Telegram file id.

## What it does

Send any message to the bot and it replies with a media group of four copies of the same cat photo, each captioned by how it was provided (`Cat by URL`, `Cat by file system`, `Cat by bytes`, `Cat by stream`). It then takes the file id from the first uploaded photo and sends the image a fifth time captioned `Cat by telegram file ID`.

On startup the bot downloads `https://http.cat/images/200.jpg` and saves it locally as `cat.jpg`; on shutdown it deletes that file. Only message updates are processed.

## How it works

- `on_startup` and `on_shutdown` are `simple::Handler`s registered via `on_startup`/`on_shutdown`. The startup handler fetches the image with `reqwest` and writes it to `cat.jpg`; the shutdown handler removes it.
- `input_file_handler` (a `telegram::Handler` on the message observer) builds four `InputFile` values: `InputFile::url`, `InputFile::fs`, `InputFile::buffered` (from `tokio::fs::read` bytes), and `InputFile::stream` (a `FramedRead` over the file using `BytesCodec`, yielding `BytesMut::freeze` chunks).
- These are wrapped in `InputMediaPhoto`s and sent together via `SendMediaGroup`. The resulting `Message` carries the uploaded photo's `file_id`, which is reused with `InputFile::id` and `SendPhoto` to send the image once more by id.
- Errors are mapped to `HandlerError`. The `Dispatcher` is built with `allowed_update(UpdateType::Message)` and run via `run_polling`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package input_file
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
