import std/[sequtils, strutils, sugar]

let
    blocks = readAll(stdin).split("\n\n").mapIt(it.splitLines)
    rows = blocks[0].len
    cols = blocks[0][0].len

var locks, keys: seq[seq[int]]

for b in blocks:
    var heights = newSeq[int](cols)
    for r in 0..<rows:
        for c in 0..<cols:
            if b[r][c] == '#':
                heights[c] += 1
    if all(b[0], c => c == '#'):
        locks.add(heights)
    else:
        keys.add(heights)

var sum = 0
for lock in locks:
    for key in keys:
        var overlaps = false
        for i in 0..<cols:
            if lock[i] + key[i] > rows:
                overlaps = true
                break
        if not overlaps:
            sum += 1
echo sum
