#!/bin/sh

chmod +x pokemon-icat.sh

sudo mkdir -p $HOME/.pokemon-icat
sudo mkdir -p $HOME/.pokemon-icat/pokemon-icons

sudo cp ./*.py $HOME/.pokemon-icat
sudo cp pokemon-icat.sh $HOME/.pokemon-icat
sudo cp nameslist.txt $HOME/.pokemon-icat

python3 $HOME/.pokemon-icat/setup_icons.py $1 $2

# non so se legge il programma o va pescato in PATH