from random import randint
import linecache

random_index = randint(0, 897)

pokemon = linecache.getline("nameslist.txt", random_index)[:-1]

print(pokemon)