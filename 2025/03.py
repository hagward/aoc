with open("input/03.txt") as f:
    lines = f.read().splitlines()

def find_max(n_str: str, k: int):
    max_num = ""
    start = 0
    end = len(n_str) - k
    while len(max_num) < k:
        part = n_str[start : end + 1]
        max_digit = max(part)
        max_index = part.index(max_digit)
        max_num += max_digit
        start += max_index + 1
        end += 1
    return int(max_num)

p1 = 0
p2 = 0
for line in lines:
    p1 += find_max(line, 2)
    p2 += find_max(line, 12)
print(p1)
print(p2)
