use aoc_2023_rust::Puzzle;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Day1 {
    input: Vec<&'static str>,
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

impl Puzzle for Day1 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/1.input");
        for line in INPUT.lines() {
            self.input.push(line);
        }
    }

    fn part1(&self) -> String {
        let mapp: &[(&str, usize)] = &[
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ];

        let mut mapping: HashMap<&str, usize> = HashMap::new();
        for (a, b) in mapp {
            mapping.insert(a, *b);
        }
        let n = self
            .input
            .iter()
            .fold(0, |acc, line| acc + get_calibration(&mapping, line));
        format!("{:}", n)
    }

    fn part2(&self) -> String {
        let mapp: &[(&str, usize)] = &[
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        let mut mapping: HashMap<&str, usize> = HashMap::new();
        for (a, b) in mapp {
            mapping.insert(a, *b);
        }
        let n = self
            .input
            .iter()
            .fold(0, |acc, line| acc + get_calibration(&mapping, line));
        format!("{:}", n)
    }
}

fn get_calibration(m: &HashMap<&str, usize>, s: &str) -> usize {
    let mut first = (s.len(), "");
    let mut last = (0, "");
    for p in m.keys() {
        let v: Vec<(usize, &str)> = s.match_indices(p).collect();
        if !v.is_empty() {
            let (pos1, _) = v[0];
            let (pos2, _) = v[v.len() - 1];
            if pos1 <= first.0 {
                first = (pos1, p)
            }
            if pos2 >= last.0 {
                last = (pos2, p)
            }
        }
    }
    m[first.1] * 10 + m[last.1]
}
