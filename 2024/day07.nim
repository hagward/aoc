import std/[sequtils, strutils]

proc concatNumbers(x: int, y: int): int =
    return (x.intToStr() & y.intToStr()).parseInt()

proc dfs(current: int, goal: int, numbers: seq[int], concat: bool): bool =
    if current > goal:
        return false
    if numbers.len == 0:
        return current == goal
    return dfs(current + numbers[0], goal, numbers[1 .. ^1], concat) or
           dfs(current * numbers[0], goal, numbers[1 .. ^1], concat) or
           concat and dfs(concatNumbers(current, numbers[0]), goal, numbers[1 .. ^1], concat)

var sumPart1: int = 0
var sumPart2: int = 0
for line in readAll(stdin).strip.splitLines:
    let parts = line.splitWhitespace()
    let testValue = parts[0][0 .. ^2].parseInt()
    let numbers = parts[1 .. ^1].mapIt(it.parseInt)
    if dfs(0, testValue, numbers, false):
        sumPart1 += testValue
    if dfs(0, testValue, numbers, true):
        sumPart2 += testValue
echo sumPart1
echo sumPart2
