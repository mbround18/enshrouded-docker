# Backups

> Runtime support: Proton is the only supported runtime for this project. The old Wine image was removed after cold-volume validation failed, so this guide assumes `mbround18/enshrouded-docker:proton-latest`. See [migration-notice.md](./migration-notice.md) before moving off older images.

This adds a sidecar container that periodically zips up your save folder and
prunes old backups, alongside the supported Proton server image.

> Seeing `ValueError: Input folder does not exist or is not a directory.` in the
> backup container's logs the first time you start everything is normal — it
> just means the server hasn't written a save yet.

[See all options for the backup sidecar image here.](https://github.com/mbround18/backup-docker)

Save this as `compose.yaml` in its own empty folder (e.g. `~/enshrouded-server/`), then run `docker compose up -d` from that folder.

```yaml
services:
  enshrouded:
    image: mbround18/enshrouded-docker:proton-latest
    stop_grace_period: 120s
    environment:
      TZ: "America/Los_Angeles"
      NAME: "My Enshrouded Server"
      SET_GROUP_ADMIN_PASSWORD: "change-me"
      SET_GROUP_GUEST_PASSWORD: "change-me-too"
    ports:
      - "15636:15636/tcp"
      - "15636:15636/udp"
      - "15637:15637/tcp"
      - "15637:15637/udp"
    volumes:
      - ./data:/home/steam/enshrouded

  backups:
    image: mbround18/backup-cron:latest
    environment:
      - SCHEDULE=*/30 * * * *
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

Backups land in `./backups` next to your compose file. Adjust `SCHEDULE` (cron syntax) and `KEEP_N_DAYS` to taste.

See the [main README](../README.md) for the full list of environment variables you can add to the `enshrouded` service.
