# Stage 1: Base setup
FROM ubuntu:26.04 AS base
ARG DEBIAN_FRONTEND=noninteractive
ENV USER=root HOME=/root

# Set up steam
RUN --mount=type=bind,source=./scripts/docker/setup-steam.sh,target=/tmp/setup-steam.sh \
    /bin/bash /tmp/setup-steam.sh

ENV LANG=en_US.UTF-8 LANGUAGE=en_US:en

# Stage 2: Wine setup
FROM base AS wine
ARG WINEARCH=win64
ENV TZ=America/Los_Angeles
ENV PYTHONUNBUFFERED=1 DISPLAY=:0 PUID=1000 PGID=1000

# Install dependencies
RUN --mount=type=bind,source=./scripts/docker/install-dependencies.sh,target=/tmp/install-dependencies.sh \
    /bin/bash /tmp/install-dependencies.sh

# Wine installation
RUN --mount=type=bind,source=./scripts/docker/install-wine.sh,target=/tmp/install-wine.sh \
    /bin/bash /tmp/install-wine.sh

# Stage 4: Final stage
FROM wine AS final
ARG GITHUB_SHA=not-set
ARG GITHUB_REF=not-set
ARG GITHUB_REPOSITORY=not-set
ENV ENSHROUDED_CONFIG_DIR=/usr/local/share/enshrouded-config

# Copy entrypoint script with correct permissions
COPY --chmod=0755 --chown=steam:steam scripts/ /home/steam/scripts/

RUN /bin/bash -o pipefail -c ' \
    usermod -u ${PUID} steam && \
    groupmod -g ${PGID} steam && \
    echo "steam ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    mkdir -p "${ENSHROUDED_CONFIG_DIR}"'

# Switch to steam user
USER steam
WORKDIR /home/steam
ENV HOME=/home/steam USER=steam
ENV LD_LIBRARY_PATH=/home/steam/.steam/sdk32:/home/steam/.steam/sdk64:/home/steam/.steam/sdk32
ENV PATH=/home/steam/.local/bin:/usr/local/share/enshrouded-config:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

# TODO: v prefix bc need to fix this in semver action
COPY --from=mbround18/gsm-reference:venshrouded-0.1.6 /app/enshrouded /usr/local/bin/enshrouded

# Set entrypoint
ENTRYPOINT ["/home/steam/scripts/entrypoint.sh"]
