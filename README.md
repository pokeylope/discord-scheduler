# Discord Scheduler

A Discord bot for scheduling group events.

## Features

* Allows users to select available days and shows a summary of results
* Can specify blackout dates within the given date range that are not available as options
* Can specify a role that is allowed to respond when creating a scheduler

## Limitations

This bot was designed for personal use rather than trying to be a universal solution, and may or may not be useful for others.

* Can specify Saturdays, Sundays, or both as options when creating a scheduler; other weekdays are not supported
* Date range is at most ten weeks due to Discord UI limitations

## Usage

Bot must be invited with `bot` and `applications.commands` scopes.

### `/schedule` Command

| Option | Description | Default |
| ------ | ----------- | ------- |
| `description` | Description of event | *Required* |
| `group` | Discord role of users allowed to respond | None (open to all) |
| `weeks` | Number of weeks to include as options | 10 |
| `skip` | Number of weeks before first available date option | 0 |
| `days` | Weekdays to include (Saturday, Sunday, or both) | Saturday + Sunday |
