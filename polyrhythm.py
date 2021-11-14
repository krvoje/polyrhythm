#!/usr/bin/env python3

import sys
import time

X='x'

def mult(l):
    res = 1
    for x in l: res = res*x
    return res

def leastCommonMultiple(measures):
    ds = set(measures)
    ms = set(measures)
    for x in ds:
        for y in ds:
            if x > y and x % y == 0 and x in ms:
                ms.remove(x)
                ms.add(int(x/y))
    return mult(set(ms))

def mono(r,lcm):
    lm = lambda x: X if x%r == 0 else '.'
    m = "".join(list(map(lm, range(lcm))))
    print(m)

def divides(ms, x):
    for m in ms:
        if x%m == 0: return True
    return False

def poly(measures):
    lcm = leastCommonMultiple(measures)
    for measure in measures:
        mono(measure, lcm)
    lz = lambda x: X if divides(measures, x) else '.'
    z = "".join(list(map(lz, range(lcm))))
    print(z)

if len(sys.argv) < 2:
    print("""
Prints out polyrhythms for a given combination of beats.
First rows are the individual beats, the last row is the combined rhythym.

Examples:

# Prints a 3:2 polyrhythm
{0} 3 2
x.x.x.
x..x..
x.xxx.

# Prints a 3:4:5 polyrhythm
{0} 3 4 5
x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..
x...x...x...x...x...x...x...x...x...x...x...x...x...x...x...
x....x....x....x....x....x....x....x....x....x....x....x....
x..xxxx.xxx.x..xx.x.xx..xx.xx.x.xx.xx..xx.x.xx..x.xxx.xxxx..
""".format(sys.argv[0]))
    print("For example: ./poliritmovi.py 3 2 for 3:2 polyrhythm")
else:
    measures = sorted([int(x) for x in sys.argv[1:]])
    poly(measures)
