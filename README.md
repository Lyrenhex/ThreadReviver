# ThreadReviver
A Discord bot to revive any archived threads (to counter Discord's auto-archive function).

> This has been superseded by [Loki](https://github.com/Lyrenhex/Loki).

## Environment variables

- `THREADREVIVER_DISCORD_TOKEN`: a bot token from the [Discord Developer Portal]

## Discord permissions

The bot requires the `MANAGE_THREADS` Discord permission to operate. Any channels it does not have this permission for will not be revivable by the bot, and these will be printed to `STDERR` at runtime to assist with permissions management.

Note that version `0.1.0` required `SEND_MESSAGES_IN_THREADS` instead, so your bot may no longer be setup correctly -- do check! (This permission is also no longer required.)

## Usage instructions

1. Install the bot: `cargo install thread_reviver`
    - Alternatively, build it yourself with `cargo build`
2. Create a bot on the [Discord Developer Portal]
3. Set the `THREADREVIVER_DISCORD_TOKEN` environment variable
4. Invite the bot to your server
    - You'll need to create an invitation link for this; see [Discord's documentation][Discord Developer Portal].
5. Run the bot.


[Discord Developer Portal]: https://discord.com/developers
