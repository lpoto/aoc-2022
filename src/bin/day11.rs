use std::collections::LinkedList;

#[derive(Clone)]
struct Monkey {
    items: LinkedList<usize>,
    operation: Vec<String>,
    test: (usize, usize, usize),
    inspect_count: usize,
}

impl Monkey {
    fn inspect_item(
        &mut self,
        worry_level_controller: usize,
    ) -> (usize, usize) {
        self.inspect_count += 1;
        let item = self.items.pop_front().unwrap();
        let operation = &self.operation;
        let (a, op, b) = (&operation[0], &operation[1], &operation[2]);
        let a = if a == "old" { item } else { a.parse::<usize>().unwrap() };
        let b = if b == "old" { item } else { b.parse::<usize>().unwrap() };
        let mut item = if op == "*" { a * b } else { a + b };
        if worry_level_controller <= 3 {
            item /= worry_level_controller;
        } else {
            item %= worry_level_controller;
        }
        (item, if item % self.test.0 == 0 { self.test.1 } else { self.test.2 })
    }

    fn parse(input: &mut LinkedList<&str>) -> Monkey {
        input.pop_front();
        Monkey {
            items: input.pop_front().unwrap()[18..]
                .split(", ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
            operation: input.pop_front().unwrap()[19..]
                .split(" ")
                .map(|x| x.to_string())
                .collect(),
            test: (
                input.pop_front().unwrap()[21..].parse::<usize>().unwrap(),
                input.pop_front().unwrap()[29..].parse::<usize>().unwrap(),
                input.pop_front().unwrap()[30..].parse::<usize>().unwrap(),
            ),
            inspect_count: 0,
        }
    }
}

fn monkey_business_level(
    monkeys: &mut Vec<Monkey>,
    rounds: usize,
    worry_level_controller: usize,
) -> usize {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let (item, throw_to) =
                    monkeys[i].inspect_item(worry_level_controller);
                monkeys[throw_to].items.push_back(item);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    monkeys[0].inspect_count * monkeys[1].inspect_count
}

fn main() {
    let input = include_str!("../../input/day11.txt").trim();
    let mut input = input.split('\n').collect::<LinkedList<&str>>();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut cd = 1;
    while input.len() > 0 {
        let monkey = Monkey::parse(&mut input);
        cd *= monkey.test.0;
        monkeys.push(monkey);
        if input.len() > 0 {
            input.pop_front();
        }
    }
    let mut monkeys2 = monkeys.clone();

    println!("P1: {}", monkey_business_level(&mut monkeys, 20, 3));
    println!("P2: {}", monkey_business_level(&mut monkeys2, 1000, cd));
}
