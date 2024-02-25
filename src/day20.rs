use aoc_2023_rust::Puzzle;
use num::Integer;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Day20 {
    input: Data<'static>,
}

impl Day20 {
    pub fn new() -> Day20 {
        Day20 {
            input: Data {
                modules: HashMap::new(),
                outputs: HashMap::new(),
            },
        }
    }

    pub fn _clear(&mut self) {
        self.input = Data {
            modules: HashMap::new(),
            outputs: HashMap::new(),
        };
    }
}

impl Puzzle for Day20 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/20.input");
        let mut tmp_modules: HashMap<&'static str, Module<'static>> = HashMap::new();
        let mut conjunctions: HashSet<&'static str> = HashSet::new();
        let mut outputs = HashMap::new();
        for line in INPUT.lines() {
            let (module, outs) = line.split_once(" -> ").unwrap();
            let outs = outs.split(", ").collect::<Vec<_>>();
            let name: &'static str;
            let mtype: ModuleType;
            let state = false;
            let memory = HashMap::new();
            let module = match module.chars().next().unwrap() {
                '%' => {
                    name = &module[1..];
                    mtype = FlipFlop;
                    Module {
                        name,
                        mtype,
                        state,
                        memory,
                    }
                }
                '&' => {
                    name = &module[1..];
                    mtype = Conjunction;
                    Module {
                        name,
                        mtype,
                        state,
                        memory,
                    }
                }
                'b' => {
                    name = module;
                    mtype = Broadcast;
                    Module {
                        name,
                        mtype,
                        state,
                        memory,
                    }
                }
                _ => panic!("Unknown module type!"),
            };
            if mtype == Conjunction {
                conjunctions.insert(name);
            }
            outputs.insert(name, outs);
            tmp_modules.insert(name, module);
        }

        self.input.modules = tmp_modules.clone();
        self.input.outputs = outputs;

        for (source, outs) in &self.input.outputs {
            for out in outs {
                if conjunctions.contains(out) {
                    if let Some(module) = self.input.modules.get_mut(out) {
                        module.memory.insert(source, Low);
                    }
                }
            }
        }
    }

    fn part1(&self) -> String {
        let mut modules = self.input.modules.clone();
        let mut high = 0u128;
        let mut low = 0u128;
        let mut times = 0;
        loop {
            let (new_modules, l, h, _) = push_button(modules, &self.input.outputs, "nothing");
            high += h;
            low += l;
            times += 1;
            modules = new_modules;
            if modules
                .values()
                .all(|module| !module.state && module.memory.values().all(|&signal| signal == Low))
                || times == 1000
            {
                break;
            }
        }

        format!("{:?}", (1000 / times) * low * (1000 / times) * high)
    }

    fn part2(&self) -> String {
        let modules = self.input.modules.clone();
        let ft = compute_cycle_len(modules, &self.input.outputs, "ft");
        let modules = self.input.modules.clone();
        let jz = compute_cycle_len(modules, &self.input.outputs, "jz");
        let modules = self.input.modules.clone();
        let sv = compute_cycle_len(modules, &self.input.outputs, "sv");
        let modules = self.input.modules.clone();
        let ng = compute_cycle_len(modules, &self.input.outputs, "ng");

        format!("{:?}", ft.lcm(&jz.lcm(&sv.lcm(&ng))))
    }
}

fn compute_cycle_len<'a>(
    mut modules: Modules<'a>,
    outputs: &'a Outputs<'a>,
    name: &'a str,
) -> u128 {
    let mut times = 0;
    loop {
        let (new_modules, _, _, received) = push_button(modules, outputs, name);
        times += 1;
        modules = new_modules;
        if received {
            return times;
        }
    }
}

fn push_button<'a>(
    mut modules: Modules<'a>,
    outputs: &'a Outputs<'a>,
    name: &'a str,
) -> (Modules<'a>, u128, u128, bool) {
    let mut queue = VecDeque::new();
    queue.push_back(("button", "broadcaster", Low));
    let mut high = 0u128;
    let mut low = 0u128;
    let mut received = false;
    while let Some((source, target, signal)) = queue.pop_front() {
        if source == name && target == "xm" && signal == High {
            received = true;
        };
        if let Some(module) = modules.get(&target) {
            if signal == Low {
                low += 1;
            } else {
                high += 1
            };
            // println!("{} -{:?}-> {}", source, signal, target);
            let (new_module, mut new_queue) = module.receive(outputs, source, signal);
            modules.insert(new_module.name, new_module);
            queue.append(&mut new_queue);
        } else {
            if signal == Low {
                low += 1;
            } else {
                high += 1
            };
        }
    }
    (modules, low, high, received)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Signal {
    High,
    Low,
}

use Signal::{High, Low};

type FlipFlopState = bool;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

use ModuleType::{Broadcast, Conjunction, FlipFlop};

type Memory<'a> = HashMap<&'a str, Signal>;

type Queue<'a> = VecDeque<(&'a str, &'a str, Signal)>;

#[derive(Debug, Clone)]
struct Data<'a> {
    modules: HashMap<&'a str, Module<'a>>,
    outputs: HashMap<&'a str, Vec<&'a str>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module<'a> {
    name: &'a str,
    mtype: ModuleType,
    state: FlipFlopState,
    memory: Memory<'a>,
}

type Modules<'a> = HashMap<&'a str, Module<'a>>;
type Outputs<'a> = HashMap<&'a str, Vec<&'a str>>;

impl<'a> Module<'a> {
    fn send(&self, outputs: &'a Outputs, signal: Signal) -> Queue<'a> {
        let mut queue = VecDeque::new();
        for out in &outputs[self.name] {
            queue.push_back((self.name, *out, signal));
        }
        queue
    }

    fn receive(
        &self,
        outputs: &'a Outputs,
        source: &'a str,
        signal: Signal,
    ) -> (Module<'a>, Queue<'a>) {
        match self.mtype {
            Broadcast => {
                let module = self.clone();
                let queue = self.send(outputs, signal);
                (module, queue)
            }
            FlipFlop => {
                if signal == Low {
                    let state = !self.state;
                    let new_signal = if state { High } else { Low };
                    let queue = self.send(outputs, new_signal);
                    (
                        Module {
                            name: self.name,
                            mtype: self.mtype,
                            state,
                            memory: self.memory.clone(),
                        },
                        queue,
                    )
                } else {
                    (self.clone(), VecDeque::new())
                }
            }
            Conjunction => {
                let mut mem = self.memory.clone();
                mem.insert(source, signal);
                let new_signal = if mem.values().all(|&v| v == High) {
                    Low
                } else {
                    High
                };
                let queue = self.send(outputs, new_signal);
                (
                    Module {
                        name: self.name,
                        mtype: self.mtype,
                        state: self.state,
                        memory: mem,
                    },
                    queue,
                )
            }
        }
    }
}
