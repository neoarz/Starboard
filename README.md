# Starboard-4
A self-hosted Starboard bot for one Discord server.

 - [Documentation](https://docs.starboard.best)
 - [Source](https://github.com/neoarz/Starboard)

## Features
 - Multiple starboards per server
 - Multiple emojis per starboard
 - 25+ configurable options for starboards
 - Per-channel setting overrides
 - Per-role permissions
 - Autostar channels

## Quickstart
 - Use `/starboards create name: starboard-name channel: #starboard` to create a starboard.
 - Use `/starboards view name: starboard-name` to view the settings.
 - Use `/starboards edit [behavior|requirements|style|embed] name: starboard-name [options...]` to edit a starboard.

## Self-hosting
Copy `.env.example` to `.env`, fill in the required values, then start Starboard and PostgreSQL with Docker Compose:

```sh
cp .env.example .env
docker compose up -d
```

The required `.env` values are:

```env
DISCORD_TOKEN=
BOT_ID=
GUILD_ID=
POSTGRES_PASSWORD=
```

`DISCORD_TOKEN` is your bot token, `BOT_ID` is your bot application's user ID, `GUILD_ID` is the only Discord server ID the bot should run in, and `POSTGRES_PASSWORD` can be any strong password for the local Compose database. The Compose setup reads `.env` automatically, creates a persistent PostgreSQL volume, and passes the database settings to Starboard. You only need to set `SB_DATABASE_URL` yourself when connecting to an existing external database.

## Migration from Starboard-3
If you're already hosting starboard-3 and want to switch to starboard-4, you have to alter the database structure.

Start by running `psql postgresql`. Then you can run:
```sql
CREATE DATABASE starboard_4 WITH OWNER <starboard username> TEMPLATE <old database name>;
```

Now exit, and then run `psql starboard_4 < migrate.sql` where `migrate.sql` is the file found in this repo.

After doing this, you should be able to launch starboard-4.
