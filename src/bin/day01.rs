// https://adventofcode.com/2022/day/1

fn main() {
    let input = include_str!("../../input/day01.txt").trim();

    let mut elven_calories: Vec<i32> = input
        .split("\n\n")
        .map(|l| l.split("\n").map(|n| n.parse::<i32>().unwrap()).sum())
        .collect();
    elven_calories.sort_by(|a, b| b.cmp(a));

    println!("P1: {}", elven_calories[0]);
    println!("P2: {}", elven_calories[0..3].to_vec().iter().sum::<i32>());
}
