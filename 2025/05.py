with open('input/05.txt') as f:
    [ranges_raw, ingredients_raw] = [s.split('\n') for s in f.read().strip().split('\n\n')]

ranges = []
for r in ranges_raw:
    [l, r] = map(int, r.split('-'))
    ranges.append([l, r])

ingredients = list(map(int, ingredients_raw))

def in_range(ingredient):
    for [l, r] in ranges:
        if ingredient >= l and ingredient <= r:
            return True
    return False

print(sum(1 for ingredient in ingredients if in_range(ingredient)))

def merge_once():
    for i in range(len(ranges) - 1):
        for j in range(i + 1, len(ranges)):
            [l1, r1] = ranges[i]
            [l2, r2] = ranges[j]
            if l2 <= l1 and r2 >= l1:
                ranges[i] = [l2, max(r1, r2)]
                ranges.pop(j)
                return True
            elif l1 <= l2 and r1 >= l2:
                ranges[i] = [l1, max(r1, r2)]
                ranges.pop(j)
                return True
    return False

while merge_once():
    pass

print(sum(r - l + 1 for [l, r] in ranges))
