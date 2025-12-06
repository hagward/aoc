with open('input/06.txt') as f:
    lines = [line.split() for line in f.read().splitlines()]

answers = [int(n) for n in lines[0]]

for i in range(1, len(lines) - 1):
    for j in range(len(lines[i])):
        if lines[len(lines) - 1][j] == '+':
            answers[j] += int(lines[i][j])
        else:
            answers[j] *= int(lines[i][j])

print(sum(answers))
