from PIL import Image
import requests
from io import BytesIO

for i, pokemon_raw in enumerate(open("nameslist.txt").readlines()):
    pokemon = pokemon_raw[:-1]

    url = "https://img.pokemondb.net/sprites/sword-shield/icon/" + pokemon + ".png"
    
    response = requests.get(url)
    img = Image.open(BytesIO(response.content))
    
    # this code is completely stolen from stackoverflow
    # i was too lazy to even think about this
    basewidth = 500

    wpercent = (basewidth / float(img.size[0]))
    hsize = int((float(img.size[1]) * float(wpercent)))

    img = img.resize((basewidth, hsize), Image.ANTIALIAS)
    img.save(f'/usr/local/opt/pokemon-icat/pokemon-icons/{pokemon}.png')

    print(f"{pokemon} saved! [{i + 1}/898]")