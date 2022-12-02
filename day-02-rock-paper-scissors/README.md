## [Day2: Rock Paper Scissors](https://adventofcode.com/2022/day/2)

### Part 1.

Simply process the input line-by-line. There are 3x3 = 9 possibilities,
fine to use a match statement for each possibility, then add up the outcomes.

```
    let s = input.trim().split("\n").map(|line| match line {
        "A X" => 1 + 3,     // rock rock: draw
        "A Y" => 2 + 6,     // rock paper: win
        "A X" => 1 + 3,     // rock rock: draw
        "A Y" => 2 + 6,     // rock paper: win
        "A Z" => 3 + 0,     // rock scissors: lose
	... etc
    }).sum::<u64>();
```

### Part 2.

Same 9 match statements, different outcome:

```
        "A X" => 1 + 3,     // rock rock: draw
        "A Y" => 2 + 6,     // rock paper: win
        "A Z" => 3 + 0,     // rock scissors: lose
	... etc
```

### Optimization.

Instead of a match statement with 9 clauses (and a default one that panics),
you could simply use a lookup table with 9 elements and index into it,
"A X" = 1, "A Y" = 2, etc. Then you get this:

```
    let table = [ 4, 8, 3, 1, 5, 7, 7, 2, 6 ];
    let s = input.trim().split("\n").map(|l| l.as_bytes()).map(|b|
        table[((b[0] - b'A') * 3 + b[2] - b'X') as usize]).sum::<u64>();
```

.. and it's about twice as fast as the match statement (50 µs vs 24 µs on my laptop).

Final note. Why `input.trim().split("\n")` instead of `input.lines("\n")`?
The former one is 3-5 µs faster (in total), for some reason.

