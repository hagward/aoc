import std/[deques, math, sequtils, strutils, sugar]

let grid = readAll(stdin).strip.splitLines.mapIt(it.toSeq)
let cols = grid[0].len
let rows = grid.len

var visited: seq[seq[bool]]
var edges: seq[seq[int]]
for i in 0..<rows:
    visited.add(newSeq[bool](cols))
    edges.add(newSeq[int](cols))

var regions: seq[seq[(int, int)]]
for i in 0..<rows:
    for j in 0..<cols:
        if visited[i][j]:
            continue
        var q = initDeque[(int, int)]()
        # echo "starting at ", j, " ", i
        q.addLast((j, i))
        visited[i][j] = true
        var plots: seq[(int, int)]
        while q.len > 0:
            let (x, y) = q.popFirst()
            # echo " visiting ", x, " ", y
            plots.add((x, y))
            for (dx, dy) in @[(1, 0), (0, -1), (-1, 0), (0, 1)]:
                let x2 = x + dx
                let y2 = y + dy
                # echo "  checking: ", x2, " ", y2
                if x2 < 0 or x2 >= cols or y2 < 0 or y2 >= rows or grid[y2][x2] != grid[y][x]:
                    edges[y][x] += 1
                if x2 >= 0 and x2 < cols and y2 >= 0 and y2 < rows and not visited[y2][x2] and grid[y2][x2] == grid[y][x]:
                    # echo "   next up: ", x2, " ", y2
                    visited[y2][x2] = true
                    q.addLast((x2, y2))
        regions.add(plots)
# echo regions
# echo edges

var price = 0
for region in regions:
    let area = region.len
    let perimeter = sum(region.map(it => edges[it[1]][it[0]]))
    price += area * perimeter
echo price