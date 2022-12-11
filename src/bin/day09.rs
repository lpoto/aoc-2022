// https://adventofcode.com/2022/day/9
use std::collections::HashSet;

fn distance_between_knots(k1: (i32, i32), k2: (i32, i32)) -> i32 {
    (k1.0 - k2.0).abs().max((k1.1 - k2.1).abs())
}

fn move_knot(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    (
        if h.0 == t.0 { t.0 } else { t.0 + (h.0 - t.0) / (h.0 - t.0).abs() },
        if h.1 == t.1 { t.1 } else { t.1 + (h.1 - t.1) / (h.1 - t.1).abs() },
    )
}

fn move_head_knot(h: (i32, i32), dir: char, n: i32) -> (i32, i32) {
    match dir {
        | 'D' => (h.0 - n, h.1),
        | 'U' => (h.0 + n, h.1),
        | 'R' => (h.0, h.1 + n),
        | _ => (h.0, h.1 - n),
    }
}

fn main() {
    let input = include_str!("../../input/day09.txt").trim();

    let mut second_knot_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut last_knot_positions: HashSet<(i32, i32)> = HashSet::new();

    input.split('\n').fold([(0, 0); 10], |mut knots, line| {
        let c = line.chars().nth(0).unwrap();
        for _ in 0..line[2..].parse::<i32>().unwrap() {
            knots[0] = move_head_knot(knots[0], c, 1);
            for i in 1..knots.len() {
                if distance_between_knots(knots[i - 1], knots[i]) > 1 {
                    knots[i] = move_knot(knots[i - 1], knots[i]);
                }
            }
            second_knot_positions.insert(knots[1]);
            last_knot_positions.insert(knots[9]);
        }
        knots
    });

    println!("P1: {}", second_knot_positions.len());
    println!("P2: {}", last_knot_positions.len());
}
