import std/[sets, sequtils, strutils, tables]

let grid = readAll(stdin).strip.splitLines.mapIt(it.toSeq)

type Pos = tuple[x: int, y: int]

proc getAntinodes(a, b: Pos): (Pos, Pos) =
    let
        dx = abs(a.x - b.x)
        dy = abs(a.y - b.y)
        k = (a.y - b.y) / (a.x - b.x)
    if k >= 0:
        return ((x: min(a.x, b.x) - dx, y: min(a.y, b.y) - dy),
                (x: max(a.x, b.x) + dx, y: max(a.y, b.y) + dy))
    else:
        return ((x: min(a.x, b.x) - dx, y: max(a.y, b.y) + dy),
                (x: max(a.x, b.x) + dx, y: min(a.y, b.y) - dy))

proc inBounds(pos: Pos): bool =
    return pos.x >= 0 and pos.x < grid[0].len and
           pos.y >= 0 and pos.y < grid.len

var table = newTable[char, seq[Pos]]()
for y in 0..<grid.len:
    for x in 0..<grid[0].len:
        let c = grid[y][x]
        if c == '.': continue
        table.mgetOrPut(c, @[]).add((x, y))

var antinodes: HashSet[Pos]
for c, nodes in table:
    for i in 0..<nodes.len-1:
        for j in i+1..<nodes.len:
            let (a, b) = getAntinodes(nodes[i], nodes[j])
            if inBounds(a): antinodes.incl(a)
            if inBounds(b): antinodes.incl(b)
echo antinodes.len

# for i in 0..<grid.len:
#     for j in 0..<grid[0].len:
#         if antinodes.contains((x: j, y: i)):
#             stdout.write '#'
#         else:
#             stdout.write grid[i][j]
#     echo ""
