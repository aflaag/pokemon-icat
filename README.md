# pokemon-icat

This script is inspired by [this project](https://gitlab.com/phoneybadger/pokemon-colorscripts), but since the output heavily depends on the font of your terminal, i decided to make a script that shows a true PNG image of the Pokémon (of course, this script requires a terminal that supports images).

![Screenshot](screenshot.png)

## Requirements

**Important**: this script currently works only on Kitty, but you can change this by editing [this line](bin/pokemon-icat.sh#L25) inside `bin/pokemon-icat.sh`, which shows the picture in the terminal.

To use the script, you must first have all the these installed:

- a terminal which supports images (for example `Kitty`, which requires `ImageMagick`)
- `Python 3.8.x`
- `numpy` (Python library)
- `aiohttp` (Python library)

## Compilation

Note that the binary is already present inside `bin/pokemon-icat`, but if you need to compile the program, simply run:

```sh
sh compile.sh
```

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
$HOME/.pokemon-icat/pokemon-icat.sh
```

note that, by default, no info about the Pokémon will be shown in the output; to show the name and the generation number of the choosen Pokémon add the `--show` flag when calling `pokemon-icat.sh` (**note**: this flag must be used _before_ any other flag regarding the output)

```sh
$HOME/.pokemon-icat/pokemon-icat.sh -s
```

If you want to specify one or more generations in particular, simply add `--generations <GENERATIONS>` at the end, for example (**note**: the generations must be comma-separated, and trailing commas are not supported):

```sh
$HOME/.pokemon-icat/pokemon-icat.sh -s -g 3,4,Hisui,5
```

If you want to show a pokemon in particular, just use the `--pokemon <POKEMON>` flag, for example:

```sh
$HOME/.pokemon-icat/pokemon-icat.sh -s -p charizard
```

## Known issues

- Archaludon doesn't get downloaded
- Image `10186.png` doesn't downloaded (maybe it's Archaludon?)

## would-like-to-do list

- add type emojis
- rename the other images to include every available sprite
- shinies every once in a while

