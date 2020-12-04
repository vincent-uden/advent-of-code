from functools import reduce

with open("./input.txt") as f:
    content = f.readlines()

content = [ l.strip() for l in content ]

width = len(content[0])
height = len(content)


trees = []
slopes = [ (1,1), (3,1), (5,1), (7,1), (1,2) ]

for slope in slopes:
    trees.append(0)
    for t in range(int(height / slope[1])):
        x = (t * slope[0]) % width
        y = (t * slope[1])

        if content[y][x] == "#":
            trees[-1] += 1

print(trees)
print(reduce(lambda x, y: x*y, trees))
