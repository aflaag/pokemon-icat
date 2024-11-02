#!/usr/bin/env bash

set -xe

ROOT=$HOME/.cache/pokemon-icat

mkdir -p $ROOT
mkdir -p $ROOT/pokemon-icons

sh compile.sh

cp -r bin/* $ROOT

rm -rf venv
python3 -m venv venv
# source venv/bin/activate

# Detect shell and use appropriate source command
if [ -n "$BASH_VERSION" ]; then
    . venv/bin/activate
elif [ -n "$ZSH_VERSION" ]; then
    . venv/bin/activate
elif [ -n "$FISH_VERSION" ]; then
    source venv/bin/activate.fish
else
    . venv/bin/activate
fi

pip install -r requirements.txt
python3 setup_icons.py $1 $2

deactivate
rm -rf venv

echo "Done! You can find the executable at $ROOT/pokemon-icat"
