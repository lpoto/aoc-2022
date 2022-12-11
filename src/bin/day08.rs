// https://adventofcode.com/2022/day/8
fn viewing_distance(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    direction: (i32, i32),
) -> (usize, bool) {
    let mut dist = 0;
    let mut cur = (start.0 as i32 + direction.0, start.1 as i32 + direction.1);
    while cur.0 >= 0
        && cur.1 >= 0
        && cur.0 < (grid.len() as i32)
        && cur.1 < grid[cur.0 as usize].len() as i32
    {
        dist += 1;
        if grid[cur.0 as usize][cur.1 as usize] >= grid[start.0][start.1] {
            return (dist, false);
        }
        cur = (cur.0 + direction.0, cur.1 + direction.1);
    }
    (dist, true)
}

fn main() {
    let input = include_str!("../../input/day08.txt").trim();

    let grid: Vec<Vec<char>> =
        input.split('\n').map(|line| line.chars().collect()).collect();
    let (visible_trees_count, max_scenic_score) =
        grid.iter().enumerate().fold((0, 1), |acc, (i, row)| {
            let r = row.iter().enumerate().fold((0, 1), |acc2, (j, _)| {
                let r2 = vec![(1, 0), (0, 1), (-1, 0), (0, -1)].iter().fold(
                    (0, 1),
                    |acc3, direction| {
                        let x = viewing_distance(&grid, (i, j), *direction);
                        ((acc3.0 == 1 || (x.1)) as u32, acc3.1 * x.0)
                    },
                );
                (acc2.0 + r2.0, if r2.1 > acc2.1 { r2.1 } else { acc2.1 })
            });
            (acc.0 + r.0, if r.1 > acc.1 { r.1 } else { acc.1 })
        });

    println!("P1: {}", visible_trees_count);
    println!("P2: {}", max_scenic_score);
}
