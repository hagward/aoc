with open("input/01.txt") as f:
    lines = f.read().splitlines()

n = 50
p1 = 0
p2 = 0

for r in lines:
    d = int(r[1:])

    if r[0] == "R":
        p2 += (n + d) // 100
        n = (n + d) % 100
    else:
        # 100 - n2 to invert it so it behaves like R.
        add = (100 - n + d) // 100

        # Standing at 0 and turning left 1 click should not count.
        if n == 0 and add > 0:
            add -= 1

        p2 += add
        n = (n - d) % 100

    if n == 0:
        p1 += 1

print(p1)
print(p2)
