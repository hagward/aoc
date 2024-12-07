import std/[sequtils, strutils]

let grid = readAll(stdin).strip.splitLines.mapIt(it.toSeq)

proc hasMas(x, y, dx, dy: int): bool =
    const mas = @['M', 'A', 'S']
    for i in 0..<mas.len:
        let xx = x + dx * (i + 1)
        let yy = y + dy * (i + 1)
        if xx < 0 or xx >= grid[0].len or yy < 0 or yy >= grid.len:
            return false
        if grid[yy][xx] != mas[i]:
            return false
    return true

var countPart1 = 0
for y in 0..<grid.len:
    for x in 0..<grid[0].len:
        if grid[y][x] != 'X': continue
        for dy in -1..1:
            for dx in -1..1:
                if dy == 0 and dx == 0: continue
                if hasMas(x, y, dx, dy):
                    countPart1 += 1
echo countPart1

var countPart2 = 0
for y in 1..<grid.len - 1:
    for x in 1..<grid[0].len - 1:
        if grid[y][x] != 'A': continue
        if grid[y-1][x-1] & grid[y-1][x+1] & grid[y+1][x+1] & grid[y+1][x-1] in
                @["MMSS", "MSSM", "SSMM", "SMMS"]:
            countPart2 += 1
echo countPart2
