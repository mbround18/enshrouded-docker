#!/usr/bin/env bash
set -Euo pipefail

# ───────────────────────────────────────────────────────────
# Welcome to the Enshrouded Docker container
# If you are modifying this script please check contributors guide! :)
# ───────────────────────────────────────────────────────────

# ───────────────────────────────────────────────────────────
# Fix ownership of mounted volumes
# ───────────────────────────────────────────────────────────
# A freshly bind-mounted host directory (e.g. docker-compose.yml's
# ./tmp/proton, ./tmp/wine) is created by the Docker daemon as root before the
# container ever starts, which the unprivileged steam user can't write into.
echo "🔧 Fixing ownership of /home/steam/enshrouded..."
sudo chown -R steam:steam /home/steam/enshrouded 2>/dev/null || true

# ───────────────────────────────────────────────────────────
# Start a virtual display (unless the base image already did)
# ───────────────────────────────────────────────────────────
# The Windows server binary is launched under Wine/Proton and needs a display
# to attach to (DXVK/Xalia fail hard without one), even though nothing is
# ever rendered on screen.
#
# On the proton image the base's 20-proton-init.sh has already started one,
# exported DISPLAY and recorded the pid in /tmp/xvfb.pid -- it has to, because
# it initializes the Proton prefix before we ever run. The wine image gets no
# such hook, so we start one here. Starting a second server on a display that
# is already taken just fails and leaves the working one alone, but it buries
# a real error in noise, so check rather than race.
XVFB_PID=""
if [ -s /tmp/xvfb.pid ] && kill -0 "$(cat /tmp/xvfb.pid)" 2>/dev/null; then
  echo "🖥️ Virtual display already running on ${DISPLAY:-:0} (pid $(cat /tmp/xvfb.pid))"
else
  echo "🖥️ Starting virtual display on ${DISPLAY:-:0}..."
  Xvfb "${DISPLAY:-:0}" -screen 0 1024x768x16 &
  XVFB_PID=$!
fi

# Run Rust setup command to initialize runtime
enshrouded setup

# ───────────────────────────────────────────────────────────
# Install/Update (if necessary)
# ───────────────────────────────────────────────────────────
if [ "${UPDATE_ON_START:-"false"}" = "true" ] || [ ! -f "/home/steam/enshrouded/enshrouded_server.exe" ]; then
  echo "⬇️ Installing/Updating Enshrouded server..."
  enshrouded install
fi

# ───────────────────────────────────────────────────────────
# Start the Enshrouded Server
# ───────────────────────────────────────────────────────────
echo "🔥 Starting Enshrouded server..."
enshrouded start

# ───────────────────────────────────────────────────────────
# Monitor the Server
# ───────────────────────────────────────────────────────────
echo "📡 Monitoring Enshrouded server logs..."
# Start the monitor in the background
enshrouded monitor &
MONITOR_PID=$!

# Set trap to run cleanup and kill the monitor process if needed
trap 'enshrouded stop; kill $MONITOR_PID ${XVFB_PID:-} 2>/dev/null || true' SIGTERM SIGINT ERR

# Wait for the monitor process to exit
wait $MONITOR_PID