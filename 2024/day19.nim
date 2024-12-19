import std/strutils

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

var sum = 0
for design in designs:
    if check("", design, patterns):
        sum += 1
echo sum
