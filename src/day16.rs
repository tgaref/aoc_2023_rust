use crate::lib::Grid;
use aoc_2023_rust::Puzzle;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Day16 {
    input: Grid<char>,
}

type Point = (usize, usize);
type Dir = (isize, isize);
type State = (Point, Dir);

fn trace(grid: &Grid<char>, (point, dir): State) -> HashSet<State> {
    let mut path: HashSet<State> = HashSet::new();
    let mut debug: Vec<State> = Vec::new();
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back((point, dir));
    while let Some(front) = queue.pop_front() {
        path.insert(front);
        debug.push(front);
        let mut new_fronts = step(grid, front)
            .into_iter()
            .filter(|state| !path.contains(state))
            .collect::<VecDeque<_>>();
        queue.append(&mut new_fronts);
    }
    path
}

fn step(grid: &Grid<char>, state: State) -> VecDeque<State> {
    let point = state.0;
    let dir = state.1;
    let i = point.0 as isize;
    let j = point.1 as isize;
    let pds: Vec<_> = match grid[point.0][point.1] {
        '.' => [((i + dir.0, j + dir.1), dir)].into(),
        '/' => match dir {
            (0, 1) => [((i - 1, j), (-1, 0))].into(),
            (0, -1) => [((i + 1, j), (1, 0))].into(),
            (1, 0) => [((i, j - 1), (0, -1))].into(),
            (-1, 0) => [((i, j + 1), (0, 1))].into(),
            _ => panic!("not possible!"),
        },
        '\\' => match dir {
            (0, 1) => [((i + 1, j), (1, 0))].into(),
            (0, -1) => [((i - 1, j), (-1, 0))].into(),
            (1, 0) => [((i, j + 1), (0, 1))].into(),
            (-1, 0) => [((i, j - 1), (0, -1))].into(),
            _ => panic!("not possible!"),
        },
        '|' => match dir {
            (1, 0) | (-1, 0) => [((i + dir.0, j + dir.1), dir)].into(),
            (0, 1) | (0, -1) => [((i - 1, j), (-1, 0)), ((i + 1, j), (1, 0))].into(),
            _ => panic!("not possible!"),
        },
        '-' => match dir {
            (0, 1) | (0, -1) => [((i + dir.0, j + dir.1), dir)].into(),
            (1, 0) | (-1, 0) => [((i, j - 1), (0, -1)), ((i, j + 1), (0, 1))].into(),
            _ => panic!("not possible!"),
        },
        _ => panic!("not possible"),
    };
    pds.into_iter()
        .filter(|&((i, j), _)| {
            i >= 0isize && i < grid.dims.0 as isize && j >= 0isize && j < grid.dims.1 as isize
        })
        .map(|((i, j), dir)| ((i as usize, j as usize), dir))
        .collect()
}

impl Day16 {
    pub fn new() -> Day16 {
        Day16 { input: Grid::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Grid::new();
    }
}

impl Puzzle for Day16 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/16.input");
        let mut rows = Vec::new();
        for line in INPUT.lines() {
            rows.push(line.chars().collect());
        }
        self.input = Grid::from_rows(rows);
    }

    fn part1(&self) -> String {
        let state = ((0, 0), (0, 1));
        let beam = trace(&self.input, state);
        let result = beam
            .into_iter()
            .map(|(point, _)| point)
            .collect::<HashSet<_>>()
            .len();
        format!("{:?}", result)
    }

    fn part2(&self) -> String {
        let (m, n) = self.input.dims;
        let mut best = 0;

        for j in 0..n {
            for state in &[((0, j), (1, 0)), ((m - 1, j), (-1, 0))] {
                let beam = trace(&self.input, *state);
                let result = beam
                    .into_iter()
                    .map(|(point, _)| point)
                    .collect::<HashSet<_>>()
                    .len();

                best = best.max(result);
            }
        }
        for i in 0..m {
            for state in &[((i, 0), (0, 1)), ((i, n - 1), (0, -1))] {
                let beam = trace(&self.input, *state);
                let result = beam
                    .into_iter()
                    .map(|(point, _)| point)
                    .collect::<HashSet<_>>()
                    .len();

                best = best.max(result);
            }
        }

        format!("{:?}", best)
    }
}
