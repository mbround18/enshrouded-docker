import os
from argparse import ArgumentParser
from jinja2 import Template


def get_args():
    # Initialize the parser
    parser = ArgumentParser()
    # Add the parameters positional/optional
    parser.add_argument("-o", "--output", help="Output destination of the config.ini")

    parser.add_argument(
        "--server-name", help="Name of the server", default=os.getenv("SERVER_NAME", "Enshrouded Server")
    )

    parser.add_argument(
        "--password", help="Password for the server", default=os.getenv("PASSWORD", "")
    )

    parser.add_argument(
        "--save-directory", help="Save directory for the game", default=os.getenv("SAVE_DIRECTORY", "./savegame")
    )

    parser.add_argument(
        "--log-directory", help="Log directory for the server", default=os.getenv("LOG_DIRECTORY", "./logs")
    )

    parser.add_argument(
        "--server-ip", help="IP address for the server", default=os.getenv("SERVER_IP", "0.0.0.0")
    )

    parser.add_argument(
        "--game-port", type=int, help="Game port for the server", default=os.getenv("GAME_PORT", 15636)
    )

    parser.add_argument(
        "--query-port", type=int, help="Query port for the server", default=os.getenv("QUERY_PORT", 15637)
    )

    parser.add_argument(
        "--slot-count", type=int, help="Number of slots for the server", default=os.getenv("SLOT_COUNT", 16)
    )

    return parser.parse_args()


def main():
    # Parse arguments
    args = get_args()

    # Create a dictionary of all arguments
    args_dict = {
        key: value
        for key, value in vars(args).items()
        if key != "output" and value is not None
    }

    # Load and render the template with arguments
    # get a path of script
    script_path = os.path.dirname(os.path.realpath(__file__))
    # join a script path to templates/config.ini.j2
    template_path = os.path.join(script_path, "templates/config.json.j2")

    template = Template(open(template_path).read())
    rendered = template.render(**args_dict)

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
