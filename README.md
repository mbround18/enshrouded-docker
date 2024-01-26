# enshrouded Server 

🌟 Welcome to the ultimate enshrouded Server Setup! 🌍 This GitHub repository is your go-to toolkit 🛠️ for launching a enshrouded server in a snap using Docker! Choose from preset worlds like 'casual' 🏖️, 'normal' 🌆, or 'hard' 🌋, or dive deep into customization with flexible settings 🎛️.


## !!Notice!! Bug with saves, developers of enshrouded working hard to fix!

With the [recent bug on save corruption](https://www.ign.com/articles/enshrouded-dev-working-to-fix-serious-bugs-including-lost-save-data),
we highly recommend you backup your save files! [Click here to see how to integrate auto backups.](./docs/compose-with-backups.md)

## Prerequisites

- Docker
- Docker Compose

## Configuration Options

The server can be configured either through environment variables or by passing arguments directly to the Docker container. The available presets are:
[They were configured based on this article](https://www.gtxgaming.co.uk/best-world-settings-for-enshrouded/)

- `casual`
- `normal`
- `hard`

Additionally, you can customize the following settings:

- `DAY_TIME_SPEED_RATE`: Control the speed of day time.
- `NIGHT_TIME_SPEED_RATE`: Control the speed of night time.
- `EXP_RATE`: Experience points rate.
- (And so on for each configurable option...)

To see a full list of supported configuration options, see the [Environment Configuration Options](./docs/environment_variables.md) page.

## Using Docker Compose

To run the server with Docker Compose, you first need to create a `docker-compose.yml` file in the root of this repository with the following content:

```yaml
version: "3.8"
services:
  enshrouded:
    image: mbround18/enshrouded-docker:latest
    environment:
      PRESET: "casual" # Options: casual, normal, hard
      # Optionally override specific settings:
      # DAY_TIME_SPEED_RATE: '1'
      # NIGHT_TIME_SPEED_RATE: '1'
      # And so on...
    ports:
      - "8211:8211" # Default game port
      - "27015:27015" # steam query port
    volumes:
      - "./data:/home/steam/enshrouded"
```

### Running the Server

To start the server with your chosen configuration, run:

```bash
docker-compose up
```

This command builds the Docker image if necessary and starts the server. The `PRESET` environment variable determines the server's configuration preset. You can also override any specific setting by adding it to the `environment` section of the `docker-compose.yml` file.

### Custom Configuration

If you wish to customize the server beyond the provided presets, simply add or modify the environment variables in the `docker-compose.yml` file. For example, to set a custom experience rate, you would add:

```yaml
environment:
  EXP_RATE: "1.5"
```

## Updating Server Settings

To update the server settings after initial setup, modify the `docker-compose.yml` file as needed and restart the server:

```bash
docker-compose down
docker-compose up
```

This process ensures that your server configuration is always up to date with your specifications.

## Contributions

Contributions to this project are welcome! Please submit a pull request or open an issue for any bugs, features, or improvements.
# enshrouded-docker
