#!/bin/sh

random_pokemon=$(python /usr/local/opt/random_pokemon.py)

echo $random_pokemon

kitty icat --align left /usr/local/opt/pokemon_icat/pokemon-icons/$random_pokemon.png