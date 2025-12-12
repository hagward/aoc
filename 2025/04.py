with open("input/04.txt") as f:
    lines = [list(line.strip()) for line in f]

rows = len(lines)
cols = len(lines[0])

counts = [[0 for _ in range(cols)] for _ in range(rows)]

def for_each_adjacent(x, y, func):
    for dx in [-1, 0, 1]:
        for dy in [-1, 0, 1]:
            if dx == 0 and dy == 0:
                continue
            x2 = x + dx
            y2 = y + dy
            if x2 < 0 or x2 >= cols or y2 < 0 or y2 >= rows:
                continue
            func(x2, y2)

def count_up(x, y):
    counts[y][x] += 1

def count_down(x, y):
    counts[y][x] -= 1

for i in range(rows):
    for j in range(cols):
        if lines[i][j] != "@":
            continue
        for_each_adjacent(j, i, count_up)

p1 = 0
for i in range(rows):
    for j in range(cols):
        if lines[i][j] == "@" and counts[i][j] < 4:
            p1 += 1
print(p1)

p2 = 0
while True:
    removed = False
    for i in range(rows):
        for j in range(cols):
            if lines[i][j] == "@" and counts[i][j] < 4:
                removed = True
                p2 += 1
                lines[i][j] = "."
                for_each_adjacent(j, i, count_down)
    if not removed:
        break
print(p2)
