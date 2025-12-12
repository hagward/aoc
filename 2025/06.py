with open("input/06.txt") as f:
    lines = f.read().splitlines()

splitted = [line.split() for line in lines]
rows = len(splitted)
cols = len(splitted[0])

answers = [int(n) for n in splitted[0]]
for i in range(1, rows - 1):
    for j in range(cols):
        if splitted[rows - 1][j] == "+":
            answers[j] += int(splitted[i][j])
        else:
            answers[j] *= int(splitted[i][j])
print(sum(answers))

column_widths = [0 for _ in range(cols)]
for row in splitted:
    for i in range(cols):
        if len(row[i]) > column_widths[i]:
            column_widths[i] = len(row[i])

ans2 = 0
start = 0
for i in range(cols):
    is_add = lines[rows - 1][start] == "+"
    part = 0 if is_add else 1
    for j in range(column_widths[i]):
        n = ""
        for k in range(rows - 1):
            if start + j < len(lines[k]):
                n += lines[k][start + j]
        if is_add:
            part += int(n.strip())
        else:
            part *= int(n.strip())
    ans2 += part
    start += column_widths[i] + 1
print(ans2)
