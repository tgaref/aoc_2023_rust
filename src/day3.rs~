use aoc_2023_rust::Puzzle;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Day3 {
    input: (Numbers, Symbols, Gears),
}

type Numbers = HashMap<(isize, isize, isize), isize>;
type Symbols = HashSet<(isize, isize)>;
type Gears = HashSet<(isize, isize)>;

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            input: (HashMap::new(), HashSet::new(), HashSet::new()),
        }
    }

    pub fn _clear(&mut self) {
        self.input = (HashMap::new(), HashSet::new(), HashSet::new());
    }
}

impl Puzzle for Day3 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/3.input");
        let mut numbers: Numbers = HashMap::new();
        let mut symbols: Symbols = HashSet::new();
        let mut gears: Gears = HashSet::new();
        for (i, line) in INPUT.lines().enumerate() {
            let mut n: isize = 0;
            let mut start = 0;
            let mut reading_number = false;
            for (j, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    reading_number = true;
                    n = n * 10 + c.to_digit(10).unwrap() as isize;
                } else {
                    if reading_number {
                        numbers.insert((i as isize, start as isize, j as isize - 1), n);
                        reading_number = false;
                        n = 0
                    }
                    start = j + 1;
                    if c != '.' {
                        symbols.insert((i as isize, j as isize));
                    }
                    if c == '*' {
                        gears.insert((i as isize, j as isize));
                    }
                }
            }
            if reading_number {
                numbers.insert((i as isize, start as isize, line.len() as isize - 1), n);
            }
        }
        self.input = (numbers, symbols, gears);
    }

    fn part1(&self) -> String {
        let mut suma = 0;
        for ((line, begin, end), n) in &self.input.0 {
            if check_for_symbol(*line, *begin, *end, &self.input.1) {
                suma += n;
            }
        }
        format!("{:?}", suma)
    }

    fn part2(&self) -> String {
        let mut suma = 0;
        for (a, b) in &self.input.2 {
            suma += check_for_numbers(*a, *b, &self.input.0)
        }
        format!("{:?}", suma)
    }
}

fn check_for_numbers(a: isize, b: isize, numbers: &HashMap<(isize, isize, isize), isize>) -> isize {
    let mut adjacent = HashSet::new();
    for ((line, begin, end), n) in numbers {
        if (*line == a - 1 || *line == a || *line == a + 1) && *begin <= b + 1 && *end >= b - 1 {
            adjacent.insert((*line, *begin, *end, *n));
        }
    }
    if adjacent.len() == 2 {
        adjacent.iter().map(|(_, _, _, n)| *n).product()
    } else {
        0
    }
}

fn check_for_symbol(line: isize, a: isize, b: isize, symbols: &HashSet<(isize, isize)>) -> bool {
    // Check lines above and below
    for j in (a - 1)..(b + 2) {
        if symbols.contains(&(line - 1, j)) || symbols.contains(&(line + 1, j)) {
            return true;
        }
    }
    // Check same line
    if symbols.contains(&(line, a - 1)) || symbols.contains(&(line, b + 1)) {
        return true;
    }
    false
}
