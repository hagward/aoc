import std/[math, sequtils, strutils, sugar]

type NodeType = enum File, Free
type Node = tuple[nodeType: NodeType, size: int, id: int]

var input = readAll(stdin).strip.map(it => int(it) - int('0'))

var
    disk: seq[int]
    nodes: seq[Node]
for i in 0..<input.len:
    let nodeType = if i mod 2 == 0: File else: Free
    nodes.add((nodeType, input[i], i.floorDiv(2)))
    for j in 0..<input[i]:
        if i mod 2 == 0:
            disk.add(i.floorDiv(2))
        else:
            disk.add(-1)

var freeI = 0
for i in countdown(disk.len-1, 0):
    if disk[i] == -1: continue
    while freeI < i and disk[freeI] != -1:
        freeI += 1
    if freeI < i:
        disk[freeI] = disk[i]
        disk[i] = -1

var sum = 0
for i in 0..<disk.len:
    if disk[i] == -1: continue
    sum += i*disk[i]
echo sum

for i in 0..<nodes.len:
    if nodes[i].nodeType != Free: continue
    for j in countdown(nodes.len-1, i+1):
        if nodes[j].nodeType != File: continue
        if nodes[j].size > nodes[i].size: continue
        nodes.insert(nodes[j], i)
        nodes[j+1].nodeType = Free
        nodes[i+1].size -= nodes[i].size
        break

var
    sum2 = 0
    index = 0
for (nodeType, size, id) in nodes:
    if nodeType == File:
        for i in index..<index+size:
            sum2 += i * id
    index += size
echo sum2
