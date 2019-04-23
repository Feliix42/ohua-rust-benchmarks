#!/bin/env python3

import random
import sys

if len(sys.argv) != 5:
    print("Usage: ./generate.py x y z n")
    sys.exit(1)

x = int(sys.argv[1])
y = int(sys.argv[2])
z = int(sys.argv[3])
numPath = int(sys.argv[4])

random.seed(0)

print("# Dimensions (x, y, z)")
print("d  {} {} {}".format(x, y, z))
print("")
print("# Paths: Sources (x, y, z) -> Destinations (x, y, z)")
assert(numPath <= (y*z))
cur_y = 0
cur_z = 0
for i in range(numPath):
    src = (0, cur_y, cur_z)
    while True:
        dst = (x-1, cur_y, cur_z)
        if dst != src:
            break;
    print("p   {0:>3d} {1:>3d} {2:>1d}   {3:>3d} {4:>3d} {5:>1d}".format(src[0], src[1], src[2], dst[0], dst[1], dst[2]))
    cur_y = (cur_y + 1) % y
    if cur_y == 0:
        cur_z += 1
