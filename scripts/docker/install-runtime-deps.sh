#!/usr/bin/env bash

set -euo pipefail

# Runtime libraries the Enshrouded server needs on top of whatever the
# steamcmd-bases image already provides.
#
# The base owns steamcmd, the `steam` user, the locale/timezone setup and the
# Proton-GE install, so none of that is repeated here. What it doesn't carry is
# the wider X11/Vulkan stack that the game's own windowing code links against.
# This script runs in the final Proton stage, so anything already present is a
# no-op for apt.
install_packages() {
  apt-get update
  apt-get install -y -qq --no-install-recommends \
    build-essential \
    htop \
    net-tools \
    nano \
    gcc \
    g++ \
    gdb \
    netcat-traditional \
    python3 \
    xvfb \
    dbus \
    libvulkan1 \
    mesa-vulkan-drivers \
    libglib2.0-0 \
    libpulse0 \
    libdbus-1-3 \
    libfontconfig1 \
    libfreetype6 \
    libfreetype6:i386 \
    libxext6 \
    libxfixes3 \
    libxi6 \
    libxrandr2 \
    libxrender1 \
    libxcb1 \
    libxcb-xfixes0 \
    libxcb-render0 \
    libxcomposite1 \
    libxcursor1 \
    libxdamage1 \
    libxinerama1 \
    libxkbcommon0 \
    libnss3 \
    libasound2-dev \
    libx11-xcb1 \
    x11-xserver-utils \
    x11-utils \
    xauth
  rm -rf /var/lib/apt/lists/*
}

# Xvfb needs a world-writable /tmp/.X11-unix for its socket, and Steam expects
# a private XDG_RUNTIME_DIR.
setup_permissions() {
  mkdir -p /tmp/dumps /tmp/runtime-steam /tmp/.X11-unix
  chmod ugo+rw /tmp/dumps
  chmod 700 /tmp/runtime-steam
  chmod 1777 /tmp/.X11-unix
  chown steam:steam /tmp/runtime-steam
}

main() {
  install_packages
  setup_permissions
}

main "$@"
