## [Day 3: Rucksack Reorganization](https://adventofcode.com/2022/day/3)

### Part 1.

- process line by line
  - walk over the letters of each line
    - convert each letter into an 0-based index 'p': a-z = 0-25, A-Z = 26-51
    - we maintain a 52-byte table 'table', initially zero.
    - if the location of the letter is <  half the line size, table[p] |= 1
    - if the location of the letter is >= half the line size, table[p] |= 2
    - the index of the table where table[p] = 3 (plus one) is the misplaced item.
  - add them up
- print total

total time time run: 36.952µs.

### Part 2.


### Optimization.

