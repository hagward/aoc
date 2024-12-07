import std/[algorithm, math, tables, sequtils, strutils]

let inputParts = readAll(stdin).strip.split("\n\n")
let (rules, updates) = (inputParts[0], inputParts[1])
let table = newTable[(int, int), bool]()

for rule in rules.split('\n'):
    let numbers = rule.split('|').mapIt(it.parseInt)
    table[(numbers[0], numbers[1])] = true
    table[(numbers[1], numbers[0])] = false

proc isCorrect(update: seq[int]): bool =
    for i in 0..<update.len - 1:
        for j in i + 1..<update.len:
            if not table.getOrDefault((update[i], update[j]), true):
                return false
    return true

proc cmp(x, y: int): int =
    if table.getOrDefault((x, y), true):
        return 1
    else:
        return -1

var sumPart1 = 0
var sumPart2 = 0
for update in updates.split('\n'):
    var numbers = update.split(',').mapIt(it.parseInt)
    if isCorrect(numbers):
        sumPart1 += numbers[floorDiv(numbers.len, 2)]
    else:
        numbers.sort(cmp)
        sumPart2 += numbers[floorDiv(numbers.len, 2)]
echo sumPart1
echo sumPart2
