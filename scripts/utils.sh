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
    python3 /home/steam/scripts/config.py --output /home/steam/enshrouded/enshrouded_server.json
}

function enshrouded_launch() {
  echo "launching enshrouded"
  enshrouded_install

  cd ~/enshrouded || exit 1

  enshrouded_configure
  wine64 /home/steam/enshrouded/enshrouded_server.exe
}
