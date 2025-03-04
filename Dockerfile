# Stage 1: Base Setup
FROM ubuntu:24.04 AS base
ARG DEBIAN_FRONTEND=noninteractive
ENV USER=root \
    HOME=/root \
    LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en

RUN set -ex && \
    echo steam steam/question select "I AGREE" | debconf-set-selections && \
    echo steam steam/license note "" | debconf-set-selections && \
    dpkg --add-architecture i386 && \
    apt-get update -y && \
    apt-get install -y --no-install-recommends ca-certificates locales steamcmd && \
    rm -rf /var/lib/apt/lists/* && \
    locale-gen en_US.UTF-8 && \
    ln -s /usr/games/steamcmd /usr/bin/steamcmd && \
    steamcmd +quit && \
    mkdir -p "$HOME/.steam" && \
    ln -s "$HOME/.local/share/Steam/steamcmd/linux32" "$HOME/.steam/sdk32" && \
    ln -s "$HOME/.local/share/Steam/steamcmd/linux64" "$HOME/.steam/sdk64" && \
    ln -s "$HOME/.steam/sdk32/steamclient.so" "$HOME/.steam/sdk32/steamservice.so" && \
    ln -s "$HOME/.steam/sdk64/steamclient.so" "$HOME/.steam/sdk64/steamservice.so"


# Stage 2: Wine Setup
FROM base AS wine
ARG WINEARCH=win64
ARG WINE_MONO_VERSION=4.9.4
ENV TZ=America/Los_Angeles \
    PYTHONUNBUFFERED=1 \
    DISPLAY=:0 \
    PUID=1000 \
    PGID=1000

RUN set -ex && \
    ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && \
    echo $TZ > /etc/timezone && \
    if [ "$(getent passwd $PUID | cut -d: -f1)" != "steam" ]; then userdel $(getent passwd $PUID | cut -d: -f1); fi && \
    apt-get update -y && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends \
      build-essential htop net-tools nano gcc g++ gdb \
      netcat-traditional curl wget zip unzip cron sudo gosu dos2unix jq tzdata && \
    rm -rf /var/lib/apt/lists/* && \
    gosu nobody true && \
    dos2unix && \
    addgroup --system steam && \
    adduser --system --home /home/steam --shell /bin/bash steam && \
    usermod -aG steam steam && \
    chmod ugo+rw /tmp/dumps

# Install Wine and winetricks
ADD https://dl.winehq.org/wine-builds/winehq.key /tmp/winehq.key
RUN set -ex && \
    dpkg --add-architecture i386 && \
    apt-get update -y && \
    apt-get install -y --no-install-recommends software-properties-common gnupg2 && \
    apt-key add /tmp/winehq.key && \
    apt-add-repository "deb https://dl.winehq.org/wine-builds/ubuntu/ bionic main" && \
    apt-get install -y --install-recommends winehq-stable winbind cabextract && \
    rm -rf /var/lib/apt/lists/*
ENV WINEDEBUG=fixme-all
ADD --chmod=755 https://raw.githubusercontent.com/Winetricks/winetricks/master/src/winetricks /usr/local/bin/winetricks

# Stage 4: Final Image
FROM wine AS final
ARG GITHUB_SHA=not-set
ARG GITHUB_REF=not-set
ARG GITHUB_REPOSITORY=not-set
ENV ENSHROUDED_CONFIG_DIR=/usr/local/share/enshrouded-config \
    CONFIG_TEMPLATE_PATH=/home/steam/scripts/templates/config.json.j2

# Copy built artifacts and scripts, ensuring proper ownership
COPY --chown=steam:steam scripts/ /home/steam/scripts/

# Administrative tweaks (as root)
USER root
RUN set -ex && \
    usermod -u ${PUID} steam && \
    groupmod -g ${PGID} steam && \
    echo "steam ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    mkdir -p "${ENSHROUDED_CONFIG_DIR}"

# Switch back to non-root for runtime
USER steam
WORKDIR /home/steam
ENV HOME=/home/steam \
    USER=steam \
    LD_LIBRARY_PATH=/home/steam/.steam/sdk32:/home/steam/.steam/sdk64:/home/steam/.steam/sdk32 \
    PATH=/home/steam/.local/bin:/usr/local/share/enshrouded-config:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

COPY --from=mbround18/gsm-reference:latest /app/enshrouded /usr/local/bin/enshrouded

ENTRYPOINT ["/home/steam/scripts/entrypoint.sh"]
