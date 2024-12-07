import std/[sequtils, strutils]

proc dfs(current: int, goal: int, numbers: seq[int]): bool =
    if current > goal:
        return false
    if numbers.len == 0:
        return current == goal
    return dfs(current + numbers[0], goal, numbers[1 .. ^1]) or
           dfs(current * numbers[0], goal, numbers[1 .. ^1])

var sum = 0
for line in readAll(stdin).strip.splitLines:
    let parts = line.splitWhitespace()
    let testValue = parts[0][0 .. ^2].parseInt()
    let numbers = parts[1 .. ^1].mapIt(it.parseInt)
    if dfs(0, testValue, numbers):
        sum += testValue
echo sum
