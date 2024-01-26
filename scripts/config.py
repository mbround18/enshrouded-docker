import os
from argparse import ArgumentParser
from jinja2 import Template


# Define presets
presets = {
    "casual": {
        "day_time_speed_rate": "1",
        "night_time_speed_rate": "1",
        "exp_rate": "2",
        "pal_capture_rate": "2",
        "pal_spawn_num_rate": "1",
        "pal_damage_rate_attack": "2",
        "pal_damage_rate_defense": "0.5",
        "pal_stomach_decreace_rate": "0.5",
        "pal_stamina_decreace_rate": "0.5",
        "pal_auto_hp_regene_rate": "2",
        "pal_auto_hp_regene_rate_in_sleep": "2",
        "player_damage_rate_attack": "2",
        "player_damage_rate_defense": "0.5",
        "player_stomach_decreace_rate": "0.3",
        "player_stamina_decreace_rate": "0.3",
        "player_auto_hp_regene_rate": "2",
        "player_auto_hp_regene_rate_in_sleep": "2",
        "build_object_damage_rate": "2",
        "build_object_deterioration_damage_rate": "0.2",
        "collection_drop_rate": "3",
        "collection_object_hp_rate": "0.5",
        "collection_object_respawn_speed_rate": "0.5",
        "enemy_drop_item_rate": "2",
        "pal_egg_default_hatching_time": "24",
    },
    "normal": {
        "day_time_speed_rate": "1",
        "night_time_speed_rate": "1",
        "exp_rate": "1",
        "pal_capture_rate": "1",
        "pal_spawn_num_rate": "1",
        "pal_damage_rate_attack": "1",
        "pal_damage_rate_defense": "1",
        "pal_stomach_decreace_rate": "1",
        "pal_stamina_decreace_rate": "1",
        "pal_auto_hp_regene_rate": "1",
        "pal_auto_hp_regene_rate_in_sleep": "1",
        "player_damage_rate_attack": "1",
        "player_damage_rate_defense": "1",
        "player_stomach_decreace_rate": "1",
        "player_stamina_decreace_rate": "1",
        "player_auto_hp_regene_rate": "1",
        "player_auto_hp_regene_rate_in_sleep": "1",
        "build_object_damage_rate": "1",
        "build_object_deterioration_damage_rate": "1",
        "collection_drop_rate": "1",
        "collection_object_hp_rate": "1",
        "collection_object_respawn_speed_rate": "1",
        "enemy_drop_item_rate": "1",
        "pal_egg_default_hatching_time": "72",
        "enable_raid_events": "Enable",
        "death_penalty": "Drop All Items",
        "max_number_of_guilds": "20",
        "gatherable_items_multiplier": "1",
        "gatherable_objects_hp_multiplier": "1",
        "gatherable_objects_respawn_interval": "1",
        "dropped_items_multiplier": "1",
    },
    "hard": {
        "day_time_speed_rate": "1",
        "night_time_speed_rate": "1",
        "exp_rate": "0.5",
        "pal_capture_rate": "1",
        "pal_spawn_num_rate": "1",
        "pal_damage_rate_attack": "0.5",
        "pal_damage_rate_defense": "2",
        "pal_stomach_decreace_rate": "1.5",
        "pal_stamina_decreace_rate": "1.5",
        "pal_auto_hp_regene_rate": "0.5",
        "pal_auto_hp_regene_rate_in_sleep": "0.5",
        "player_damage_rate_attack": "0.7",
        "player_damage_rate_defense": "4",
        "player_stomach_decreace_rate": "1",
        "player_stamina_decreace_rate": "1",
        "player_auto_hp_regene_rate": "0.6",
        "player_auto_hp_regene_rate_in_sleep": "0.6",
        "build_object_damage_rate": "0.7",
        "build_object_deterioration_damage_rate": "1",
        "collection_drop_rate": "0.8",
        "collection_object_hp_rate": "1",
        "collection_object_respawn_speed_rate": "2",
        "enemy_drop_item_rate": "0.7",
        "pal_egg_default_hatching_time": "72",
        "enable_raid_events": "Enable",  # Assuming this translates to a boolean or specific handling in the script
        "death_penalty": "Drop all Items and all Pals on Team",  # Assuming handling of this textual description in your script
        "max_number_of_guilds": "20",  # Assuming this is directly applicable
        "gatherable_items_multiplier": "0.8",
        "gatherable_objects_hp_multiplier": "1",
        "gatherable_objects_respawn_interval": "2",
        "dropped_items_multiplier": "0.7",
    },
}

default_values = {
    "difficulty": "None",
    "day_time_speed_rate": "1.000000",
    "night_time_speed_rate": "1.000000",
    "exp_rate": "1.000000",
    "pal_capture_rate": "1.000000",
    "pal_spawn_num_rate": "1.000000",
    "pal_damage_rate_attack": "1.000000",
    "pal_damage_rate_defense": "1.000000",
    "pal_stomach_decreace_rate": "1.000000",
    "pal_stamina_decreace_rate": "1.000000",
    "pal_auto_hp_regene_rate": "1.000000",
    "pal_auto_hp_regene_rate_in_sleep": "2.000000",
    "player_damage_rate_attack": "1.000000",
    "player_damage_rate_defense": "1.000000",
    "player_stomach_decreace_rate": "1.000000",
    "player_stamina_decreace_rate": "1.000000",
    "player_auto_hp_regene_rate": "1.000000",
    "player_auto_hp_regene_rate_in_sleep": "2.000000",
    "build_object_damage_rate": "1.000000",
    "build_object_deterioration_damage_rate": "1.000000",
    "collection_drop_rate": "1.500000",
    "collection_object_hp_rate": "1.000000",
    "collection_object_respawn_speed_rate": "1.000000",
    "enemy_drop_item_rate": "1.200000",
    "death_penalty": "None",
    "b_enable_player_to_player_damage": "False",
    "b_enable_friendly_fire": "False",
    "b_enable_invader_enemy": "True",
    "b_active_unko": "False",
    "enable_raid_events": "Enable",
    "enable_aim_assist_pad": "True",
    "enable_aim_assist_keyboard": "False",
    "drop_item_max_num": "3000",
    "drop_item_max_num_unko": "3000",
    "base_camp_max_num": "128",
    "base_camp_worker_max_num": "15",
    "drop_item_alive_max_hours": "1",
    "auto_reset_guild_no_online_players": "False",
    "auto_reset_guild_time_non_online_players": "72",
    "guild_player_max_num": "20",
    "pal_egg_default_hatching_time": "24",
    "work_speed_rate": "1.0",
    "is_multiplay": "True",
    "is_pvp": "False",
    "can_pickup_other_guild_death_penalty_drop": "False",
    "enable_non_login_penalty": "True",
    "enable_fast_travel": "True",
    "is_start_location_select_by_map": "True",
    "exist_player_after_logout": "False",
    "enable_defense_other_guild_player": "False",
    "coop_player_max_num": "16",
    "server_player_max_num": "16",
    "server_name": "My enshrouded Server",
    "server_description": "",
    "admin_password": "",
    "server_password": "",
    "public_port": "8211",
    "public_ip": "",
    "rcon_enabled": "False",
    "rcon_port": "25575",
    "use_auth": "True",
}


def get_args():
    # Initialize the parser
    parser = ArgumentParser()
    # Add the parameters positional/optional
    parser.add_argument("-o", "--output", help="Output destination of the config.ini")
    parser.add_argument(
        "--preset",
        choices=["casual", "normal", "hard"],
        help="Preset configurations: casual, normal, hard",
        default=None,
    )
    parser.add_argument("-d", "--dry-run", help="Dry run and print to stdout")
    parser.add_argument(
        "--difficulty",
        help="Game difficulty setting",
        default=os.getenv("DIFFICULTY", None),
    )
    parser.add_argument(
        "--day_time_speed_rate",
        help="Day time speed rate",
        default=os.getenv("DAY_TIME_SPEED_RATE", None),
    )
    parser.add_argument(
        "--night_time_speed_rate",
        help="Night time speed rate",
        default=os.getenv("NIGHT_TIME_SPEED_RATE", None),
    )
    parser.add_argument(
        "--exp_rate", help="Experience rate", default=os.getenv("EXP_RATE", None)
    )
    parser.add_argument(
        "--pal_capture_rate",
        help="Pal capture rate",
        default=os.getenv("PAL_CAPTURE_RATE", None),
    )
    parser.add_argument(
        "--pal_spawn_num_rate",
        help="Pal spawn number rate",
        default=os.getenv("PAL_SPAWN_NUM_RATE", None),
    )
    parser.add_argument(
        "--pal_damage_rate_attack",
        help="Pal damage rate for attack",
        default=os.getenv("PAL_DAMAGE_RATE_ATTACK", None),
    )
    parser.add_argument(
        "--pal_damage_rate_defense",
        help="Pal damage rate for defense",
        default=os.getenv("PAL_DAMAGE_RATE_DEFENSE", None),
    )
    parser.add_argument(
        "--player_damage_rate_attack",
        help="Player damage rate for attack",
        default=os.getenv("PLAYER_DAMAGE_RATE_ATTACK", None),
    )
    parser.add_argument(
        "--player_damage_rate_defense",
        help="Player damage rate for defense",
        default=os.getenv("PLAYER_DAMAGE_RATE_DEFENSE", None),
    )
    parser.add_argument(
        "--player_stomach_decreace_rate",
        help="Player stomach decrease rate",
        default=os.getenv("PLAYER_STOMACH_DECREACE_RATE", None),
    )
    parser.add_argument(
        "--player_stamina_decreace_rate",
        help="Player stamina decrease rate",
        default=os.getenv("PLAYER_STAMINA_DECREACE_RATE", None),
    )
    parser.add_argument(
        "--player_auto_hp_regene_rate",
        help="Player auto HP regeneration rate",
        default=os.getenv("PLAYER_AUTO_HP_REGENE_RATE", None),
    )
    parser.add_argument(
        "--player_auto_hp_regene_rate_in_sleep",
        help="Player auto HP regeneration rate in sleep",
        default=os.getenv("PLAYER_AUTO_HP_REGENE_RATE_IN_SLEEP", None),
    )
    parser.add_argument(
        "--pal_stomach_decreace_rate",
        help="Pal stomach decrease rate",
        default=os.getenv("PAL_STOMACH_DECREACE_RATE", None),
    )
    parser.add_argument(
        "--pal_stamina_decreace_rate",
        help="Pal stamina decrease rate",
        default=os.getenv("PAL_STAMINA_DECREACE_RATE", None),
    )
    parser.add_argument(
        "--pal_auto_hp_regene_rate",
        help="Pal auto HP regeneration rate",
        default=os.getenv("PAL_AUTO_HP_REGENE_RATE", None),
    )
    parser.add_argument(
        "--pal_auto_hp_regene_rate_in_sleep",
        help="Pal auto HP regeneration rate in sleep",
        default=os.getenv("PAL_AUTO_HP_REGENE_RATE_IN_SLEEP", None),
    )
    parser.add_argument(
        "--build_object_damage_rate",
        help="Build object damage rate",
        default=os.getenv("BUILD_OBJECT_DAMAGE_RATE", None),
    )
    parser.add_argument(
        "--build_object_deterioration_damage_rate",
        help="Build object deterioration damage rate",
        default=os.getenv("BUILD_OBJECT_DETERIORATION_DAMAGE_RATE", None),
    )
    parser.add_argument(
        "--collection_drop_rate",
        help="Collection drop rate",
        default=os.getenv("COLLECTION_DROP_RATE", None),
    )
    parser.add_argument(
        "--collection_object_hp_rate",
        help="Collection object HP rate",
        default=os.getenv("COLLECTION_OBJECT_HP_RATE", None),
    )
    parser.add_argument(
        "--collection_object_respawn_speed_rate",
        help="Collection object respawn speed rate",
        default=os.getenv("COLLECTION_OBJECT_RESPAWN_SPEED_RATE", None),
    )
    parser.add_argument(
        "--enemy_drop_item_rate",
        help="Enemy drop item rate",
        default=os.getenv("ENEMY_DROP_ITEM_RATE", None),
    )
    parser.add_argument(
        "--death_penalty",
        help="Death penalty setting",
        default=os.getenv("DEATH_PENALTY", None),
    )
    parser.add_argument(
        "--b_enable_player_to_player_damage",
        help="Enable player to player damage",
        default=os.getenv("B_ENABLE_PLAYER_TO_PLAYER_DAMAGE", None),
    )
    parser.add_argument(
        "--b_enable_friendly_fire",
        help="Enable friendly fire",
        default=os.getenv("B_ENABLE_FRIENDLY_FIRE", None),
    )
    parser.add_argument(
        "--b_enable_invader_enemy",
        help="Enable invader enemy",
        default=os.getenv("B_ENABLE_INVADER_ENEMY", None),
    )
    parser.add_argument(
        "--b_active_unko",
        help="Active UNKO setting",
        default=os.getenv("B_ACTIVE_UNKO", None),
    )

    parser.add_argument(
        "--enable_aim_assist_pad",
        help="Enable aim assist for pad",
        default=os.getenv("ENABLE_AIM_ASSIST_PAD", None),
    )
    parser.add_argument(
        "--enable_aim_assist_keyboard",
        help="Enable aim assist for keyboard",
        default=os.getenv("ENABLE_AIM_ASSIST_KEYBOARD", None),
    )

    parser.add_argument(
        "--drop_item_max_num",
        help="Maximum number of dropped items",
        default=os.getenv("DROP_ITEM_MAX_NUM", None),
    )
    # Note: DropItemMaxNum_UNKO seems to be a fixed value and might not need an argument unless you want it configurable
    parser.add_argument(
        "--base_camp_max_num",
        help="Maximum number of base camps",
        default=os.getenv("BASE_CAMP_MAX_NUM", None),
    )
    parser.add_argument(
        "--base_camp_worker_max_num",
        help="Maximum number of workers per base camp",
        default=os.getenv("BASE_CAMP_WORKER_MAX_NUM", None),
    )
    parser.add_argument(
        "--drop_item_alive_max_hours",
        help="Maximum hours a dropped item remains",
        default=os.getenv("DROP_ITEM_ALIVE_MAX_HOURS", None),
    )
    parser.add_argument(
        "--auto_reset_guild_no_online_players",
        help="Auto-reset guild with no online players",
        default=os.getenv("AUTO_RESET_GUILD_NO_ONLINE_PLAYERS", None),
    )
    parser.add_argument(
        "--auto_reset_guild_time_non_online_players",
        help="Time until auto-reset for guild with no online players (hours)",
        default=os.getenv("AUTO_RESET_GUILD_TIME_NON_ONLINE_PLAYERS", None),
    )
    parser.add_argument(
        "--guild_player_max_num",
        help="Maximum number of players in a guild",
        default=os.getenv("GUILD_PLAYER_MAX_NUM", None),
    )
    parser.add_argument(
        "--pal_egg_default_hatching_time",
        help="Default hatching time for pal eggs (hours)",
        default=os.getenv("PAL_EGG_DEFAULT_HATCHING_TIME", None),
    )
    parser.add_argument(
        "--work_speed_rate",
        help="Work speed rate",
        default=os.getenv("WORK_SPEED_RATE", None),
    )
    parser.add_argument(
        "--is_multiplay",
        help="Is multiplay enabled",
        default=os.getenv("IS_MULTIPLAY", None),
    )
    parser.add_argument(
        "--is_pvp", help="Is PvP enabled", default=os.getenv("IS_PVP", None)
    )
    parser.add_argument(
        "--can_pickup_other_guild_death_penalty_drop",
        help="Can pick up drops from other guilds' death penalties",
        default=os.getenv("CAN_PICKUP_OTHER_GUILD_DEATH_PENALTY_DROP", None),
    )
    parser.add_argument(
        "--enable_non_login_penalty",
        help="Enable penalty for not logging in",
        default=os.getenv("ENABLE_NON_LOGIN_PENALTY", None),
    )
    parser.add_argument(
        "--enable_fast_travel",
        help="Enable fast travel",
        default=os.getenv("ENABLE_FAST_TRAVEL", None),
    )
    parser.add_argument(
        "--is_start_location_select_by_map",
        help="Is start location selected by map",
        default=os.getenv("IS_START_LOCATION_SELECT_BY_MAP", None),
    )
    parser.add_argument(
        "--exist_player_after_logout",
        help="Does player exist after logout",
        default=os.getenv("EXIST_PLAYER_AFTER_LOGOUT", None),
    )
    parser.add_argument(
        "--enable_defense_other_guild_player",
        help="Enable defense against other guild players",
        default=os.getenv("ENABLE_DEFENSE_OTHER_GUILD_PLAYER", None),
    )
    parser.add_argument(
        "--coop_player_max_num",
        help="Maximum number of cooperative players",
        default=os.getenv("COOP_PLAYER_MAX_NUM", None),
    )
    parser.add_argument(
        "--server_player_max_num",
        help="Maximum number of players on the server",
        default=os.getenv("SERVER_PLAYER_MAX_NUM", None),
    )
    parser.add_argument(
        "--server_name",
        help="Server name",
        default=os.getenv("SERVER_NAME", None),
    )
    parser.add_argument(
        "--server_description",
        help="Server description",
        default=os.getenv("SERVER_DESCRIPTION", None),
    )
    parser.add_argument(
        "--admin_password",
        help="Admin password",
        default=os.getenv("ADMIN_PASSWORD", None),
    )
    parser.add_argument(
        "--server_password",
        help="Server password",
        default=os.getenv("SERVER_PASSWORD", None),
    )
    parser.add_argument(
        "--public_port", help="Public port", default=os.getenv("PUBLIC_PORT", None)
    )
    parser.add_argument(
        "--public_ip", help="Public IP address", default=os.getenv("PUBLIC_IP", None)
    )
    parser.add_argument(
        "--rcon_enabled",
        help="RCON enabled",
        default=os.getenv("RCON_ENABLED", None),
    )
    parser.add_argument(
        "--rcon_port", help="RCON port", default=os.getenv("RCON_PORT", None)
    )
    parser.add_argument(
        "--use_auth", help="Use authentication", default=os.getenv("USE_AUTH", None)
    )

    return parser.parse_args()


def main():
    # Parse arguments
    args = get_args()

    # Check for preset and apply preset values first
    preset_values = {}
    if args.preset:
        preset_values = presets.get(args.preset, {})

    # Create a dictionary of all arguments
    args_dict = {
        key: value
        for key, value in vars(args).items()
        if key != "output" and value is not None
    }
    combined_values = {**default_values, **preset_values, **args_dict}

    # Load and render the template with arguments
    # get a path of script
    script_path = os.path.dirname(os.path.realpath(__file__))
    # join a script path to templates/config.ini.j2
    template_path = os.path.join(script_path, "templates/config.ini.j2")

    template = Template(open(template_path).read())
    rendered = template.render(**combined_values)

    # Output to either stdout or a file
    if args.output:
        # get a dir path of output and create folders if not exists
        output_dir = os.path.dirname(args.output)
        if not os.path.exists(output_dir):
            os.makedirs(output_dir)
        with open(args.output, "w") as file:
            file.write(rendered)
    else:
        print(rendered)


if __name__ == "__main__":
    main()

# use jinja2 to render the templates/config.ini.j2 file
