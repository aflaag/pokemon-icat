#!/usr/bin/env bash

ROOT="$HOME/.pokemon-icat"

chmod +x pokemon-icat.sh

mkdir -p $ROOT
mkdir -p "$ROOT/pokemon-icons"

cp ./*.py $ROOT
cp pokemon-icat $ROOT
cp pokemon-icat.sh $ROOT

python3 "$ROOT/setup_icons.py" $1 $2
