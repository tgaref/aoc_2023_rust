use aoc_2023_rust::Puzzle;

#[derive(Debug, Clone)]
pub struct Day6 {
    input1: Vec<(i128, i128)>,
    input2: (i128, i128),
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 {
            input1: Vec::new(),
            input2: (0, 0),
        }
    }

    pub fn _clear(&mut self) {
        self.input1 = Vec::new();
        self.input2 = (0, 0);
    }
}

impl Puzzle for Day6 {
    fn load_input(&mut self) {
        self.input1 = vec![(56, 546), (97, 1927), (78, 1131), (75, 1139)];
        self.input2 = (56977875, 546192711311139);
    }

    fn part1(&self) -> String {
        let n: i128 = self
            .input1
            .iter()
            .map(|(total_time, record)| compute_interval_len(*total_time, *record))
            .product();
        format!("{:?}", n)
    }

    fn part2(&self) -> String {
        let (total_time, record) = &self.input2;
        let n = compute_interval_len(*total_time, *record);
        format!("{:?}", n)
    }
}

fn compute_interval_len(total_time: i128, record: i128) -> i128 {
    let delta = total_time.pow(2) - 4 * record;
    let left = (total_time as f64 - (delta as f64).sqrt()) / 2.0;
    let right = (total_time as f64 + (delta as f64).sqrt()) / 2.0;
    let l = if left.ceil() > left {
        left.ceil() as i128
    } else {
        left.ceil() as i128 + 1
    };
    let r = if right.floor() < right {
        right.floor() as i128
    } else {
        right.floor() as i128 - 1
    };
    r - l + 1
}
