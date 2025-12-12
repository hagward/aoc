from collections import defaultdict

with open("input/11.txt") as f:
    lines = f.read().splitlines()

connections = defaultdict(list)

for line in lines:
    splitted = line.split()
    key = splitted[0][:-1]
    connections[key] = splitted[1:]

def dfs(node):
    if node == "out":
        return 1
    return sum([dfs(x) for x in connections[node]])

mem = {}

def dfs2(node, dac, fft):
    if node == "out":
        return 1 if dac and fft else 0
    if (node, dac, fft) in mem:
        return mem[(node, dac, fft)]
    result = sum([dfs2(x, dac or node == "dac", fft or node == "fft") for x in connections[node]])
    mem[(node, dac, fft)] = result
    return result

print(dfs("you"))
print(dfs2("svr", False, False))
