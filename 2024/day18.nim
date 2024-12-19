import std/[heapqueue, options, sequtils, strformat, strutils, sugar, tables]

type
  Point = tuple[x, y: int]
  Node = ref object
    point: Point
    g: int
    f: int

proc `<`(a, b: Node): bool = a.f < b.f

proc h(a, b: Point): int = abs(a.x - b.x) + abs(a.y - b.y)

let input = readAll(stdin).strip.splitLines.map(line => line.split(",").mapIt(it.parseInt))

var grid: array[71, array[71, bool]]
for i in 0..<1024:
    let
        x = input[i][0]
        y = input[i][1]
    grid[y][x] = true

proc aStar(start, goal: Point): Option[int] =
    let startNode = Node(point: start, g: 0, f: h(start, goal))
    var
        heap = [startNode].toHeapQueue
        visited: Table[Point, Node]
    visited[start] = startNode

    while heap.len > 0:
        let currentNode = heap.pop

        if currentNode.point == goal:
            return some(currentNode.g)

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            let
                x = currentNode.point.x + dx
                y = currentNode.point.y + dy
            if x < 0 or x >= grid[0].len or y < 0 or y >= grid.len or grid[y][x]:
                continue
            let neighbor = (x, y)
            let neighborNode = if visited.contains(neighbor): visited[neighbor] else: Node(point: neighbor, g: high(int), f: high(int))
            let tentativeGScore = currentNode.g + 1
            if tentativeGScore < neighborNode.g:
                neighborNode.g = tentativeGScore
                neighborNode.f = tentativeGScore + h(neighbor, goal)
                heap.push(neighborNode)
                visited[neighbor] = neighborNode
    return none(int)

let start = Point((0, 0))
let goal = Point((grid[0].len - 1, grid.len - 1))

echo aStar(start, goal).get()

for i in 1024..<input.len:
    let
        x = input[i][0]
        y = input[i][1]
    grid[y][x] = true
    if aStar(start, goal).isNone:
        echo fmt"{x},{y}"
        break
