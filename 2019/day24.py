def step(src, dest):
  for i in range(len(src)):
    for j in range(len(src[i])):
      adjacent_bugs = num_adjacent(src, i, j, '#')
      if src[i][j] == '#' and adjacent_bugs != 1:
        dest[i][j] = '.'
      elif src[i][j] == '.' and (adjacent_bugs == 1 or adjacent_bugs == 2):
        dest[i][j] = '#'
      else:
        dest[i][j] = src[i][j]

def num_adjacent(src, i, j, c):
  count = 0
  if i-1 >= 0 and src[i-1][j] == c: count += 1
  if i+1 < len(src) and src[i+1][j] == c: count += 1
  if j-1 >= 0 and src[i][j-1] == c: count += 1
  if j+1 < len(src[i]) and src[i][j+1] == c: count += 1
  return count

def biodiversity_rating(src):
  rating = 0
  for i in range(len(src)):
    for j in range(len(src[i])):
      if src[i][j] == '#': rating += 2 ** (len(src[i]) * i + j)
  return rating

def serialize(src):
  return ''.join(item for row in src for item in row)

m1 = [
  list('..##.'),
  list('#....'),
  list('.....'),
  list('#.#.#'),
  list('#..#.')
]
m2 = [['' for c in row] for row in m1]

curr_serialized = serialize(m1)

seen = set()
even = True

while curr_serialized not in seen:
  seen.add(curr_serialized)
  if even: step(m1, m2)
  else: step(m2, m1)
  curr_serialized = serialize(m2 if even else m1)
  even = not even

m = m1 if even else m2

for row in m: print(''.join(row))
print('\n', biodiversity_rating(m))
