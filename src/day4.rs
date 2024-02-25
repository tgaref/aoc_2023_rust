use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::{map, map_res},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
use std::collections::HashSet;

use aoc_2023_rust::Puzzle;

#[derive(Debug, Clone)]
pub struct Day4 {
    input: Vec<Card>,
}

#[derive(Debug, Clone)]
struct Card {
    winning_nums: HashSet<usize>,
    actual_nums: Vec<usize>,
}

impl Card {
    fn eval(&self) -> u32 {
        self.actual_nums.iter().fold(0, |count, n| {
            if self.winning_nums.contains(n) {
                count + 1
            } else {
                count
            }
        })
    }
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_decimal(s: &str) -> IResult<&str, usize> {
    map_res(digit1, |d: &str| d.parse::<usize>())(s)
}

fn parse_number_list(s: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(multispace1, parse_decimal)(s)
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    map(
        tuple((
            ws(tag("Card")),
            digit1,
            ws(tag(":")),
            parse_number_list,
            ws(tag("|")),
            parse_number_list,
        )),
        |(_, _, _, list1, _, list2)| Card {
            winning_nums: list1.into_iter().collect::<HashSet<_>>(),
            actual_nums: list2,
        },
    )(s)
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

impl Puzzle for Day4 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/4.input");
        for line in INPUT.lines() {
            self.input.push(parse_card(line).unwrap().1);
        }
    }

    fn part1(&self) -> String {
        let result: usize = self
            .input
            .iter()
            .map(|card| {
                let e = card.eval();
                if e == 0 {
                    0
                } else {
                    usize::pow(2, e - 1)
                }
            })
            .sum();
        format!("{:?}", result)
    }

    fn part2(&self) -> String {
        let mut multiplicities: Vec<usize> = Vec::with_capacity(self.input.len());
        self.input.iter().for_each(|_| {
            multiplicities.push(1);
        });
        self.input.iter().enumerate().for_each(|(i, card)| {
            let k = card.eval() as usize;
            for j in (i + 1)..(i + 1 + k) {
                multiplicities[j] += multiplicities[i];
            }
        });
        let total = multiplicities.iter().sum::<usize>();
        format!("{:?}", total)
    }
}
