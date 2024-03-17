#!/usr/bin/env bash

ROOT=$HOME/.pokemon-icat
PICAT_PATH=$ROOT/pokemon-icat

get_pokemon() {
    IFS=' ' read -r -a split <<< "$OUTPUT"

    POKEMON=${split[0]}
}

if [ "$1" = "--show" ] || [ "$1" = "-s" ]
then
    OUTPUT=$($PICAT_PATH ${@:2})
    echo $OUTPUT
else
    OUTPUT=$($PICAT_PATH ${@:1})
fi

get_pokemon $OUTPUT

echo " "

# CHANGE THIS LINE IF YOU NEED TO USE THIS SCRIPT ON ANOTHER TERMINAL
kitten icat --align left --silent $ROOT/pokemon-icons/$POKEMON.png
