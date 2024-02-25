use aoc_2023_rust::Puzzle;
use num::bigint::BigInt;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Day21 {
    input: Input,
}

impl Day21 {
    pub fn new() -> Day21 {
        Day21 {
            input: Input::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Input::new();
    }
}

impl Puzzle for Day21 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/21.input");
        let m = INPUT.lines().count();
        let n = INPUT.lines().next().unwrap().chars().count();
        self.input.dims = (m, n);
        for (i, line) in INPUT.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    self.input.stones.insert((i, j));
                } else if c == 'S' {
                    self.input.start = (i, j);
                }
            }
        }
        let dist = dijkstra(&self.input, self.input.start);
        for (pos, &d) in &dist {
            if d == usize::MAX && !self.input.stones.contains(pos) {
                self.input.stones.insert(*pos);
            }
        }
    }

    fn part1(&self) -> String {
        let steps = 64;
        let parity = steps % 2;
        let dist = dijkstra(&self.input, self.input.start);
        let count = dist
            .iter()
            .filter(|(_position, &d)| d % 2 == parity && d <= steps)
            .count();

        format!("{:?}", count)
    }

    fn part2(&self) -> String {
        // I will solve it in the special case that m=n is odd
        let n = self.input.dims.0;
        let steps = 26501365;
        // let steps = 1000;
        let parity = steps % 2;
        let mut count = BigInt::from(0);
        let (x, y) = self.input.start;

        count += count_cardinal(&self.input, (n - 1, y), x + 1, steps); // north
        count += count_cardinal(&self.input, (0, y), n - x, steps); // south
        count += count_cardinal(&self.input, (x, 0), n - y, steps); // east
        count += count_cardinal(&self.input, (x, n - 1), y + 1, steps); // west

        count += count_diagonal(&self.input, (n - 1, 0), x + 1 + n - y, steps); // north-east
        count += count_diagonal(&self.input, (n - 1, n - 1), x + 1 + y + 1, steps); // north-west
        count += count_diagonal(&self.input, (0, 0), n - x + n - y, steps); // south-east
        count += count_diagonal(&self.input, (0, n - 1), n - x + y + 1, steps); // south-west

        count += BigInt::from(
            dijkstra(&self.input, self.input.start)
                .iter()
                .filter(|(_position, &d)| d % 2 == parity && d < usize::MAX)
                .count(),
        );

        format!("{:?}", count)
    }
}

fn count_cardinal(data: &Input, crossing: Position, d0: usize, steps: usize) -> BigInt {
    let parity = steps % 2;
    let n = data.dims.0;
    let dist = dijkstra(data, crossing);
    let mut count = BigInt::from(0);
    for i in 0..n {
        for j in 0..n {
            let target = (i, j);
            if data.stones.contains(&target) {
                continue;
            }
            let d = dist[&target];
            let reserve = d0 + d;
            let nblocks = (steps - reserve) / n + 1;
            count += nblocks / 2
                + if nblocks % 2 == 1 && reserve % 2 == parity {
                    1
                } else {
                    0
                };
        }
    }
    count
}

fn count_diagonal(data: &Input, crossing: Position, d0: usize, steps: usize) -> BigInt {
    let parity = steps % 2;
    let n = data.dims.0;
    let dist = dijkstra(data, crossing);
    let mut count = BigInt::from(0);
    for i in 0..n {
        for j in 0..n {
            let target = (i, j);
            if data.stones.contains(&target) {
                continue;
            }
            let d = dist[&target];
            let reserve = d0 + d;
            let nblocks = (steps - reserve) / n + 1;
            count += if nblocks % 2 == 0 && reserve % 2 == parity {
                (nblocks / 2) * (nblocks / 2)
            } else if nblocks % 2 == 0 && reserve % 2 != parity {
                (nblocks / 2) * (nblocks / 2 + 1)
            } else if nblocks % 2 == 1 && reserve % 2 == parity {
                (nblocks / 2 + 1) * (nblocks / 2 + 1)
            } else {
                (nblocks / 2) * (nblocks / 2 + 1)
            };
        }
    }
    count
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Position,
    dist: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on cost.
        other.dist.cmp(&self.dist)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(data: &Input, start: Position) -> HashMap<Position, usize> {
    let n = data.dims.0;
    let mut dist: HashMap<Position, usize> = HashMap::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            dist.insert((i, j), usize::MAX);
        }
    }

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);

    heap.push(State {
        pos: start,
        dist: 0,
    });

    while let Some(state) = heap.pop() {
        // Important as we may have already found a better way
        if state.dist > dist[&state.pos] {
            continue;
        }
        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for next in neighbours(data, state) {
            // If so, add it to the frontier and continue
            if next.dist < dist[&next.pos] {
                dist.insert(next.pos, next.dist);
                heap.push(next);
            }
        }
    }
    dist
}

fn neighbours(data: &Input, state: State) -> Vec<State> {
    let mut nei: Vec<State> = Vec::new();
    for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let p = (state.pos.0 as isize + i, state.pos.1 as isize + j);
        if p.0 >= 0
            && p.0 < data.dims.0 as isize
            && p.1 >= 0
            && p.1 < data.dims.1 as isize
            && !data.stones.contains(&(p.0 as usize, p.1 as usize))
        {
            let new_state = State {
                pos: (p.0 as usize, p.1 as usize),
                dist: state.dist + 1,
            };
            nei.push(new_state);
        }
    }
    nei
}

type Stones = HashSet<Position>;
type Position = (usize, usize);

#[derive(Debug, Clone)]
struct Input {
    stones: Stones,
    dims: (usize, usize),
    start: Position,
}

impl Input {
    fn new() -> Self {
        Input {
            stones: HashSet::new(),
            dims: (0, 0),
            start: (0, 0),
        }
    }
}
