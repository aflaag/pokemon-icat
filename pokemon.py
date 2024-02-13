from os.path import expanduser
import sys

from random import randint, choice

from converter import FILENAME_TO_NAME, NAME_TO_GENERATION

MAX_GEN = 9
REGIONS = ["Hisui"]

def is_valid_generation(generation):
    try:
        return 1 <= int(generation) <= MAX_GEN
    except:
        return generation == "Hisui"

def roman_numerals(generation):
    roman = {
        "1": "I",
        "2": "II",
        "3": "III",
        "4": "IV",
        "5": "V",
        "6": "VI",
        "7": "VII",
        "8": "VIII",
        "Hisui": "Hisui",
        "9": "IX",
    }

    return roman[generation]

def get_random_pokemon_from_gen(generation):
    ranges = {
        "1": (1, 151),
        "2": (152, 251),
        "3": (252, 386),
        "4": (387, 493),
        "5": (494, 649),
        "6": (650, 721),
        "7": (722, 809),
        "8": (810, 898),
        "Hisui": (899, 905),
        "9": (906, 1025),
    }
    
    start, stop = ranges[generation]

    names = list(NAME_TO_GENERATION.keys())

    return names[randint(start, stop) - 1]

def main():
    # by default, a random pokemon from a random generation is picked
    generation = choice([str(x + 1) for x in range(MAX_GEN)] + REGIONS)
    gen_roman = roman_numerals(generation)

    pokemon = get_random_pokemon_from_gen(generation)

    if len(sys.argv) > 1:
        command = sys.argv[1]
    
        if command == "--pokemon" or command == "-p": # choose the pokemon manually
            try:
                pokemon = sys.argv[2]
               
                gen_roman = roman_numerals(NAME_TO_GENERATION[pokemon])
            except:
                raise ValueError("Missing pokemon.")
        elif command == "--gen" or command == "-g": # choose from a set of generations
            generations = sys.argv[2:]
    
            if generations:
                if not all(map(lambda g: is_valid_generation(g), generations)):
                    raise ValueError("Invalid generation.")
    
                gen = choice(generations)

                pokemon = get_random_pokemon_from_gen(gen)

                gen_roman = roman_numerals(gen)
            else:
                raise ValueError("Missing generation.")

    print(f"{pokemon} - {gen_roman} " + ("Generation" if gen_roman not in REGIONS else "Region"))

if __name__ == "__main__":
    main()
