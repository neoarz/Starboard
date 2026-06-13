# Starboard-4
A feature-rich and reliable Starboard bot, trusted by thousands of servers.

 - [Invite Starboard](https://discord.com/api/oauth2/authorize?client_id=700796664276844612&permissions=275683339328&scope=applications.commands%20bot)
 - [Get Support](https://discord.gg/3gK8mSA)
 - [Documentation](https://starboard.best)

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
You're welcome to self-host this bot if you like, as well as fork it and add your own changes. If there are features you wish the main bot had, you're also welcome to open PRs or make an issue requesting features.

Currently, I offer support for self-hosting the bot if you get stuck - just join the support server, and create a thread with the "Self Hosting" tag.

This guide assumes that you already have a VPS to host the bot on. Some good low-cost providers are https://www.netcup.eu (what the main bot uses) and https://alphavps.com.

Copy `.env.example` to `.env`, fill in the required values, then start Starboard and PostgreSQL with Docker Compose:

```sh
cp .env.example .env
docker compose up -d
```

The required `.env` values are:

```env
DISCORD_TOKEN=
BOT_ID=
POSTGRES_PASSWORD=
```

`DISCORD_TOKEN` is your bot token, `BOT_ID` is your bot application's user ID, and `POSTGRES_PASSWORD` can be any strong password for the local Compose database. The Compose setup reads `.env` automatically, creates a persistent PostgreSQL volume, and passes the database settings to Starboard. You only need to set `SB_DATABASE_URL` yourself when connecting to an existing external database.

To use the pre-built Docker image from Docker Hub instead, pull `circuitsacul/starboard:latest` and start the bot with `docker run -d --env-file .env --network=host circuitsacul/starboard:latest`.

## Migration from Starboard-3
If you're already hosting starboard-3 and want to switch to starboard-4, you have to alter the database structure.

Start by running `psql postgresql`. Then you can run:
```sql
CREATE DATABASE starboard_4 WITH OWNER <starboard username> TEMPLATE <old database name>;
```

Now exit, and then run `psql starboard_4 < migrate.sql` where `migrate.sql` is the file found in this repo.

After doing this, you should be able to launch starboard-4.
