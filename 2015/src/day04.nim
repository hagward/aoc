import std/strutils
import checksums/md5

let input = readAll(stdin).strip

var n = 0
while true:
    if getMD5(input & intToStr(n))[0..4] == "00000":
        echo n
        break
    n += 1

while true:
    if getMD5(input & intToStr(n))[0..5] == "000000":
        echo n
        break
    n += 1
