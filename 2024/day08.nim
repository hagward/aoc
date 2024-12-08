import std/[sets, sequtils, strutils, tables]

let grid = readAll(stdin).strip.splitLines.mapIt(it.toSeq)

type Pos = tuple[x: int, y: int]

proc inBounds(pos: Pos): bool =
    return pos.x >= 0 and pos.x < grid[0].len and
           pos.y >= 0 and pos.y < grid.len

proc getTwoAntinodes(a, b: Pos): seq[Pos] =
    let
        dx = abs(a.x - b.x)
        dy = abs(a.y - b.y)
        k = (a.y - b.y) / (a.x - b.x)
    if k >= 0:
        return @[(x: min(a.x, b.x) - dx, y: min(a.y, b.y) - dy),
                 (x: max(a.x, b.x) + dx, y: max(a.y, b.y) + dy)].filter(inBounds)
    else:
        return @[(x: min(a.x, b.x) - dx, y: max(a.y, b.y) + dy),
                 (x: max(a.x, b.x) + dx, y: min(a.y, b.y) - dy)].filter(inBounds)

proc getAllAntinodes(a, b: Pos): seq[Pos] =
    let
        dx = abs(a.x - b.x)
        dy = abs(a.y - b.y)
        k = (a.y - b.y) / (a.x - b.x)
    var
        x = a.x
        y = a.y
        antinodes: seq[Pos]
    while inBounds((x, y)):
        antinodes.add((x, y))
        x += dx
        if k >= 0: y += dy else: y -= dy
    x = a.x - dx
    if k >= 0: y = a.y - dy else: y = a.y + dy
    while inBounds((x, y)):
        antinodes.add((x, y))
        x -= dx
        if k >= 0: y -= dy else: y += dy
    return antinodes

var antennasByFrequency = newTable[char, seq[Pos]]()
for y in 0..<grid.len:
    for x in 0..<grid[0].len:
        let c = grid[y][x]
        if c == '.': continue
        antennasByFrequency.mgetOrPut(c, @[]).add((x, y))

var
    antinodesPart1: HashSet[Pos]
    antinodesPart2: HashSet[Pos]
for c, nodes in antennasByFrequency:
    for i in 0..<nodes.len-1:
        for j in i+1..<nodes.len:
            for node in getTwoAntinodes(nodes[i], nodes[j]):
                antinodesPart1.incl(node)
            for node in getAllAntinodes(nodes[i], nodes[j]):
                antinodesPart2.incl(node)
echo antinodesPart1.len
echo antinodesPart2.len
