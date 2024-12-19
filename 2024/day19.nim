import std/[strutils, tables]

let input = readAll(stdin).strip.split("\n\n")
let patterns = input[0].split(", ")
let designs = input[1].splitLines

proc check(current: string, target: string, patterns: seq[string]): bool =
    if current == target:
        return true
    for i in 0..<patterns.len:
        let newCurrent = current & patterns[i]
        if target.startsWith(newCurrent):
            if check(newCurrent, target, patterns):
                return true
    return false

var cache: Table[(string, string), int]
proc countWays(current: string, target: string, patterns: seq[string]): int =
    let key = (current, target)
    if key in cache:
        return cache[key]
    if current == target:
        return 1
    var sum = 0
    for i in 0..<patterns.len:
        let newCurrent = current & patterns[i]
        if target.startsWith(newCurrent):
            let ways = countWays(newCurrent, target, patterns)
            cache[(newCurrent, target)] = ways
            sum += ways
        else:
            cache[(newCurrent, target)] = 0
    return sum

var sum1, sum2: int
for design in designs:
    if check("", design, patterns):
        sum1 += 1
    sum2 += countWays("", design, patterns)
echo sum1
echo sum2
