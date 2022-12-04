
fn parse_range(range: &str) -> u128 {
    let (l, r) = range.split_once('-').unwrap();
    let (l, r) = (l.parse::<u8>().unwrap(), r.parse::<u8>().unwrap());
    (1u128 << (r + 1)) - (1u128 << l)
}

pub fn part1(input: &str) {
    let r = input
        .trim()
        .split("\n")
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            let l = parse_range(l);
            let r = parse_range(r);
            ((l & r) == l || (l & r) == r) as u32
        })
        .sum::<u32>();
    println!("part1: {}", r);
}

pub fn part2(input: &str) {
    let r = input
        .trim()
        .split("\n")
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            let l = parse_range(l);
            let r = parse_range(r);
            ((l & r) > 0) as u32
        })
        .sum::<u32>();
    println!("part2: {}", r);
}

fn next_num(r: &[u8], idx: &mut usize) -> u8 {
    let mut n = 0;
    while *idx < r.len() && r[*idx] >= b'0' && r[*idx] <= b'9' {
        n = n * 10 + r[*idx] - b'0';
        *idx += 1;
    }
    *idx += 1;
    n
}

pub fn part1_2(input: &str) {
    let mut p1 = 0u32;
    let mut p2 = 0u32;
    for line in input.trim().as_bytes().split(|b| *b == b'\n') {
        let mut idx = 0;
        let l1 = next_num(line, &mut idx);
        let r1 = next_num(line, &mut idx);
        let l2 = next_num(line, &mut idx);
        let r2 = next_num(line, &mut idx);
        p1 += ((l1 >= r1 && r1 <= l1) || (r1 >= l1 && l1 <= r1)) as u32;
        p2 += (l2 <= r1 && l1 <= r2) as u32;
    }
    println!("part1: {}", p1);
    println!("part2: {}", p2);
}
