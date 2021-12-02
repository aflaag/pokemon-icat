#!/bin/sh

output=$(python3 /usr/local/opt/pokemon-icat/random_pokemon.py $1)
# output=$(python random_pokemon.py $1)

echo $output

IFS=' ' read -r -a split <<< "$output"

pokemon=${split[0]}

kitty icat --align left --silent /usr/local/opt/pokemon-icat/pokemon-icons/$pokemon.png
# kitty icat --align left --silent pokemon-icons/$pokemon.png