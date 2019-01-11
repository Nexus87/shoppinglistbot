# shoppinglistbot

Telegram Bot for adding tasks to Todoist projects

## Building

```
cargo build
```

## Running

```
cargo run
```
Needed env variables:
* **TELEGRAM_BOT_TOKEN** Api token for telegram
* **TODOIST_TOKEN** Api token for todoist
* **PROJECT_ID** Id of the todoist project, that should be used
* **CLIENT_IDS** Comma separated list of telegram client ids, the bot should listen to
