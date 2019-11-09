import math
import sys

def part2(n):
    a = int(math.log(n, 3))
    d = n - 3**a
    w = 0
    for i in range(d):
        w += 1 if i < 3**a else 2
    return w

print(part2(int(sys.argv[1])))
