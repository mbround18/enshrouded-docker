# Build arguments
ARG RUST_VERSION=1.99.0
# Published base images from github.com/mbround18/steamcmd-bases. They own
# steamcmd, the `steam` user, Wine, the Proton-GE install and the Xvfb/prefix
# initialization that has to happen before Proton is ever invoked.
#
# Pinned to a release tag deliberately. The `*-latest` tags are stale -- last
# rebuilt 2026-08 and still on Ubuntu 24.04 -- and the bare `0.0.8` git tag
# produces no image at all, because that repo's workflow only builds on `v*`.
ARG STEAMCMD_BASE_VERSION=v0.0.8

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

# Stage 1: Wine runtime - Self-contained final image with Wine
FROM mbround18/steamcmd:wine-${STEAMCMD_BASE_VERSION} AS wine
USER root
ARG DEBIAN_FRONTEND=noninteractive
ENV WINEDEBUG=fixme-all

# X11/Vulkan libraries the game needs under Wine. The wine base carries Wine
# itself but not Xvfb or the wider X11 stack (only the proton base does), so
# this runs for both final images.
RUN --mount=type=bind,source=./scripts/docker/install-runtime-deps.sh,target=/tmp/install-runtime-deps.sh \
    /bin/bash /tmp/install-runtime-deps.sh

# Copy the binary from the `rust-build` stage
COPY --from=rust-build /app/target/release/enshrouded /usr/local/bin/enshrouded

# Copy entrypoint script with correct permissions
COPY --chmod=0755 --chown=steam:steam scripts/ /home/steam/scripts/

ENV ENSHROUDED_CONFIG_DIR=/usr/local/share/enshrouded-config
RUN mkdir -p "${ENSHROUDED_CONFIG_DIR}"

USER steam
WORKDIR /home/steam
ENV HOME=/home/steam USER=steam
ENV PATH=/home/steam/.local/bin:/usr/local/share/enshrouded-config:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

# The base image owns ENTRYPOINT: it runs /opt/steamcmd-bases/scripts.d/* (Wine
# prefix init, and on the proton image Xvfb plus Proton detection), persists the
# environment those produce, then execs this command. Overriding ENTRYPOINT here
# would skip all of it, so our script is the CMD instead.
CMD ["/home/steam/scripts/entrypoint.sh"]

# Liveness/readiness endpoints served by `enshrouded monitor` (HTTP_PORT, set
# it to 0 to disable). The start period is deliberately generous: a first run
# downloads the whole game through steamcmd before the server ever boots, and
# a cold world load takes minutes on top of that.
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=10s --start-period=15m --retries=3 \
    CMD ["/usr/local/bin/enshrouded", "health"]

# Stage 2: Proton runtime - Self-contained final image with Proton-GE
FROM mbround18/steamcmd:proton-${STEAMCMD_BASE_VERSION} AS proton
USER root
ARG DEBIAN_FRONTEND=noninteractive

RUN --mount=type=bind,source=./scripts/docker/install-runtime-deps.sh,target=/tmp/install-runtime-deps.sh \
    /bin/bash /tmp/install-runtime-deps.sh

# Copy the binary from the `rust-build` stage
COPY --from=rust-build /app/target/release/enshrouded /usr/local/bin/enshrouded

# Copy entrypoint script with correct permissions
COPY --chmod=0755 --chown=steam:steam scripts/ /home/steam/scripts/

ENV ENSHROUDED_CONFIG_DIR=/usr/local/share/enshrouded-config
RUN mkdir -p "${ENSHROUDED_CONFIG_DIR}"

USER steam
WORKDIR /home/steam
ENV HOME=/home/steam USER=steam
ENV PATH=/home/steam/.local/bin:/usr/local/share/enshrouded-config:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

# See the note on the wine stage: the base's ENTRYPOINT does the Proton setup
# this image depends on, so we hand it a CMD rather than replacing it.
CMD ["/home/steam/scripts/entrypoint.sh"]

EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=10s --start-period=15m --retries=3 \
    CMD ["/usr/local/bin/enshrouded", "health"]
