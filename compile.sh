#!/usr/bin/env bash

sudo -v

cargo build --release

while true; do sudo -n true; sleep 60; kill -0 "$$" || exit; done 2>/dev/null &

sudo cp target/release/pokemon-icat /usr/bin/
