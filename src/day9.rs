use aoc_2023_rust::Puzzle;

#[derive(Debug, Clone)]
pub struct Day9 {
    input: Input,
}

#[derive(Debug, Clone, Default)]
struct Input {
    histories: Vec<History>,
    predictions: Vec<(isize, isize)>,
}

#[derive(Debug, Clone)]
struct History {
    seq: Vec<isize>,
}

type Summary = Vec<(isize, isize)>;

impl Day9 {
    pub fn new() -> Day9 {
        Day9 {
            input: Input::default(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Input::default()
    }
}

impl History {
    fn diffs(self) -> Self {
        History {
            seq: self
                .seq
                .iter()
                .skip(1)
                .zip(self.seq.iter())
                .map(|(b, a)| b - a)
                .collect(),
        }
    }

    fn create_summary(self) -> Summary {
        let mut seq = self.seq;
        let mut summary = vec![(seq[0], seq[seq.len() - 1])];
        while !seq.iter().all(|&n| n == 0) {
            let new_seq = History { seq }.diffs().seq;
            summary.push((new_seq[0], new_seq[new_seq.len() - 1]));
            seq = new_seq;
        }
        summary
    }

    fn predict(summary: &Summary) -> (isize, isize) {
        let mut current_first = 0;
        let mut current_last = 0;
        for (first, last) in summary.iter().rev().skip(1) {
            current_first = first - current_first;
            current_last += last;
        }
        (current_first, current_last)
    }
}

impl Puzzle for Day9 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/9.input");
        for line in INPUT.lines() {
            let seq: Vec<isize> = line
                .split(' ')
                .map(|s| s.parse::<isize>().unwrap())
                .collect();
            self.input.histories.push(History { seq });
        }
        self.input.predictions = self
            .input
            .histories
            .iter()
            .map(|history| History::predict(&history.clone().create_summary()))
            .collect();
    }

    fn part1(&self) -> String {
        let result: isize = self.input.predictions.iter().map(|(_, x)| x).sum();

        format!("{:?}", result)
    }

    fn part2(&self) -> String {
        let result: isize = self.input.predictions.iter().map(|(x, _)| x).sum();

        format!("{:?}", result)
    }
}
