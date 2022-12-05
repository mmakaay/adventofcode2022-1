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

TBD.
