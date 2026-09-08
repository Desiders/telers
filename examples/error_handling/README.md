# error_handling

Handles the errors of handlers in one place with the `error` observer, the way `aiogram` does it with error handlers, instead of wrapping every handler body.

## What it does

- `/age <age>` replies with the age, or fails with a custom `InvalidAge` error when the argument is missing or isn't a number.
- `/name <name>` replies with the name, or fails with a custom `InvalidName` error when the argument is missing.
- `InvalidAge` errors are answered in the chat by a dedicated error handler that receives the error already downcast to its type.
- Any other error whose text starts with `Invalid` (so `InvalidName`) is answered in the chat with the text of the error.
- Every remaining error, for example a failed request to the Telegram API, is only logged.

## How it works

The message handlers return `Err(HandlerError::new(...))` with the custom errors. When the propagation of an update fails, the dispatcher propagates the error event to the `error` observer of the router, and the error handlers are picked by their filters like any other handlers: `ErrorType::<InvalidAge>` passes only for that error type, `ErrorMessage::one(Regex::new("^Invalid"))` matches the text of the error, and the last handler has no filter.

The update that caused the error is still the update of the request, so the error handlers take the `Message` and `Bot` arguments as usual. The error itself is taken as `EventError(err): EventError<InvalidAge>` when its type is known (the type must be `Clone`), or as `err: EventErrorKind` when it doesn't matter. If an error handler finishes, its response replaces the failed one; if no error handler handles the error, the result of the propagation stays as it was.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package error_handling
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
