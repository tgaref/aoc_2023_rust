use crate::lib::Grid;
use aoc_2023_rust::Puzzle;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Day14 {
    input: Platform,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {
    grid: Grid<char>,
}

impl Platform {
    fn new() -> Platform {
        Platform { grid: Grid::new() }
    }

    fn north(&mut self) {
        let (m, n) = self.grid.dims;
        for j in 0..n {
            for i in 1..m {
                if self.grid[i][j] == 'O' {
                    roll_north(&mut self.grid, i, j);
                }
            }
        }
    }

    fn south(&mut self) {
        let (m, n) = self.grid.dims;
        for j in 0..n {
            for i in (0..m - 1).rev() {
                if self.grid[i][j] == 'O' {
                    roll_south(&mut self.grid, i, j);
                }
            }
        }
    }

    fn west(&mut self) {
        let (m, n) = self.grid.dims;
        for i in 0..m {
            for j in 1..n {
                if self.grid[i][j] == 'O' {
                    roll_west(&mut self.grid, i, j);
                }
            }
        }
    }

    fn east(&mut self) {
        let (m, n) = self.grid.dims;
        for i in 0..m {
            for j in (0..n - 1).rev() {
                if self.grid[i][j] == 'O' {
                    roll_east(&mut self.grid, i, j);
                }
            }
        }
    }

    fn cycle(&mut self, n: usize) {
        for _ in 0..n {
            self.north();
            self.west();
            self.south();
            self.east();
        }
    }

    fn compute_load(&self) -> usize {
        let m = self.grid.dims.0;
        let mut suma: usize = 0;
        for i in 0..m {
            suma += self
                .grid
                .row(i)
                .map(|&c| if c == 'O' { m - i } else { 0 })
                .sum::<usize>();
        }
        suma
    }
}

fn roll_north(grid: &mut Grid<char>, i: usize, j: usize) {
    let mut k = i;
    while k > 0 && grid[k - 1][j] == '.' {
        k -= 1;
    }
    if k != i {
        grid[k][j] = 'O';
        grid[i][j] = '.';
    }
}

fn roll_south(grid: &mut Grid<char>, i: usize, j: usize) {
    let mut k = i;
    let m = grid.dims.0;
    while k < m - 1 && grid[k + 1][j] == '.' {
        k += 1;
    }
    if k != i {
        grid[k][j] = 'O';
        grid[i][j] = '.';
    }
}

fn roll_west(grid: &mut Grid<char>, i: usize, j: usize) {
    let mut k = j;
    while k > 0 && grid[i][k - 1] == '.' {
        k -= 1;
    }
    if k != j {
        grid[i][k] = 'O';
        grid[i][j] = '.';
    }
}

fn roll_east(grid: &mut Grid<char>, i: usize, j: usize) {
    let mut k = j;
    let n = grid.dims.1;
    while k < n - 1 && grid[i][k + 1] == '.' {
        k += 1;
    }
    if k != j {
        grid[i][k] = 'O';
        grid[i][j] = '.';
    }
}

impl Day14 {
    pub fn new() -> Day14 {
        Day14 {
            input: Platform::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Platform::new();
    }
}

impl Puzzle for Day14 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/14.input");
        let rows: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
        self.input = Platform {
            grid: Grid::from_rows(rows),
        };
    }

    fn part1(&self) -> String {
        let mut platform = self.input.clone();
        platform.north();
        format!("{:?}", platform.compute_load())
    }

    fn part2(&self) -> String {
        let times = 1000000000;
        let mut platform = self.input.clone();
        let mut seen: HashMap<Platform, usize> = HashMap::new();
        let mut cycle = 0;
        while seen.get(&platform).is_none() {
            seen.insert(platform.clone(), cycle);
            cycle += 1;
            platform.cycle(1);
        }
        let n = seen.get(&platform).unwrap();
        let period = cycle - n;

        let r = times % period;
        let index = if r >= *n { r - *n } else { period + r - *n };
        platform.cycle(index);

        format!("{:?}", platform.compute_load())
    }
}
