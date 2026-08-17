#!/usr/bin/env bash

set -euo pipefail

setup_timezone() {
  ln -snf "/usr/share/zoneinfo/${TZ:-UTC}" /etc/localtime
  echo "${TZ:-UTC}" >/etc/timezone
}

cleanup_existing_user() {
  local existing_user
  existing_user=$(getent passwd "${PUID:-1000}" | cut -d: -f1 || true)
  if [[ -n "$existing_user" && "$existing_user" != "steam" ]]; then
    userdel "$existing_user"
  fi
}

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
    cron \
    tzdata \
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
    libnss3 \
    libasound2-dev \
    libx11-xcb1 \
    x11-xserver-utils \
    x11-utils \
    xauth
  rm -rf /var/lib/apt/lists/*
}

validate_gosu() {
  gosu nobody true
}

setup_steam_user() {
  addgroup --system steam
  adduser --system --home /home/steam --shell /bin/bash steam
  usermod -aG steam steam
}

setup_permissions() {
  mkdir -p /tmp/dumps /tmp/runtime-steam /tmp/.X11-unix
  chmod ugo+rw /tmp/dumps
  chmod 700 /tmp/runtime-steam
  chmod 1777 /tmp/.X11-unix
  chown steam:steam /tmp/runtime-steam
}

main() {
  setup_timezone
  cleanup_existing_user
  install_packages
  validate_gosu
  setup_steam_user
  setup_permissions
}

main "$@"
