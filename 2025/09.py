with open("input/09.txt") as f:
    coords = [list(map(int, line.split(','))) for line in f.read().splitlines()]

rows = len(coords)

max_area = 0
for i in range(rows):
    for j in range(i + 1, rows):
        [x1, y1] = coords[i]
        [x2, y2] = coords[j]
        area = (abs(x2 - x1) + 1) * (abs(y2 - y1) + 1)
        if area > max_area:
            max_area = area
print(max_area)
