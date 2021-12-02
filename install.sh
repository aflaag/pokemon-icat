#!/bin/sh

chmod +x pokemon-icat.sh

sudo mkdir -p /usr/local/opt
sudo mkdir /usr/local/opt/pokemon-icat
sudo mkdir /usr/local/opt/pokemon-icat/pokemon-icons

sudo cp ./*.py /usr/local/opt/pokemon-icat
sudo cp pokemon-icat.sh /usr/local/opt/pokemon-icat
sudo cp nameslist.txt /usr/local/opt/pokemon-icat

python3 /usr/local/opt/pokemon-icat/setup_icons.py $1 $2

# non so se legge il programma o va pescato in PATH