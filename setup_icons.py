from os.path import expanduser
import sys

from PIL import Image
import numpy as np

from io import BytesIO
from time import sleep

from aiohttp import ClientSession
import requests
import asyncio

from converter import FILENAME_TO_NAME

# urls
HEADERS = {"Accept": "application/vnd.github+json", "X-GitHub-Api-Version": "2022-11-28"}
URL_TREE = "https://api.github.com/repos/PokeAPI/sprites/git/trees/c87f4ced89853ad94e3a474306c07d329a28d59c"
URL_POINT = "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png"
# URL_POINT = "https://raw.githubusercontent.com/msikma/pokesprite/master/pokemon-gen8/regular/{}.png"

# dirs
NAME_LIST = expanduser("~") + "/.pokemon-icat/nameslist.txt"
SAVE_POINT = expanduser("~") + "/.pokemon-icat/pokemon-icons/{}.png"

# 3D sprites to be ignored (yes, i found them manually)
RANGE_3D = [
    "10094.png",
    "10095.png",
    "10096.png",
    "10097.png",
    "10098.png",
    "10099.png",
    "100117.png",
    "100121.png",
    "100122.png",
    "100144.png",
    "100145.png",
    "100148.png",
    "100149.png",
    "100150.png",
    "100151.png",
]

ignored = []

counter = 0
pkm_qtd = 0
curr_pokemon = 0

upscale = 3

# this thing is complete garbage
def remove_horizontal_margins(rgba):
    y = 0

    while y < len(rgba):
        if not any(map(lambda p: p[3], rgba[y])):
            rgba.pop(y)

            continue

        y += 1

def build_new_image(png_idx_img):
    # convert the image into RGBA format
    rgba_img = png_idx_img.convert("RGBA")

    # create the image array
    rgba_img_arr = np.array(rgba_img)

    # create a list of lists of lists from the array
    rgba_img_list = rgba_img_arr.tolist()

    # remove horizontal margins
    remove_horizontal_margins(rgba_img_list)

    # build the new RGBA image array from the modified list
    new_rgba_img_arr = np.array(rgba_img_list, dtype=np.uint8)

    # transpose the new RGBA array
    new_rgba_img_arr_t = np.transpose(new_rgba_img_arr, (1, 0, 2))

    # create a list of lists of lists from the transposed new array
    new_rgba_img_list_t = new_rgba_img_arr_t.tolist()

    # remove horizontal margins from the transposed array,
    # so this action removes the vertical margins
    remove_horizontal_margins(new_rgba_img_list_t)

    # build the array of the new transposed list
    final_rgba_img_arr_t = np.array(new_rgba_img_list_t, dtype=np.uint8)

    # transpose the array again, to restore the image
    final_rgba_img_arr = np.transpose(final_rgba_img_arr_t, (1, 0, 2))

    # return the new image from the new RGBA array
    return Image.fromarray(final_rgba_img_arr)

async def gather_pokemons(pokemons):
    global pkm_qtd
    global curr_pokemon
    pkm_qtd = len(pokemons)

    async with ClientSession() as session:
        while pokemons:
            batch = pokemons[:10]
            del pokemons[:10]

            # create a task for each image
            tasks = [asyncio.create_task(get_pokemon(pokemon, session)) for pokemon in batch]

            # gather the entire batch and sleep 1 second
            await asyncio.gather(*tasks)
            await asyncio.sleep(1)

async def get_pokemon(pokemon, session):
    global counter

    url = URL_POINT.format(pokemon)

    success = True

    name = FILENAME_TO_NAME.get(pokemon, pokemon)

    async with session.get(url) as response:
        try:
            # load the image in PNG indexed format
            png_idx_img = Image.open(BytesIO(await response.read()))
        except:
            # ignore images that can't be loaded
            # (currently, only 10186.png does not work, due to unknown reasons)
            print(f"ERROR: An error occurred while trying to dump '{name}.png'; it will be ignored.")

            success = False

            ignored.append(pokemon + ".png")

    if success:
        # return the new image from the new RGBA array
        new_rgba_img = build_new_image(png_idx_img)

        # upscale the RGBA image with the upscaling factor
        img = new_rgba_img.resize((new_rgba_img.width * upscale, new_rgba_img.height * upscale), Image.BOX)

        # save the processed RGBA image
        img.save(SAVE_POINT.format(name))

        # update counter and log progress
        counter += 1
        print(f"[{counter}/{pkm_qtd}] {name} saved!")

def image_number(p):
    if p == "substitute":
        return -1

    if p == "egg":
        return -2
    
    if p == "egg-manaphy":
        return -3

    # remove weird characters from the name
    filtered_p = "".join(map(lambda c: c if c in "0123456789" else ' ', p))

    # return image number
    return int(filtered_p.split()[0])

def filter_pokemon(png_name):
    return png_name.endswith(".png") \
        and "-mega" not in png_name \
        and "-primal" not in png_name \
        and png_name not in RANGE_3D

def dump_names():
    response = requests.get(URL_TREE, headers=HEADERS)

    pokemon_folder = response.json()["tree"]

    pokemons = list(
        map(
            lambda p: p[:-4],
            filter(
                filter_pokemon,
                map(
                    lambda pokemon_obj: pokemon_obj["path"],
                    pokemon_folder
                )
            )
        )
    )

    pokemons.sort(key=lambda p: (image_number(p), len(p), p))

    return pokemons

def main():
    if len(sys.argv) > 1:
        command = sys.argv[1] # this must exist

        if command == "--upscale" or command == "-u":
            try:
                global upscale
                upscale = int(sys.argv[2])
            except:
                raise SyntaxError("Missing upscaling factor.")

    pokemons = dump_names()

    asyncio.run(gather_pokemons(pokemons))

    if len(ignored) != 0:
        print("These images were ignored due tue errors: '" + "', '".join(ignored) + "'.")

if __name__ == "__main__":
    main()
