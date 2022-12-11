// https://adventofcode.com/2022/day/2

fn main() {
    let input = include_str!("../../input/day02.txt").trim();

    let score = input.split("\n").fold((0, 0), |sum, l| {
        let elf = l.chars().nth(0).unwrap() as u32 - ('A' as u32);
        let me = l.chars().nth(2).unwrap() as u32 - ('X' as u32);
        (
            sum.0 + (me + 1) + ((me + 4 - elf) % 3) * 3,
            sum.1 + (me * 3) + (((elf + me + 2) % 3) + 1),
        )
    });

    println!("P1: {}", score.0);
    println!("P2: {}", score.1);
}
