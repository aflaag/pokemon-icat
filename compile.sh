#!/usr/bin/env bash

# Chiede la password sudo all'inizio
sudo -v

# Mantiene attiva la sessione sudo fino alla fine dello script
while true; do sudo -n true; sleep 60; kill -0 "$$" || exit; done 2>/dev/null &

sudo cp target/release/pokemon-icat /usr/bin/
