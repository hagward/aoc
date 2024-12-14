import std/[re, sequtils, strutils]

let rows = 103
let cols = 101

var robots: seq[(int, int, int, int)]
for line in readAll(stdin).strip.splitLines:
    let nums = re.findAll(line, re"(-?\d+)+").mapIt(it.parseInt)
    robots.add((nums[0], nums[1], nums[2], nums[3]))

for i in 0..<100:
    for (px, py, vx, vy) in robots.mitems:
        px = (px + vx + cols) mod cols
        py = (py + vy + rows) mod rows

var counts: seq[seq[int]]

for i in 0..<rows:
    var c: seq[int]
    for j in 0..<cols:
        var sum = 0
        for (px, py, vx, vy) in robots:
            if px == j and py == i:
                sum += 1
        c.add(sum)
    counts.add(c)

let
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
