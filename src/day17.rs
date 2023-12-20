use aoc_2023_rust::{Grid, Puzzle};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone)]
pub struct Day17 {
    input: City,
}

type City = Grid<usize>;
type Position = (isize, isize);
type Direction = (isize, isize);

impl Day17 {
    pub fn new() -> Day17 {
        Day17 { input: Grid::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Grid::new();
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: Position,
    dir: Direction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    pos: Position,
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on cost.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn next_dir(dir: Direction) -> Direction {
    if dir.0 != 0 {
        (0, 1)
    } else {
        (1, 0)
    }
}

fn neighbours(grid: &City, state: State, at_least: isize, at_most: isize) -> Vec<State> {
    let mut neigh = Vec::new();
    let m = grid.dims.0 as isize;
    let n = grid.dims.1 as isize;
    let dir = next_dir(state.dir);
    for step in (-at_most..=-at_least).chain(at_least..=at_most) {
        let p = (state.pos.0 + step * dir.0, state.pos.1 + step * dir.1);
        if p.0 >= 0 && p.0 < m && p.1 >= 0 && p.1 < n {
            neigh.push(State {
                cost: state.cost + cost(grid, state.pos, p),
                pos: p,
                dir,
            });
        }
    }
    neigh
}

fn cost(grid: &City, from: Position, to: Position) -> usize {
    let mut cost = 0;
    if from.0 == to.0 {
        let a = from.1.min(to.1) as usize;
        let b = from.1.max(to.1) as usize;

        for k in a..=b {
            cost += grid[from.0 as usize][k]
        }
        cost -= grid[from.0 as usize][from.1 as usize];
    } else {
        let a = from.0.min(to.0) as usize;
        let b = from.0.max(to.0) as usize;

        for k in a..=b {
            cost += grid[k][from.1 as usize]
        }
        cost -= grid[from.0 as usize][from.1 as usize];
    }
    cost
}

fn dijkstra(
    grid: &City,
    start: Position,
    goal: Position,
    at_least: isize,
    at_most: isize,
) -> Option<State> {
    let (m, n) = grid.dims;
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<Node, usize> = HashMap::with_capacity(2 * m * n);
    for i in 0isize..(m as isize) {
        for j in 0isize..(n as isize) {
            dist.insert(
                Node {
                    pos: (i, j),
                    dir: (1, 0),
                },
                usize::MAX,
            );
            dist.insert(
                Node {
                    pos: (i, j),
                    dir: (0, 1),
                },
                usize::MAX,
            );
        }
    }

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(
        Node {
            pos: start,
            dir: (1, 0),
        },
        0,
    );
    dist.insert(
        Node {
            pos: start,
            dir: (0, 1),
        },
        0,
    );

    heap.push(State {
        cost: 0,
        pos: start,
        dir: (0, 1),
    });

    heap.push(State {
        cost: 0,
        pos: start,
        dir: (1, 0),
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(state) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if state.pos == goal {
            return Some(state);
        }
        // Important as we may have already found a better way
        if state.cost
            > dist[&Node {
                pos: state.pos,
                dir: state.dir,
            }]
        {
            continue;
        }
        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for next in neighbours(grid, state, at_least, at_most) {
            // If so, add it to the frontier and continue
            let node = Node {
                pos: next.pos,
                dir: next.dir,
            };
            if next.cost < dist[&node] {
                dist.insert(node, next.cost);
                heap.push(next);
            }
        }
    }
    // Goal not reachable
    None
}

impl Puzzle for Day17 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/17.input");
        let mut rows: Vec<Vec<usize>> = Vec::new();
        for line in INPUT.lines() {
            rows.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect(),
            );
        }
        self.input = Grid::from_rows(rows);
    }

    fn part1(&self) -> String {
        let m = self.input.dims.0 as isize;
        let n = self.input.dims.1 as isize;
        let cst = dijkstra(&self.input, (0, 0), (m - 1, n - 1), 1, 3);
        format!("{:?}", cst.unwrap().cost)
    }

    fn part2(&self) -> String {
        let m = self.input.dims.0 as isize;
        let n = self.input.dims.1 as isize;
        let cst = dijkstra(&self.input, (0, 0), (m - 1, n - 1), 4, 10);
        format!("{:?}", cst.unwrap().cost)
    }
}
