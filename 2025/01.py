with open('input/01.txt') as f:
    lines = f.read().splitlines()

n = 50
ans = 0
n2 = 50
ans2 = 0
for r in lines:
    if r[0] == 'L':
        n = (n - int(r[1:])) % 100
    else:
        n = (n + int(r[1:])) % 100
    if n == 0:
        ans += 1

    for i in range(int(r[1:])):
        if r[0] == 'L':
            n2 = (n2 - 1) % 100
        else:
            n2 = (n2 + 1) % 100
        if n2 == 0:
            ans2 += 1
print(ans)
print(ans2)
