#!/usr/bin/env bash
set -Euo pipefail

# ───────────────────────────────────────────────────────────
# Welcome to the Enshrouded Docker container
# If you are modifying this script please check contributors guide! :)
# ───────────────────────────────────────────────────────────

# ───────────────────────────────────────────────────────────
# Start a virtual display
# ───────────────────────────────────────────────────────────
# The Windows server binary is launched under Wine/Proton and needs a display
# to attach to (DXVK/Xalia fail hard without one), even though nothing is
# ever rendered on screen.
echo "🖥️ Starting virtual display on ${DISPLAY:-:0}..."
Xvfb "${DISPLAY:-:0}" -screen 0 1024x768x16 &
XVFB_PID=$!

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
trap 'enshrouded stop; kill $MONITOR_PID $XVFB_PID' SIGTERM SIGINT ERR

# Wait for the monitor process to exit
wait $MONITOR_PID