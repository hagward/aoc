import std/[sets, sequtils, strutils, tables]

let grid = readAll(stdin).strip.splitLines.mapIt(it.toSeq)

proc inBounds(x, y: int): bool = x >= 0 and x < grid[0].len and y >= 0 and y < grid.len

var antennasByFrequency = newTable[char, seq[(int, int)]]()
for y in 0..<grid.len:
    for x in 0..<grid[0].len:
        let c = grid[y][x]
        if c == '.': continue
        antennasByFrequency.mgetOrPut(c, @[]).add((x, y))

var
    antinodesPart1: HashSet[(int, int)]
    antinodesPart2: HashSet[(int, int)]
for antennas in antennasByFrequency.values:
    for i in 0..<antennas.len-1:
        for j in i+1..<antennas.len:
            let
                (x1, y1) = antennas[i]
                (x2, y2) = antennas[j]
                dx = x2 - x1
                dy = y2 - y1
            antinodesPart1.incl((x1 - dx, y1 - dy))
            antinodesPart1.incl((x2 + dx, y2 + dy))
            var
                x = x1
                y = y1
            while inBounds(x, y):
                antinodesPart2.incl((x, y))
                x += dx
                y += dy
            x = x1
            y = y1
            while inBounds(x, y):
                antinodesPart2.incl((x, y))
                x -= dx
                y -= dy
echo antinodesPart1.toSeq.filterIt(inBounds(it[0], it[1])).len
echo antinodesPart2.len
