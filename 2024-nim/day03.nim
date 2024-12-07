import std/[re, sequtils, strutils]

let input = readAll(stdin).strip

var sumPart1 = 0
var sumPart2 = 0
var enabled = true
for match in findAll(input, re"do\(\)|don't\(\)|mul\(\d+,\d+\)"):
    if match == "do()":
        enabled = true
    elif match == "don't()":
        enabled = false
    else:
        let numbers = match[4 .. ^2].split(',').mapIt(it.parseInt)
        let product = numbers[0] * numbers[1]
        sumPart1 += product
        if enabled:
            sumPart2 += product
echo sumPart1
echo sumPart2
