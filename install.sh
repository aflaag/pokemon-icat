#!/usr/bin/env bash

set -xe

export POKEMON_ICAT_DATA="$HOME/.local/share/pokemon-icat"

USER_SHELL=$(basename "$SHELL")

add_line_if_missing() {
  local line="$1"
  local file="$2"
  grep -qxF "$line" "$file" 2>/dev/null || echo "$line" >> "$file"
}

add_line_if_missing "export POKEMON_ICAT_DATA=$POKEMON_ICAT_DATA" "$HOME/.profile"

case "$USER_SHELL" in
  zsh)
    add_line_if_missing "source ~/.profile" "$HOME/.zshrc"
    add_line_if_missing "source ~/.profile" "$HOME/.zprofile"
    add_line_if_missing "pokemon-icat # https://github.com/aflaag/pokemon-icat" "$HOME/.zshrc"
    ;;
  fish)
    add_line_if_missing "set -x POKEMON_ICAT_DATA $POKEMON_ICAT_DATA" "$HOME/.config/fish/config.fish"
    ;;
  *)
    echo "please add 'export POKEMON_ICAT_DATA=$POKEMON_ICAT_DATA' to your shell config manually!"
    ;;
esac

mkdir -p $POKEMON_ICAT_DATA/pokemon-icons/normal
mkdir -p $POKEMON_ICAT_DATA/pokemon-icons/shiny

sh compile.sh

cp -r bin/* "$POKEMON_ICAT_DATA"

rm -rf venv
python3 -m venv venv
# source venv/bin/activate

# Detect shell and use appropriate source command
if [ -n "$BASH_VERSION" ]; then
    . venv/bin/activate
elif [ -n "$ZSH_VERSION" ]; then
    . venv/bin/activate
elif [ -n "$FISH_VERSION" ]; then
    source venv/bin/activate.fish
else
    . venv/bin/activate
fi

pip install -r requirements.txt
python3 setup_icons.py "$@"

deactivate
rm -rf venv

echo "pokemon-icat was successfully installed! :)"
