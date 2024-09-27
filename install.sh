#!/usr/bin/env bash

ROOT=$HOME/.cache/pokemon-icat

mkdir -p $ROOT
mkdir -p $ROOT/pokemon-icons

cp -r bin/* $ROOT

python3 setup_icons.py $1 $2
