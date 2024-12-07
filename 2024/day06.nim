import std/[sets, sequtils, strutils]

var grid = readAll(stdin).strip.splitLines.mapIt(it.toSeq)
let
    rows = grid.len
    cols = grid[0].len

proc findGuard(): (int, int) =
    for i in 0..<rows:
        for j in 0..<cols:
            if grid[i][j] == '^':
                return (i, j)

let (ys, xs) = findGuard()

proc findVisited(): HashSet[(int, int)] =
    var
        y = ys
        x = xs
        dy = -1
        dx = 0
        visited: HashSet[(int, int)]
    while true:
        visited.incl((y, x))
        let
            yn = y + dy
            xn = x + dx
        if yn < 0 or yn >= rows or xn < 0 or xn >= cols:
            return visited
        if grid[yn][xn] == '#':
            (dy, dx) = (dx, -dy)
        else:
            (y, x) = (yn, xn)

let visited = findVisited()
echo visited.len

proc hasLoop(): bool =
    var
        y = ys
        x = xs
        dy = -1
        dx = 0
        visited: HashSet[(int, int, int, int)]
    while true:
        visited.incl((y, x, dy, dx))
        let
            yn = y + dy
            xn = x + dx
        if yn < 0 or yn >= rows or xn < 0 or xn >= cols:
            return false
        if grid[yn][xn] == '#':
            (dy, dx) = (dx, -dy)
        else:
            (y, x) = (yn, xn)
        if (y, x, dy, dx) in visited:
            return true

var loops = 0
for (y, x) in visited:
    if y == ys and x == xs:
        continue
    grid[y][x] = '#'
    if hasLoop():
        loops += 1
    grid[y][x] = '.'
echo loops
