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
    for key, value in update.items():
        if isinstance(value, dict) and key in base and isinstance(base[key], dict):
            merge_configs(base[key], value)
        elif isinstance(value, list) and key in base and isinstance(base[key], list):
            base[key].extend(value)
        else:
            base[key] = value
    return base


def main():
    args = get_args()
    args_dict = {key: value for key, value in vars(args).items() if value is not None}
    logging.info(
        f"Options dict (password omitted): {{key: value for key, value in args_dict.items() if key != "
        f"'password'}}"
    )

    script_path = os.path.dirname(os.path.realpath(__file__))
    template_path = os.environ.get(
        "CONFIG_TEMPLATE_PATH", os.path.join(script_path, "templates/config.ini.j2")
    )
    template = load_template(template_path)
    rendered_config = render_template(template, args_dict)

    if args.output:
        if os.path.exists(args.output):
            logging.info(f"Existing configuration found at {args.output}")
            if os.stat(args.output).st_size == 0:
                existing_content = {}
            else:
                with open(args.output, "r") as file:
                    existing_content = json.load(file)
            merged_config = merge_configs(existing_content, args_dict)
            logging.info(f"Merged configuration: {merged_config}")
            with open(args.output, "w") as f:
                json.dump(merged_config, f, indent=2)
        else:
            logging.info(
                f"No existing configuration found. Creating new one at {args.output}"
            )
            with open(args.output, "w") as file:
                file.write(rendered_config)
    else:
        print(rendered_config)


if __name__ == "__main__":
    main()
