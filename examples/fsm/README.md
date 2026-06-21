# fsm

Demonstrates a finite state machine (FSM) in telers: driving a multi-step conversation by storing per-user state and data, and routing each incoming message to the handler registered for the current state.

## What it does

Runs a small onboarding dialog:

1. Send `/start` and the bot asks `Hello! What's your name?`.
2. Reply with any text; the bot stores it and asks for your native language.
3. Reply with your language. If it is `english`/`en` the bot answers `<name>, let's talk!` and ends the conversation; otherwise it asks you to choose another language and stays in the language step.

Only message updates are processed.

## How it works

- A `State` enum (`Name`, `Language`) models the conversation step; it implements `AsRef<str>` (so states can be persisted) and `PartialEq<&str>` (so they can be compared in filters).
- The FSM is wired up by registering `FSMContext` (the outer middleware, aliased `FSMContextMiddleware`) on the router's update observer, backed by `MemoryStorage` and the `Strategy::UserInChat` keying strategy.
- Handlers receive a `Fsm` (`FSMContext<MemoryStorage>`) argument and call `set_state`, `set_value`/`get_value`, and `finish` to advance the dialog and stash the user's name between steps.
- Routing is done with filters on the message observer: `start_handler` uses `Command::one("start")` plus `StateFilter::none()` (no active state); `name_handler` and `language_handler` use `MessageType::one(Text)` plus `StateFilter::one(State::Name)` / `StateFilter::one(State::Language)`.
- The `Dispatcher` is built with `allowed_update(UpdateType::Message)` and run via `run_polling`. `MemoryStorage` is convenient for testing but does not persist across restarts; any `Storage` implementation can be swapped in.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package fsm
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
