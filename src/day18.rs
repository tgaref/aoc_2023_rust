use aoc_2023_rust::Puzzle;

#[derive(Debug, Clone)]
pub struct Day18 {
    input: (Vec<Instruction>, Vec<Instruction>),
}

impl Day18 {
    pub fn new() -> Day18 {
        Day18 {
            input: (Vec::new(), Vec::new()),
        }
    }

    pub fn _clear(&mut self) {
        self.input = (Vec::new(), Vec::new());
    }
}

type Position = (isize, isize);
type Direction = (isize, isize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    dir: Direction,
    steps: isize,
}

fn step(pos: Position, instr: &Instruction) -> Position {
    let (i, j) = instr.dir;
    (pos.0 + i * instr.steps, pos.1 + j * instr.steps)
}

fn polygon_area(corners: &Vec<Position>) -> isize {
    let mut a = 0;
    let n = corners.len();
    for i in 0..n - 1 {
        a += (corners[i].1 + corners[i + 1].1) * (corners[i].0 - corners[i + 1].0);
    }
    a += (corners[n - 1].1 + corners[1].1) * (corners[n - 1].0 - corners[1].0);
    a / 2
}

impl Puzzle for Day18 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/18.input");
        for line in INPUT.lines() {
            let v: Vec<_> = line.split(' ').collect();
            let steps = v[1].parse::<isize>().unwrap();
            match v[0] {
                "R" => self.input.0.push(Instruction { dir: (0, 1), steps }),
                "L" => self.input.0.push(Instruction {
                    dir: (0, -1),
                    steps,
                }),
                "D" => self.input.0.push(Instruction { dir: (1, 0), steps }),
                "U" => self.input.0.push(Instruction {
                    dir: (-1, 0),
                    steps,
                }),
                _ => panic!("not possible!"),
            }
            let color = v[2].chars().skip(2).take(6).collect::<String>();
            let steps =
                isize::from_str_radix(&color.chars().take(5).collect::<String>(), 16).unwrap();
            let digit = color.chars().last().unwrap();
            let dir = match digit {
                '0' => (0, 1),
                '1' => (1, 0),
                '2' => (0, -1),
                '3' => (-1, 0),
                _ => panic!("Unrecognized direction!"),
            };
            self.input.1.push(Instruction { dir, steps });
        }
    }

    fn part1(&self) -> String {
        let mut pos = (0, 0);
        let mut corners: Vec<Position> = vec![(0, 0)];
        for instr in &self.input.0 {
            let new_pos = step(pos, instr);
            corners.push(new_pos);
            pos = new_pos;
        }
        let perimeter = self.input.0.iter().map(|instr| instr.steps).sum::<isize>();
        let area = polygon_area(&corners).abs();
        let interior = area - perimeter / 2 + 1;

        format!("{:?}", interior + perimeter)
    }

    fn part2(&self) -> String {
        let mut pos = (0, 0);
        let mut corners: Vec<Position> = vec![(0, 0)];
        for instr in &self.input.1 {
            let new_pos = step(pos, instr);
            corners.push(new_pos);
            pos = new_pos;
        }
        let perimeter = self.input.1.iter().map(|instr| instr.steps).sum::<isize>();
        let area = polygon_area(&corners).abs();
        let interior = area - perimeter / 2 + 1;

        format!("{:?}", interior + perimeter)
    }
}
