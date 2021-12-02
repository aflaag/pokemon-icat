#!/bin/sh

# random_pokemon=$(python /usr/local/opt/random_pokemon.py)
random_pokemon=$(python random_pokemon.py -g 2 4 5 1)

echo $random_pokemon

# kitty icat --align left /usr/local/opt/pokemon_icat/pokemon-icons/$random_pokemon.png
kitty icat --align left pokemon-icons/$random_pokemon.png