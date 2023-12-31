use aoc_2023_rust::Puzzle;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Day25 {
    input: Graph<'static>,
}

impl Day25 {
    pub fn new() -> Day25 {
        Day25 {
            input: Graph::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Graph::new();
    }
}

impl Puzzle for Day25 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/25.input");
        for line in INPUT.lines() {
            let (left, right) = line.split_once(": ").unwrap();
            let mut edges = right.split(' ').map(|n| (left, n)).collect::<Vec<_>>();
            self.input.sizes.insert(left, 1);
            for (_, b) in &edges {
                self.input.sizes.insert(b, 1);
            }
            self.input.edges.append(&mut edges);
        }
    }

    fn part1(&self) -> String {
        let g = self.input.clone();
        let (left, right) = g.min_cut();
        format!("{:?}", left * right)
    }

    fn part2(&self) -> String {
        format!("{:?}", "Push the Big Red Button!")
    }
}

type Edge<'a> = (&'a str, &'a str);

#[derive(Debug, Clone)]
struct Graph<'a> {
    edges: Vec<Edge<'a>>,
    sizes: HashMap<&'a str, usize>,
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a> {
        Graph {
            edges: Vec::new(),
            sizes: HashMap::new(),
        }
    }
    fn contract(mut self, e: &Edge<'a>) -> Graph<'a> {
        let edges: Vec<Edge<'a>> = self
            .edges
            .iter()
            .filter_map(|(x, y)| {
                if (*x, *y) == (e.0, e.1) || (*x, *y) == (e.1, e.0) {
                    None
                } else if *x == e.1 {
                    Some((e.0, *y))
                } else if *y == e.1 {
                    Some((*x, e.0))
                } else {
                    Some((*x, *y))
                }
            })
            .collect();
        let size_b = self.sizes[e.1];
        let size_a = self.sizes[e.0];
        self.sizes.remove(e.1);
        self.sizes.insert(e.0, size_a + size_b);
        self.edges = edges;
        self
    }

    fn min_cut(self) -> (usize, usize) {
        let mut min = usize::MAX;
        let mut sizes = (0, 0);
        while min > 3 {
            let mut rng = rand::thread_rng();
            let mut g = self.clone();
            let mut n = g.sizes.len();
            while n > 2 {
                let e = *g.edges.choose(&mut rng).unwrap();
                let gg = g.contract(&e);
                n = gg.sizes.len();
                g = gg;
            }
            let s = g.sizes.iter().map(|(a, n)| n).copied().collect::<Vec<_>>();
            sizes = (s[0], s[1]);
            min = g.edges.len();
        }
        sizes
    }
}
