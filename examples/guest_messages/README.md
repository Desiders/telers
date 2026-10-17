# guest_messages

Receives `guest_message` updates and replies through `answerGuestQuery`.

Enable **Guest Mode** in your bot's settings in BotFather's Mini App, then run:

```bash
BOT_TOKEN=<your_bot_token> cargo run --package guest_messages
```

Send a message mentioning `@your_bot` in another non-secret chat. The bot replies with "Guest mode works!".
See Telegram's [guest bot guide](https://core.telegram.org/bots/features#guest-bots).
