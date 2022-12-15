use std::{collections::HashMap, str::FromStr};

#[derive(Clone)]
struct Cave {
    max_depth: i32,
    blocked: HashMap<(i32, i32), ()>,
    sand_poured: u32,
}

impl Cave {
    fn get_next_sand_position(
        &self,
        (x, y): (i32, i32),
        floor_exists: bool,
    ) -> (i32, i32) {
        if floor_exists && y == self.max_depth + 1 {
            return (x, y);
        }
        if !self.blocked.contains_key(&(x, y + 1)) {
            return (x, y + 1);
        }
        if !self.blocked.contains_key(&(x - 1, y + 1)) {
            return (x - 1, y + 1);
        }
        if !self.blocked.contains_key(&(x + 1, y + 1)) {
            return (x + 1, y + 1);
        }
        return (x, y);
    }

    fn pour_sand(&mut self, floor_exists: bool) -> u32 {
        loop {
            let mut sand_position = (500, 0);
            loop {
                let next_sand_position =
                    self.get_next_sand_position(sand_position, floor_exists);
                if floor_exists && next_sand_position.1 <= 0 {
                    return self.sand_poured + 1;
                } else if !floor_exists && next_sand_position.1 > self.max_depth
                {
                    return self.sand_poured;
                }
                if next_sand_position.eq(&sand_position) {
                    self.sand_poured += 1;
                    self.blocked.insert(sand_position, ());
                    break;
                }
                sand_position = next_sand_position;
            }
        }
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut blocked, mut max_depth) = (HashMap::new(), 0);
        s.lines().for_each(|line| {
            let split = line
                .split(" -> ")
                .map(|s| {
                    let split: Vec<&str> = s.split(",").collect();
                    (
                        split[0].parse::<i32>().unwrap(),
                        split[1].parse::<i32>().unwrap(),
                    )
                })
                .collect::<Vec<(i32, i32)>>();
            for i in 0..(split.len() - 1) {
                let (mut x1, mut y1) = split[i];
                let (x2, y2) = split[i + 1];
                blocked.insert((x1, y1), ());
                if y1 > max_depth {
                    max_depth = y1;
                }
                while x1 != x2 || y1 != y2 {
                    x1 = if x1 < x2 { x1 + 1 } else { x1 - (x1 > x2) as i32 };
                    y1 = if y1 < y2 { y1 + 1 } else { y1 - (y1 > y2) as i32 };
                    blocked.insert((x1, y1), ());
                    if y1 > max_depth {
                        max_depth = y1;
                    }
                }
            }
        });
        Ok(Cave {
            max_depth,
            blocked,
            sand_poured: 0,
        })
    }
}

fn main() {
    let input = include_str!("../../input/day14.txt");

    let mut cave = input.parse::<Cave>().unwrap();
    let mut cave2 = cave.clone();

    println!("P1: {:?}", cave.pour_sand(false));
    println!("P2: {:?}", cave2.pour_sand(true));
}
