# router_tree

Demonstrates composing multiple `Router`s into a tree with `include`, and attaching an outer middleware to one branch of that tree.

## What it does

This is an echo bot built from nested routers:

- In a private chat, send `/start` and the bot greets you with `Hello! I'm echo bot that will repeat all your messages!`.
- Any other message is echoed back to the same chat.
- Send `/stats` or `/statistics` and the bot reports how many updates the echo branch has handled so far.

## How it works

Three routers are wired into a tree:

- `private_router` (named `private`) filters its `on_message` observer with `ChatType::one(Private)` and registers `start_private` behind `Command::one("start")`, replying with `SendMessage`.
- `echo_router` (named `echo`) registers an outer middleware and two handlers: `stats_echo_router` behind `Command::many(["stats", "statistics"])`, and a catch-all `echo_handler` that re-sends the message with `CopyMessage`.
- `main_router` (named `main`) uses `.include(private_router)` and `.include(echo_router)`; updates not handled by an earlier router fall through to the next.

The middleware `IncomingEchoRouterUpdates` implements `OuterMiddleware`. It is registered on the echo router's `on_update` observer, increments a shared `Arc<AtomicUsize>` counter for every update reaching that branch, and stores the count in the `Context` under `incoming_echo_router_updates_counter` (read back via `context.get::<usize>(..)`). It returns `(request, EventReturn::Finish)`. `allowed_updates` is derived from `resolve_used_update_types()`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package router_tree
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
