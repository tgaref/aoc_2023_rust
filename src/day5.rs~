use aoc_2023_rust::Puzzle;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Day5 {
    input: Input,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Range {
    a: usize,
    b: usize,
}

impl Range {
    fn contains(self, value: usize) -> bool {
        self.a <= value && value <= self.b
    }

    fn intersect(self, other: Range) -> Option<Range> {
        let left = self.a.max(other.a);
        let right = self.b.min(other.b);
        if left <= right {
            Some(Range { a: left, b: right })
        } else {
            None
        }
    }

    fn apply(self, map: &Map) -> HashSet<Range> {
        let mut result: HashSet<Range> = HashSet::new();
        for (from, to) in map {
            let range = self.intersect(*from);
            if let Some(range) = range {
                result.insert(Range {
                    a: to.a + range.a - from.a,
                    b: to.b + range.b - from.b,
                });
            }
        }
        result
    }
}

type Map = Vec<(Range, Range)>;

#[derive(Debug, Clone)]
struct Input {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Input {
    fn new() -> Input {
        let mut maps: Vec<Map> = Vec::new();
        for _ in 0..7 {
            maps.push(Vec::new());
        }
        Input {
            seeds: Vec::new(),
            maps,
        }
    }
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5 {
            input: Input::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Input::new();
    }
}

impl Puzzle for Day5 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/5.input");
        let mut current_map = 0;
        for line in INPUT.lines() {
            if line.is_empty() {
                continue;
            }
            match &line[..5] {
                "seeds" => {
                    self.input.seeds = line[7..]
                        .split(' ')
                        .map(|seed| seed.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                }
                "seed-" => current_map = 0, //"seed_to_soil",
                "soil-" => current_map = 1, //"soil_to_fertilizer",
                "ferti" => current_map = 2, //"fertilizer_to_water",
                "water" => current_map = 3, //"water_to_light",
                "light" => current_map = 4, //"light_to_temperature",
                "tempe" => current_map = 5, //"temperature_to_humidity",
                "humid" => current_map = 6, //"humidity_to_location",
                _ => {
                    let mut tmp = line.split(' ');
                    let a = tmp.next().unwrap().parse::<usize>().unwrap();
                    let b = tmp.next().unwrap().parse::<usize>().unwrap();
                    let c = tmp.next().unwrap().parse::<usize>().unwrap();
                    self.input.maps[current_map]
                        .push((Range { a: b, b: b + c - 1 }, Range { a, b: a + c - 1 }));
                }
            }
        }
        for map in &mut self.input.maps {
            map.sort_by(|(from1, _), (from2, _)| from1.a.cmp(&from2.a));
            complete_range(map);
        }
    }

    fn part1(&self) -> String {
        let mut min_location: usize = 999999999999;
        for seed in &self.input.seeds {
            let mut value = *seed;
            for m in &self.input.maps {
                value = map_value(value, m);
            }
            min_location = min_location.min(value);
        }
        format!("{:?}", min_location)
    }

    fn part2(&self) -> String {
        let mut ranges: HashSet<Range> = HashSet::new();
        let s = &self.input.seeds;
        for i in 0..(s.len() / 2) {
            ranges.insert(Range {
                a: s[2 * i],
                b: s[2 * i] + s[2 * i + 1] - 1,
            });
        }

        for map in &self.input.maps {
            let mut new_ranges: HashSet<Range> = HashSet::new();
            for range in &ranges {
                new_ranges.extend(range.apply(map).iter());
            }
            ranges = new_ranges;
        }
        let mut min_location = 99999999999999;
        for range in &ranges {
            min_location = min_location.min(range.a);
        }

        format!("{:?}", min_location)
    }
}

fn map_value(value: usize, map: &Map) -> usize {
    for (from, to) in map {
        if from.contains(value) {
            return to.a + value - from.a;
        }
    }
    value
}

fn complete_range(map: &mut Map) {
    let mut result: Vec<(Range, Range)> = Vec::new();
    let mut edge = 0;
    for (from, _) in &*map {
        if from.a == 0 {
            edge = from.b + 1;
            continue;
        }
        if edge < from.a {
            let r = Range {
                a: edge,
                b: from.a - 1,
            };
            result.push((r, r));
        }
        edge = from.b + 1;
    }
    let r = Range {
        a: edge,
        b: 999999999999,
    };
    result.push((r, r));
    map.extend(result.iter());
}
