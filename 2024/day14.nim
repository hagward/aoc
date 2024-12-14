import std/[re, sequtils, strutils]

let rows = 103
let cols = 101

proc runRobots(robots: var seq[(int, int, int, int)]) =
    for (px, py, vx, vy) in robots.mitems:
        px = (px + vx + cols) mod cols
        py = (py + vy + rows) mod rows

proc countRobots(robots: seq[(int, int, int, int)]): seq[seq[int]] =
    var counts = newSeq[seq[int]](rows)
    for i in 0..<rows:
        counts[i] = newSeq[int](cols)
    for (px, py, _, _) in robots.items:
        counts[py][px] += 1
    return counts

proc countConnectedRobots(robots: seq[(int, int, int, int)]): int =
    let counts = countRobots(robots)
    var sum = 0
    for i in 0..<rows:
        for j in 0..<cols:
            if counts[i][j] == 0: continue
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                let (x, y) = (j + dx, i + dy)
                if x >= 0 and x < cols and y >= 0 and y < rows and counts[y][x] > 0:
                    sum += 1
                    continue
    return sum

var robots1: seq[(int, int, int, int)]
for line in readAll(stdin).strip.splitLines:
    let nums = re.findAll(line, re"(-?\d+)+").mapIt(it.parseInt)
    robots1.add((nums[0], nums[1], nums[2], nums[3]))
var robots2 = robots1

for i in 0..<100:
    runRobots(robots1)

let
    counts = countRobots(robots1)
    xm1 = int((cols-1)/2)
    xm2 = int((cols+1)/2)
    ym1 = int((rows-1)/2)
    ym2 = int((rows+1)/2)

var safetyFactor = 1
for (xmin, xmax, ymin, ymax) in [(0, xm1, 0, ym1), (xm2, cols, 0, ym1), (0, xm1, ym2, rows), (xm2, cols, ym2, rows)]:
    var sum = 0
    for y in ymin..<ymax:
        for x in xmin..<xmax:
            sum += counts[y][x]
    safetyFactor *= sum
echo safetyFactor

for i in 1..10000:
    runRobots(robots2)
    if countConnectedRobots(robots2) >= int(float(robots2.len) / 1.5):
        echo i
        break
