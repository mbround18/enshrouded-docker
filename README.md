# Enshrouded Docker

Welcome to the ultimate Enshrouded Server toolkit! This guide details how to deploy and configure your server, with all settings—including game parameters—now fully overridable via environment variables.

## Migration Notice

> This project now supports Proton only. The Wine target was removed because it failed a cold-volume first-boot validation; it never reached a healthy `/ready` state after install. If you were previously using a Wine-based image or a tag that references `wine`, update to `mbround18/enshrouded-docker:proton-latest` and use a fresh volume or a newly provisioned host path. See [docs/migration-notice.md](./docs/migration-notice.md) for the migration steps.

---

## Table of Contents

- [Migration Notice](#migration-notice)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
  - [Option A: One-off `docker run`](#option-a-one-off-docker-run)
  - [Option B: Docker Compose (recommended)](#option-b-docker-compose-recommended)
- [Environment Variables](#environment-variables)
  - [General Settings](#general-settings)
  - [Game Settings](#game-settings)
  - [User Group Overrides](#user-group-overrides)
- [Docker Compose Setup](#docker-compose-setup)
- [Health Checks, Liveness & Readiness](#health-checks-liveness--readiness)
- [Updating Server Settings](#updating-server-settings)
- [Editing `enshrouded_server.json` by Hand](#editing-enshrouded_serverjson-by-hand)
- [Contributions](#contributions)

---

## Prerequisites

- **Docker** — [install guide](https://docs.docker.com/engine/install/) if you're new to it. Docker Compose ships with it (`docker compose`, no hyphen) on any recent install.
- **Linux kernel 6.14+ with the `ntsync` driver loaded** (`lsmod | grep ntsync`, device present at `/dev/ntsync`) — recommended for smoother Proton startup. Without it, Proton falls back to `fsync` for NT synchronization primitives, which is markedly less stable during the server's multithreaded startup and can crash before Steamworks finishes initializing. If your host doesn't have it (or you're not sure), just drop the `devices:` block from whichever compose example below you use — the server still runs, just less reliably at startup.

---

## Quick Start

> **New to Docker?** You don't need to clone this repository at all. Everything
> you need is a published image on Docker Hub and one file you create
> yourself. The `compose.yaml` at the root of *this* repo is a different
> thing — it's for people developing the project itself (it builds the image
> from source). Don't use it to run your server; use one of the two options
> below instead.

### Runtime

This project publishes the Proton image only. It is the only runtime we have
validated on a cold volume and confirmed reaches a healthy `/ready` state.

| Image tag                             | Runtime | Notes |
| -------------------------------------- | ------- | ----- |
| `mbround18/enshrouded-docker:proton-latest` | Proton | Supported and validated. Use this image for production. |

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
      LAUNCH_MODE: "proton"
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
| `STOP_DELAY`                 | Seconds to wait after sending the "stopping" webhook notification before actually stopping the server (requires `WEBHOOK_URL`)                                                                                | _(none)_                | `30`                          |
| `IP`                         | IP address the server binds to                                                                                                                                                                                  | `0.0.0.0`               | `0.0.0.0`                     |
| `GAME_PORT`                  | Game port                                                                                                                                                                                                        | `15636`                 | `15636`                       |
| `QUERY_PORT`                 | Query port                                                                                                                                                                                                       | `15637`                 | `15637`                       |
| `SLOT_COUNT`                 | Max number of player slots                                                                                                                                                                                      | `16`                     | `8`                            |
| `VOICE_CHAT_MODE`            | Voice chat mode (`Proximity` or `Global`)                                                                                                                                                                       | `Proximity`              | `Global`                      |
| `ENABLE_VOICE_CHAT`          | Flag to enable in-game voice chat                                                                                                                                                                               | `false`                  | `true`                        |
| `ENABLE_TEXT_CHAT`           | Flag to enable in-game text chat                                                                                                                                                                                | `false`                  | `true`                        |
| `GAME_SETTINGS_PRESET`       | Game settings preset. **Must be `Custom` for any `Game Settings` overrides below to take effect**                                                                                                              | `Default`                | `Custom`                      |
| `PUID`                       | UID the `steam` user runs as inside the container — match it to the owner of your mounted volume to avoid permission errors                                                                                    | `1000`                   | `1000`                        |
| `PGID`                       | GID the `steam` user runs as inside the container — match it to the owner of your mounted volume to avoid permission errors                                                                                    | `1000`                   | `1000`                        |
| `HTTP_PORT`                  | Port for the liveness/readiness endpoints (`/live`, `/ready`, `/health`). Set to `0` to disable. [See the guide.](./docs/health-checks.md)                                                       | `3000`                   | `8080`                        |
| `RUST_LOG`                   | Log verbosity for the CLI (`error`, `warn`, `info`, `debug`, `trace`)                                                                                                                                           | `info`                   | `debug`                       |

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
| `TAMING_STARTLE_REPERCUSSION`           | string | `LoseSomeProgress`     | `NoPenalty`      |
| `DAY_TIME_DURATION`                    | int    | `1800000000000`        | `1500000000000`  |
| `NIGHT_TIME_DURATION`                  | int    | `720000000000`         | `600000000000`   |
| `CURSE_MODIFIER`                       | string | `Normal`               | `Hard`           |

### User Group Overrides

Override user group settings in your configuration by prefixing with `SET_GROUP_`, followed by the group name and field name:

| Variable Pattern                               | Description                     | Example                                          |
| ---------------------------------------------- | ------------------------------- | ------------------------------------------------ |
| `SET_GROUP_<GROUPNAME>_PASSWORD`               | Overrides the group password    | `SET_GROUP_ADMIN_PASSWORD: "secret"`             |
| `SET_GROUP_<GROUPNAME>_CAN_KICK_BAN`           | Toggle kick/ban permission      | `SET_GROUP_ADMIN_CAN_KICK_BAN: "true"`           |
| `SET_GROUP_<GROUPNAME>_CAN_ACCESS_INVENTORIES` | Toggle inventory access         | `SET_GROUP_ADMIN_CAN_ACCESS_INVENTORIES: "true"` |
| `SET_GROUP_<GROUPNAME>_CAN_EDIT_BASE`          | Toggle base editing             | `SET_GROUP_FRIEND_CAN_EDIT_BASE: "true"`         |
| `SET_GROUP_<GROUPNAME>_CAN_EXTEND_BASE`        | Toggle base extending           | `SET_GROUP_FRIEND_CAN_EXTEND_BASE: "true"`       |
| `SET_GROUP_<GROUPNAME>_RESERVED_SLOTS`         | Slots reserved for this group   | `SET_GROUP_ADMIN_RESERVED_SLOTS: "2"`            |

The default config ships `Admin` and `Guest` groups, but `<GROUPNAME>` doesn't
have to be one of them — naming a group that doesn't exist yet creates it, so
`SET_GROUP_FRIEND_PASSWORD` gets you a `FRIEND` group with the default
permissions. Group names with underscores work too
(`SET_GROUP_MY_FRIENDS_PASSWORD` targets a group named `MY_FRIENDS`).

---

## Docker Compose Setup

Below is the full `compose.yaml` from [Quick Start](#option-b-docker-compose-recommended)
with every environment variable override from the tables above added in.
Trim it down to just the ones you actually want to change — anything you
omit just uses its default.

```yaml
services:
  enshrouded:
    image: mbround18/enshrouded-docker:proton-latest
    # Proton is the only supported runtime for this project.
    stop_grace_period: 120s
    environment:
      TZ: "America/Los_Angeles"
      NAME: "My Enshrouded Server"
      WEBHOOK_URL: "https://your-webhook.url"
      AUTO_UPDATE: "true"
      AUTO_UPDATE_SCHEDULE: "0 3 * * *"
      SCHEDULED_RESTART: "true"
      SCHEDULED_RESTART_SCHEDULE: "0 4 * * *"
      UPDATE_ON_START: "true"
      # Server settings (optional)
      IP: "0.0.0.0"
      GAME_PORT: "15636"
      QUERY_PORT: "15637"
      SLOT_COUNT: "16"
      VOICE_CHAT_MODE: "Proximity"
      ENABLE_VOICE_CHAT: "false"
      ENABLE_TEXT_CHAT: "false"
      GAME_SETTINGS_PRESET: "Default" # Must be "Custom" for the Game Settings overrides below to apply
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
      TAMING_STARTLE_REPERCUSSION: "LoseSomeProgress"
      DAY_TIME_DURATION: "1800000000000"
      NIGHT_TIME_DURATION: "720000000000"
      CURSE_MODIFIER: "Normal"
      # User Group Overrides (optional)
      SET_GROUP_ADMIN_PASSWORD: "YourAdminPassword"
      SET_GROUP_ADMIN_CAN_KICK_BAN: "true"
      SET_GROUP_ADMIN_CAN_ACCESS_INVENTORIES: "true"
      SET_GROUP_ADMIN_RESERVED_SLOTS: "2"
      # Health endpoints (optional)
      HTTP_PORT: "3000"
    ports:
      - "15636:15636/udp"
      - "15636:15636/tcp"
      - "15637:15637/udp"
      - "15637:15637/tcp"
      - "3000:3000/tcp" # /live, /ready, /health -- see docs/health-checks.md
    volumes:
      - ./data:/home/steam/enshrouded
```

---

## Health Checks, Liveness & Readiness

The container serves liveness and readiness endpoints on `HTTP_PORT`
(default `3000`) for Kubernetes, Docker, or your own monitoring:

| Route                  | `200 OK` when                                  |
| ---------------------- | ---------------------------------------------- |
| `/live`, `/liveness`   | the server process is running                  |
| `/ready`, `/readiness` | the game port is accepting connections         |
| `/health`              | both of the above                              |

A Docker `HEALTHCHECK` is built into the image already, so `docker ps` shows
`healthy`/`unhealthy` with no setup on your part. Publish `3000:3000` in your
compose file if you want to reach the endpoints from outside the container.

**[Full guide, including a ready-to-use Kubernetes StatefulSet →](./docs/health-checks.md)**

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

## Editing `enshrouded_server.json` by Hand

You can. Edit the file in your mounted volume and restart the container — your
changes stick, including settings this project doesn't know about (new ones
Enshrouded adds, or anything else you put in the file).

Two rules decide what happens on each start:

1. **An environment variable always wins** over the matching value in the
   file. If you set `SLOT_COUNT: "8"` in your compose file and then hand-edit
   `slotCount` to `16`, it goes back to `8` on the next restart. Remove the
   variable from your compose file if you'd rather manage that setting by
   hand.
2. **Everything else is left alone.** Fields no environment variable covers
   are never touched.

If the file can't be parsed (a stray comma, a missing brace), the container
says so in the logs and **leaves the file exactly as it is** rather than
replacing it with defaults — fix the JSON and restart. Before any rewrite, the
previous version is copied next to it as
`enshrouded_server.bak.<timestamp>.json`, and the five most recent backups are
kept.

---

## Contributions

Contributions are welcome! If you encounter issues, have feature requests, or want to improve the codebase, please open an issue or submit a pull request. See [docs/development.md](./docs/development.md) for how to build and run this project from source.

---

Happy hosting and may your adventures be epic!
