#!/bin/sh

chmod +x pokemon-icat.sh

mkdir /usr/local/opt/pokemon-icat
rkdir /usr/local/opt/pokemon-icat/pokemon-icons

cp ./*.py /usr/local/opt/pokemon-icat
cp pokemon-icat.sh /usr/local/opt/pokemon-icat
cp nameslist.txt /usr/local/opt/pokemon-icat

python /usr/local/opt/pokemon-icat/setup_icons.py

# non so se legge il programma o va pescato in PATH