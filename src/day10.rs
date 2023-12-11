use crate::lib::Grid;
use aoc_2023_rust::Puzzle;

#[derive(Debug, Clone)]
pub struct Day10 {
    input: Map,
}

type Map = Grid<Pipe>;

impl Day10 {
    pub fn new() -> Day10 {
        Day10 { input: Grid::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Grid::new()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Pipe {
    NorthSouth,
    EastWest,
    SouthWest,
    SouthEast,
    NorthWest,
    NorthEast,
    Empty,
    Start,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum CommingFrom {
    North(usize, usize),
    South(usize, usize),
    East(usize, usize),
    West(usize, usize),
}

use CommingFrom::{East, North, South, West};

impl CommingFrom {
    fn next(self, pipe: Pipe) -> Self {
        match pipe {
            Pipe::NorthSouth => match self {
                North(i, j) => North(i + 1, j),
                South(i, j) => South(i - 1, j),
                _ => panic!("Deadend!"),
            },
            Pipe::EastWest => match self {
                East(i, j) => East(i, j - 1),
                West(i, j) => West(i, j + 1),
                _ => panic!("Deadend!"),
            },
            Pipe::NorthEast => match self {
                North(i, j) => West(i, j + 1),
                East(i, j) => South(i - 1, j),
                _ => panic!("Deadend!"),
            },
            Pipe::NorthWest => match self {
                North(i, j) => East(i, j - 1),
                West(i, j) => South(i - 1, j),
                _ => panic!("Deadend!"),
            },
            Pipe::SouthEast => match self {
                South(i, j) => West(i, j + 1),
                East(i, j) => North(i + 1, j),
                _ => panic!("Deadend! {:?}", self),
            },
            Pipe::SouthWest => match self {
                South(i, j) => East(i, j - 1),
                West(i, j) => North(i + 1, j),
                _ => panic!("Deadend!"),
            },
            Pipe::Empty => panic!("Deadend!"),
            Pipe::Start => panic!("Back to start!"),
        }
    }

    fn get_position(self) -> (usize, usize) {
        match self {
            North(i, j) => (i, j),
            South(i, j) => (i, j),
            East(i, j) => (i, j),
            West(i, j) => (i, j),
        }
    }
}

impl Puzzle for Day10 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/10.input");
        let rows = INPUT
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '|' => Pipe::NorthSouth,
                        '-' => Pipe::EastWest,
                        '7' => Pipe::SouthWest,
                        'F' => Pipe::SouthEast,
                        'J' => Pipe::NorthWest,
                        'L' => Pipe::NorthEast,
                        '.' => Pipe::Empty,
                        'S' => Pipe::Start,
                        s => panic!("Unknown symbol {:?}", s),
                    })
                    .collect()
            })
            .collect();
        self.input = Grid::from_rows(rows);
    }

    fn part1(&self) -> String {
        let start = find_start(&self.input);
        let grid = &self.input;
        let mut pipe = grid[start.0][start.1 + 1];
        let mut pos = West(start.0, start.1 + 1);
        let mut path = vec![pos];
        while pipe != Pipe::Start {
            pos = pos.next(pipe);
            let (i, j) = pos.get_position();
            pipe = grid[i][j];
            path.push(pos);
        }
        format!("{:?}", path.len() / 2)
    }

    // For this I am using Pick's formula:
    // Interior points = Area - (Points on the border) / 2 + 1.
    // To compute the area I am using the trapezoid formula.
    // The triangle formula or the shoelace formula could be used instead.
    fn part2(&self) -> String {
        let start = find_start(&self.input);
        let grid = &self.input;
        let mut pipe = grid[start.0][start.1 + 1];
        let mut pos = West(start.0, start.1 + 1);
        let mut path = vec![pos];
        while pipe != Pipe::Start {
            pos = pos.next(pipe);
            let (i, j) = pos.get_position();
            pipe = grid[i][j];
            path.push(pos);
        }
        let mut area = 0isize;
        for i in 0..path.len() - 1 {
            let (y, x) = path[i].get_position();
            let (yy, xx) = path[i + 1].get_position();
            area += (y as isize + yy as isize) * (x as isize - xx as isize);
        }
        let (y, x) = path[path.len() - 1].get_position();
        let (yy, xx) = path[0].get_position();
        area += (y as isize + yy as isize) * (x as isize - xx as isize);
        area /= 2;
        format!("{:?}", area - (path.len() / 2) as isize + 1isize)
    }
}

fn find_start(grid: &Map) -> (usize, usize) {
    let (m, n) = grid.dims;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == Pipe::Start {
                return (i, j);
            }
        }
    }
    (0, 0)
}
