import std/[math, sequtils, strutils, tables, sets]

proc nextSecret(num: int): int =
    var n = num
    n = ((n * 64) xor n) mod 16777216
    n = (n.floorDiv(32) xor n) mod 16777216
    n = ((n * 2048) xor n) mod 16777216
    return n

let numbers = readAll(stdin).splitLines.mapIt(it.parseInt)

var sum = 0
for number in numbers:
    var secret = number
    for i in 0..<2000:
        secret = nextSecret(secret)
    sum += secret
echo sum

var bananas: Table[(int, int, int, int), int]
for number in numbers:
    var
        secret = number
        prices = @[secret mod 10]
    for i in 0..<2000:
        secret = nextSecret(secret)
        prices.add(secret mod 10)

    var seen: HashSet[(int, int, int, int)]
    for i in 0..<prices.len-4:
        let
            a = prices[i]
            b = prices[i+1]
            c = prices[i+2]
            d = prices[i+3]
            e = prices[i+4]
            key = (b-a, c-b, d-c, e-d)
        if seen.contains(key): continue
        seen.incl(key)
        if not bananas.contains(key): bananas[key] = 0
        bananas[key] += e

echo max(bananas.values.toSeq)

# 
# var bananas: Table[(int, int, int, int), int]
# for number in numbers:
#     var
#         window = [number mod 10].toDeque
#         seen: HashSet[(int, int, int, int)]
#         secret = number
#     for i in 0..<2000:
#         secret = nextSecret(secret)
#         window.addLast(secret mod 10)
#         if window.len >= 5:
#             let
#                 a = window[0]
#                 b = window[1]
#                 c = window[2]
#                 d = window[3]
#                 e = window[4]
#                 key = (b-a, c-b, d-c, e-d)
#             if seen.contains(key): continue
#             seen.incl(key)
#             if not bananas.contains(key): bananas[key] = 0
#             bananas[key] += e
#             window.popFirst
# # echo bananas
# 
# echo max(bananas.values.toSeq)
# # var maxKey = (0, 0, 0, 0)
# # var max = 0
# # for key, val in bananas:
# #     if val > max:
# #         max = val
# #         maxKey = key
# # echo max
# # echo maxKey
# # echo bananas[(-2, 1, -1, 3)]