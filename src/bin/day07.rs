// https://adventofcode.com/2022/day/7
use std::collections::LinkedList;

fn get_dir_sizes(
    terminal_output: &mut LinkedList<&str>,
    dir_sizes: &mut LinkedList<i32>,
) -> i32 {
    let mut size = 0;
    loop {
        match terminal_output.pop_front() {
            | None => break,
            | Some("$ cd ..") => break,
            | Some(s) => {
                if s.starts_with("$ cd") {
                    size += get_dir_sizes(terminal_output, dir_sizes);
                } else if !s.starts_with("dir") && !s.starts_with("$ ls") {
                    size += s.split(" ").collect::<Vec<&str>>()[0]
                        .parse::<i32>()
                        .unwrap();
                }
            }
        };
    }
    dir_sizes.push_back(size);
    size
}

fn main() {
    let input = include_str!("../../input/day07.txt").trim();

    let mut terminal_output: LinkedList<&str> = input.split('\n').collect();
    let mut dir_sizes: LinkedList<i32> = LinkedList::new();
    let outmost_size = get_dir_sizes(&mut terminal_output, &mut dir_sizes);
    let limited_dir_sizes_sum = dir_sizes
        .iter()
        .fold(0, |sum, e| return if *e > 100_000 { sum } else { sum + e });
    let smallest_dir_to_delete = dir_sizes
        .iter()
        .filter(|e| 70_000_000 - outmost_size + *e >= 30_000_000)
        .min()
        .unwrap();

    println!("P1: {}", limited_dir_sizes_sum);
    println!("P2: {}", smallest_dir_to_delete);
}
