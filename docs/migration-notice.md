# Migration notice

This project is Proton-only. The Wine target was removed after it failed cold-volume validation: the install path never completed on a fresh volume, the container never reached a healthy `/ready` state, and the runtime was not actually a supported plain-Wine setup.

## What changed

- The published image is now `mbround18/enshrouded-docker:proton-latest`.
- The repo and release automation only build and publish the Proton target.
- Any remaining `wine` or `wine-latest` references in older images or compose files are not supported.

## Migration steps

1. Stop the old container.
2. Replace the image tag in your `compose.yaml` or `docker run` command with:

   ```bash
   mbround18/enshrouded-docker:proton-latest
   ```

3. Use a fresh volume or a new host directory for the game data. If the old data volume was created while the Wine target was still being used, it may contain a partial installation that does not match the supported Proton path.
4. Start the container with a clean first boot. The install step is expected to download the game and then reach `/ready` once the server is ready.
5. Keep the `LAUNCH_MODE` unset or set it explicitly to `proton`.

## Example

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
      LAUNCH_MODE: "proton"
    ports:
      - "15636:15636/tcp"
      - "15636:15636/udp"
      - "15637:15637/tcp"
      - "15637:15637/udp"
    volumes:
      - ./data:/home/steam/enshrouded
```

## Why this matters

Warm-volume success was not meaningful evidence for the install path because the game files could already exist and skip the install step entirely. The verified supported behavior is a cold-volume first boot on Proton, which reaches a healthy ready state after the game is installed.
