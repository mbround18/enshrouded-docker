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
  apt-get update --quiet --quiet
  apt-get install --yes --quiet --no-install-recommends \
    tzdata
  apt-get clean --yes --quiet
  rm -rf /var/lib/apt/lists/*
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
  setup_steam_user
  setup_permissions
}

main "$@"
