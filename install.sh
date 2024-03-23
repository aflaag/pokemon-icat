#!/usr/bin/env bash

ROOT=$HOME/.pokemon-icat

mkdir -p $ROOT
mkdir -p $ROOT/pokemon-icons

cp -r bin/* $ROOT

python3 $ROOT/setup_icons.py $1 $2
