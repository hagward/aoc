with open('input/05.txt') as f:
    [ranges_raw, ingredients_raw] = [s.split('\n') for s in f.read().strip().split('\n\n')]

ranges = []
for r in ranges_raw:
    [min, max] = map(int, r.split('-'))
    ranges.append([min, max])

ingredients = list(map(int, ingredients_raw))

def in_range(ingredient):
    for [min, max] in ranges:
        if ingredient >= min and ingredient <= max:
            return True
    return False

ans = 0
for ingredient in ingredients:
    if in_range(ingredient):
        ans += 1
print(ans)
