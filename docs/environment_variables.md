# Environment Configuration Options

Below is a detailed table of configuration options available for customizing the enshrouded server. These options can be set via environment variables when deploying the server.

| Variable Name                               | Description                                               | Default Value        |
| ------------------------------------------- | --------------------------------------------------------- | -------------------- |
| `DIFFICULTY`                                | Sets the game's overall difficulty level.                 | `None`               |
| `DAY_TIME_SPEED_RATE`                       | Controls the progression speed of daytime.                | `1.00000`            |
| `NIGHT_TIME_SPEED_RATE`                     | Controls the progression speed of nighttime.              | `1.000`              |
| `EXP_RATE`                                  | Multiplier for experience points gained by players.       | `1.0`                |
| `PAL_CAPTURE_RATE`                          | Multiplier affecting the capture rate of Pals.            | _Empty_              |
| `PAL_SPAWN_NUM_RATE`                        | Multiplier for the spawn rate of Pals.                    | `1`                  |
| `PAL_DAMAGE_RATE_ATTACK`                    | Multiplier for damage dealt by Pals to enemies.           | `1`                  |
| `PAL_DAMAGE_RATE_DEFENSE`                   | Multiplier for damage received by Pals from enemies.      | `1.000000`           |
| `PLAYER_DAMAGE_RATE_ATTACK`                 | Multiplier for damage dealt by the player to enemies.     | `1.000000`           |
| `PLAYER_DAMAGE_RATE_DEFENSE`                | Multiplier for damage received by the player.             | `1.000000`           |
| `PLAYER_STOMACH_DECREACE_RATE`              | Rate at which the player's hunger decreases.              | `1.000000`           |
| `PLAYER_STAMINA_DECREACE_RATE`              | Rate at which the player's stamina decreases.             | `1.000000`           |
| `PLAYER_AUTO_HP_REGENE_RATE`                | Rate of automatic health regeneration for the player.     | `1.000000`           |
| `PLAYER_AUTO_HP_REGENE_RATE_IN_SLEEP`       | Health regeneration rate for the player while sleeping.   | `2.000000`           |
| `BUILD_OBJECT_DAMAGE_RATE`                  | Damage rate to buildable objects.                         | `1.000000`           |
| `BUILD_OBJECT_DETERIORATION_DAMAGE_RATE`    | Deterioration rate of buildable objects.                  | `1.000000`           |
| `COLLECTION_DROP_RATE`                      | Drop rate multiplier for collectible items.               | `1.500000`           |
| `COLLECTION_OBJECT_HP_RATE`                 | Health points rate for collectible objects.               | `1.000000`           |
| `COLLECTION_OBJECT_RESPAWN_SPEED_RATE`      | Respawn speed rate for collectible objects.               | `1.000000`           |
| `ENEMY_DROP_ITEM_RATE`                      | Drop rate multiplier for items dropped by enemies.        | `1.200000`           |
| `DEATH_PENALTY`                             | Specifies the death penalty for players.                  | `None`               |
| `ENABLE_PLAYER_TO_PLAYER_DAMAGE`            | Enables or disables player-to-player damage.              | `False`              |
| `ENABLE_FRIENDLY_FIRE`                      | Enables or disables friendly fire.                        | `False`              |
| `ENABLE_INVADER_ENEMY`                      | Enables or disables invader enemies.                      | `True`               |
| `ACTIVE_UNKO`                               | Activates or deactivates UNKO.                            | `False`              |
| `ENABLE_AIM_ASSIST_PAD`                     | Enables or disables aim assist for gamepads.              | `True`               |
| `ENABLE_AIM_ASSIST_KEYBOARD`                | Enables or disables aim assist for keyboards.             | `False`              |
| `DROP_ITEM_MAX_NUM`                         | Maximum number of dropped items in the world.             | `3000`               |
| `DROP_ITEM_MAX_NUM_UNKO`                    | Specific setting related to UNKO items.                   | `100`                |
| `BASE_CAMP_MAX_NUM`                         | Maximum number of base camps.                             | `128`                |
| `BASE_CAMP_WORKER_MAX_NUM`                  | Maximum number of workers per base camp.                  | `15`                 |
| `DROP_ITEM_ALIVE_MAX_HOURS`                 | Hours a dropped item remains before disappearing.         | `1`                  |
| `AUTO_RESET_GUILD_NO_ONLINE_PLAYERS`        | Auto-resets a guild if no players are online.             | `False`              |
| `AUTO_RESET_GUILD_TIME_NO_ONLINE_PLAYERS`   | Time until a guild is reset if no players are online.     | `72`                 |
| `GUILD_PLAYER_MAX_NUM`                      | Maximum number of players in a guild.                     | `20`                 |
| `PAL_EGG_DEFAULT_HATCHING_TIME`             | Default hatching time for Pal eggs (in hours).            | `24`                 |
| `WORK_SPEED_RATE`                           | Work speed rate for crafting and building.                | `1.0`                |
| `IS_MULTIPLAY`                              | Enables or disables multiplayer mode.                     | `True`               |
| `IS_PVP`                                    | Enables or disables player vs. player combat.             | `False`              |
| `CAN_PICKUP_OTHER_GUILD_DEATH_PENALTY_DROP` | Allows picking up items dropped on death by other guilds. | `False`              |
| `ENABLE_NON_LOGIN_PENALTY`                  | Enables or disables penalty for not logging in.           | `True`               |
| `ENABLE_FAST_TRAVEL`                        | Enables or disables fast travel.                          | `True`               |
| `IS_START_LOCATION_SELECT_BY_MAP`           | Allows starting location to be selected by map.           | `True`               |
| `EXIST_PLAYER_AFTER_LOGOUT`                 | Keeps player in the world after logout.                   | `False`              |
| `ENABLE_DEFENSE_OTHER_GUILD_PLAYER`         | Enables or disables defense against other guild players.  | `False`              |
| `COOP_PLAYER_MAX_NUM`                       | Maximum number of cooperative players.                    | `16`                 |
| `SERVER_PLAYER_MAX_NUM`                     | Maximum number of players on the server.                  | `16`                 |
| `SERVER_NAME`                               | Name of the server.                                       | `My enshrouded Server` |
| `SERVER_DESCRIPTION`                        | Description of the server.                                | _Empty_              |
| `ADMIN_PASSWORD`                            | Password for server admin access.                         | _Empty_              |
| `SERVER_PASSWORD`                           | Password for server access by players.                    | _Empty_              |
| `PUBLIC_PORT`                               | Public port for the server.                               | `8211`               |
| `PUBLIC_IP`                                 | Public IP address for the server.                         | _Empty_              |
| `RCON_ENABLED`                              | Enables or disables RCON.                                 | `False`              |
| `RCON_PORT`                                 | Port for RCON access.                                     | `25575`              |
| `REGION`                                    | Geographic region of the server.                          | _Not applicable_     |
| `USE_AUTH`                                  | Enables or disables server authentication.                | `True`               |

_Note: The `PAL_CAPTURE_RATE` is intentionally left empty to allow for game defaults or custom modifications._
