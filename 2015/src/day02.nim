import std/[algorithm, sequtils, strutils]

var
    totalArea = 0
    totalRibbon = 0
for line in readAll(stdin).strip.splitLines:
    let
        sides = line.split('x').mapIt(it.parseInt).sorted
        a = sides[0] * sides[1]
        b = sides[1] * sides[2]
        c = sides[0] * sides[2]
        area = 2*a + 2*b + 2*c + a
        ribbon = 2*sides[0] + 2*sides[1] + sides[0]*sides[1]*sides[2]
    totalArea += area
    totalRibbon += ribbon
echo totalArea
echo totalRibbon
