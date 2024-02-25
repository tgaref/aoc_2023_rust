use aoc_2023_rust::Puzzle;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Day11 {
    input: Input,
}

type Image = HashSet<(isize, isize)>;

#[derive(Debug, Clone)]
struct Input {
    image: Image,
    dims: (usize, usize),
}

impl Input {
    fn new() -> Self {
        Input {
            image: HashSet::new(),
            dims: (0, 0),
        }
    }

    fn expand(&self, scale: isize) -> Input {
        let mut occupied_rows = HashSet::new();
        let mut occupied_cols = HashSet::new();
        for (i, j) in &self.image {
            occupied_rows.insert(*i);
            occupied_cols.insert(*j);
        }
        let all_rows: HashSet<isize> = (0isize..self.dims.0 as isize).collect();
        let empty_rows: HashSet<isize> = all_rows.difference(&occupied_rows).copied().collect();
        let all_cols: HashSet<isize> = (0isize..self.dims.1 as isize).collect();
        let empty_cols: HashSet<isize> = all_cols.difference(&occupied_cols).copied().collect();

        let mut image_new_rows = HashSet::new();
        for (a, b) in &self.image {
            let shift_down = empty_rows.iter().filter(|&i| i < a).count() as isize;
            image_new_rows.insert((a + (scale - 1) * shift_down, b));
        }
        let mut image_new_cols = HashSet::new();
        for (a, b) in image_new_rows {
            let shift_right = empty_cols.iter().filter(|&j| j < b).count() as isize;
            image_new_cols.insert((a, b + (scale - 1) * shift_right));
        }
        Input {
            image: image_new_cols,
            dims: (
                self.dims.0 + empty_rows.len(),
                self.dims.1 + empty_cols.len(),
            ),
        }
    }
}

impl Day11 {
    pub fn new() -> Day11 {
        Day11 {
            input: Input::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Input::new()
    }
}

impl Puzzle for Day11 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/11.input");
        let mut row_count = 0;
        let mut col_count = 0;
        for (i, line) in INPUT.lines().enumerate() {
            row_count += 1;
            let mut current_col_count = 0;
            for (j, c) in line.chars().enumerate() {
                current_col_count += 1;
                if c == '#' {
                    self.input.image.insert((i as isize, j as isize));
                }
            }
            col_count = col_count.max(current_col_count);
        }
        self.input.dims = (row_count, col_count);
    }

    fn part1(&self) -> String {
        let expanded_image = self.input.expand(2).image;
        let mut suma = 0;
        for galaxy1 in &expanded_image {
            for galaxy2 in &expanded_image {
                suma += hamming_dist(*galaxy1, *galaxy2);
            }
        }
        format!("{:?}", suma / 2)
    }

    fn part2(&self) -> String {
        let expanded_image = self.input.expand(1000000).image;
        let mut suma = 0;
        for galaxy1 in &expanded_image {
            for galaxy2 in &expanded_image {
                suma += hamming_dist(*galaxy1, *galaxy2);
            }
        }
        format!("{:?}", suma / 2)
    }
}

fn hamming_dist((a, b): (isize, isize), (c, d): (isize, isize)) -> isize {
    (c - a).abs() + (d - b).abs()
}
