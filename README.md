# Enshrouded Server 

🌟 Welcome to the ultimate enshrouded Server Setup! 
🌍 This GitHub repository is your go-to toolkit 
🛠️ for launching an enshrouded server in a snap using Docker!


## Dont forget to backup your saves!

we highly recommend you back up your save files! [Click here to see how to integrate auto backups.](./docs/compose-with-backups.md)

## Prerequisites

- Docker
- Docker Compose

## Using Docker Compose

To run the server with Docker Compose, you first need to create a `docker-compose.yml` file in the root of this repository with the following content:

```yaml
services:
  enshrouded:
    image: mbround18/enshrouded-docker:latest
    build:
      context: .
      dockerfile: Dockerfile
      platforms:
        - linux/amd64
    environment:
      SERVER_NAME: "My Enshrouded Server" # Optional, Name of the server
    #      PASSWORD: "" # Optional, Password for the server
    #      ADMIN_PASSWORD: "adminpassword" # Optional, Admin password for the server
    #      SAVE_DIRECTORY: ./savegame # Optional, Save directory for the game
    #      LOG_DIRECTORY: ./logs # Optional, Log directory for the server
    #      SERVER_IP: 0.0.0.0 # Optional, IP address for the server
    #      GAME_PORT: 15636 # Optional, Game port for the server
    #      QUERY_PORT: 15637 # Optional, Query port for the server
    #      SLOT_COUNT: 16 # Optional, Number of slots for the server
    ports:
      - "15636:15636/udp"
      - "15636:15636/tcp"
      - "15637:15637/udp"
      - "15637:15637/tcp"
    volumes:
      - ./data:/home/steam/enshrouded
```

### Running the Server

To start the server with your chosen configuration, run:

```bash
docker-compose up
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

