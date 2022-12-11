// https://adventofcode.com/2022/day/6
use std::{collections::HashSet, collections::LinkedList, iter::FromIterator};

fn find_start_marker_index(input: &str, n: usize) -> usize {
    let mut v: LinkedList<char> = LinkedList::new();
    for (i, c) in input.chars().enumerate() {
        v.push_back(c);
        if i < n - 1 {
            continue;
        }
        let s: HashSet<&char> = HashSet::from_iter(v.iter());
        if s.len() == n {
            return i + 1;
        }
        v.pop_front();
    }
    0
}

fn main() {
    let input = include_str!("../../input/day06.txt").trim();

    println!("P1: {}", find_start_marker_index(&input, 4));
    println!("P2: {}", find_start_marker_index(&input, 14));
}
