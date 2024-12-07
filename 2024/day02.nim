import std/[sequtils, strutils]

proc diffIsOk(pair: (int, int)): bool =
    let diff = abs(pair[0] - pair[1])
    return diff >= 1 and diff <= 3

proc increasing(pair: (int, int)): bool = pair[0] < pair[1]

proc decreasing(pair: (int, int)): bool = pair[0] > pair[1]

proc isSafe(levels: seq[int]): bool =
    let pairs = zip(levels, levels[1 .. ^1])
    return pairs.allIt(it.diffIsOk) and (pairs.allIt(it.increasing) or pairs.allIt(it.decreasing))

var countPart1 = 0
var countPart2 = 0
for line in readAll(stdin).strip.splitLines:
    let levels = line.splitWhitespace().mapIt(it.parseInt)
    if isSafe(levels):
        countPart1 += 1
        countPart2 += 1
    else:
        for i in 0..<levels.len:
            if isSafe(concat(levels[0 .. i - 1], levels[i + 1 .. ^1])):
                countPart2 += 1
                break
echo countPart1
echo countPart2
