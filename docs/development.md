# Development

> Runtime support: Proton is the only supported runtime for this project. The Wine target was removed after cold-volume validation failed, so any new work or local testing should target the Proton image and the Proton paths in the Dockerfile and compose files. See [migration-notice.md](./migration-notice.md) if you are migrating off an older Wine-based setup.

The `compose.yaml` at the repo root is for people working on this project, not
for running a server — it builds the supported Proton image from source (with
a `build:` block, buildx cache, etc.) instead of pulling the published image
from Docker Hub. If you just want to host a server, use the
[README's Quick Start](../README.md#quick-start) instead.

## Building and running from source

```bash
docker compose up --build
```

This builds the Proton target defined in the `Dockerfile` and starts the
service with `./tmp/proton` mounted at `/home/steam/enshrouded` so you can
inspect server state directly from the repo.

## Iterating on the CLI without rebuilding the image

The CLI (`cli/`) is what actually drives setup/install/start/stop/monitor
inside the container. Rebuilding the whole image on every change is slow, so
during CLI development it's faster to bind-mount a locally built binary over
the one baked into the image:

```yaml
services:
  enshrouded:
    image: mbround18/enshrouded-docker:proton-latest
    environment:
      TZ: "America/Los_Angeles"
      NAME: "Dev Server"
      RUST_LOG: "debug"
    ports:
      - "15636:15636/udp"
      - "15636:15636/tcp"
      - "15637:15637/udp"
      - "15637:15637/tcp"
    volumes:
      - ./tmp/dev:/home/steam/enshrouded
      - type: bind
        source: ./cli/target/debug/enshrouded
        target: /usr/local/bin/enshrouded
        read_only: true
```

Build the binary first with `cargo build` inside `cli/`, then
`docker compose up` picks up the bind-mounted binary instead of the one baked
into the image. Rerun `cargo build` and restart the container to pick up
changes.

## Running the CLI's own test suite

```bash
cd cli
cargo test
```
