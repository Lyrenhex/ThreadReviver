# ThreadReviver
A Discord bot to revive any archived threads (to counter Discord's auto-archive function).

This bot functions by automatically sending a message to any threads which archive whilst the bot is running (which it can see), and immediately attempts to delete the message afterwards.

## Environment variables

- `THREADREVIVER_DISCORD_TOKEN`: a bot token from the [Discord Developer Portal]

## Usage instructions

1. Install the bot: `cargo install thread_reviver`
    - Alternatively, build it yourself with `cargo build`
2. Create a bot on the [Discord Developer Portal]
3. Set the `THREADREVIVER_DISCORD_TOKEN` environment variable
4. Invite the bot to your server
    - You'll need to create an invitation link for this; see [Discord's documentation][Discord Developer Portal].
5. Run the bot.


[Discord Developer Portal]: https://discord.com/developers
