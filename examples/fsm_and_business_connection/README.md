# fsm_and_business_connection

Demonstrates the same FSM-driven onboarding conversation as the sibling [`fsm`](../fsm) example, but running over a Telegram **business connection** instead of ordinary chat messages.

## What it does

Runs the same multi-step dialog as [`fsm`](../fsm): `/start` asks for your name, your reply is stored and the bot asks for your native language, and an `english`/`en` answer finishes the dialog while anything else asks you to pick another language. The difference is that the flow operates on business messages — messages sent through an account connected to the bot via Telegram Business.

## How it works

The structure mirrors [`fsm`](../fsm) with three business-connection-specific changes:

- Handlers are registered on the router's `on_business_message` observer (not `on_message`), and the `Dispatcher` is built with `allowed_update(UpdateType::BusinessMessage)`.
- The `FSMContext` middleware uses `Strategy::UserInChatAndConnection`, so FSM state is keyed by user, chat, and business connection — keeping business-connection conversations isolated from regular ones.
- Every outgoing `SendMessage` is tagged with `.business_connection_id(...)`, read from `message.business_connection_id()`, so replies are delivered through the same business connection.

Everything else — the `State` enum, the `MemoryStorage`-backed FSM, the `Command`/`MessageType`/`State` filters, and the `set_state`/`set_value`/`get_value`/`finish` calls — is identical to [`fsm`](../fsm).

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package fsm_and_business_connection
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
