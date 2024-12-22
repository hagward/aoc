import std/[heapqueue, tables, sequtils, strutils]

type Node = ref object
    x, y, dx, dy, priority: int

proc `<`(a, b: Node): bool = a.priority < b.priority

let rotations = {
    ((1, 0), (1, 0)): 0,
    ((1, 0), (0, -1)): 1,
    ((1, 0), (-1, 0)): 2,
    ((1, 0), (0, 1)): 1,

    ((0, -1), (0, -1)): 0,
    ((0, -1), (-1, 0)): 1,
    ((0, -1), (0, 1)): 2,
    ((0, -1), (1, 0)): 1,

    ((-1, 0), (-1, 0)): 0,
    ((-1, 0), (0, 1)): 1,
    ((-1, 0), (1, 0)): 2,
    ((-1, 0), (0, -1)): 1,

    ((0, 1), (0, 1)): 0,
    ((0, 1), (1, 0)): 1,
    ((0, 1), (0, -1)): 2,
    ((0, 1), (-1, 0)): 1,
}.toTable

let grid = readAll(stdin).splitLines.mapIt(it.toSeq)

let
    sx = 1
    sy = grid.len - 2
    sdx = 1
    sdy = 0
    ex = grid[0].len - 2
    ey = 1

var
    q = [Node(x: sx, y: sy, dx: sdx, dy: sdy, priority: 0)].toHeapQueue
    dist = {(sx, sy, sdx, sdy): 0}.toTable

while q.len > 0:
    let u = q.pop
    for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
        let
            nx = u.x + dx
            ny = u.y + dy
        if grid[ny][nx] == '#': continue
        let r = rotations[((dx, dy), (u.dx, u.dy))] * 1000
        let alt = dist[(u.x, u.y, u.dx, u.dy)] + r + 1
        if alt < dist.getOrDefault((nx, ny, dx, dy), high(int)):
            dist[(nx, ny, dx, dy)] = alt
            q.push(Node(x: nx, y: ny, dx: dx, dy: dy, priority: alt))

var ans = high(int)
for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
    if dist.contains((ex, ey, dx, dy)):
        ans = min(ans, dist[(ex, ey, dx, dy)])
echo ans
