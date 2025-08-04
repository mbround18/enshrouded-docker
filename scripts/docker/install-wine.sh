#!/usr/bin/env bash

set -euo pipefail

install_wine() {
  # Download Wine repository key
  wget -O /tmp/winehq.key https://dl.winehq.org/wine-builds/winehq.key

  # Add i386 architecture and update package lists
  dpkg --add-architecture i386
  apt-get update

  # Install required packages for repository management
  apt-get install -y software-properties-common gnupg2

  # Add Wine repository key
  apt-key add /tmp/winehq.key

  # Add Wine repository
  apt-add-repository "deb https://dl.winehq.org/wine-builds/ubuntu/ bionic main"

  # Install Wine and related packages
  apt-get install -y --install-recommends winehq-stable winbind cabextract

  # Clean up
  rm -rf /var/lib/apt/lists/*
  rm -f /tmp/winehq.key
}

main() {
  install_wine
}

main "$@"
