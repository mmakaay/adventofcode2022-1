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

total time time to run: 34.175µs.

### Part 2.

- we maintain a 52-byte table 'table'
- process line by line
  - if this is  the first line of a group of three, zero the table.
  - walk over the letters of each line
    - convert each letter into an 0-based index 'p': a-z = 0-25, A-Z = 26-51
    - first line: table[p] |= 1. second line: |= 2. third line: |= 4
    - if this is the third line, find the index of the table where
      table[p] == 7. This is the item that is common in all three rucksacks.
  - add them up
- print total

total time time to run: 25.662µs.


### Optimization.

TBD
