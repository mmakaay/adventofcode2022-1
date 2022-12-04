## [Day 4: Camp Cleanup](https://adventofcode.com/2022/day/4)

### Part 1.

The puzzle is about sets of ranges, like "1-4, 5-10", "4-5, 3-20".
There are always two ranges in a set. Is one of the ranges in the set
completely contained by the other range in the set.

Solution: as the puzzle input shows that all numbers are < 100, we can
put the ranges into 128-bit integers and use AND to check. This will 
probably not scale for part 2, but we'll see.

Runtime: 233.928µs. After setting `RUSTFLAGS='-C target-cpu=native'`,
220.074µs. It helps, but not a lot.

### Part 2.

Well that was easy. It's the slightly simpler version of part1. Just
checking for any overlap, not if one range is completely contained
by the other.

Runtime: 213.215µs.

### Optimization.

TBD
