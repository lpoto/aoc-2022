// https://adventofcode.com/2022/day/3
use std::collections::HashSet;

fn priority(a: char) -> i32 {
    if a >= 'a' {
        a as i32 - 'a' as i32 + 1
    } else {
        a as i32 - 'A' as i32 + 27
    }
}

fn main() {
    let input = include_str!("../../input/day03.txt").trim();

    let mut potential_badges: HashSet<_> = HashSet::new();
    let priority_sum = input.split("\n").enumerate().fold(
        (0, 0),
        |priority_sum, (i, line)| {
            potential_badges = if i % 3 == 0 {
                line.chars().collect::<HashSet<char>>()
            } else {
                potential_badges
                    .intersection(&line.chars().collect::<HashSet<char>>())
                    .map(|c| *c)
                    .collect::<HashSet<char>>()
            };
            let l = line.len() / 2;
            (
                priority_sum.0
                    + line[l..]
                        .chars()
                        .collect::<HashSet<char>>()
                        .intersection(
                            &line[0..l].chars().collect::<HashSet<char>>(),
                        )
                        .fold(0, |sum, c| sum + priority(*c)),
                if i > 0 && (i + 1) % 3 == 0 {
                    priority_sum.1
                        + priority(*potential_badges.iter().next().unwrap())
                } else {
                    priority_sum.1
                },
            )
        },
    );

    println!("P1: {}", priority_sum.0);
    println!("P2: {}", priority_sum.1);
}
