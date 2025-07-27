# pokemon-icat

This script is inspired by [this project](https://gitlab.com/phoneybadger/pokemon-colorscripts), but since the output heavily depends on the font of your terminal, I decided to make a script that shows a true PNG image of the Pokémon (of course, this script requires a terminal that supports images).

![Screenshot](screenshot.png)

## Requirements

**Important**: this program relies on [viuer](https://github.com/atanunq/viuer), so check if your terminal is supported first.

To use the script, you must first install the required dependencies:
- a supported terminal
- `Python 3.9.x` or newer
- Run:
    ```shell
    pip install -r requirements.txt
    ```

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

### Arch

If you would like to contribute, an AUR package would be awesome!

### NixOS

The current flake *does not work* yet, it is still WIP.

## Usage

To show a random Pokémon, simply run:

```sh
pokemon-icat
```

If you want to specify one or more generations in particular, simply add `--generations <GENERATIONS>` at the end, for example (**note**: the generations must be comma-separated, and trailing commas are not supported):

```sh
pokemon-icat -g 3,4,Hisui,5
```

Shiny Pokémons are supported too, and the default shiny rate can be changed as follows:

```sh
pokemon-icat --shiny-probability=10
```

If you want to show a Pokémon in particular, just use the `--pokemon <POKEMON>` flag, for example:

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
- Image `678.png` doesn't get downloaded

## would-like-to-do list

- AUR package (very requested)
- Nix package
- rename the other images to include every available sprite
    - do they contain regional forms?

## Development

Check out [development.md](development.md)
