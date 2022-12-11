# [Day 10: Cathode-Ray Tube](https://adventofcode.com/2022/day/10) solution

## Expected result
```
$ make run
java CathodeRayTube.java example.txt
Part 1: 13140
Part 2: 
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....

java CathodeRayTube.java data.txt
Part 1: 12520
Part 2: 
####.#..#.###..####.###....##..##..#....
#....#..#.#..#....#.#..#....#.#..#.#....
###..####.#..#...#..#..#....#.#....#....
#....#..#.###...#...###.....#.#.##.#....
#....#..#.#....#....#....#..#.#..#.#....
####.#..#.#....####.#.....##...###.####.

$ make test
java CathodeRayTube.java example.txt | diff - example.out
java CathodeRayTube.java data.txt | diff - data.out
```

## Tested on
```
$ java --version
openjdk 17.0.5 2022-10-18
OpenJDK Runtime Environment (build 17.0.5+1)
OpenJDK 64-Bit Server VM (build 17.0.5+1, mixed mode)
```
