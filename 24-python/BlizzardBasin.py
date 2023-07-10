#!/usr/bin/env python3

from copy import copy
from heapq import heappush, heappop
from sys import argv


class PriorityQueue:
    def __init__(self):
        self.elements = []

    def empty(self):
        return not self.elements

    def put(self, item, priority):
        heappush(self.elements, (priority, item))

    def get(self):
        return heappop(self.elements)[1]


def parse_map(text):
    tiles, walls, blizzards = [], [], []
    for y, row in enumerate(text.splitlines()):
        for x, col in enumerate(row):
            match col:
                case "#": walls.append((x, y))
                case ".": tiles.append((x, y))
                case "^" | ">" | "v" | "<":
                    tiles.append((x, y))
                    blizzards.append((x, y, col))

    return tiles, walls, frozenset(blizzards)


def point_diff(a, b):
    return tuple(coord_a - coord_b for (coord_a, coord_b) in zip(a, b))


def manhattan_dist(a, b):
    return sum(abs(coord) for coord in point_diff(a, b))


def simulate_blizzards(current, walls):
    next_blizzards = []
    max_x, max_y = max(p[0] for p in walls), max(p[1] for p in walls)
    for x, y, dir in current:
        match dir:
            case "^": y -= 1
            case ">": x += 1
            case "v": y += 1
            case "<": x -= 1
        if x <= 0: x = max_x - 1
        elif x >= max_x: x = 1
        elif y <= 0: y = max_y - 1
        elif y >= max_y: y = 1
        next_blizzards.append((x, y, dir))

    return frozenset(next_blizzards)


def next_positions(current, tiles, get_blizzards):
    ns = []
    for diff in ((0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)):
        pos = (current[0] + diff[0], current[1] + diff[1], current[2] + 1)
        if pos[:2] in tiles and pos[:2] not in get_blizzards(pos[2]):
            ns.append(pos)
    return ns


def solve_astar(tiles, walls, blizzards, start, end, minute):
    full_cache, partial_cache = {minute: blizzards}, {minute: frozenset(p[:2] for p in blizzards)}

    def get_blizzards(curr):
        if curr not in full_cache:
            prev = full_cache[curr - 1]
            full_cache[curr] = simulate_blizzards(prev, walls)
            partial_cache[curr] = frozenset(p[:2] for p in full_cache[curr])
        return partial_cache[curr]

    frontier = PriorityQueue()
    frontier.put(start, 0)
    came_from = {start: None}
    cost = {start: 0}

    while frontier:
        current = frontier.get()
        if current[:2] == end:
            return current[2]
        for next in next_positions(current, tiles, get_blizzards):
            new_cost = cost[current] + 1
            if next not in cost or new_cost < cost[next]:
                cost[next] = new_cost
                priority = new_cost + manhattan_dist(next[:2], end)
                frontier.put(next, priority)
                came_from[next] = current


def part_1(tiles, walls, blizzards):
    start = (1, 0, 0)
    end = max(tiles, key=lambda p: p[1])
    return solve_astar(tiles, walls, blizzards, start, end, 0)


def part_2(tiles, walls, blizzards1):
    start1, end1 = (1, 0, 0), max(tiles, key=lambda p: p[1])
    first = solve_astar(tiles, walls, blizzards1, start1, end1, 0)

    start2, end2 = (*end1, first), (1, 0)
    blizzards2 = copy(blizzards1)
    for i in range(first): blizzards2 = simulate_blizzards(blizzards2, walls)
    second = solve_astar(tiles, walls, blizzards2, start2, end2, first)

    start3, end3 = (1, 0, second), end1
    blizzards3 = copy(blizzards2)
    for i in range(second - first): blizzards3 = simulate_blizzards(blizzards3, walls)
    return solve_astar(tiles, walls, blizzards3, start3, end3, second)


with open(argv[1], encoding="utf-8") as input_file:
    tiles, walls, blizzards = parse_map(input_file.read())

print(f"Part 1: {part_1(tiles, walls, blizzards)}")
print(f"Part 2: {part_2(tiles, walls, blizzards)}")
