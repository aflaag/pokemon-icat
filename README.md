# pokemon-icat

This script is inspired by [this project](https://gitlab.com/phoneybadger/pokemon-colorscripts), but since the output heavily depends on the font of your terminal, i decided to make a script that shows a true PNG image of the Pokémon (of course, this script requires a terminal that supports images).

![Screenshot](screenshot.png)

## Requirements

**Important**: this program relies on [viuer](https://crates.io/crates/viuer), so check if your terminal is supported first.

To use the script, you must first have all the these installed:

- a supported terminal
- `Python 3.8.x`
- `numpy` (Python library)
- `aiohttp` (Python library)

## Compilation

Note that the binary is already present inside `bin/pokemon-icat`, but if you need to compile the program, simply run:

```sh
sh compile.sh
```

(note that this script requires `cargo`).

## Installation

After making sure that you have all of the requirements, run the following command:

```sh
git clone https://github.com/aflaag/pokemon-icat && cd pokemon-icat && sh install.sh
```

which should start the installation process of the script, by downloading every picture of every Pokémon.

By default, this will download every Pokémon with an upscaling factor of the original image of `3`, but if you want to change this behaviour run the last command with the option `--upscale <FACTOR>`, for example:

```sh
sh install.sh -u 15
```

## Usage

To show a random pokemon, simply run:

```sh
pokemon-icat
```

(the executable is inside [this folder](bin/pokemon-icat)).

If you want to specify one or more generations in particular, simply add `--generations <GENERATIONS>` at the end, for example (**note**: the generations must be comma-separated, and trailing commas are not supported):

```sh
pokemon-icat -g 3,4,Hisui,5
```

If you want to show a pokemon in particular, just use the `--pokemon <POKEMON>` flag, for example:

```sh
pokemon-icat -p charizard
```

and if you want to suppress the Pokémon info, use the `--quiet` flag:

```sh
pokemon-icat -p charizard -q
```

To check all the available options, use the `--help` option.

## Known issues

- last DLC pokemons don't get downloaded (change the csv when this is fixed)
- Image `10186.png` doesn't get downloaded

## would-like-to-do list

- AUR package (very requested)
- Nix package
- add type emojis
- rename the other images to include every available sprite
- shinies every once in a while

