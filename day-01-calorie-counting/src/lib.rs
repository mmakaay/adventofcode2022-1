fn calorie_list(input: &str) -> Vec<u64> {
    let mut cals = input.split("\n\n")
        .map(|grp| grp.trim().split("\n").map(|i| i.parse::<u64>().unwrap()).sum::<u64>())
        .collect::<Vec<_>>();
    cals.sort();
    cals.reverse();
    cals
}

pub fn part1(input: &str) {
    println!("highest: {}", calorie_list(input)[0]);
}

pub fn part2(input: &str) {
    let cals = calorie_list(input);
    println!("sum of 3 higest: {}", cals[0] + cals[1] + cals[2]);
}
