use aoc_2023_rust::Puzzle;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Day23 {
    input: Input,
}

impl Day23 {
    pub fn new() -> Day23 {
        Day23 {
            input: Input {
                map: HashMap::new(),
                dim: 0,
            },
        }
    }

    pub fn _clear(&mut self) {
        self.input = Input {
            map: HashMap::new(),
            dim: 0,
        };
    }
}

impl Puzzle for Day23 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/23.input");
        for (row, line) in INPUT.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                self.input.map.insert((row as isize, col as isize), c);
            }
        }
        self.input.dim = INPUT.lines().count() as isize;
    }

    fn part1(&self) -> String {
        let d = max_path(
            &self.input,
            (0, 1),
            (self.input.dim - 1, self.input.dim - 2),
        );
        format!("{:?}", d)
    }

    fn part2(&self) -> String {
        let graph = compute_graph(&self.input, (0, 1));
        let d = bfs(&graph, (0, 1), (self.input.dim - 1, self.input.dim - 2));
        format!("{:?}", d)
    }
}

#[derive(Debug, Clone)]
struct Input {
    map: Map,
    dim: isize,
}

type Position = (isize, isize);
type Map = HashMap<Position, char>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos: Position,
    visited: HashSet<Position>,
    dist: usize,
}

fn step(input: &Input, position: Position, prev: Position) -> Vec<Position> {
    let mut nei: Vec<Position> = Vec::new();
    for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let pos = (position.0 + i, position.1 + j);
        if pos.0 >= 0
            && pos.0 < input.dim
            && pos.1 >= 0
            && pos.1 < input.dim
            && input.map[&pos] != '#'
            && pos != prev
        {
            nei.push(pos);
        }
    }
    nei
}

struct Graph {
    adj: HashMap<Position, HashSet<Position>>,
    weight: HashMap<(Position, Position), usize>,
}

fn straight_path(
    data: &Input,
    position: Position,
    mut prev: Position,
) -> (Position, usize, Vec<Position>) {
    let mut d = 0;
    let mut pos = position;
    let mut nei = step(data, pos, prev);
    while nei.len() == 1 {
        d += 1;
        prev = pos;
        pos = nei[0];
        nei = step(data, pos, prev);
    }
    (pos, d, nei)
}

fn compute_graph(data: &Input, start: Position) -> Graph {
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(start);
    let mut queue: VecDeque<(Position, Position)> = VecDeque::new();
    let mut graph: Graph = Graph {
        adj: HashMap::new(),
        weight: HashMap::new(),
    };
    let (new_position, d, nei) = straight_path(data, start, start);
    graph.weight.insert((start, new_position), d);
    graph.weight.insert((new_position, start), d);
    if let Some(v) = graph.adj.get_mut(&start) {
        v.insert(new_position);
    } else {
        let mut v: HashSet<Position> = HashSet::new();
        v.insert(new_position);
        graph.adj.insert(start, v);
    }
    if let Some(v) = graph.adj.get_mut(&new_position) {
        v.insert(start);
    } else {
        let mut v: HashSet<Position> = HashSet::new();
        v.insert(start);
        graph.adj.insert(new_position, v);
    }
    for p in nei {
        queue.push_back((new_position, p));
    }
    visited.insert(new_position);
    while let Some((position, next)) = queue.pop_front() {
        let (new_position, d, nei) = straight_path(data, next, position);
        graph.weight.insert((position, new_position), d + 1);
        graph.weight.insert((new_position, position), d + 1);

        if let Some(v) = graph.adj.get_mut(&position) {
            v.insert(new_position);
        } else {
            let mut v: HashSet<Position> = HashSet::new();
            v.insert(new_position);
            graph.adj.insert(position, v);
        }
        if let Some(v) = graph.adj.get_mut(&new_position) {
            v.insert(position);
        } else {
            let mut v: HashSet<Position> = HashSet::new();
            v.insert(position);
            graph.adj.insert(new_position, v);
        }
        if !visited.contains(&new_position) {
            for p in nei {
                queue.push_back((new_position, p));
            }
        }
        visited.insert(new_position);
    }
    graph
}

fn bfs(graph: &Graph, start: Position, goal: Position) -> usize {
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut visited: HashSet<_> = HashSet::new();
    visited.insert(start);
    queue.push_back(State {
        pos: start,
        visited,
        dist: 0,
    });

    let mut dist: usize = 0;

    while let Some(state) = queue.pop_front() {
        if state.pos == goal {
            dist = dist.max(state.dist);
        }
        if let Some(nei) = graph.adj.get(&state.pos) {
            for next in nei {
                if !state.visited.contains(next) {
                    let mut visited = state.visited.clone();
                    visited.insert(*next);
                    queue.push_back(State {
                        pos: *next,
                        visited,
                        dist: state.dist + graph.weight[&(state.pos, *next)],
                    });
                }
            }
        }
    }
    dist
}

fn max_path(data: &Input, start: Position, goal: Position) -> usize {
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut visited: HashSet<_> = HashSet::new();
    visited.insert(start);
    queue.push_back(State {
        pos: start,
        visited,
        dist: 0,
    });

    let mut dist: usize = 0;

    while let Some(state) = queue.pop_front() {
        if state.pos == goal {
            dist = dist.max(state.visited.len() - 1);
        }
        for next in neighbours(data, &state) {
            queue.push_back(next);
        }
    }
    dist
}

fn neighbours(input: &Input, state: &State) -> Vec<State> {
    let mut nei: Vec<State> = Vec::new();
    for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let pos = (state.pos.0 + i, state.pos.1 + j);
        if pos.0 >= 0
            && pos.0 < input.dim
            && pos.1 >= 0
            && pos.1 < input.dim
            && !state.visited.contains(&pos)
        {
            let c = input.map[&pos];
            if c == '.' || (is_slope(c) && along_slope((i, j), c)) {
                let mut visited = state.visited.clone();
                visited.insert(pos);
                nei.push(State {
                    pos,
                    visited,
                    dist: state.dist + 1,
                });
            }
        }
    }
    nei
}

fn is_slope(c: char) -> bool {
    c == '^' || c == 'v' || c == '<' || c == '>'
}

fn along_slope(dir: (isize, isize), c: char) -> bool {
    (dir == (-1, 0) && c == '^')
        || (dir == (1, 0) && c == 'v')
        || (dir == (0, -1) && c == '<')
        || (dir == (0, 1) && c == '>')
}
