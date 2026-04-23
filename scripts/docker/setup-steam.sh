#!/bin/bash
set -euo pipefail

setup_steam_repository() {
  echo steam steam/question select "I AGREE" | debconf-set-selections
  echo steam steam/license note "" | debconf-set-selections
  dpkg --add-architecture i386
}

install_steam_packages() {
  apt-get update --quiet --quiet
  apt-get install --yes --no-install-recommends \
    ca-certificates \
    locales \
    steamcmd \
    jq \
    curl \
    zip \
    unzip \
    sudo \
    dos2unix
  apt-get autoclean --yes --quiet
  rm --recursive --force /var/lib/apt/lists/*
}

setup_locale() {
  locale-gen en_US.UTF-8
}

setup_steamcmd() {
  ln --symbolic /usr/games/steamcmd /usr/local/bin/steamcmd
  steamcmd +quit
}

setup_steam_directories() {
  mkdir --parents "$HOME/.steam"
  ln --symbolic "$HOME/.local/share/Steam/steamcmd/linux32" "$HOME/.steam/sdk32"
  ln --symbolic "$HOME/.local/share/Steam/steamcmd/linux64" "$HOME/.steam/sdk64"
}

setup_steam_libraries() {
  ln --symbolic "$HOME/.steam/sdk32/steamclient.so" "$HOME/.steam/sdk32/steamservice.so"
  ln --symbolic "$HOME/.steam/sdk64/steamclient.so" "$HOME/.steam/sdk64/steamservice.so"
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
