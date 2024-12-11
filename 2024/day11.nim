import std/[sequtils, strutils]

var stones = readAll(stdin).strip.splitWhitespace.mapIt(it.parseInt)

for i in 0..<25:
    var j = 0
    while j < stones.len:
        let stone = stones[j]
        if stone == 0:
            stones[j] = 1
        else:
            let s = intToStr(stone)
            if s.len mod 2 == 0:
                let
                    l = parseInt(s[0 .. int(s.len/2)-1])
                    r = parseInt(s[int(s.len/2) .. ^1])
                stones[j] = l
                stones.insert(r, j + 1)
                j += 1
            else:
                stones[j] = stone * 2024
        j += 1

echo stones.len
