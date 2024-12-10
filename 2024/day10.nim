import std/[sequtils, strutils, sugar]

let grid = readAll(stdin).strip.splitLines.mapIt(it.toSeq.map(c => int(c) - int('0')))

var
    startingPoints: seq[(int, int)]
    endingPoints: seq[(int, int)]
for y in 0..<grid.len:
    for x in 0..<grid[0].len:
        if grid[y][x] == 0:
            startingPoints.add((x, y))
        if grid[y][x] == 9:
            endingPoints.add((x, y))

proc dfs(x, y: int, visited: var seq[seq[int]]) =
    visited[y][x] += 1
    if grid[y][x] == 9:
        return
    for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
        let
            x2 = x + dx
            y2 = y + dy
        if y2 < 0 or y2 >= grid.len or x2 < 0 or x2 >= grid[0].len:
            continue
        if grid[y2][x2] - grid[y][x] != 1:
            continue
        dfs(x2, y2, visited)

var
    sum1 = 0
    sum2 = 0
for (x, y) in startingPoints:
    var visited: seq[seq[int]]
    for i in 0..<grid.len:
        visited.add(newSeq[int](grid[0].len))
    dfs(x, y, visited)
    for (x2, y2) in endingPoints:
        sum1 += clamp(visited[y2][x2], 0, 1)
        sum2 += visited[y2][x2]
echo sum1
echo sum2
