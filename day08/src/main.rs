use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INSTRUCTION: Regex = Regex::new(r"^(nop|acc|jmp) (-\d+|\+\d+)$").unwrap();
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Acc(isize),
    Nop(isize),
    Jmp(isize),
}
#[derive(Debug, Clone)]
pub enum ExecutionStatus {
    Success,
    Failure,
}

#[derive(Debug, Clone)]
pub struct Computer {
    instructions: Vec<Instruction>,
    accumulator: isize,
    ip: isize,
    execution_status: Option<ExecutionStatus>,
}

impl Computer {
    pub fn parse<'a, I>(lines: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut instructions = Vec::new();

        for instruction in lines {
            let instruction = match INSTRUCTION
                .captures(&instruction)
                .and_then(|captures| Some((captures.get(1)?.as_str(), captures.get(2)?.as_str())))
            {
                Some((op, value)) => match op {
                    "acc" => (Instruction::Acc(value.parse::<isize>().unwrap())),
                    "nop" => (Instruction::Nop(value.parse::<isize>().unwrap())),
                    "jmp" => (Instruction::Jmp(value.parse::<isize>().unwrap())),
                    _ => panic!("Operation not permitted"),
                },
                None => panic!("Could not parse operation"),
            };

            instructions.push(instruction);
        }

        Computer {
            instructions,
            accumulator: 0,
            ip: 0,
            execution_status: None,
        }
    }

    pub fn execute(&mut self) {
        let instructions_len = self.instructions.len();
        let mut history: HashSet<isize> = HashSet::new();

        while !history.contains(&self.ip) && (self.ip as usize) < instructions_len {
            history.insert(self.ip);

            match self.instructions[self.ip as usize] {
                Instruction::Acc(value) => {
                    self.accumulator += value;
                    self.ip += 1;
                }
                Instruction::Jmp(value) => self.ip += value,
                Instruction::Nop(_) => self.ip += 1,
            }
        }

        if (self.ip as usize) == instructions_len {
            self.execution_status = Some(ExecutionStatus::Success);
        } else {
            self.execution_status = Some(ExecutionStatus::Failure);
        }
    }

    pub fn replace(&mut self, instruction: Instruction, ip: usize) {
        self.instructions[ip] = instruction;
    }

    pub fn executed_successfully(&self) -> bool {
        match self.execution_status {
            Some(ExecutionStatus::Success) => true,
            _ => false,
        }
    }
}

pub fn part1(computer: &mut Computer) -> isize {
    computer.execute();
    computer.accumulator
}

pub fn part2(computer: &mut Computer) -> isize {
    for (line, instruction) in
        computer
            .instructions
            .iter()
            .enumerate()
            .filter(|inst| match inst.1 {
                Instruction::Nop(_) | Instruction::Jmp(_) => true,
                _ => false,
            })
    {
        let mut new_computer = computer.clone();
        let instruction = match instruction {
            Instruction::Nop(value) => Instruction::Jmp(*value),
            Instruction::Jmp(value) => Instruction::Nop(*value),
            _ => panic!("Invalid instruction"),
        };

        new_computer.replace(instruction, line);
        new_computer.execute();

        if new_computer.executed_successfully() {
            return new_computer.accumulator;
        }
    }

    panic!("No answer was found!");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = Computer::parse(input.lines());
    println!("{:?}", part1(&mut computer.clone()));
    println!("{:?}", part2(&mut computer.clone()));
}
