from collections import defaultdict
from math import prod

with open("input/08.txt") as f:
    rows = [list(map(int, line.split(","))) for line in f.read().splitlines()]

dists = []
for i in range(len(rows)):
    [x1, y1, z1] = rows[i]
    for j in range(i + 1, len(rows)):
        [x2, y2, z2] = rows[j]
        dists.append(((x2 - x1) ** 2 + (y2 - y1) ** 2 + (z2 - z1) ** 2, i, j))
dists.sort()

union_find = list(range(len(rows)))

def find(x):
    if union_find[x] != x:
        union_find[x] = find(union_find[x])
    return union_find[x]

def union(x, y):
    root_x = find(x)
    root_y = find(y)
    if root_x != root_y:
        union_find[root_y] = root_x
        return True
    return False

connected_count = 0
for n in range(1000):
    (_, i, j) = dists[n]
    if union(i, j):
        connected_count += 1

sizes = defaultdict(int)
for i in range(len(rows)):
    sizes[find(i)] += 1

print(prod(sorted(sizes.values(), reverse=True)[0:3]))

for n in range(1001, len(dists)):
    (_, i, j) = dists[n]
    if union(i, j):
        connected_count += 1
    if connected_count == len(rows) - 1:
        print(rows[i][0] * rows[j][0])
        break
