from os.path import expanduser
import linecache
import sys

from random import randint, choice

from converter import FILENAME_TO_NAME, NAME_TO_GENERATION

def is_valid_generation(generation):
    return 1 <= generation <= 8

def roman_numerals(generation):
    roman = {
        1: "I",
        2: "II",
        3: "III",
        4: "IV",
        5: "V",
        6: "VI",
        7: "VII",
        8: "VIII",
    }

    return roman[generation]

def get_random_pokemon_from_gen(generation):
    ranges = {
        1: (1, 151),
        2: (152, 251),
        3: (252, 386),
        4: (387, 493),
        5: (494, 649),
        6: (650, 721),
        7: (722, 809),
        8: (810, 898),
    }
    
    start, stop = ranges[generation]

    names = list(FILENAME_TO_NAME.values())

    return names[randint(start, stop) - 1]

def main():
    # by default, a random pokemon from a random generation is picked
    generation = randint(1, 8)
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
                # TODO: CHECK IF IT'S AN INT OTHERWISE THE SCRIPT CRASHES
                if not all(map(lambda g: is_valid_generation(int(g)), generations)):
                    raise ValueError("Invalid generation.")

                gen = int(choice(generations))
    
                pokemon = get_random_pokemon_from_gen(gen)

                gen_roman = roman_numerals(gen)
            else:
                raise ValueError("Missing generation.")

    print(f"{pokemon} - {gen_roman} Generation")

if __name__ == "__main__":
    main()
