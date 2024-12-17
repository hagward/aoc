import std/[math, sequtils, strutils]

let input = readAll(stdin).splitLines

var reg: array[3, int]
reg[0] = parseInt(input[0][12..^1])
reg[1] = parseInt(input[1][12..^1])
reg[2] = parseInt(input[2][12..^1])
let program = input[4][9..^1].split(',').mapIt(it.parseInt)

proc combo(operand: int): int =
    case operand:
        of 0, 1, 2, 3:
            return operand
        of 4, 5, 6:
            return reg[operand-4]
        else:
            raise newException(ValueError, "Invalid operand: " & intToStr(operand)) 

var hasPrinted = false
var ip = 0
while ip < program.len:
    let opcode = program[ip]
    let operand = program[ip+1]

    case opcode:
        of 0:
            reg[0] = reg[0].floorDiv(2 ^ combo(operand))
            ip += 2
        of 1:
            reg[1] = reg[1] xor operand
            ip += 2
        of 2:
            reg[1] = combo(operand) mod 8
            ip += 2
        of 3:
            if reg[0] != 0:
                ip = operand
            else:
                ip += 2
        of 4:
            reg[1] = reg[1] xor reg[2]
            ip += 2
        of 5:
            if hasPrinted:
                stdout.write ','
            hasPrinted = true
            stdout.write combo(operand) mod 8
            ip += 2
        of 6:
            reg[1] = reg[0].floorDiv(2 ^ combo(operand))
            ip += 2
        of 7:
            reg[2] = reg[0].floorDiv(2 ^ combo(operand))
            ip += 2
        else:
            raise newException(ValueError, "Invalid opcode: " & intToStr(opcode))
echo ""
