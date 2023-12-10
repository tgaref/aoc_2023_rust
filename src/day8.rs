use num::Integer;
use num_bigint::BigUint;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use aoc_2023_rust::Puzzle;

#[derive(Debug, Clone)]
pub struct Day8 {
    input: Input<'static, Node<'static>>,
}

#[derive(Debug, Clone, Default)]
struct Input<'a, N> {
    directions: &'a str,
    map: Graph<N>,
}

type Node<'a> = &'a str;

#[derive(Debug, Clone, Default)]
struct Graph<N> {
    neighbors: HashMap<N, (N, N)>,
}

impl<'a, N: Eq + Hash + Debug> Input<'a, N> {
    fn traverse<F>(&self, start: N, finished: F) -> usize
    where
        F: Fn(&N) -> bool,
    {
        let mut current = &start;
        for (steps, dir) in self.directions.chars().cycle().enumerate() {
            if finished(current) {
                return steps;
            } else {
                current = if dir == 'L' {
                    &self.map.neighbors[&current].0
                } else {
                    &self.map.neighbors[&current].1
                };
            }
        }
        usize::MAX
    }
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8 {
            input: Input::default(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Input::default()
    }
}

impl Puzzle for Day8 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/8.input");
        let mut lines = INPUT.lines();
        self.input.directions = lines.next().unwrap();
        lines.next();
        for line in lines {
            let (node, pair) = line.split_once(" = ").unwrap();
            let (left, right) = pair[1..(pair.len() - 1)].split_once(", ").unwrap();
            self.input.map.neighbors.insert(node, (left, right));
        }
    }

    fn part1(&self) -> String {
        let start = "AAA";
        let finished = |node: &Node<'static>| (*node == "ZZZ");
        let steps = self.input.traverse(start, finished);
        format!("{:?}", steps)
    }

    fn part2(&self) -> String {
        let finished = |node: &Node<'static>| node.ends_with('Z');
        let steps = self
            .input
            .map
            .neighbors
            .keys()
            .filter(|node| node.ends_with('A'))
            .map(|node| BigUint::from(self.input.traverse(node, finished)))
            .fold(BigUint::from(1usize), |acc, s| acc.lcm(&s));
        format!("{:?}", steps)
    }
}
