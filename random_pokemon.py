from random import randint, choice
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

def evaluate_generation():
    generation = randint(1, 8) # default

    if len(sys.argv) > 1:
        command = sys.argv[1]
    
        if command == "--gen" or command == "-g":
            generations = sys.argv[2:]
    
            if generations:
                if not all(map(lambda g: is_valid_generation(int(g)), generations)):
                    raise ValueError("Invalid generation.")
    
                if len(generations) == 1:
                    generation = int(sys.argv[2])
                else:
                    generation = int(choice(generations))
    
                if not is_valid_generation(generation):
                    raise ValueError("Invalid generation.")
            else:
                raise ValueError("Missing generation.")

    return generation

def evaluate_index(generation):
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
    
    return randint(start, stop)

def main():
    generation = evaluate_generation()

    pokemon = linecache.getline("nameslist.txt", evaluate_index(generation))[:-1]

    print(f"{pokemon} - {roman_numerals(generation)} Generation")

if __name__ == "__main__":
    main()