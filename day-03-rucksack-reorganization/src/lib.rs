
fn index(b: u8) -> usize {
    // a-z = 0-25, A-Z = 26-51.
    let r = (b as usize & 31) + 26 * ((b & 32) == 0) as usize - 1;
    assert!(r < 52);
    r
}

pub fn part1(input: &str) {
    let r = input
        .trim()
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(|line| {
            let mut table = [0u8; 52];
            let middle = line.len() / 2;
            for i in 0 .. line.len() {
                let p = index(line[i]);
                table[p] |= 1 + (i >= middle) as u8;
            }
            (0..52).find_map(|i| (table[i] == 3).then(|| i)).unwrap() + 1
        })
        .sum::<usize>();
    println!("part1: {}", r);
}

pub fn part2(_input: &str) {
}
