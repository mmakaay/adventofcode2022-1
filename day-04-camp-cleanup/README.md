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

Runtime after optimizations: 59.07µs.

### Part 2.

Well that was easy. It's the slightly simpler version of part1. Just
checking for any overlap, not if one range is completely contained
by the other.

Runtime: 213.215µs.
Runtime after optimizations: 52.811µs.

### Optimizations.

1. We don't have to loop over the ranges on each line, it's always two ranges.
   So just use `split_once(range, ',').unwrap()`.
2. Setting a range of bits in a loop is slow, much faster is
   `(1u128 << (to + 1)) - (1u128 << from)`.
3. Both part1 and part2 are basically doing the same work, so combining
   them into one function should halve the total time, and it does.
   Total runtime for both solutions: 52.36µs.
4. Splitting and parsing strings could probably be made faster. Add a
   simple "parse next number and advance index until next non-numeric char"
   and use that to step over "1-", "2," "3-", "4".
   Total runtime for both solutions now down to: 28.453µs.

Interesting to note, with 1000 lines of input the difference between
`input.lines()` and `input.trim().split("\n")` is about 28µs.
That is a lot!

