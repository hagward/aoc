import std/[deques, sequtils, strutils, tables]

proc bfs(sr, sc, er, ec: int, grid: seq[seq[char]]): int =
    var
        q = [(sr, sc)].toDeque
        dist = {(sr, sc): 0}.toTable

    while q.len > 0:
        let (cr, cc) = q.popFirst

        if cr == er and cc == ec:
            return dist[(cr, cc)]

        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
            let (nr, nc) = (cr + dr, cc + dc)
            if nr < 0 or nr >= grid.len or nc < 0 or nc >= grid[0].len: continue
            if grid[nr][nc] == '#': continue
            if dist.contains((nr, nc)): continue
            dist[(nr, nc)] = dist[(cr, cc)] + 1
            q.addLast((nr, nc))

var
    grid = readAll(stdin).splitLines.mapIt(it.toSeq)
    rows = grid.len
    cols = grid[0].len

var sr, sc, er, ec: int
for r in 0..<rows:
    for c in 0..<cols:
        if grid[r][c] == 'S':
            sr = r
            sc = c
        if grid[r][c] == 'E':
            er = r
            ec = c

let dist = bfs(sr, sc, er, ec, grid)
var sum = 0
for r in 1..<rows-1:
    for c in 1..<cols-1:
        if grid[r][c] != '#': continue
        grid[r][c] = '.'
        if dist - bfs(sr, sc, er, ec, grid) >= 100: sum += 1
        grid[r][c] = '#'
echo sum
