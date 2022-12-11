// https://adventofcode.com/2022/day/4
use std::cmp::{max, min};

fn main() {
    let input = include_str!("../../input/day04.txt").trim();

    let overlaps = input.split('\n').fold((0, 0), |sum, line| {
        let l = line
            .split(',')
            .map(|pair| {
                pair.split("-")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        (
            sum.0
                + (l[0][0] <= l[1][0] && l[0][1] >= l[1][1]
                    || l[0][0] >= l[1][0] && l[0][1] <= l[1][1])
                    as i32,
            sum.1 + (max(l[0][0], l[1][0]) <= min(l[0][1], l[1][1])) as i32,
        )
    });

    println!("P1: {}", overlaps.0);
    println!("P2: {}", overlaps.1);
}
