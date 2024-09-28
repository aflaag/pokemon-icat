#!/usr/bin/env bash

set -xe

ROOT=$HOME/.cache/pokemon-icat

mkdir -p $ROOT
mkdir -p $ROOT/pokemon-icons

sh compile.sh

cp -r bin/* $ROOT

rm -rf venv
python3 -m venv venv
source venv/bin/activate

pip install -r requirements.txt
python3 setup_icons.py $1 $2

deactivate
rm -rf venv
