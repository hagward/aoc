with open('input/04.txt') as f:
    lines = [list(line.strip()) for line in f]

rows = len(lines)
cols = len(lines[0])
counts = []
for i in range(rows):
    row = []
    for j in range(cols):
        row.append(0)
    counts.append(row)

for i in range(rows):
    for j in range(cols):
        if lines[i][j] != '@':
            continue
        for dy in [-1, 0, 1]:
            for dx in [-1, 0, 1]:
                if dy == 0 and dx == 0:
                    continue
                y = i + dy
                x = j + dx
                if y < 0 or y >= rows or x < 0 or x >= cols:
                    continue
                counts[y][x] += 1

ans = 0
for i in range(rows):
    for j in range(cols):
        if lines[i][j] == '@' and counts[i][j] < 4:
            ans += 1
print(ans)

ans2 = 0
while True:
    removed = False
    for i in range(rows):
        for j in range(cols):
            if lines[i][j] == '@' and counts[i][j] < 4:
                removed = True
                ans2 += 1
                lines[i][j] = '.'
                for dy in [-1, 0, 1]:
                    for dx in [-1, 0, 1]:
                        if dy == 0 and dx == 0:
                            continue
                        y = i + dy
                        x = j + dx
                        if y < 0 or y >= rows or x < 0 or x >= cols:
                            continue
                        counts[y][x] -= 1
    if not removed:
        break
print(ans2)
