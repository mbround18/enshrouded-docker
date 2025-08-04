#!/usr/bin/env bash
set -Euo pipefail

# ───────────────────────────────────────────────────────────
# Welcome to the Enshruded Docker container
# If you are modifying this script please check contributors guide! :)
# ───────────────────────────────────────────────────────────

echo "──────────────────────────────────────────────────────────"
echo "🚀 Enshrouded Docker - $(date)"
echo "──────────────────────────────────────────────────────────"

# System Info
echo "🔹 Hostname: $(hostname)"
echo "🔹 Kernel: $(uname -r)"
echo "🔹 OS: $(grep PRETTY_NAME /etc/os-release | cut -d= -f2 | tr -d '\"')"
echo "🔹 CPU: $(lscpu | grep 'Model name' | cut -d: -f2 | sed 's/^ *//')"
echo "🔹 Memory: $(free -h | awk '/^Mem:/ {print $2}')"
echo "🔹 Disk Space: $(df -h / | awk 'NR==2 {print $4}')"
echo "──────────────────────────────────────────────────────────"

# User & Permission Check
echo "👤 Running as user: $(whoami) (UID: $(id -u), GID: $(id -g))"
echo "👥 Groups: $(id -Gn)"

# Directory checks
if [ ! -d "/home/steam/enshrouded" ]; then
  echo "⚠️ Directory /home/steam/enshrouded does not exist. Creating..."
  mkdir -p /home/steam/enshrouded/logs
fi

# Permission check
echo "🔍 Checking permissions for /home/steam/enshrouded..."
ls -ld /home/steam/enshrouded
chmod +x /usr/local/bin/enshrouded

echo "🔄 Updating ownership to match user..."
sudo chown -R "$(id -u):$(id -g)" /home/steam/enshrouded 2>/dev/null || true

# ───────────────────────────────────────────────────────────
# Setup and Initialization
# ───────────────────────────────────────────────────────────
export WINEPREFIX="/home/steam/.wine"
export DISPLAY=:1

echo "🧹 Cleaning up cache..."
rm -rf /home/steam/.cache

echo "📦 Ensuring necessary directories exist..."
mkdir -p /home/steam/enshrouded
mkdir -p /home/steam/enshrouded/logs

echo "🔧 Running SteamCMD to ensure dependencies are up to date..."
steamcmd +quit

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
