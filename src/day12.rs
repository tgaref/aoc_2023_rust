use std::collections::HashMap;

use aoc_2023_rust::Puzzle;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Day12 {
    input: Input,
}

#[derive(Debug, Clone)]
struct Input {
    data: Vec<(Vec<char>, Vec<usize>)>,
}

impl Input {
    fn new() -> Self {
        Input { data: Vec::new() }
    }
}

impl Day12 {
    pub fn new() -> Day12 {
        Day12 {
            input: Input::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Input::new();
    }
}

impl Puzzle for Day12 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/12.input");
        for line in INPUT.lines() {
            let (spring_str, spring_nums) = line.split_once(' ').unwrap();
            self.input.data.push((
                spring_str.chars().collect::<Vec<_>>(),
                spring_nums
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect(),
            ));
        }
    }

    fn part1(&self) -> String {
        let mut data = self.input.data.clone();
        for (springs, _) in &mut data {
            springs.push('.');
        }
        let result: usize = data
            .par_iter()
            .map(|(s, c)| {
                let mut memo = HashMap::new();
                calc(s, c, State { amount: 0, goal: 0 }, &mut memo)
            })
            .sum();
        format!("{:?}", result)
    }

    fn part2(&self) -> String {
        let mut tmp = self.input.data.clone();
        for (s, _) in &mut tmp {
            s.push('?');
        }
        let mut data = Vec::new();
        for (springs, counts) in tmp {
            data.push((
                springs
                    .iter()
                    .cycle()
                    .take(springs.len() * 5)
                    .copied()
                    .collect::<Vec<_>>(),
                counts
                    .iter()
                    .cycle()
                    .take(counts.len() * 5)
                    .copied()
                    .collect::<Vec<_>>(),
            ));
        }
        for (s, _) in &mut data {
            s.pop();
            s.push('.');
        }
        let result: usize = data
            .par_iter()
            .map(|(s, c)| {
                let mut memo = HashMap::new();
                calc(s, c, State { amount: 0, goal: 0 }, &mut memo)
            })
            .sum();
        format!("{:?}", result)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    amount: usize,
    goal: usize,
}

fn dot(
    remaining: &[char],
    counts: &[usize],
    state: State,
    memo: &mut HashMap<(State, usize), usize>,
) -> usize {
    if state.amount > 0 {
        if state.amount == counts[state.goal] {
            let new_state = State {
                amount: 0,
                goal: state.goal + 1,
            };
            let key = (new_state, remaining.len());
            if let Some(&a) = memo.get(&key) {
                a
            } else {
                let a = calc(remaining, counts, new_state, memo);
                memo.insert(key, a);
                a
            }
        } else {
            0
        }
    } else {
        let new_state = State {
            amount: 0,
            goal: state.goal,
        };
        let key = (new_state, remaining.len());
        if let Some(&a) = memo.get(&key) {
            a
        } else {
            let a = calc(remaining, counts, new_state, memo);
            memo.insert(key, a);
            a
        }
    }
}

fn kang(
    remaining: &[char],
    counts: &[usize],
    state: State,
    memo: &mut HashMap<(State, usize), usize>,
) -> usize {
    if state.goal == counts.len() {
        return 0;
    }
    if state.amount < counts[state.goal] {
        let new_state = State {
            amount: state.amount + 1,
            goal: state.goal,
        };
        let key = (new_state, remaining.len());
        if let Some(&b) = memo.get(&key) {
            b
        } else {
            let b = calc(remaining, counts, new_state, memo);
            memo.insert(key, b);
            b
        }
    } else {
        0
    }
}

fn calc(
    springs: &[char],
    counts: &[usize],
    state: State,
    memo: &mut HashMap<(State, usize), usize>,
) -> usize {
    let remaining = if springs.len() > 1 {
        &springs[1..]
    } else {
        &[]
    };
    let key = (state, remaining.len());
    if springs.is_empty() {
        if state.goal == counts.len() {
            memo.insert(key, 1);
            return 1;
        } else {
            memo.insert(key, 1);
            return 0;
        }
    };

    match springs[0] {
        '.' => dot(remaining, counts, state, memo),
        '#' => kang(remaining, counts, state, memo),
        '?' => dot(remaining, counts, state, memo) + kang(remaining, counts, state, memo),
        _ => panic!("This should not happen!"),
    }
}
