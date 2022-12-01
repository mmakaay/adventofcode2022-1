fn calorie_list() -> Vec<u64> {
    let lines = include_str!("../input/input.txt");
    let mut cals = lines.split("\n\n")
        .map(|grp| grp.trim().split("\n").map(|i| i.parse::<u64>().unwrap()).sum::<u64>())
        .collect::<Vec<_>>();
    cals.sort();
    cals.reverse();
    cals
}

fn part1() {
    println!("highest: {}", calorie_list()[0]);
}

fn part2() {
    let cals = calorie_list();
    println!("sum of 3 higest: {}", cals[0] + cals[1] + cals[2]);
}

fn main() {
    part1();
    part2();
}
