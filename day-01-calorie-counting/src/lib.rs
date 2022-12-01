fn calorie_list(input: &str) -> Vec<u64> {
    let mut cals = input.split("\n\n")
        .map(|grp| grp.split("\n").filter_map(|i| i.parse::<u64>().ok()).sum::<u64>())
        .collect::<Vec<_>>();
    cals.sort_unstable();
    cals
}

pub fn part1(input: &str) {
    let cals = calorie_list(input);
    println!("highest: {}", cals.iter().rev().next().unwrap());
}

pub fn part2(input: &str) {
    let cals = calorie_list(input);
    println!("sum of 3 higest: {}", cals.iter().rev().take(3).sum::<u64>());
}
