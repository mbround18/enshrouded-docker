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
    if [ ! -d "${ENSHROUDED_CONFIG_DIR}" ]; then
        echo -e "\nERROR: ${ENSHROUDED_CONFIG_DIR} does not exist. "
        exit 1
    fi

    "${ENSHROUDED_CONFIG_DIR}/config" --output /home/steam/enshrouded/enshrouded_server.json
}

function enshrouded_launch() {
  echo "launching enshrouded"
  enshrouded_install

  cd ~/enshrouded || exit 1

  enshrouded_configure
  wine64 /home/steam/enshrouded/enshrouded_server.exe
}
