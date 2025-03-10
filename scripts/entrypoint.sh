#!/usr/bin/env bash
set -euxo pipefail

# Ensure Wine environment is set up
export WINEPREFIX="/home/steam/.wine"
export DISPLAY=:1

rm -rf /home/steam/.cache
mkdir -p chome/steam/enshrouded
mkdir -p chome/steam/enshrouded/logs

enshrouded install

enshrouded start

enshrouded monitor
