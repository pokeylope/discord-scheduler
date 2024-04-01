# Discord Scheduler

A Discord bot for scheduling group events.

## Features

* Allows users to select available days and shows a summary of results
* Can specify blackout dates within the given date range that are not available as options
* Can specify a role that is allowed to respond when creating a scheduler

## Limitations

This bot was designed for personal use rather than trying to be a universal solution, and may or may not be useful for others.

* Can specify Saturdays + Sundays or any individual weekday when creating a scheduler; other combinations are not supported
* Number of date options is at most 25 due to Discord UI limitations

## Usage

Bot must be invited with `bot` and `applications.commands` scopes and must have "Read Messages/View Channels", "Send
Messages", and "Embed Links" permissions. To run, set `DISCORD_TOKEN` to the bot's auth token, either as an
environment variable or in a [`.env`](https://docs.rs/dotenv/latest/dotenv/) file.

### `/schedule create` Command

| Option | Description | Default |
| ------ | ----------- | ------- |
| `description` | Description of event | *Required* |
| `group` | Discord role of users allowed to respond | None (open to all) |
| `limit` | Number of dates to include as options | 25 |
| `skip` | Number of weeks before first available date option | 0 |
| `days` | Weekdays to include (any single day or "Saturday + Sunday") | Saturday + Sunday |

### `/schedule repost` Command

This command posts a new copy of an exising scheduler message, leaving the original in place. Responses can be submitted
using either message and both will get updated with any changes. This can be useful if the original message is too far
back in the channel history, because the ephemeral message created for submitting a response doesn't appear if the user
is scrolled too far back when it is sent. It can also be used to add a copy to a different channel, but note that only
one repost can exisit for each scheduler message; if it is reposted again, the previous repost will be deleted.

| Option | Description |
| ------ | ----------- |
| `id` | Discord message ID of existing scheduler |
