# Enshrouded Docker

Welcome to the ultimate Enshrouded Server toolkit! This guide details how to deploy and configure your server, with all settings—including game parameters—now fully overridable via environment variables.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
  - [Proton or Wine?](#proton-or-wine)
  - [Option A: One-off `docker run`](#option-a-one-off-docker-run)
  - [Option B: Docker Compose (recommended)](#option-b-docker-compose-recommended)
- [Environment Variables](#environment-variables)
  - [General Settings](#general-settings)
  - [Game Settings](#game-settings)
  - [User Group Overrides](#user-group-overrides)
- [Docker Compose Setup](#docker-compose-setup)
- [Updating Server Settings](#updating-server-settings)
- [Contributions](#contributions)

---

## Prerequisites

- **Docker** — [install guide](https://docs.docker.com/engine/install/) if you're new to it. Docker Compose ships with it (`docker compose`, no hyphen) on any recent install.
- **Linux kernel 6.14+ with the `ntsync` driver loaded** (`lsmod | grep ntsync`, device present at `/dev/ntsync`) — recommended for smoother startup. Without it, Wine/Proton fall back to `fsync` for NT synchronization primitives, which is markedly less stable during the server's multithreaded startup and can crash before Steamworks finishes initializing. If your host doesn't have it (or you're not sure), just drop the `devices:` block from whichever compose example below you use — the server still runs, just less reliably at startup.

---

## Quick Start

> **New to Docker?** You don't need to clone this repository at all. Everything
> you need is a published image on Docker Hub and one file you create
> yourself. The `compose.yaml` at the root of *this* repo is a different
> thing — it's for people developing the project itself (it builds the image
> from source). Don't use it to run your server; use one of the two options
> below instead.

### Proton or Wine?

This project publishes two image variants that run the Windows server binary
on Linux through a different compatibility layer. Functionally they're the
same server — pick one:

| Image tag                             | Runtime  | Notes                                                              |
| -------------------------------------- | -------- | ------------------------------------------------------------------- |
| `mbround18/enshrouded-docker:proton-latest` | Proton   | Steam's own compatibility layer. **Recommended default.**          |
| `mbround18/enshrouded-docker:wine-latest`   | Wine     | Good fallback if Proton gives you trouble on your host.            |

If in doubt, use `proton-latest`.

### Option A: One-off `docker run`

Fastest way to try it out. Replace `~/enshrouded-data` with wherever you want
your save files to live on the host:

```bash
docker run -d \
  --name enshrouded \
  -p 15636:15636/tcp -p 15636:15636/udp \
  -p 15637:15637/tcp -p 15637:15637/udp \
  -v ~/enshrouded-data:/home/steam/enshrouded \
  -e NAME="My Enshrouded Server" \
  -e SET_GROUP_ADMIN_PASSWORD="change-me" \
  -e SET_GROUP_GUEST_PASSWORD="change-me-too" \
  --stop-timeout 120 \
  mbround18/enshrouded-docker:proton-latest
```

`--stop-timeout 120` matters: the server needs time to save the world when you
stop the container, and 120 seconds gives it enough room to do that safely
instead of getting killed mid-save.

### Option B: Docker Compose (recommended)

Create a new, empty folder for your server (e.g. `~/enshrouded-server/`), and
save this as `compose.yaml` inside it:

```yaml
services:
  enshrouded:
    image: mbround18/enshrouded-docker:proton-latest
    # See "Option A" above for why this isn't the default 10s.
    stop_grace_period: 120s
    environment:
      TZ: "America/Los_Angeles"
      NAME: "My Enshrouded Server"
      SET_GROUP_ADMIN_PASSWORD: "change-me"
      SET_GROUP_GUEST_PASSWORD: "change-me-too"
    ports:
      - "15636:15636/tcp"
      - "15636:15636/udp"
      - "15637:15637/tcp"
      - "15637:15637/udp"
    volumes:
      - ./data:/home/steam/enshrouded
```

Then, from that same folder:

```bash
docker compose up -d    # start it in the background
docker compose logs -f  # watch it boot (first run downloads/installs the game — be patient)
docker compose down     # stop it (gracefully, see stop_grace_period above)
```

Your save files, logs, and server config end up in `./data` next to your
`compose.yaml`. See [Environment Variables](#environment-variables) below for
everything else you can set, and [docs/compose-with-backups.md](./docs/compose-with-backups.md)
to add automatic backups on top of this.

---

## Dont forget to backup your saves!

we highly recommend you back up your save files! [Click here to see how to integrate auto backups.](./docs/compose-with-backups.md)

Below is a modernized README that now reflects the full spectrum of environment variable configuration—including game settings defined in `/src/game_settings.rs`—and an updated Docker Compose example. Review the tables below for a quick reference to all configurable options and how to override them.

## Environment Variables

### General Settings

These variables control the overall server configuration:

| Variable                     | Description                                                                                                                                                                                                     | Default Value          | Example Value                |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- | ---------------------------- |
| `TZ`                         | Timezone for the server (Unix only)                                                                                                                                                                             | `America/Los_Angeles`  | `Europe/London`              |
| `NAME`                       | Server name                                                                                                                                                                                                     | `My Enshrouded Server` | `Epic Dungeons`              |
| `WEBHOOK_URL`                | URL for webhook notifications, this can be a discord webhook! [Click here for a guide on setting up discord webhooks for a channel.](https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks) | _(none)_               | `https://hooks.example.com/` |
| `AUTO_UPDATE`                | Flag to enable automatic server updates                                                                                                                                                                         | _(disabled)_           | `true`                       |
| `AUTO_UPDATE_SCHEDULE`       | Cron schedule for auto-update                                                                                                                                                                                   | `0 3 * * *`            | `30 2 * * *`                 |
| `SCHEDULED_RESTART`          | Flag to enable scheduled restarts                                                                                                                                                                               | _(disabled)_           | `true`                       |
| `SCHEDULED_RESTART_SCHEDULE` | Cron schedule for automatic server restarts                                                                                                                                                                     | `0 4 * * *`            | `15 4 * * *`                 |
| `UPDATE_ON_START`            | Flag to enable server update on startup                                                                                                                                                                         | _(disabled)_           | `true`                       |

### Game Settings

> NOTE!!!! To set custom values you MUST change PRESET to Custom!

The in-game configuration parameters are now env configurable. Use environment variables matching the field names (in uppercase with underscores) to override defaults in your `enshrouded_server.json`.
[Check out the full list of server settings over here.](https://enshrouded.zendesk.com/hc/en-us/articles/20453241249821-Server-Gameplay-Settings) Any setting can be configured via an env variable but the key must be in SCREAMING_SNAKE_CASE format.

| Setting Name                           | Type   | Default Value          | Example Override |
| -------------------------------------- | ------ | ---------------------- | ---------------- |
| `PLAYER_HEALTH_FACTOR`                 | float  | `1.0`                  | `1.5`            |
| `PLAYER_MANA_FACTOR`                   | float  | `1.0`                  | `1.2`            |
| `PLAYER_STAMINA_FACTOR`                | float  | `1.0`                  | `0.8`            |
| `PLAYER_BODY_HEAT_FACTOR`              | float  | `1.0`                  | `0.9`            |
| `ENABLE_DURABILITY`                    | bool   | `true`                 | `false`          |
| `ENABLE_STARVING_DEBUFF`               | bool   | `false`                | `true`           |
| `FOOD_BUFF_DURATION_FACTOR`            | float  | `1.0`                  | `1.3`            |
| `FROM_HUNGER_TO_STARVING`              | int    | `600000000000`         | `500000000000`   |
| `SHROUD_TIME_FACTOR`                   | float  | `1.0`                  | `1.1`            |
| `TOMBSTONE_MODE`                       | string | `AddBackpackMaterials` | `KeepItems`      |
| `ENABLE_GLIDER_TURBULENCES`            | bool   | `true`                 | `false`          |
| `WEATHER_FREQUENCY`                    | string | `Normal`               | `Frequent`       |
| `MINING_DAMAGE_FACTOR`                 | float  | `1.0`                  | `1.5`            |
| `PLANT_GROWTH_SPEED_FACTOR`            | float  | `1.0`                  | `0.8`            |
| `RESOURCE_DROP_STACK_AMOUNT_FACTOR`    | float  | `1.0`                  | `1.2`            |
| `FACTORY_PRODUCTION_SPEED_FACTOR`      | float  | `1.0`                  | `1.3`            |
| `PERK_UPGRADE_RECYCLING_FACTOR`        | float  | `0.5`                  | `0.7`            |
| `PERK_COST_FACTOR`                     | float  | `1.0`                  | `0.9`            |
| `EXPERIENCE_COMBAT_FACTOR`             | float  | `1.0`                  | `1.2`            |
| `EXPERIENCE_MINING_FACTOR`             | float  | `1.0`                  | `1.1`            |
| `EXPERIENCE_EXPLORATION_QUESTS_FACTOR` | float  | `1.0`                  | `1.4`            |
| `RANDOM_SPAWNER_AMOUNT`                | string | `Normal`               | `High`           |
| `AGGRO_POOL_AMOUNT`                    | string | `Normal`               | `Large`          |
| `ENEMY_DAMAGE_FACTOR`                  | float  | `1.0`                  | `1.2`            |
| `ENEMY_HEALTH_FACTOR`                  | float  | `1.0`                  | `1.3`            |
| `ENEMY_STAMINA_FACTOR`                 | float  | `1.0`                  | `1.0`            |
| `ENEMY_PERCEPTION_RANGE_FACTOR`        | float  | `1.0`                  | `1.5`            |
| `BOSS_DAMAGE_FACTOR`                   | float  | `1.0`                  | `1.8`            |
| `BOSS_HEALTH_FACTOR`                   | float  | `1.0`                  | `2.0`            |
| `THREAT_BONUS`                         | float  | `1.0`                  | `1.2`            |
| `PACIFY_ALL_ENEMIES`                   | bool   | `false`                | `true`           |
| `TAMING_STARTLE_REPERUSSION`           | string | `LoseSomeProgress`     | `NoPenalty`      |
| `DAY_TIME_DURATION`                    | int    | `1800000000000`        | `1500000000000`  |
| `NIGHT_TIME_DURATION`                  | int    | `720000000000`         | `600000000000`   |

### User Group Overrides

Override user group settings in your configuration by prefixing with `SET_GROUP_`, followed by the group name and field name:

| Variable Pattern                               | Description                  | Example                                          |
| ---------------------------------------------- | ---------------------------- | ------------------------------------------------ |
| `SET_GROUP_<GROUPNAME>_PASSWORD`               | Overrides the group password | `SET_GROUP_ADMIN_PASSWORD: "secret"`             |
| `SET_GROUP_<GROUPNAME>_CAN_KICK_BAN`           | Toggle kick/ban permission   | `SET_GROUP_ADMIN_CAN_KICK_BAN: "true"`           |
| `SET_GROUP_<GROUPNAME>_CAN_ACCESS_INVENTORIES` | Toggle inventory access      | `SET_GROUP_ADMIN_CAN_ACCESS_INVENTORIES: "true"` |

---

## Docker Compose Setup

Below is the full `compose.yaml` from [Quick Start](#option-b-docker-compose-recommended)
with every environment variable override from the tables above added in.
Trim it down to just the ones you actually want to change — anything you
omit just uses its default.

```yaml
services:
  enshrouded:
    image: mbround18/enshrouded-docker:proton-latest # or :wine-latest
    stop_grace_period: 120s
    environment:
      TZ: "America/Los_Angeles"
      NAME: "My Enshrouded Server"
      WEBHOOK_URL: "https://your-webhook.url"
      AUTO_UPDATE: "true"
      AUTO_UPDATE_SCHEDULE: "0 3 * * *"
      SCHEDULED_RESTART: "true"
      SCHEDULED_RESTART_SCHEDULE: "0 4 * * *"
      # Game Settings Overrides (optional)
      PLAYER_HEALTH_FACTOR: "1.0"
      PLAYER_MANA_FACTOR: "1.0"
      PLAYER_STAMINA_FACTOR: "1.0"
      PLAYER_BODY_HEAT_FACTOR: "1.0"
      ENABLE_DURABILITY: "true"
      ENABLE_STARVING_DEBUFF: "false"
      FOOD_BUFF_DURATION_FACTOR: "1.0"
      FROM_HUNGER_TO_STARVING: "600000000000"
      SHROUD_TIME_FACTOR: "1.0"
      TOMBSTONE_MODE: "AddBackpackMaterials"
      ENABLE_GLIDER_TURBULENCES: "true"
      WEATHER_FREQUENCY: "Normal"
      MINING_DAMAGE_FACTOR: "1.0"
      PLANT_GROWTH_SPEED_FACTOR: "1.0"
      RESOURCE_DROP_STACK_AMOUNT_FACTOR: "1.0"
      FACTORY_PRODUCTION_SPEED_FACTOR: "1.0"
      PERK_UPGRADE_RECYCLING_FACTOR: "0.5"
      PERK_COST_FACTOR: "1.0"
      EXPERIENCE_COMBAT_FACTOR: "1.0"
      EXPERIENCE_MINING_FACTOR: "1.0"
      EXPERIENCE_EXPLORATION_QUESTS_FACTOR: "1.0"
      RANDOM_SPAWNER_AMOUNT: "Normal"
      AGGRO_POOL_AMOUNT: "Normal"
      ENEMY_DAMAGE_FACTOR: "1.0"
      ENEMY_HEALTH_FACTOR: "1.0"
      ENEMY_STAMINA_FACTOR: "1.0"
      ENEMY_PERCEPTION_RANGE_FACTOR: "1.0"
      BOSS_DAMAGE_FACTOR: "1.0"
      BOSS_HEALTH_FACTOR: "1.0"
      THREAT_BONUS: "1.0"
      PACIFY_ALL_ENEMIES: "false"
      TAMING_STARTLE_REPERUSSION: "LoseSomeProgress"
      DAY_TIME_DURATION: "1800000000000"
      NIGHT_TIME_DURATION: "720000000000"
      # User Group Overrides (optional)
      SET_GROUP_ADMIN_PASSWORD: "YourAdminPassword"
      SET_GROUP_ADMIN_CAN_KICK_BAN: "true"
      SET_GROUP_ADMIN_CAN_ACCESS_INVENTORIES: "true"
    ports:
      - "15636:15636/udp"
      - "15636:15636/tcp"
      - "15637:15637/udp"
      - "15637:15637/tcp"
    volumes:
      - ./data:/home/steam/enshrouded
```

---

## Updating Server Settings

To update your server settings after the initial setup:

1. **Modify Environment Variables:**  
   Update the `compose.yaml` you created in [Quick Start](#option-b-docker-compose-recommended) with any new variable values.

2. **Restart the Server:**  
   From that same folder, run:
   ```bash
   docker compose down
   docker compose up -d
   ```

This process ensures that your server is always running with the latest configuration overrides.

---

## Contributions

Contributions are welcome! If you encounter issues, have feature requests, or want to improve the codebase, please open an issue or submit a pull request. See [docs/development.md](./docs/development.md) for how to build and run this project from source.

---

Happy hosting and may your adventures be epic!
