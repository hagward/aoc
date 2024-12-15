import std/[sequtils, strutils]

let input = readAll(stdin).strip.split("\n\n")
var grid = input[0].splitLines.mapIt(it.toSeq)
let moves = input[1]

let rows = grid.len
let cols = grid[0].len

proc findRobot(): (int, int) =
    for y in 0..<rows:
        for x in 0..<cols:
            if grid[y][x] == '@':
                return (x, y)

var (rx, ry) = findRobot()

proc moveRobot(move: char) =
    var dx, dy: int
    if move == '>': dx = 1
    elif move == '<': dx = -1
    elif move == 'v': dy = 1
    elif move == '^': dy = -1
    else: return

    let x = rx + dx
    let y = ry + dy
    if x < 0 or x >= cols or y < 0 or y >= rows:
        return

    # echo "move from ", rx, ",", ry, " to ", x, ",", y
    if grid[y][x] == '.':
        grid[ry][rx] = '.'
        grid[y][x] = '@'
        rx = x
        ry = y
    elif grid[y][x] == 'O':
        # echo "found box"
        var xx = x + dx
        var yy = y + dy
        var steps = 1
        while xx >= 0 and xx < cols and yy >= 0 and yy < rows and grid[yy][xx] == 'O':
            xx += dx
            yy += dy
            steps += 1
        if grid[yy][xx] == '#': return
        for i in 0..steps:
            grid[yy][xx] = grid[yy - dy][xx - dx]
            xx -= dx
            yy -= dy
            grid[yy][xx] = '.'
        rx = x
        ry = y

proc printGrid() =
    for y in 0..<rows:
        for x in 0..<cols:
            stdout.write grid[y][x]
        echo ""

# printGrid()
# echo ""
for move in moves:
    moveRobot(move)
    # echo move
    # printGrid()
    # echo ""
# printGrid()

var sum = 0
for y in 0..<rows:
    for x in 0..<cols:
        if grid[y][x] != 'O': continue
        sum += 100 * y + x
echo sum
