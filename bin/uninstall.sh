#!/usr/bin/env bash

sudo -v

while true; do sudo -n true; sleep 60; kill -0 "$$" || exit; done 2>/dev/null &

sudo rm -f /usr/bin/pokemon-icat
rm -rf $HOME/.cache/pokemon-icat

echo "pokemon-icat was successfully uninstalled! :("
