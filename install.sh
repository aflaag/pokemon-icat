#!/bin/bash

chmod +x pokemon-icat.sh

mkdir -p $HOME/.pokemon-icat
mkdir -p $HOME/.pokemon-icat/pokemon-icons

cp ./*.py $HOME/.pokemon-icat
cp pokemon-icat.sh $HOME/.pokemon-icat
cp nameslist.txt $HOME/.pokemon-icat

python3 $HOME/.pokemon-icat/setup_icons.py $1 $2
