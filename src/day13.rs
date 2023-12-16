use std::collections::HashSet;

use crate::lib::Grid;
use aoc_2023_rust::Puzzle;

#[derive(Debug, Clone)]
pub struct Day13 {
    input: Vec<Pattern>,
}

impl Day13 {
    pub fn new() -> Day13 {
        Day13 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new();
    }
}

#[derive(Debug, Clone)]
struct Pattern {
    grid: Grid<char>,
}

impl Pattern {
    fn horizontal(&self) -> usize {
        let m = self.grid.dims.0;
        let mut patterns: Vec<usize> = Vec::new();
        for i in 0..m {
            let pat = &self
                .grid
                .row(i)
                .fold(0, |acc, &c| if c == '.' { acc * 2 } else { acc * 2 + 1 });
            patterns.push(*pat);
        }
        find_symmetry(&patterns)
    }

    fn horizontals(&self) -> HashSet<usize> {
        let m = self.grid.dims.0;
        let mut patterns: Vec<usize> = Vec::new();
        for i in 0..m {
            let pat = &self
                .grid
                .row(i)
                .fold(0, |acc, &c| if c == '.' { acc * 2 } else { acc * 2 + 1 });
            patterns.push(*pat);
        }
        find_symmetries(&patterns)
    }

    fn vertical(&self) -> usize {
        let n = self.grid.dims.1;
        let mut patterns: Vec<usize> = Vec::new();
        for j in 0..n {
            let pat = &self
                .grid
                .col(j)
                .fold(0, |acc, &c| if c == '.' { acc * 2 } else { acc * 2 + 1 });
            patterns.push(*pat);
        }
        find_symmetry(&patterns)
    }
    fn verticals(&self) -> HashSet<usize> {
        let n = self.grid.dims.1;
        let mut patterns: Vec<usize> = Vec::new();
        for j in 0..n {
            let pat = &self
                .grid
                .col(j)
                .fold(0, |acc, &c| if c == '.' { acc * 2 } else { acc * 2 + 1 });
            patterns.push(*pat);
        }
        find_symmetries(&patterns)
    }
}

fn find_symmetry<T>(v: &[T]) -> usize
where
    T: Eq,
{
    let m = v.len();
    for i in 0..(m - 1) {
        if v[i] == v[i + 1] && (1..i.min(m - i - 2) + 1).all(|j| v[i - j] == v[i + 1 + j]) {
            return i + 1;
        }
    }
    0
}

fn find_symmetries<T>(v: &[T]) -> HashSet<usize>
where
    T: Eq,
{
    let m = v.len();
    let mut symmetries = HashSet::new();
    for i in 0..(m - 1) {
        if v[i] == v[i + 1] && (1..i.min(m - i - 2) + 1).all(|j| v[i - j] == v[i + 1 + j]) {
            symmetries.insert(i + 1);
        }
    }
    symmetries
}

impl Puzzle for Day13 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/13.input");
        let mut rows = Vec::new();
        for line in INPUT.lines() {
            if line.is_empty() {
                self.input.push(Pattern {
                    grid: Grid::from_rows(rows),
                });
                rows = Vec::new();
            } else {
                rows.push(line.chars().collect())
            }
        }
        self.input.push(Pattern {
            grid: Grid::from_rows(rows),
        });
    }

    fn part1(&self) -> String {
        let result: usize = self
            .input
            .iter()
            .map(|pattern| pattern.horizontal() * 100 + pattern.vertical())
            .sum();

        format!("{:?}", result)
    }

    fn part2(&self) -> String {
        let mut input = self.input.clone();
        let previous = self
            .input
            .iter()
            .map(|pattern| (pattern.horizontal(), pattern.vertical()));

        let result: usize = input
            .iter_mut()
            .zip(previous)
            .map(|(pattern, (h, v))| {
                let (m, n) = pattern.grid.dims;
                for i in 0..m {
                    for j in 0..n {
                        flip(pattern, i, j);
                        let new_h = pattern.horizontals();
                        let new_v = pattern.verticals();
                        if !new_h.is_empty() {
                            for hh in new_h {
                                if hh != h {
                                    return hh * 100;
                                }
                            }
                        }
                        if !new_v.is_empty() {
                            for vv in new_v {
                                if vv != v {
                                    return vv;
                                }
                            }
                        }
                        flip(pattern, i, j);
                    }
                }
                0
            })
            .sum();

        println!("{}", self.input[0].grid);
        format!("{:?}", result)
    }
}

fn flip(pat: &mut Pattern, i: usize, j: usize) {
    if pat.grid[i][j] == '.' {
        pat.grid[i][j] = '#';
    } else {
        pat.grid[i][j] = '.';
    };
}
