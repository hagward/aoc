from collections import deque

with open("input/10.txt") as f:
    lines = f.read().splitlines()

def press_button(lights, button):
    result = list(lights)
    for i in button:
        result[i] = "#" if result[i] == "." else "."
    return "".join(result)

def min_button_presses(lights, buttons):
    visited = set()
    q = deque([(0, lights.replace("#", "."))])
    while q:
        (dist, node) = q.popleft()
        if node == lights:
            return dist
        if node not in visited:
            visited.add(node)
            for button in buttons:
                new_node = press_button(node, button)
                if new_node not in visited:
                    q.append((dist + 1, new_node))
    raise

p1 = 0
for line in lines:
    splitted = line.split()
    lights = splitted[0][1:-1]
    buttons = [list(map(int, s[1:-1].split(","))) for s in splitted[1:-1]]
    p1 += min_button_presses(lights, buttons)
print(p1)
