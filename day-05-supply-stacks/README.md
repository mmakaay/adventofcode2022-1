## [Day 5: Supply Stacks](https://adventofcode.com/2022/day/5)

The assignment for today is to move crates around. The crates each
have an identifier (a letter) and are placed on a stack (1 ..).

```
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
```

Then there is a set of commands to be executed by the cratemover crane:

```
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
```

### Part 1.

Execute commands, moving crates one-by-one. This is simple: `src.pop() -> dst.push()`.

Runs in 368.185µs.

### Part 2.

Ditto, but the crates are moved all at once. Basically the same thing,
but remove multiple crates from the source stack and push them all at once
on the destination stack. So `src.split_at(top - num) -> dst.extend_from_slice()`.

Runs in 376.317µs.

### Optimizations.

These are the timings with the naive implementation:

```
day-05: parsing: 8.341µs
day-05: part1: SPFMVDTZT
day-05: part1: 363.975µs
day-05: part2: ZFSJBPRFP
day-05: part2: 372.807µs
took 759.159µs
```

We do the parsing of the commands twice. Let's parse the commands once and
put them as structs in a Vec, to be processed by part1 and part2.

So, now:

```
day-05: parse stacks: 7.977µs
day-05: parse cmds: 357.277µs
day-05: part1: SPFMVDTZT
day-05: part1: 16.602µs
day-05: part2: ZFSJBPRFP
day-05: part2: 31.962µs
took 429.895µs
```

Ok, the overhead is clearly in `scan_fmt`. Let's replace that.

```
day-05: parse stacks: 7.914µs
day-05: parse cmds: 56.698µs
day-05: part1: SPFMVDTZT
day-05: part1: 16.12µs
day-05: part2: ZFSJBPRFP
day-05: part2: 33.004µs
took 127.869µs
```

Yes, much better. Let's see if replacing `parse::<usize>()` with a handwritten
`atoi` (`atou`, in this case) can shave off a few µs more.

```
day-05: parse stacks: 8.093µs
day-05: parse cmds: 51.445µs
day-05: part1: SPFMVDTZT
day-05: part1: 16.253µs
day-05: part2: ZFSJBPRFP
day-05: part2: 32.449µs
took 121.241µs
```

The difference is insignificant.

Note that part1 is faster than part 2. This is probably because we use
`split_off` which allocates. Let's just take a slice from the top, copy
it, then truncate.

```
day-05: parse stacks: 8.191µs
day-05: parse cmds: 57.149µs
day-05: part1: SPFMVDTZT
day-05: part1: 15.526µs
day-05: part2: ZFSJBPRFP
day-05: part2: 11.598µs
took 105.175µs
```

Now take out the extra timer-println's, and:

```
day-05: supply-stacks
 == start ==
day-05: part1: SPFMVDTZT
day-05: part2: ZFSJBPRFP
took 94.657µs
```

