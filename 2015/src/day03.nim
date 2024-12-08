import std/[sets, strutils]

type Pos = tuple[x: int, y: int]

proc move(c: char, pos: var Pos) =
    case c
        of '>': pos.x += 1
        of 'v': pos.y += 1
        of '<': pos.x -= 1
        of '^': pos.y -= 1
        else: raise newException(ValueError, "invalid")

let input = readAll(stdin).strip

var
    pos: Pos = (x: 0, y: 0)
    visited = toHashSet([pos])
for c in input:
    move(c, pos)
    visited.incl(pos)
echo visited.len

visited.clear
pos = (x: 0, y: 0)
var pos2: Pos = (x: 0, y: 0)
for i in countup(0, input.len-1, 2):
    let
        c1 = input[i]
        c2 = input[i+1]
    move(c1, pos)
    move(c2, pos2)
    visited.incl(pos)
    visited.incl(pos2)
echo visited.len
