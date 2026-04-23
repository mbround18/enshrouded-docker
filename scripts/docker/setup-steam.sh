#!/usr/bin/env bash

set -euo pipefail

setup_steam_repository() {
  echo steam steam/question select "I AGREE" | debconf-set-selections
  echo steam steam/license note "" | debconf-set-selections
  dpkg --add-architecture i386
}

install_steam_packages() {
  apt-get update -y
  apt-get install -y --no-install-recommends \
    ca-certificates \
    locales \
    steamcmd \
    jq \
    curl \
    wget \
    zip \
    unzip \
    sudo \
    dos2unix
  rm -rf /var/lib/apt/lists/*
}

setup_locale() {
  locale-gen en_US.UTF-8
}

setup_steamcmd() {
  ln -s /usr/games/steamcmd /usr/bin/steamcmd
  steamcmd +quit
}

setup_steam_directories() {
  mkdir -p "$HOME/.steam"
  ln -s "$HOME/.local/share/Steam/steamcmd/linux32" "$HOME/.steam/sdk32"
  ln -s "$HOME/.local/share/Steam/steamcmd/linux64" "$HOME/.steam/sdk64"
}

setup_steam_libraries() {
  ln -s "$HOME/.steam/sdk32/steamclient.so" "$HOME/.steam/sdk32/steamservice.so"
  ln -s "$HOME/.steam/sdk64/steamclient.so" "$HOME/.steam/sdk64/steamservice.so"
}

main() {
  setup_steam_repository
  install_steam_packages
  setup_locale
  setup_steamcmd
  setup_steam_directories
  setup_steam_libraries
}

main "$@"
