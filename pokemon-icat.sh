#!/bin/sh

output=$(python3 /usr/local/opt/random_pokemon.py $1)
# output=$(python random_pokemon.py $1)

echo $output

IFS=' ' read -r -a split <<< "$output"

pokemon=${split[0]}

kitty icat --align left --silent /usr/local/opt/pokemon_icat/pokemon-icons/$pokemon.png
# kitty icat --align left --silent pokemon-icons/$pokemon.png