# pokemon-icat

This script is inspired by [this project](https://gitlab.com/phoneybadger/pokemon-colorscripts), but since the output heavily depends on the font of your terminal, i decided to make a script that shows a true PNG image of the Pokémon (of course, this script requires a terminal that supports images).

![Screenshot](screenshot.png)

## Installation

**Important**: this script currently works only on Kitty, but in [pokemon-icat.sh](pokemon-icat.sh) you can change this behaviour by editing the last line, which shows the picture in the terminal.

To install the script, you must first have all the necessary packages installed:

- Python 3.x
- pip3
- numpy

After making sure that you have all of these installed, run this command:

```sh
git clone https://github.com/ph04/pokemon-icat && cd pokemon-icat && chmod +x install.sh && ./install.sh
```

which should start the installation process of the script, by downloading every picture of every Pokémon.

By default, this will download every Pokémon with an upscaling factor of the original image of `9`, but if you want to change this, run the last command with the option `--upscale [upscaling factor]`, for example:

```sh
./install.sh -u 15
```

## Usage

To show a random pokemon, simply run:

```sh
$HOME/.pokemon-icat/pokemon-icat.sh
```

If you want to specify one or more generations in particular, simply add `"--gen [numbers]"` at the end, **between quotes**, for example:

```sh
$HOME/.pokemon-icat/pokemon-icat.sh "-g 3 4 5"
```
