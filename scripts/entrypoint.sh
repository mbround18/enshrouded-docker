#!/usr/bin/env bash
set -euxo pipefail

# Start virtual display
Xvfb :1 -screen 0 1024x768x16 &

# Ensure Wine environment is set up
export WINEPREFIX="/home/steam/.wine"
export DISPLAY=:1

rm -rf /home/steam/.cache

enshrouded install

enshrouded start

enshrouded monitor
