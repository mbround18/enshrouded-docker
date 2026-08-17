#!/usr/bin/env bash
set -Euo pipefail

# ───────────────────────────────────────────────────────────
# Welcome to the Enshrouded Docker container
# If you are modifying this script please check contributors guide! :)
# ───────────────────────────────────────────────────────────

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
trap 'enshrouded stop; kill $MONITOR_PID' SIGTERM SIGINT ERR

# Wait for the monitor process to exit
wait $MONITOR_PID