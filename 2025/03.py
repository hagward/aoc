with open('input/03.txt') as f:
    lines = f.read().splitlines()

def find_max(n_str):
    max_digit = max(n_str[:-1])
    max_index = n_str.index(max_digit)
    max_digit_2 = max(n_str[max_index+1:])
    return int(max_digit + max_digit_2)

ans = 0
for line in lines:
    ans += find_max(line)
print(ans)
