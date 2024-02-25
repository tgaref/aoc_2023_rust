use aoc_2023_rust::Puzzle;
use core::fmt;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Day15 {
    input: Vec<Instruction<'static>>,
}

#[derive(Debug, Clone)]
struct Instruction<'a> {
    data: &'a str,
    lens: Lens<'a>,
}

impl<'a> Instruction<'a> {
    fn hash_all(&self) -> usize {
        self.data
            .chars()
            .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256)
    }

    fn hash(&self) -> usize {
        self.lens
            .label
            .chars()
            .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Lens<'a> {
    label: &'a str,
    focal: Option<usize>,
}

impl<'a> Debug for Lens<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let focal = if self.focal.is_some() {
            self.focal.unwrap().to_string()
        } else {
            " ".to_string()
        };
        write!(f, "[{} {}]", self.label, focal)
    }
}

#[derive(Debug, Clone)]
struct Box<'a> {
    queue: Vec<Lens<'a>>,
    labels: HashMap<&'a str, Lens<'a>>,
}

impl<'a> Box<'a> {
    fn remove(&mut self, lens: Lens<'a>) {
        // Remove old lens if it exists
        if self.labels.get(&lens.label).is_some() {
            let mut i = 0;
            while self.queue[i].label != lens.label {
                i += 1;
            }
            self.queue.remove(i);
            self.labels.remove(lens.label);
        }
    }

    fn insert(&mut self, lens: Lens<'a>) {
        if let Some(old_lens) = self.labels.get_mut(&lens.label) {
            let mut i = 0;
            while self.queue[i].label != lens.label {
                i += 1;
            }
            self.queue[i].focal = lens.focal;
            *old_lens = lens;
        } else {
            self.queue.push(lens.clone());
            self.labels.insert(lens.label, lens);
        }
    }

    fn focusing_power(&self) -> usize {
        self.queue
            .iter()
            .enumerate()
            .map(|(i, lens)| (i + 1) * lens.focal.unwrap())
            .sum()
    }
}

impl Day15 {
    pub fn new() -> Day15 {
        Day15 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new();
    }
}

impl Puzzle for Day15 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/15.input");
        for line in INPUT.lines() {
            for s in line.split(',') {
                if let Some(t) = s.strip_suffix('-') {
                    self.input.push(Instruction {
                        data: s,
                        lens: Lens {
                            label: t,
                            focal: None,
                        },
                    });
                } else {
                    let (label, focal) = s.split_once('=').unwrap();
                    self.input.push(Instruction {
                        data: s,
                        lens: Lens {
                            label,
                            focal: Some(focal.parse::<usize>().unwrap()),
                        },
                    });
                }
            }
        }
    }

    fn part1(&self) -> String {
        let result = self.input.iter().map(|w| w.hash_all()).sum::<usize>();

        format!("{:?}", result)
    }

    fn part2(&self) -> String {
        let mut boxes: Vec<Box<'static>> = vec![
            Box {
                queue: Vec::new(),
                labels: HashMap::new()
            };
            256
        ];
        for instruction in &self.input {
            let box_no = instruction.hash();
            if instruction.lens.focal.is_some() {
                boxes[box_no].insert(Lens {
                    label: instruction.lens.label,
                    focal: instruction.lens.focal,
                })
            } else {
                boxes[box_no].remove(Lens {
                    label: instruction.lens.label,
                    focal: None,
                })
            }
        }

        let result: usize = boxes
            .iter()
            .enumerate()
            .map(|(i, b)| (i + 1) * b.focusing_power())
            .sum();
        format!("{:?}", result)
    }
}
