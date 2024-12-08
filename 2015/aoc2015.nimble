# Package

version       = "0.1.0"
author        = "Anders Hagward"
description   = "A new awesome nimble package"
license       = "MIT"
srcDir        = "src"
binDir        = "bin"
bin           = @["day01", "day02", "day03", "day04"]

# Dependencies

requires "nim >= 2.2.0"

requires "checksums >= 0.2.1"