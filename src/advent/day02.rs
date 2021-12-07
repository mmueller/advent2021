use crate::advent::AdventSolver;
use crate::shared::io;
use anyhow::{Error, format_err};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Default)]
pub struct Solver;

#[derive(Copy,Clone)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

lazy_static! {
    static ref COMMAND_REGEX: Regex =
        Regex::new(r"(?P<command>forward|down|up) (?P<amount>\d+)")
              .unwrap();
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let commands = Self::read_commands(input_path)?;

        // Part 1
        let mut pos = 0;
        let mut depth = 0;
        for command in commands.iter() {
            match command {
                Command::Forward(amount) => pos += amount,
                Command::Down(amount) => depth += amount,
                Command::Up(amount) => depth -= amount,
            }
        }
        println!("P1: Pos {}, depth {} (product: {})", pos, depth, pos*depth);

        // Part 2
        let mut aim = 0;
        let mut pos = 0;
        let mut depth = 0;
        for command in commands.iter() {
            match command {
                Command::Forward(amount) => {
                    pos += amount;
                    depth += amount * aim;
                },
                Command::Down(amount) => aim += amount,
                Command::Up(amount) => aim -= amount,
            }
        }
        println!("P2: Pos {}, depth {} (product: {})", pos, depth, pos*depth);

        Ok(())
    }
}

impl Solver {
    fn read_commands(input_path: &str) -> Result<Vec<Command>, Error> {
        io::read_lines_from_file(input_path)?
           .iter()
           .map(|line| Self::parse_command(line))
           .collect::<Result<Vec<Command>, Error>>()
    }

    fn parse_command(text: &str) -> Result<Command, Error> {
        match COMMAND_REGEX.captures(text) {
            Some(caps) => {
                let amount = caps["amount"].parse::<i64>()?;
                match &caps["command"] {
                    "forward" => Ok(Command::Forward(amount)),
                    "down" => Ok(Command::Down(amount)),
                    "up" => Ok(Command::Up(amount)),
                    _ => unreachable!(),
                }
            },
            None => {
                Err(format_err!("Couldn't parse command: {}", text))
            }
        }
    }
}
