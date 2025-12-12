with open("input/05.txt") as f:
    [ranges_raw, ingredients_raw] = [s.split("\n") for s in f.read().strip().split("\n\n")]

ranges = [list(map(int, r.split("-"))) for r in ranges_raw]
ingredients = list(map(int, ingredients_raw))

def in_range(ingredient):
    for [start, end] in ranges:
        if ingredient >= start and ingredient <= end:
            return True
    return False

print(sum(1 for ingredient in ingredients if in_range(ingredient)))

def merge_once():
    for i in range(len(ranges) - 1):
        for j in range(i + 1, len(ranges)):
            [start1, end1] = ranges[i]
            [start2, end2] = ranges[j]
            if start2 <= start1 and end2 >= start1:
                ranges[i] = [start2, max(end1, end2)]
                ranges.pop(j)
                return True
            elif start1 <= start2 and end1 >= start2:
                ranges[i] = [start1, max(end1, end2)]
                ranges.pop(j)
                return True
    return False

while merge_once():
    pass

print(sum(end - start + 1 for [start, end] in ranges))
