
// Match driven solution.
pub fn part1(input: &str) {
    let s = input.trim().split("\n").map(|line| match line {
        "A X" => 1 + 3,     // rock rock: draw
        "A Y" => 2 + 6,     // rock paper: win
        "A Z" => 3 + 0,     // rock scissors: lose
        "B X" => 1 + 0,     // paper rock: lose
        "B Y" => 2 + 3,     // paper paper: draw
        "B Z" => 3 + 6,     // paper scissors: win
        "C X" => 1 + 6,     // scissors rock: win
        "C Y" => 2 + 0,     // scissors paper: lose
        "C Z" => 3 + 3,     // scissors scissors: draw
        _ => panic!("unexpected: {}", line),
    }).sum::<u64>();
    println!("part1: score {}", s);
}

// Match driven solution.
pub fn part2(input: &str) {
    let s = input.trim().split("\n").map(|line| match line {
        "A X" => 3 + 0,     // lose: scissors
        "A Y" => 1 + 3,     // draw: rock
        "A Z" => 2 + 6,     // win: paper
        "B X" => 1 + 0,     // lose: rock
        "B Y" => 2 + 3,     // draw: paper
        "B Z" => 3 + 6,     // win: scissors
        "C X" => 2 + 0,     // lose: paper
        "C Y" => 3 + 3,     // darw: scissors
        "C Z" => 1 + 6,     // win: rock
        _ => panic!("unexpected: {}", line),
    }).sum::<u64>();
    println!("part2: score {}", s);
}

// Table driven solution.
pub fn part1a(input: &str) {
    let table = [ 4, 8, 3, 1, 5, 7, 7, 2, 6 ];
    let s = input.trim().split("\n").map(|l| l.as_bytes()).map(|b|
        table[((b[0] - b'A') * 3 + b[2] - b'X') as usize]).sum::<u64>();
    println!("part1: score {}", s);
}

// Table driven solution.
pub fn part2a(input: &str) {
    let table = [ 3, 4, 8, 1, 5, 9, 2, 6, 7 ];
    let s = input.trim().split("\n").map(|l| l.as_bytes()).map(|b|
        table[((b[0] - b'A') * 3 + b[2] - b'X') as usize]).sum::<u64>();
    println!("part2: score {}", s);
}
