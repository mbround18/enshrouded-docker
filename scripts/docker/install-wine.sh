#!/usr/bin/env bash

set -euo pipefail

install_wine() {
  # Add i386 architecture and update package lists
  dpkg --add-architecture i386
  apt-get update

  # Install required packages for repository management
  apt-get install -y software-properties-common gnupg2 curl

  # Download and add Wine repository key (modern method)
  curl -fsSL https://dl.winehq.org/wine-builds/winehq.key | gpg --dearmor --output /usr/share/keyrings/winehq-archive.gpg

  # Add Wine repository for Ubuntu 25.04 (use jammy as fallback)
  echo "deb [signed-by=/usr/share/keyrings/winehq-archive.gpg] https://dl.winehq.org/wine-builds/ubuntu/ jammy main" > /etc/apt/sources.list.d/winehq.list

  # Update package lists
  apt-get update

  # Install Wine and related packages
  apt-get install -y --install-recommends winehq-stable winbind cabextract

  # Clean up
  rm -rf /var/lib/apt/lists/*
}

main() {
  install_wine
}

main "$@"
