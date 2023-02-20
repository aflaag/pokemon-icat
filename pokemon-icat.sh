#!/bin/bash

output=$(python3 $HOME/.pokemon-icat/pokemon.py ${@:1})

echo $output

IFS=' ' read -r -a split <<< "$output"

pokemon=${split[0]}

echo " "

### CHANGE THIS LINE IF YOU NEED TO USE THIS SCRIPT ON ANOTHER TERMINAL
kitten icat --align left --silent $HOME/.pokemon-icat/pokemon-icons/$pokemon.png

echo " "
