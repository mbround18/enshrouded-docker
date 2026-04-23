#!/bin/bash
set -euo pipefail

# Download Wine repository key
curl --silent --fail --show-error --location \
  'https://dl.winehq.org/wine-builds/winehq.key' \
  --output /etc/apt/keyrings/winehq.asc

# Add Wine repository
. /etc/os-release

# Set up WineHQ repository
cat > /etc/apt/sources.list.d/winehq.sources <<EOS
Types: deb
URIs: https://dl.winehq.org/wine-builds/$ID/
Suites: $VERSION_CODENAME
Components: main
Signed-By: /etc/apt/keyrings/winehq.asc
EOS

# Add i386 architecture and update package lists
# We still need i386 arch for now
dpkg --add-architecture i386
apt-get update --quiet --quiet

# Install Wine and related packages
apt-get install --yes --install-recommends winehq-stable

# Clean up
apt-get clean --yes --quiet
rm --recursive --force /var/lib/apt/lists/*
