from os.path import expanduser
from PIL import Image
import requests
from io import BytesIO
import numpy as np
import sys

def remove_horizontal_margins(rgba):
    y = 0

    while y < len(rgba):
        if not any(map(lambda p: p[3], rgba[y])):
            rgba.pop(y)

            y -= 1

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

def main():
    upscale = 9 # defualt value

    if len(sys.argv) > 1:
        command = sys.argv[1] # this must exist

        if command == "--upscale" or command == "-u":
            try:
                upscale = int(sys.argv[2])
            except:
                raise SyntaxError("Missing upscaling factor.")

    home = expanduser("~")

    for i, pokemon_raw in enumerate(open(home + "/.pokemon-icat/nameslist.txt").readlines()):
        # remove the '\n' a the end
        pokemon = pokemon_raw[:-1]
    
        # download the image in PNG indexed format
        url = "https://img.pokemondb.net/sprites/sword-shield/icon/" + pokemon + ".png"
        response = requests.get(url)
    
        # load the image in PNG indexed format
        png_idx_img = Image.open(BytesIO(response.content))

        # return the new image from the new RGBA array
        new_rgba_img = build_new_image(png_idx_img)

        # upscale the RGBA image with the upscaling factor
        img = new_rgba_img.resize((new_rgba_img.width * upscale, new_rgba_img.height * upscale), Image.BOX)

        # save the processed RGBA image
        img.save(f"$HOME/.pokemon-icat/pokemon-icons/{pokemon}.png")
    
        print(f"{pokemon} saved! [{i + 1}/898]")

if __name__ == "__main__":
    main()