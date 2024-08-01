import os
import logging
import json
from argparse import ArgumentParser
from jinja2 import Template

logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)


def get_args():
    parser = ArgumentParser(description="Generate and merge configuration files.")
    parser.add_argument("-o", "--output", help="Output destination of the config.ini")
    parser.add_argument(
        "--server-name",
        help="Name of the server",
        default=os.getenv("SERVER_NAME", "Enshrouded Server"),
    )
    parser.add_argument(
        "--password", help="Password for the server", default=os.getenv("PASSWORD", "")
    )
    parser.add_argument(
        "--admin-password",
        help="Admin password for the server",
        default=os.getenv("ADMIN_PASSWORD", ""),
    )

    parser.add_argument(
        "--save-directory",
        help="Save directory for the game",
        default=os.getenv("SAVE_DIRECTORY", "./savegame"),
    )
    parser.add_argument(
        "--log-directory",
        help="Log directory for the server",
        default=os.getenv("LOG_DIRECTORY", "./logs"),
    )
    parser.add_argument(
        "--server-ip",
        help="IP address for the server",
        default=os.getenv("SERVER_IP", "0.0.0.0"),
    )
    parser.add_argument(
        "--game-port",
        type=int,
        help="Game port for the server",
        default=os.getenv("GAME_PORT", 15636),
    )
    parser.add_argument(
        "--query-port",
        type=int,
        help="Query port for the server",
        default=os.getenv("QUERY_PORT", 15637),
    )
    parser.add_argument(
        "--slot-count",
        type=int,
        help="Number of slots for the server",
        default=os.getenv("SLOT_COUNT", 16),
    )

    args = parser.parse_args()
    logging.info(
        f"Arguments received (password omitted): {[(k, v) for k, v in vars(args).items() if k != 'password']}"
    )
    return args


def load_template(template_path):
    logging.info(f"Loading template from {template_path}")
    with open(template_path, "r") as file:
        return Template(file.read())


def render_template(template, args_dict):
    logging.info("Rendering template with provided arguments")
    return template.render(**args_dict)


def merge_configs(base, update):
    logging.info("Merging existing and new configurations")
    if not isinstance(update, dict):
        logging.critical(f"Update is not a dictionary: {update}")
        return base

    for key, value in update.items():
        if key == "userGroups" and isinstance(value, list):
            if key not in base:
                base[key] = value
            else:
                # Create a map of existing userGroups by name
                existing_groups = {group["name"]: group for group in base[key]}
                for item in value:
                    if item["name"] in existing_groups:
                        existing_group = existing_groups[item["name"]]
                        existing_group_index = base[key].index(existing_group)
                        base[key][existing_group_index] = merge_configs(
                            existing_group, item
                        )
                    else:
                        base[key].append(item)
        elif isinstance(value, dict):
            if key not in base:
                base[key] = value
            else:
                base[key] = merge_configs(base.get(key, {}), value)
        else:
            base[key] = value
    return base


def main():
    args = get_args()
    args_dict = {key: value for key, value in vars(args).items() if value is not None}
    script_path = os.path.dirname(os.path.realpath(__file__))
    template_path = os.environ.get(
        "CONFIG_TEMPLATE_PATH", os.path.join(script_path, "templates/config.ini.j2")
    )
    template = load_template(template_path)
    rendered_config = render_template(template, args_dict)
    rendered_config = json.loads(rendered_config)
    if args.output:
        if os.path.exists(args.output):
            logging.info(f"Existing configuration found at {args.output}")
            if os.stat(args.output).st_size == 0:
                existing_content = {}
            else:
                with open(args.output, "r") as file:
                    existing_content = json.load(file)
            merged_config = merge_configs(existing_content, rendered_config)
            with open(args.output, "w") as f:
                json.dump(merged_config, f, indent=2)
        else:
            logging.info(
                f"No existing configuration found. Creating new one at {args.output}"
            )
            with open(args.output, "w") as f:
                json.dump(rendered_config, f, indent=2)
    else:
        print(rendered_config)


if __name__ == "__main__":
    main()
