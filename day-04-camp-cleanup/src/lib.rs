
fn parse_range(range: &str, set: &mut u128) {
    let (l, r) = range.split_once('-').unwrap();
    let (l, r) = (l.parse::<u8>().unwrap(), r.parse::<u8>().unwrap());
    for i in l ..=r {
        *set |= 1 << i;
    }
}

pub fn part1(input: &str) {
    let r = input
        .trim()
        .split("\n")
        .map(|line| {
            let mut ranges = [0u128; 2];
            line
                .split(",")
                .enumerate()
                .for_each(|(i, r)| parse_range(r, &mut ranges[i]));
            let and = ranges[0] & ranges[1];
            (ranges[0] == and || ranges[1] == and) as u32
        })
        .sum::<u32>();
    println!("part1: {}", r);
}


pub fn part2(_input: &str) {
}
