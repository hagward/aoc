import std/[math, sequtils, strutils]

let numbers = readAll(stdin).splitLines.mapIt(it.parseInt)

var sum = 0
for number in numbers:
    var n = number
    for i in 0..<2000:
        n = ((n * 64) xor n) mod 16777216
        n = (n.floorDiv(32) xor n) mod 16777216
        n = ((n * 2048) xor n) mod 16777216
    sum += n
echo sum
