# deployments-tracking-bot

Bot for tracking new deployments on DigitalOcean and sending notifications to messengers (at the moment, only Telegram available).

## Installation

1. Fork this repository to your folder
1. Talk to [@BotFather](https://t.me/botfather) and go through some dialog options until you've successfully created a bot. You should receive a token in the format of `123456789:blablabla`
1. Copy `.env.example` to `.env`
1. Edit `.env` by changing the environment variables (see below)
1. Run the project with `cargo run`

## Environment Variables

### Work configuration

- `WORK_INTERVAL` - how often to check in seconds. The recommended range is 15-60 seconds.

### DigitalOcean configuration

- `DO_TOKEN` - You can generate an OAuth token by visiting the [Applications & API](https://cloud.digitalocean.com/account/api/tokens) section of the DigitalOcean control panel for your account. Set scope to READ only value.

### Database configuration

- `DB_PATH` - the folder where the database will be located

### Messages configuration

- `MSG_PATH` - message template folder

### Telegram configuration

- `TG_TOKEN` - the token you received when you created the bot
- `TG_CHAT_ID` - the ID of the chat you added the bot to. See below.

### Logging

- `LOG_SHOW_DATETIME` - set to `true` if you want the time and date to be displayed in the logs, `false` if you don't.
- `LOG_LEVEL` - [log level](https://docs.rs/simplelog/latest/simplelog/enum.LevelFilter.html#variants). Recommended values: `debug` for debugging and `info` for working in production.
