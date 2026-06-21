# stats_incoming_updates_middleware

Demonstrates telers' middleware system by combining an outer middleware and an inner middleware that keep running counters and pass their values to a handler through the `Context`.

## What it does

Send the bot any message and it replies with a line like:

> Hello! Users sent me N updates and I processed M handlers successfully for them.

`N` is the total number of updates received so far; `M` is the number of handlers that finished successfully before this one. Because the handler counter is read before `next` runs, the reported `M` lags the incoming count by the current update.

## How it works

Two shared counters are kept as `Arc<AtomicUsize>` inside cloneable middleware structs:

- `IncomingUpdates` implements `OuterMiddleware`: on every update it increments the counter and inserts `"incoming_updates_counter"` into `request.context`, then returns `EventReturn::Finish` to let processing continue.
- `ProcessedHandlers` implements `InnerMiddleware`: it inserts the current `"processed_handlers_counter"` into the context, calls `next(request).await`, and increments the counter only after the handler returns successfully (a failing handler does not count).

Registration happens on the `Router`: `on_all` registers the inner middleware across every telegram observer, `on_update` registers the outer middleware on the update observer, and `on_message` registers the handler. The handler extracts `Bot`, `Update`, and `Context`, reads both counters with `context.get::<usize>(...)`, and replies via `SendMessage` when `update.chat()` is present. The `Dispatcher` allows all update types (`UpdateType::all()`) and runs with `run_polling()`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package stats_incoming_updates_middleware
```
