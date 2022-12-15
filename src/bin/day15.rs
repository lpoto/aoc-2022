use std::{collections::HashSet, str::FromStr};

struct Scanner {
    x: isize,
    y: isize,
    beacon: (isize, isize),
    range: isize,
}

impl Scanner {
    fn in_range(&self, (x, y): (isize, isize)) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.range
    }
}

impl FromStr for Scanner {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(",", "").replace(":", "").replace("=", " ");
        let parts: Vec<&str> = s.split_whitespace().collect();
        let ((x, y), (x2, y2)): ((isize, isize), (isize, isize)) = (
            (parts[3].parse().unwrap(), parts[5].parse().unwrap()),
            (parts[11].parse().unwrap(), parts[13].parse().unwrap()),
        );
        Ok(Scanner {
            x,
            y,
            beacon: (x2, y2),
            range: (x2 - x).abs() + (y2 - y).abs(),
        })
    }
}

fn get_line_coverage(scanners: &Vec<Scanner>, y: isize) -> usize {
    let mut line_coverage: HashSet<(isize, isize)> = HashSet::new();
    scanners.iter().for_each(|scanner| {
        for i in [-1, 1] {
            let mut x = scanner.x;
            while scanner.in_range((x, y)) {
                if !(x, y).eq(&(scanner.beacon)) {
                    line_coverage.insert((x, y));
                }
                x = x + i;
            }
        }
    });
    line_coverage.len()
}

fn position_covered(
    (x, y): (isize, isize),
    scanners: &Vec<Scanner>,
    limit: isize,
) -> bool {
    if x < 0 || y < 0 || x > limit || y > limit {
        return false;
    }
    scanners.iter().all(|scanner| !scanner.in_range((x, y)))
}

fn determine_tuning_frequency(scanners: &Vec<Scanner>, limit: isize) -> isize {
    for scanner in scanners {
        let (x, y, d) = (scanner.x, scanner.y, scanner.range + 1);
        for i in 0..d {
            for (a, b) in
                [(i, y - i), (d - i, i), (-i, y + -i), (-d + i, -1)].iter()
            {
                if position_covered((x + a, y + b), &scanners, limit) {
                    return (x + a) * 4_000_000 + (y + b);
                }
            }
        }
    }
    0
}

fn main() {
    let input = include_str!("../../input/day15.txt");

    let scanners = input
        .lines()
        .map(|line| line.parse::<Scanner>().unwrap())
        .collect::<Vec<Scanner>>();

    println!("P1: {:?}", get_line_coverage(&scanners, 2_000_000));
    println!("P1: {:?}", determine_tuning_frequency(&scanners, 4_000_000));
}
