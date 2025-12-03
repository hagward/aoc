with open('input/03.txt') as f:
    lines = f.read().splitlines()

def find_max(n_str: str, k: int):
    ret = ''
    l = 0
    r = len(n_str) - k
    while len(ret) < k:
        part = n_str[l:r+1]
        max_digit = max(part)
        max_index = part.index(max_digit)
        ret += max_digit
        l += max_index + 1
        r += 1
    return int(ret)

ans = 0
ans2 = 0
for line in lines:
    ans += find_max(line, 2)
    ans2 += find_max(line, 12)
print(ans)
print(ans2)
