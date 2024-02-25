use aoc_2023_rust::Puzzle;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone)]
pub struct Day2 {
    input: Vec<Game>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Group {
    red: usize,
    green: usize,
    blue: usize,
}

type Game = Vec<Group>;

impl Day2 {
    pub fn new() -> Day2 {
        Day2 { input: Vec::new() }
    }

    pub fn _clear(&mut self) {
        self.input = Vec::new()
    }
}

fn parse_decimal(s: &str) -> IResult<&str, usize> {
    map_res(digit1, |d: &str| d.parse::<usize>())(s)
}

fn parse_cube(s: &str) -> IResult<&str, Cube> {
    map(
        tuple((
            parse_decimal,
            tag(" "),
            alt((tag("red"), tag("green"), tag("blue"))),
        )),
        |t| match t {
            (n, _, "red") => Cube::Red(n),
            (n, _, "green") => Cube::Green(n),
            (n, _, "blue") => Cube::Blue(n),
            (_, _, _) => panic!("Color not expected!"),
        },
    )(s)
}

fn parse_group(s: &str) -> IResult<&str, Group> {
    map(separated_list1(tag(", "), parse_cube), |cubes| {
        let mut group = Group {
            red: 0,
            green: 0,
            blue: 0,
        };
        for cube in cubes {
            match cube {
                Cube::Red(n) => {
                    group.red = n;
                }
                Cube::Green(n) => {
                    group.green = n;
                }
                Cube::Blue(n) => {
                    group.blue = n;
                }
            }
        }
        group
    })(s)
}

fn parse_game(s: &str) -> IResult<&str, Game> {
    map(
        tuple((
            tag("Game "),
            digit1,
            tag(": "),
            separated_list1(tag("; "), parse_group),
        )),
        |(_, _, _, list)| list,
    )(s)
}

fn valid_game(game: &Game, bounds: (usize, usize, usize)) -> bool {
    game.iter()
        .all(|group| group.red <= bounds.0 && group.green <= bounds.1 && group.blue <= bounds.2)
}

fn game_power(game: &Game) -> usize {
    let (red, green, blue) = game.iter().fold((0, 0, 0), |(red, green, blue), group| {
        (
            red.max(group.red),
            green.max(group.green),
            blue.max(group.blue),
        )
    });
    red * green * blue
}

impl Puzzle for Day2 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/2.input");
        for line in INPUT.lines() {
            self.input.push(parse_game(line).unwrap().1);
        }
    }

    fn part1(&self) -> String {
        let red_bound = 12;
        let green_bound = 13;
        let blue_bound = 14;
        let result: usize = self
            .input
            .iter()
            .enumerate()
            .filter(|(_, game)| valid_game(game, (red_bound, green_bound, blue_bound)))
            .map(|(i, _)| i + 1)
            .sum();
        format!("{:?}", result)
    }

    fn part2(&self) -> String {
        let result: usize = self.input.iter().map(|game: &Game| game_power(game)).sum();
        format!("{:?}", result)
    }
}
