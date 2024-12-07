import std/[algorithm, tables, sequtils, strutils]

proc splitToTuple(s: string): (int, int) =
    let parts = s.splitWhitespace().mapIt(it.parseInt)
    return (parts[0], parts[1])

var (left, right) = readAll(stdin).strip.splitLines.mapIt(it.splitToTuple).unzip
left.sort()
right.sort()

var distance = 0
for (a, b) in zip(left, right):
    distance += abs(a - b)
echo distance

let counts = toCountTable(right)
var similarity = 0
for x in left:
    similarity += counts.getOrDefault(x, 0) * x
echo similarity
