# [Day 11: Monkey in the Middle](https://adventofcode.com/2022/day/11) solution

For part 2 of this day, even the u128 type is not enough to store the worry level.
I tried to use BigUints from the `num` crate, but unfortunately, this only revealed
another problem - performance. It is simply not feasible to store and perform calculations
on whole numbers. We can still use some special properties of our data, though.

First of all, all tests check if the worry level is divisible by some number.
This means that we don't actually care about the worry level value, but rather
its remainder for different tests moduli.

But how do we calculate the new worry levels? Well, it turns out that in our data only
addition and multiplication is used in monkeys' formulas. Luckily, those operations are
well-defined under [modular arithmetic](https://en.wikipedia.org/wiki/Modular_arithmetic).
Don't worry about division, as the divisor is 1 for part two.

So, to pull it all together, our abstract worry level will have entries with the test modulus
and their remainders, and addition/multiplication operations on that type will apply modular
arithmetic to all entries. This way, we can use one generic function to compute the solutions for both parts!

## Expected result
```
$ cargo run --release example.txt
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/monkey_in_the_middle example.txt`
Part 1: 10605
Part 2: 2713310158
$ cargo run --release data.txt
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/monkey_in_the_middle data.txt`
Part 1: 58786
Part 2: 14952185856
```

## Tested on
```
$ rustc --version
rustc 1.65.0 (897e37553 2022-11-02)
```
