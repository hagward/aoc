import std/[heapqueue, math, sequtils, strutils, sugar, tables]

var grid: array[71, array[71, bool]]

var i = 0
for nums in readAll(stdin).strip.splitLines.map(line => line.split(",").mapIt(it.parseInt)):
    if i == 1024: break
    i += 1
    let
        x = nums[0]
        y = nums[1]
    grid[y][x] = true

# for i in 0..<grid.len:
#     for j in 0..<grid[0].len:
#         if grid[i][j]:
#             stdout.write '#'
#         else:
#             stdout.write '.'
#     echo ""

let start = (0, 0)
let goal = (grid[0].len-1, grid.len-1)

proc h(node: (int, int)): int = abs(goal[0] - node[0]) + abs(goal[1] - node[1])

var gScore: Table[(int, int), int]
gScore[start] = 0

var fScore: Table[(int, int), int]
fScore[start] = h(start)

proc `<`(a, b: (int, int)): bool = fScore.getOrDefault(a, high(int)) < fScore.getOrDefault(b, high(int))
var openSet = [start].toHeapQueue()

while openSet.len > 0:
    let current = openSet.pop
    if current == goal:
        echo gScore[goal]
        break
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        let x = current[0] + dx
        let y = current[1] + dy
        if x < 0 or x >= grid[0].len or y < 0 or y >= grid.len:
            continue
        if grid[y][x]:
            continue
        let neighbor = (x, y)
        let tentativeGScore = gScore.getOrDefault(current, high(int)) + 1
        if tentativeGScore < gScore.getOrDefault(neighbor, high(int)):
            gScore[neighbor] = tentativeGScore
            fScore[neighbor] = tentativeGScore + h(neighbor)
            if not openSet.contains(neighbor):
                openSet.push(neighbor)
