import std/[re, sequtils, strutils]

proc tokens(ax, ay, bx, by, px, py: float64): int64 =
    let b: float64 = (py*ax - px*ay)/(by*ax - bx*ay)
    let a: float64 = (py - b*by)/ay
    let aInt = int64(a)
    let bInt = int64(b)
    if float64(aInt) == a and float64(bInt) == b:
        return 3*aInt + bInt
    return 0

var tokens1: int64 = 0
var tokens2: int64 = 0
for machine in readAll(stdin).strip.split("\n\n"):
    let ns: seq[float64] = re.findAll(machine, re"\d+").mapIt(it.parseFloat)
    let (ax, ay, bx, by, px, py) = (ns[0], ns[1], ns[2], ns[3], ns[4], ns[5])
    let px2: float64 = px + float64(10000000000000)
    let py2: float64 = py + float64(10000000000000)
    tokens1 += tokens(ax, ay, bx, by, px, py)
    tokens2 += tokens(ax, ay, bx, by, px2, py2)

echo tokens1
echo tokens2
