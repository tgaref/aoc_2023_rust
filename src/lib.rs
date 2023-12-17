use core::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::iter::{DoubleEndedIterator, ExactSizeIterator, Iterator};
use std::ops::{Index, IndexMut};

pub trait Puzzle {
    fn load_input(&mut self);

    fn part1(&self) -> String;

    fn part2(&self) -> String;

    fn run(&mut self) -> (String, String) {
        self.load_input();
        (self.part1(), self.part2())
    }
}

pub fn print_day(year: usize, day: usize, (part1, part2): (String, String)) {
    println!();
    println!("----------- AoC {:04}, Day {:02} -----------", year, day);
    println!("part 1: {:}", part1);
    println!("part 2: {:}", part2);
}

#[derive(Clone)]
pub struct Grid<T> {
    pub dims: (usize, usize),
    pub array: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Grid {
            array: Vec::new(),
            dims: (0, 0),
        }
    }

    pub fn from_rows(data: Vec<Vec<T>>) -> Self {
        let dims = (data.len(), data[0].len());
        let mut array = Vec::with_capacity(dims.0 * dims.1);
        for mut v in data {
            array.append(&mut v)
        }
        Grid { array, dims }
    }

    pub fn row(&self, i: usize) -> GridRowIter<T> {
        GridRowIter {
            array: &self.array,
            dims: self.dims,
            row: i,
            front: self.dims.1 * i,
            back: Some(self.dims.1 * (i + 1) - 1),
            seen: 0,
        }
    }

    pub fn col(&self, j: usize) -> GridColIter<T> {
        GridColIter {
            array: &self.array,
            dims: self.dims,
            front: j,
            back: Some(self.dims.1 * (self.dims.0 - 1) + j),
            seen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct GridRowIter<'a, T> {
    array: &'a [T],
    dims: (usize, usize),
    row: usize,
    front: usize,
    back: Option<usize>,
    seen: usize,
}

impl<'a, T> Iterator for GridRowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.dims.1 * (self.row + 1) {
            None
        } else {
            let v = &self.array[self.front];
            self.front += 1;
            self.seen += 1;
            Some(v)
        }
    }
}

impl<'a, T> DoubleEndedIterator for GridRowIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(back) = self.back {
            let v = &self.array[back];
            self.back = if back > self.dims.1 * self.row {
                Some(back - 1)
            } else {
                None
            };
            self.seen += 1;
            Some(v)
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for GridRowIter<'a, T> {
    fn len(&self) -> usize {
        self.dims.1 - self.seen
    }
}

#[derive(Copy, Clone)]
pub struct GridColIter<'a, T> {
    array: &'a [T],
    dims: (usize, usize),
    front: usize,
    back: Option<usize>,
    seen: usize,
}

impl<'a, T> Iterator for GridColIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.dims.1 * self.dims.0 {
            None
        } else {
            let v = &self.array[self.front];
            self.front += self.dims.1;
            self.seen += 1;
            Some(v)
        }
    }
}

impl<'a, T> DoubleEndedIterator for GridColIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(back) = self.back {
            let v = &self.array[back];
            self.back = if back >= self.dims.1 {
                Some(back - self.dims.1)
            } else {
                None
            };
            self.seen += 1;
            Some(v)
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for GridColIter<'a, T> {
    fn len(&self) -> usize {
        self.dims.0 - self.seen
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &[T] {
        let start = row * self.dims.1;
        &self.array[start..start + self.dims.1]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, row: usize) -> &mut [T] {
        let start = row * self.dims.1;
        &mut self.array[start..start + self.dims.1]
    }
}

impl<T: Debug + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.dims.0 {
            str.push_str(&self.row(i).map(|c| format!("{}", c)).collect::<String>());
            str.push('\n');
        }
        write!(f, "{}", str)
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.dims.0 {
            str.push_str(&self.row(i).map(|c| format!("{:?} ", c)).collect::<String>());
            str.push('\n');
        }
        write!(f, "{}", str)
    }
}

impl<T: PartialEq> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.dims == other.dims && self.array == other.array
    }
}

impl<T: Eq> Eq for Grid<T> {}

impl<T: Hash> Hash for Grid<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for v in &self.array {
            v.hash(state);
        }
    }
}
