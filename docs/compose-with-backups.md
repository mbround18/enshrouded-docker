# Compose with Backups

> Seeing this error `ValueError: Input folder does not exist or is not a directory.` is normal if you are starting a new server for the first time.
> It just means no saves have been recorded yet. 

[Click here to see all options for the backup cron](https://github.com/mbround18/backup-docker)

```yaml
version: "3.8"
services:
  enshrouded:
    image: mbround18/enshrouded-docker:latest
    build:
      context: .
      dockerfile: Dockerfile
    environment:
      SERVER_NAME: "My Enshrouded Server" # Optional, Name of the server
    #      PASSWORD: "" # Optional, Password for the server
    #      SAVE_DIRECTORY: ./savegame # Optional, Save directory for the game
    #      LOG_DIRECTORY: ./logs # Optional, Log directory for the server
    #      SERVER_IP: 0.0.0.0 # Optional, IP address for the server
    #      GAME_PORT: 15636 # Optional, Game port for the server
    #      QUERY_PORT: 15637 # Optional, Query port for the server
    #      SLOT_COUNT: 16 # Optional, Number of slots for the server
    ports:
      - "15636:15636"
      - "15637:15637"
    volumes:
      - ./data:/home/steam/enshrouded
  backups:
    image: mbround18/backup-cron:latest
    environment:
      - SCHEDULE=*/10 * * * *
      - INPUT_FOLDER=/home/steam/enshrouded/savegame
      - OUTPUT_FOLDER=/home/steam/backups
      - OUTPUT_USER=1000
      - OUTPUT_GROUP=1000
      - KEEP_N_DAYS=5
    volumes:
      - ./data:/home/steam/enshrouded
      - ./backups:/home/steam/backups
    restart: unless-stopped
```
