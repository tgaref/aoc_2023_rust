use aoc_2023_rust::Puzzle;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Day22 {
    input: Vec<Brick>,
}

impl Day22 {
    pub fn new() -> Day22 {
        Day22 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new();
    }
}

impl Puzzle for Day22 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/22.input");
        let mut bricks: Vec<Brick> = Vec::new();
        for line in INPUT.lines() {
            let (start, end) = line.split_once('~').unwrap();
            let start_coord = start
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let end_coord = end
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let x_range = Range {
                left: start_coord[0].min(end_coord[0]),
                right: start_coord[0].max(end_coord[0]),
            };
            let y_range = Range {
                left: start_coord[1].min(end_coord[1]),
                right: start_coord[1].max(end_coord[1]),
            };
            let z_range = Range {
                left: start_coord[2].min(end_coord[2]),
                right: start_coord[2].max(end_coord[2]),
            };
            bricks.push(Brick {
                x: x_range,
                y: y_range,
                z: z_range,
            });
        }
        bricks.sort_by(|a, b| a.z.left.cmp(&b.z.left));
        for brick in &bricks {
            self.input.push(brick.fall(&self.input));
        }
    }

    fn part1(&self) -> String {
        let mut nonsafe: HashSet<Brick> = HashSet::new();
        for brick in &self.input {
            let sup = brick.supported_by(&self.input);
            if sup.len() == 1 {
                nonsafe.insert(sup[0]);
            }
        }
        format!("{:?}", self.input.len() - nonsafe.len())
    }

    fn part2(&self) -> String {
        let mut supported: HashMap<Brick, HashSet<Brick>> = HashMap::new();
        let mut supports: HashMap<Brick, Vec<Brick>> = HashMap::new();
        for brick in &self.input {
            supported.insert(
                *brick,
                brick
                    .supported_by(&self.input)
                    .into_iter()
                    .collect::<HashSet<_>>(),
            );

            supports.insert(*brick, brick.supports(&self.input));
        }

        let result = self
            .input
            .iter()
            .map(|brick| {
                let mut sup = supported.clone();
                cascade(*brick, &mut sup, &supports).len()
            })
            .sum::<usize>();

        format!("{:?}", result)
    }
}

fn cascade(
    brick: Brick,
    support: &mut HashMap<Brick, HashSet<Brick>>,
    supports: &HashMap<Brick, Vec<Brick>>,
) -> HashSet<Brick> {
    let mut fallen: HashSet<Brick> = HashSet::new();
    let mut toremove: VecDeque<Brick> = VecDeque::new();
    toremove.push_back(brick);
    while let Some(br) = toremove.pop_front() {
        if let Some(supported) = supports.get(&br) {
            for b in supported {
                if let Some(bs) = support.get_mut(b) {
                    if bs.remove(&br) && bs.is_empty() {
                        fallen.insert(*b);
                        toremove.push_back(*b);
                    }
                }
            }
        }
    }
    fallen
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Range {
    left: usize,
    right: usize,
}

impl Range {
    fn intersects(self, other: &Range) -> bool {
        let left = self.left.max(other.left);
        let right = self.right.min(other.right);
        left <= right
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Brick {
    x: Range,
    y: Range,
    z: Range,
}

impl Brick {
    fn obstructed_by(&self, other: &Self) -> bool {
        self.x.intersects(&other.x) && self.y.intersects(&other.y) && other.z.right < self.z.left
    }

    fn find_obstruction(&self, bricks: &Vec<Brick>) -> Brick {
        let mut obstruction = Brick {
            z: Range { left: 0, right: 0 },
            ..*self
        };
        for b in bricks {
            if self.obstructed_by(b) && b.z.right > obstruction.z.right {
                obstruction = *b;
            }
        }
        obstruction
    }

    fn fall(&self, bricks: &Vec<Brick>) -> Brick {
        let obsrtuction = self.find_obstruction(bricks);
        let z_range = Range {
            left: obsrtuction.z.right + 1,
            right: obsrtuction.z.right + 1 + self.z.right - self.z.left,
        };
        Brick {
            z: z_range,
            ..*self
        }
    }

    fn supports(&self, bricks: &Vec<Brick>) -> Vec<Brick> {
        let mut supported: Vec<Brick> = Vec::new();
        for brick in bricks {
            if brick.obstructed_by(self) && brick.z.left == self.z.right + 1 {
                supported.push(*brick);
            }
        }
        supported
    }

    fn supported_by(&self, bricks: &Vec<Brick>) -> Vec<Brick> {
        let mut supported_by: Vec<Brick> = Vec::new();
        for brick in bricks {
            if self.obstructed_by(brick) && self.z.left == brick.z.right + 1 {
                supported_by.push(*brick);
            }
        }
        supported_by
    }
}
