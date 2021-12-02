from random import randint
import linecache
import sys

generation = randint(1, 8)

if len(sys.argv) > 1:
    command = sys.argv[1]

    if command == "--gen" or command == "-g":
        try:
            generation = int(sys.argv[2])

            if not 1 <= generation <= 8:
                raise ValueError("Invalid generation.")
        except:
            raise ValueError("Missing generation.")

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

random_index = randint(start, stop)

pokemon = linecache.getline("nameslist.txt", random_index)[:-1]

print(pokemon)