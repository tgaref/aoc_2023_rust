use aoc_2023_rust::Puzzle;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Day19 {
    input: Input<'static>,
}

impl Day19 {
    pub fn new() -> Day19 {
        Day19 {
            input: Input::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Input::new();
    }
}

#[derive(Debug, Clone)]
struct Input<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
}

impl<'a> Input<'a> {
    fn new() -> Input<'a> {
        Input {
            workflows: HashMap::new(),
            parts: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow<'a> {
    name: &'a str,
    conditions: Vec<Condition<'a>>,
    default: &'a str,
}

#[derive(Debug, Clone)]
struct Condition<'a> {
    category: Category,
    range: Range,
    next: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparison {
    LT,
    GT,
}

use Comparison::{GT, LT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
}

use Category::{A, M, S, X};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Range {
    a: usize,
    b: usize,
}

impl Range {
    fn contains(self, value: usize) -> bool {
        self.a <= value && value <= self.b
    }

    fn intersect(self, other: &Range) -> Range {
        let left = self.a.max(other.a);
        let right = self.b.min(other.b);
        Range { a: left, b: right }
    }

    fn complement(self) -> Range {
        if self.a > 1 {
            Range {
                a: 1,
                b: self.a - 1,
            }
        } else {
            Range {
                a: self.b + 1,
                b: 4000,
            }
        }
    }

    fn len(&self) -> u128 {
        (self.b - self.a + 1) as u128
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State<'a> {
    name: &'a str,
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl<'a> State<'a> {
    fn process(&self, workflows: &'a HashMap<&'a str, Workflow<'a>>) -> Vec<State<'a>> {
        let mut new_states: Vec<State> = Vec::new();
        let wf = &workflows[self.name];
        for (
            i,
            Condition {
                category,
                range,
                next,
            },
        ) in wf.conditions.iter().enumerate()
        {
            let mut new_state = self.clone();
            new_state.name = next;
            for j in 0..i {
                let cond = &wf.conditions[j];
                match cond.category {
                    X => new_state.x = new_state.x.intersect(&cond.range.complement()),
                    M => new_state.m = new_state.m.intersect(&cond.range.complement()),
                    A => new_state.a = new_state.a.intersect(&cond.range.complement()),
                    S => new_state.s = new_state.s.intersect(&cond.range.complement()),
                }
            }
            match category {
                X => new_state.x = new_state.x.intersect(range),
                M => new_state.m = new_state.m.intersect(range),
                A => new_state.a = new_state.a.intersect(range),
                S => new_state.s = new_state.s.intersect(range),
            }
            if new_state.non_empty() {
                new_states.push(new_state);
            }
        }
        let mut new_state = self.clone();
        new_state.name = wf.default;
        for cond in &wf.conditions {
            match cond.category {
                X => new_state.x = new_state.x.intersect(&cond.range.complement()),
                M => new_state.m = new_state.m.intersect(&cond.range.complement()),
                A => new_state.a = new_state.a.intersect(&cond.range.complement()),
                S => new_state.s = new_state.s.intersect(&cond.range.complement()),
            }
        }
        if new_state.non_empty() {
            new_states.push(new_state);
        }
        new_states
    }

    fn non_empty(&self) -> bool {
        self.x.a <= self.x.b && self.m.a <= self.m.b && self.a.a <= self.a.b && self.s.a <= self.s.b
    }

    fn volume(&self) -> u128 {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn through_one<'a>(&'a self, workflow: &'a Workflow) -> &'a str {
        for Condition {
            category,
            range,
            next,
        } in &workflow.conditions
        {
            match category {
                X => {
                    if range.contains(self.x) {
                        return next;
                    }
                }
                M => {
                    if range.contains(self.m) {
                        return next;
                    }
                }
                A => {
                    if range.contains(self.a) {
                        return next;
                    }
                }
                S => {
                    if range.contains(self.s) {
                        return next;
                    }
                }
            }
        }
        workflow.default
    }

    fn through<'a>(&'a self, workflows: &'a HashMap<&'a str, Workflow>) -> &'a str {
        let mut current = "in";
        while current != "A" && current != "R" {
            current = self.through_one(&workflows[current]);
        }
        current
    }
}

fn parse_name(s: &str) -> IResult<&str, &str> {
    alpha1(s)
}

fn parse_category(s: &str) -> IResult<&str, Category> {
    map(
        alt((tag("x"), tag("m"), tag("a"), tag("s"))),
        |s: &str| match s {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => panic!("Unknown category"),
        },
    )(s)
}

fn parse_comparison(s: &str) -> IResult<&str, Comparison> {
    map(alt((tag("<"), tag(">"))), |s: &str| match s {
        "<" => LT,
        ">" => GT,
        _ => panic!("Unknown comparison"),
    })(s)
}

fn parse_decimal(s: &str) -> IResult<&str, usize> {
    map_res(digit1, |d: &str| d.parse::<usize>())(s)
}

fn parse_condition(s: &str) -> IResult<&str, Condition> {
    map(
        tuple((
            parse_category,
            parse_comparison,
            parse_decimal,
            tag(":"),
            parse_name,
        )),
        |(category, comparison, other, _, next)| {
            let range = match comparison {
                LT => Range { a: 1, b: other - 1 },
                GT => Range {
                    a: other + 1,
                    b: 4000,
                },
            };
            Condition {
                category,
                range,
                next,
            }
        },
    )(s)
}

fn parse_conditions(s: &str) -> IResult<&str, Vec<Condition>> {
    separated_list0(tag(","), parse_condition)(s)
}

fn parse_workflow(s: &str) -> IResult<&str, Workflow> {
    map(
        tuple((parse_name, tag("{"), parse_conditions, tag(","), parse_name)),
        |(name, _, conditions, _, default)| Workflow {
            name,
            conditions,
            default,
        },
    )(s)
}

fn parse_part(s: &str) -> IResult<&str, Part> {
    map(
        tuple((
            tag("{x="),
            parse_decimal,
            tag(",m="),
            parse_decimal,
            tag(",a="),
            parse_decimal,
            tag(",s="),
            parse_decimal,
            tag("}"),
        )),
        |(_, x, _, m, _, a, _, s, _)| Part { x, m, a, s },
    )(s)
}

impl Puzzle for Day19 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/19.input");
        let mut parsing_workflows = true;
        for line in INPUT.lines() {
            if line.is_empty() {
                parsing_workflows = false;
                continue;
            }
            if parsing_workflows {
                let workflow = parse_workflow(line).unwrap().1;
                self.input.workflows.insert(workflow.name, workflow);
            } else {
                self.input.parts.push(parse_part(line).unwrap().1);
            }
        }
    }

    fn part1(&self) -> String {
        let result: usize = self
            .input
            .parts
            .iter()
            .filter(|part| part.through(&self.input.workflows) == "A")
            .map(|Part { x, m, a, s }| x + m + a + s)
            .sum();

        format!("{:?}", result)
    }

    fn part2(&self) -> String {
        let init = State {
            name: "in",
            x: Range { a: 1, b: 4000 },
            m: Range { a: 1, b: 4000 },
            a: Range { a: 1, b: 4000 },
            s: Range { a: 1, b: 4000 },
        };
        let accepted = bfs(init, &self.input.workflows);
        let count: u128 = accepted.iter().map(|s| s.volume()).sum();

        format!("{:?}", count)
    }
}

fn bfs<'a>(init: State<'a>, workflows: &'a HashMap<&'a str, Workflow<'a>>) -> HashSet<State<'a>> {
    let mut accepted: HashSet<State> = HashSet::new();
    let mut queue: Vec<State> = vec![init];

    while let Some(state) = queue.pop() {
        if state.name == "R" {
            continue;
        }
        if state.name == "A" {
            accepted.insert(state);
            continue;
        }
        let mut new_states = state.process(workflows);
        queue.append(&mut new_states)
    }
    accepted
}
