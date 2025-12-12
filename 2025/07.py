with open("input/07.txt") as f:
    lines = f.read().splitlines()

S = lines[0].find("S")

visited = set()
def count_splits(row, col):
    if row >= len(lines) or (row, col) in visited:
        return 0
    visited.add((row, col))
    if lines[row][col] == "^":
        return count_splits(row, col - 1) + count_splits(row, col + 1) + 1
    return count_splits(row + 1, col)

print(count_splits(0, S))

mem = {}
def count_timelines(row, col):
    if row >= len(lines):
        return 1
    if (row, col) in mem:
        return mem[(row, col)]
    if lines[row][col] == "^":
        mem[(row, col)] = count_timelines(row, col - 1) + count_timelines(row, col + 1)
    else:
        mem[(row, col)] = count_timelines(row + 1, col)
    return mem[(row, col)]

print(count_timelines(0, S))
