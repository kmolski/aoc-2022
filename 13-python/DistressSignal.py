#!/usr/bin/env python3

from itertools import chain, zip_longest
from sys import argv


def parse_msg_pair(pair):
    return tuple(eval(s) for s in pair.splitlines())


def parse_messages(text):
    return [parse_msg_pair(p) for p in text.split("\n\n")]


def sorted_composite(left, right):
    for t in zip_longest(left, right):
        match t:
            case (None, _):
                return True
            case (_, None):
                return False
            case (fst, snd):
                cmp = sorted_pair(fst, snd)
                if cmp is not None: return cmp


def sorted_pair(left, right):
    match left, right:
        case (int(fst), int(snd)) if fst != snd:
            return fst < snd
        case (list(_), list(_)):
            cmp = sorted_composite(left, right)
            if cmp is not None: return cmp
        case (list(_), int(_)):
            return sorted_pair(left, [right])
        case (int(_), list(_)):
            return sorted_pair([left], right)


def part_1(pairs):
    index_sum = 0
    for i, (left, right) in enumerate(pairs, start=1):
        index_sum += i if sorted_pair(left, right) else 0
    return index_sum


def part_2(pairs):
    packets = list(chain.from_iterable(pairs))
    packets += [ [[2]], [[6]] ]

    # bubble sort
    for i in range(len(packets) - 1, 0, -1):
        for j in range(i):
            if not sorted_pair(packets[j], packets[j + 1]):
                packets[j], packets[j + 1] = packets[j + 1], packets[j]

    return (packets.index([[2]]) + 1) * (packets.index([[6]]) + 1)


with open(argv[1]) as input_file:
    pairs = parse_messages(input_file.read())

print(f"Part 1: {part_1(pairs)}")
print(f"Part 2: {part_2(pairs)}")
