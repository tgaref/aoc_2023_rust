use std::{cmp::Ordering, collections::HashMap};

use aoc_2023_rust::Puzzle;

#[derive(Debug, Clone)]
pub struct Day7 {
    input: Vec<Hand>,
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new();
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPairs,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    hand: Vec<usize>,
    stats: HashMap<usize, usize>,
    bid: usize,
}

fn group(hand: &[usize]) -> HashMap<usize, usize> {
    let mut hash: HashMap<usize, usize> = HashMap::new();
    for card in hand {
        if let Some(count) = hash.get_mut(card) {
            *count += 1;
        } else {
            hash.insert(*card, 1);
        }
    }
    hash
}

impl Hand {
    fn eval(&self) -> Rank {
        let mut counts = self.stats.values().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(a));
        match counts[0] {
            5 => Rank::Five,
            4 => Rank::Four,
            3 => {
                if *counts[1] == 2 {
                    Rank::Full
                } else {
                    Rank::Three
                }
            }
            2 => {
                if *counts[1] == 2 {
                    Rank::TwoPairs
                } else {
                    Rank::OnePair
                }
            }
            1 => Rank::HighCard,
            _ => panic!("How many cards did you get?"),
        }
    }

    fn adjust(&self) -> Self {
        let new_hand = self
            .hand
            .iter()
            .map(|&card| if card == 11 { 0 } else { card })
            .collect();

        let mut new_stats: HashMap<usize, usize> = self.stats.clone();
        if let Some(count) = new_stats.remove(&11) {
            let mut max_key = None;
            let mut max_count = 0;
            for (&card, &count) in &new_stats {
                if count > max_count {
                    max_count = count;
                    max_key = Some(card);
                }
            }
            if let Some(max_key) = max_key {
                if let Some(x) = new_stats.get_mut(&max_key) {
                    *x += count;
                }
            } else {
                new_stats.insert(11, 5);
            }
        }
        Hand {
            hand: new_hand,
            stats: new_stats,
            bid: self.bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_rank = self.eval();
        let other_rank = other.eval();
        if self_rank == other_rank {
            for (x, y) in self.hand.iter().zip(other.hand.iter()) {
                if x != y {
                    return x.cmp(y);
                }
            }
            Ordering::Equal
        } else {
            self_rank.cmp(&other_rank)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Puzzle for Day7 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/7.input");
        for line in INPUT.lines() {
            let (hand_str, bid_str) = line.split_once(' ').unwrap();
            let bid = bid_str.parse::<usize>().unwrap();
            let hand: Vec<usize> = hand_str
                .chars()
                .map(|c| {
                    if c.is_ascii_digit() {
                        c.to_digit(10).unwrap() as usize
                    } else {
                        match c {
                            'T' => 10,
                            'J' => 11,
                            'Q' => 12,
                            'K' => 13,
                            'A' => 14,
                            _ => panic!("What is this card?"),
                        }
                    }
                })
                .collect();
            let stats = group(&hand);
            self.input.push(Hand { hand, stats, bid });
        }
    }

    fn part1(&self) -> String {
        let mut hands = self.input.clone();
        hands.sort();
        let win: usize = hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bid)
            .sum();

        format!("{:?}", win)
    }

    fn part2(&self) -> String {
        let mut hands: Vec<Hand> = self.input.iter().map(|hand| hand.adjust()).collect();
        hands.sort();
        let win: usize = hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bid)
            .sum();

        format!("{:?}", win)
    }
}
