import std/strutils

var
    floor = 0
    pos = 0
    basement = 0
for c in readAll(stdin).strip:
    pos += 1
    if c == '(':
        floor += 1
    else:
        floor -= 1
    if floor == -1 and basement == 0:
        basement = pos
echo floor
echo basement
