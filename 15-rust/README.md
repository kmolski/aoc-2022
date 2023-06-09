# [Day 15: Beacon Exclusion Zone](https://adventofcode.com/2022/day/15) solution

For part 2 of this problem, the search area contains a whopping 16 trillion positions.
Storing, or even checking every single one is not (and will never be) a sensible solution.

Thankfully, the problem description contains a hint: the beacon in question was not detected
by any sensor **and** can only have a single position. This implies that all other positions are
covered by sensors, so the distress beacon must lie on the border of 4 sensor detection ranges. 

## Expected result
```
$ cargo run --release data.txt
    Finished release [optimized] target(s) in 0.37s
     Running `target/release/beacon_exclusion_zone data.txt`
Part 1: 5127797
Part 2: 12518502636475
```

## Tested on
```
$ rustc --version
rustc 1.68.0 (2c8cc3432 2023-03-06)
```
