use std::collections::{HashMap, LinkedList};

struct Graph {
    grid: Vec<Vec<char>>,
}

impl Graph {
    fn parse(input: &str) -> Graph {
        Graph {
            grid: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn find(&self, c: char) -> (usize, usize) {
        let lines = self.grid.iter().enumerate();
        for (i, line) in lines {
            if let Some(j) = line.iter().position(|&x| x == c) {
                return (i, j);
            }
        }
        (0, 0)
    }

    fn get(&self, (i, j): (usize, usize)) -> (char, char) {
        match self.grid[i][j] {
            | 'E' => ('E', 'z'),
            | 'S' => ('S', 'a'),
            | x => (x, x),
        }
    }

    fn edges(
        &self,
        (i, j): (usize, usize),
        reverse: bool,
    ) -> Vec<(usize, usize)> {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter(|&(i2, j2)| {
                (i as i32) + i2 >= 0
                    && (j as i32) + j2 >= 0
                    && (i as i32) + i2 < self.grid.len() as i32
                    && (j as i32) + j2 < self.grid[0].len() as i32
            })
            .map(|&(i2, j2)| {
                ((i2 + (i as i32)) as usize, (j2 + (j as i32)) as usize)
            })
            .filter(|&(i2, j2)| {
                let ((_, a), (_, b)) = (self.get((i, j)), self.get((i2, j2)));
                (!reverse && a as i32 - b as i32 >= -1)
                    || (reverse && a as i32 - b as i32 <= 1)
            })
            .collect()
    }
}

fn dfs(graph: &Graph, reverse: bool) -> usize {
    let v = if reverse { graph.find('E') } else { graph.find('S') };
    let mut queue: LinkedList<((usize, usize), usize)> = LinkedList::new();
    let mut seen: HashMap<(usize, usize), ()> = HashMap::new();
    seen.insert(v, ());
    queue.push_back((v, 0));
    while !queue.is_empty() {
        let (v, step) = queue.pop_front().unwrap();
        if reverse {
            if let (_, 'a') = graph.get(v) {
                return step;
            }
        } else if let ('E', _) = graph.get(v) {
            return step;
        }
        for e in graph.edges(v, reverse) {
            if !seen.contains_key(&e) {
                seen.insert(e, ());
                queue.push_back((e, step + 1));
            }
        }
    }
    0
}

fn main() {
    let graph: Graph = Graph::parse(include_str!("../../input/day12.txt"));

    println!("P1: {}", dfs(&graph, false));
    println!("P2: {}", dfs(&graph, true));
}
