import std/[math, sequtils, strutils, sugar, tables]

var stones = readAll(stdin).strip.splitWhitespace.mapIt(it.parseInt)

let cache = newTable[(int, int), int]()
proc count(stone, blinksLeft: int): int =
    if (stone, blinksLeft) in cache:
        return cache[(stone, blinksLeft)]
    var returnValue: int
    if blinksLeft == 0:
        returnValue = 1
    elif stone == 0:
        returnValue = count(1, blinksLeft - 1)
    else:
        let stoneStr = intToStr(stone)
        if stoneStr.len mod 2 == 0:
            let
                l = parseInt(stoneStr[0 .. int(stoneStr.len/2)-1])
                r = parseInt(stoneStr[int(stoneStr.len/2) .. ^1])
            returnValue = count(l, blinksLeft - 1) + count(r, blinksLeft - 1)
        else:
            returnValue = count(stone * 2024, blinksLeft - 1)
    cache[(stone, blinksLeft)] = returnValue
    return returnValue

echo stones.map(stone => count(stone, 25)).sum()
echo stones.map(stone => count(stone, 75)).sum()
