import std/[math, sequtils, strutils, sugar]

let disk = readAll(stdin).strip.map(it => int(it) - int('0'))

var expanded: seq[int]
for i in 0..<disk.len:
    for j in 0..<disk[i]:
        if i mod 2 == 0:
            expanded.add(i.floorDiv(2))
        else:
            expanded.add(-1)

# for e in expanded:
#     if e == -1:
#         stdout.write "."
#     else:
#         stdout.write e
# echo ""

var freeI = 0
for i in countdown(expanded.len-1, 0):
    if expanded[i] == -1: continue
    while freeI < i and expanded[freeI] != -1:
        freeI += 1
    if freeI < i:
        expanded[freeI] = expanded[i]
        expanded[i] = -1

# for e in expanded:
#     if e == -1:
#         stdout.write "."
#     else:
#         stdout.write e
# echo ""

var sum = 0
for i in 0..<expanded.len:
    if expanded[i] == -1: continue
    sum += i*expanded[i]
echo sum
