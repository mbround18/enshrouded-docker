#!/usr/bin/env bash

if [ ! -f ~/enshrouded ]; then
    mkdir -p ~/enshrouded && cd ~/enshrouded || exit 1
fi

# Copy Root Steam Files to Steam User
sudo cp -R /root/.local /home/steam/.local
sudo chown -R steam:steam /home/steam/.local
sudo chown -R steam:steam ~/enshrouded

# Link SteamCMD Files to Steam User
mkdir -p $HOME/.steam \
 && ln -s $HOME/.local/share/Steam/steamcmd/linux32 $HOME/.steam/sdk32 \
 && ln -s $HOME/.local/share/Steam/steamcmd/linux64 $HOME/.steam/sdk64 \
 && ln -s $HOME/.steam/sdk32/steamclient.so $HOME/.steam/sdk32/steamservice.so \
 && ln -s $HOME/.steam/sdk64/steamclient.so $HOME/.steam/sdk64/steamservice.so

source /home/steam/scripts/utils.sh

enshrouded_configure

enshrouded_launch
