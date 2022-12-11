// https://adventofcode.com/2022/day/5

fn build_initial_stack(stack_draving: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    stack_draving.split('\n').for_each(|l| {
        for i in (1..l.len()).step_by(4) {
            match l.chars().nth(i) {
                | Some(c) if c.is_ascii_alphabetic() => {
                    let idx = (i - 1) / 4;
                    for _ in (stacks.len())..(idx + 1) {
                        stacks.push(Vec::new());
                    }
                    stacks[idx].insert(0, c);
                }
                | _ => (),
            }
        }
    });
    stacks
}

fn main() {
    let input = include_str!("../../input/day05.txt").trim_end();

    let split = input.split("\n\n").collect::<Vec<&str>>();
    let mut stacks = build_initial_stack(split[0]);
    let mut stacks2 = stacks.clone();
    split[1].split('\n').for_each(|l| {
        let split = l.split(' ').collect::<Vec<&str>>();
        let count = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;
        let cur_len = stacks2[to].len();
        for _ in 0..count {
            let c1 = stacks[from].pop().unwrap();
            let c2 = stacks2[from].pop().unwrap();
            stacks[to].push(c1);
            stacks2[to].insert(cur_len, c2);
        }
    });

    print!("P1: ");
    stacks.iter().for_each(|s| print!("{}", (s).last().unwrap()));
    print!("\nP2: ");
    stacks2.iter().for_each(|s| print!("{}", (s).last().unwrap()));
    println!();
}
