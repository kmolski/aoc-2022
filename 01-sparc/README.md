# [Day 1: Calorie Counting](https://adventofcode.com/2022/day/1) solution

## Expected result
```
$ make run
qemu-sparc64 a.out example.txt
Part 1: 24000
Part 2: 45000
qemu-sparc64 a.out data.txt
Part 1: 66186
Part 2: 196804
$ make test
qemu-sparc64 a.out example.txt | diff - example.out
qemu-sparc64 a.out data.txt | diff - data.out
```

## Tested on
```
$ sparc64-linux-gnu-gcc-10 --version
sparc64-linux-gnu-gcc-10 (Debian 10.2.1-6) 10.2.1 20210110
Copyright (C) 2020 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
$ qemu-sparc64 --version
qemu-sparc64 version 7.1.0
Copyright (c) 2003-2022 Fabrice Bellard and the QEMU Project developers
```
