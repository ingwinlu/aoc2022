
from functools import cmp_to_key
from math import prod

input =  open("input/day13.txt").read()


def cmp(l, r):
    match l, r:
        case int(), int(): return (l>r) - (l<r)
        case int(), list(): return cmp([l], r)
        case list(), int(): return cmp(l, [r])
        case list(), list():
            for z in map(cmp, l, r):
                if z: return z
            return cmp(len(l), len(r))

packets = [[*map(eval, x.split())] for x in input.split('\n\n')]
print(sum(i for i, p in enumerate(packets, 1) if cmp(*p) == -1))

packets = sorted(sum(packets, [[2], [6]]), key=cmp_to_key(cmp))
print(prod(i for i, p in enumerate(packets, 1) if p in [[2], [6]]))