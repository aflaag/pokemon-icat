from random import randint, choice
from os.path import expanduser
import linecache
import sys

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

def evaluate_index_from_generation(generation):
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
    
    return randint(start, stop) - 1

# TODO: i bet it can be done better, but hey, this script is garbage anyway
def evaluate_index_from_name(name_target):
    with open("nameslist.txt") as f:
        names = f.read().split("\n")
        
        for idx, name in enumerate(names):
            if name == name_target:
                return idx + 1

def evaluate_generation_from_index(index):
    ranges = {
        (1, 151): 1,
        (152, 251): 2,
        (252, 386): 3,
        (387, 493): 4,
        (494, 649): 5,
        (650, 721): 6,
        (722, 809): 7,
        (810, 898): 8,
    }

    for k in ranges.keys():
        if k[0] <= index <= k[1]:
            return ranges[k]

def main():
    generation = randint(1, 8) # default

    home = expanduser("~")

    pokemon = linecache.getline(home + "/.pokemon-icat/nameslist.txt", evaluate_index_from_generation(generation))[:-1]
    gen_roman = roman_numerals(generation)

    if len(sys.argv) > 1:
        command = sys.argv[1]
    
        if command == "--pokemon" or command == "-p":
            try:
                name = sys.argv[2]

                index = evaluate_index_from_name(name)

                pokemon = linecache.getline(home + "/.pokemon-icat/nameslist.txt", index)[:-1]
               
                gen_roman = roman_numerals(evaluate_generation_from_index(index))
            except:
                raise ValueError("Missing pokemon.")
        elif command == "--gen" or command == "-g":
            generations = sys.argv[2:]
    
            if generations:
                if not all(map(lambda g: is_valid_generation(int(g)), generations)):
                    raise ValueError("Invalid generation.")

                generation = int(choice(generations))
    
                if not is_valid_generation(generation):
                    raise ValueError("Invalid generation.")

                pokemon = linecache.getline(home + "/.pokemon-icat/nameslist.txt", evaluate_index_from_generation(generation))[:-1]

                gen_roman = roman_numerals(generation)
            else:
                raise ValueError("Missing generation.")

    print(f"{pokemon} - {gen_roman} Generation")

if __name__ == "__main__":
    main()