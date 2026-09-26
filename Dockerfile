# Build arguments
ARG RUST_VERSION=1.97.1
ARG UBUNTU_VERSION=26.04

# Stage 0: Rust Build - Compiles the Rust binary
FROM rust:${RUST_VERSION} AS rust-build
WORKDIR /app

# Copy manifest files first to leverage Docker's build cache for dependencies.
# This prevents re-downloading dependencies on every code change.
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./cli/Cargo.toml ./cli/
# Create a dummy src/main.rs to build dependencies
RUN mkdir -p ./cli/src && echo "fn main() {}" > ./cli/src/main.rs && cargo build --release
# Build dependencies - cargo-chef or similar can make this more robust

# Copy the rest of the source code and build the final binary
COPY ./cli ./
RUN cargo build --release

# Stage 1: Proton Download - Pre-fetch Proton-GE for rootless runtime
FROM ubuntu:${UBUNTU_VERSION} AS proton-download
# Ensure a failed curl fails the pipeline instead of feeding jq an empty/partial response.
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl gzip tar jq && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /tmp/proton-download
# Fetch latest Proton-GE release and download the tar.gz
RUN curl -sL https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest \
    -H "User-Agent: enshrouded-docker" | \
    jq -r '.assets[] | select(.name | endswith("-x86_64.tar.gz")) | .browser_download_url' | head -1 > url.txt && \
    url=$(cat url.txt) && \
    if [ -z "$url" ]; then echo "Failed to get download URL"; exit 1; fi && \
    curl -L -o proton.tar.gz "$url" && \
    tar -xzf proton.tar.gz && \
    mkdir -p /opt/proton && \
    mv GE-Proton*/* /opt/proton/ && \
    chmod +x /opt/proton/proton

# Stage 2: Base setup - Common for both Wine and Proton runtimes
FROM ubuntu:${UBUNTU_VERSION} AS base
ARG DEBIAN_FRONTEND=noninteractive
ENV USER=root HOME=/root

# Set up steam and shared dependencies.
RUN --mount=type=bind,source=./scripts/docker/setup-steam.sh,target=/tmp/setup-steam.sh \
    /bin/bash /tmp/setup-steam.sh

RUN --mount=type=bind,source=./scripts/docker/install-dependencies.sh,target=/tmp/install-dependencies.sh \
    /bin/bash /tmp/install-dependencies.sh

ENV LANG=en_US.UTF-8 LANGUAGE=en_US:en
ENV TZ=America/Los_Angeles
ENV PYTHONUNBUFFERED=1 DISPLAY=:0 PUID=1000 PGID=1000

# Stage 3: Wine runtime - Self-contained final image with Wine
FROM base AS wine
ARG WINEARCH=win64
ARG WINE_MONO_VERSION=4.9.4
ENV WINEDEBUG=fixme-all

# Wine installation
RUN --mount=type=bind,source=./scripts/docker/install-wine.sh,target=/tmp/install-wine.sh \
    /bin/bash /tmp/install-wine.sh

# Add winetricks for the Wine runtime
ADD --chmod=755 https://raw.githubusercontent.com/Winetricks/winetricks/master/src/winetricks /usr/local/bin/winetricks

# Copy the binary from the `rust-build` stage
COPY --from=rust-build /app/target/release/enshrouded /usr/local/bin/enshrouded

# Copy entrypoint script with correct permissions
COPY --chmod=0755 --chown=steam:steam scripts/ /home/steam/scripts/

# Standard user and directory setup
ENV ENSHROUDED_CONFIG_DIR=/usr/local/share/enshrouded-config
RUN /bin/bash -o pipefail -c ' \
    usermod -u ${PUID} steam && \
    groupmod -g ${PGID} steam && \
    echo "steam ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    mkdir -p "${ENSHROUDED_CONFIG_DIR}"'
USER steam
WORKDIR /home/steam
ENV HOME=/home/steam USER=steam
ENV LD_LIBRARY_PATH=/home/steam/.steam/sdk32:/home/steam/.steam/sdk64:/home/steam/.steam/sdk32
ENV PATH=/home/steam/.local/bin:/usr/local/share/enshrouded-config:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ENTRYPOINT ["/home/steam/scripts/entrypoint.sh"]

# Stage 4: Proton runtime - Self-contained final image with Proton-GE
FROM base AS proton
# Copy pre-downloaded Proton-GE
COPY --from=proton-download --chown=steam:steam /opt/proton /home/steam/.proton

# Copy the binary from the `rust-build` stage
COPY --from=rust-build /app/target/release/enshrouded /usr/local/bin/enshrouded

# Copy entrypoint script with correct permissions
COPY --chmod=0755 --chown=steam:steam scripts/ /home/steam/scripts/

# Standard user and directory setup
ENV ENSHROUDED_CONFIG_DIR=/usr/local/share/enshrouded-config
RUN /bin/bash -o pipefail -c ' \
    usermod -u ${PUID} steam && \
    groupmod -g ${PGID} steam && \
    echo "steam ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    mkdir -p "${ENSHROUDED_CONFIG_DIR}"'
USER steam
WORKDIR /home/steam
ENV HOME=/home/steam USER=steam
ENV LD_LIBRARY_PATH=/home/steam/.steam/sdk32:/home/steam/.steam/sdk64:/home/steam/.steam/sdk32
ENV PATH=/home/steam/.local/bin:/usr/local/share/enshrouded-config:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ENTRYPOINT ["/home/steam/scripts/entrypoint.sh"]
