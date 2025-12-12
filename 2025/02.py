with open("input/02.txt") as f:
    ranges = f.read().split(",")

def is_invalid_1(n_str):
    return n_str[: len(n_str) // 2] == n_str[len(n_str) // 2 :]

def is_invalid_2(n_str):
    for i in range(len(n_str) // 2):
        if len(n_str) % (i + 1) != 0:
            continue
        if n_str[: i + 1] * (len(n_str) // (i + 1)) == n_str:
            return True
    return False

p1 = 0
p2 = 0
for ran in ranges:
    start, end = map(int, ran.split("-"))
    for i in range(start, end + 1):
        i_str = str(i)
        if is_invalid_1(str(i)):
            p1 += i
        if is_invalid_2(str(i)):
            p2 += i
print(p1)
print(p2)
