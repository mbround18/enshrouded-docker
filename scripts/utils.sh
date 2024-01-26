#!/usr/bin/env bash


function enshrouded_install() {
  echo  "installing enshrouded"
  steamcmd +@sSteamCmdForcePlatformType windows +force_install_dir "/home/steam/enshrouded" +login anonymous  +app_update 2278520 validate +quit
  echo "enshrouded installed"
}

function enshrouded_update() {
    enshrouded_install
}

function enshrouded_configure() {
    echo "configuring enshrouded"
    python3 /home/steam/scripts/config.py --output /home/steam/enshrouded/Pal/Saved/Config/LinuxServer/enshroudedSettings.ini
}

function enshrouded_launch() {
  echo "launching enshrouded"
  enshrouded_install

  cd ~/enshrouded || exit 1

  START_COMMAND="/home/steam/enshrouded/PalServer.sh"

  if [ "${COMMUNITY}" = true ]; then
      START_COMMAND="${START_COMMAND} EpicApp=PalServer"
  fi

  if [ -n "${PUBLIC_IP}" ]; then
      START_COMMAND="${START_COMMAND} -publicip=\"${PUBLIC_IP}\""
  fi

  if [ -n "${PUBLIC_PORT:-"8211"}" ]; then
      START_COMMAND="${START_COMMAND} -publiport=${PUBLIC_PORT:-"8211"}"
  fi

  if [ -n "${SERVER_NAME:-"My enshrouded Server"}" ]; then
      START_COMMAND="${START_COMMAND} -servername=\"${SERVER_NAME:-"My enshrouded Server"}\""
  fi

  if [ -n "${SERVER_PASSWORD}" ]; then
      START_COMMAND="${START_COMMAND} -serverpassword=\"${SERVER_PASSWORD}\""
  fi

  if [ -n "${ADMIN_PASSWORD}" ]; then
      START_COMMAND="${START_COMMAND} -adminpassword=\"${ADMIN_PASSWORD}\""
  fi

  if [ "${MULTITHREADING}" = true ]; then
      START_COMMAND="${START_COMMAND} -useperfthreads -NoAsyncLoadingThread -UseMultithreadForDS"
  fi

#  enshrouded_configure
  wine64 ./enshroudedServer.exe
#  eval "bash ${START_COMMAND}"
}
