# Enshrouded Docker

Welcome to the ultimate Enshrouded Server toolkit! This guide details how to deploy and configure your server, with all settings—including game parameters—now fully overridable via environment variables.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Environment Variables](#environment-variables)
  - [General Settings](#general-settings)
  - [Game Settings](#game-settings)
  - [User Group Overrides](#user-group-overrides)
- [Docker Compose Setup](#docker-compose-setup)
- [Updating Server Settings](#updating-server-settings)
- [Contributions](#contributions)

---

## Prerequisites

- **Docker**
- **Docker Compose**

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

Below is an updated Docker Compose snippet that incorporates all of the above environment variable overrides. Customize it as needed:

```yaml
services:
  enshrouded:
    image: mbround18/enshrouded-docker:latest
    build:
      context: .
      dockerfile: Dockerfile
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
   Update your `docker-compose.yml` with any new variable values.

2. **Restart the Server:**  
   Run the following commands to apply changes:
   ```bash
   docker-compose down
   docker-compose up
   ```

This process ensures that your server is always running with the latest configuration overrides.

---

## Contributions

Contributions are welcome! If you encounter issues, have feature requests, or want to improve the codebase, please open an issue or submit a pull request.

---

## Relevant Links

- [Original README](citeturn0file1)
- [Game Settings Implementation]()

Happy hosting and may your adventures be epic!
