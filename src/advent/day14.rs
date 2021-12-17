use crate::advent::AdventSolver;
use crate::shared::io;
use anyhow::{Error, format_err};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default)]
pub struct Solver;

#[derive(Debug)]
struct Rule {
    c1: char,
    c2: char,
    i: char,
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let (polymer, rules) = Self::read_input(input_path)?;
        let mut pair_counts = Self::initial_pair_counts(&polymer);
        let last_char = polymer.chars().last().unwrap();

        for _ in 0..10 {
            pair_counts = Self::apply_rules(&pair_counts, &rules);
        }
        println!("After 10 iterations, diff of most/least common: {}",
                 Self::calculate_solution(&pair_counts, last_char));
        for _ in 0..30 {
            pair_counts = Self::apply_rules(&pair_counts, &rules);
        }
        println!("After 40 iterations, diff of most/least common: {}",
                 Self::calculate_solution(&pair_counts, last_char));
        Ok(())
    }
}

impl Solver {
    fn apply_rules(pair_counts: &HashMap<(char, char), usize>,
                   rules: &Vec<Rule>) -> HashMap<(char, char), usize>
    {
        let mut new_counts = HashMap::new();
        for (&(c1, c2), count) in pair_counts.iter() {
            for rule in rules.iter() {
                if rule.c1 == c1 && rule.c2 == c2 {
                    *new_counts.entry((c1, rule.i)).or_insert(0) += count;
                    *new_counts.entry((rule.i, c2)).or_insert(0) += count;
                    break;
                }
            }
        }
        new_counts
    }

    fn initial_pair_counts(polymer: &str) -> HashMap<(char, char), usize>
    {
        let mut result = HashMap::new();
        for (c1, c2) in polymer.chars().tuple_windows() {
            *result.entry((c1, c2)).or_insert(0) += 1
        }
        result
    }

    fn calculate_solution(pair_counts: &HashMap<(char, char), usize>,
                          last_char: char) -> usize
    {
        // Count occurrences of the first character of each pair in the
        // pair_counts hash, so we don't double-count. Afterward, the last
        // char of the polymer (which never changes) needs to be included.
        let mut counts: HashMap<char, usize> = HashMap::new();
        for ((c1, _c2), count) in pair_counts.iter() {
            *counts.entry(*c1).or_insert(0) += count;
        }
        *counts.entry(last_char).or_insert(0) += 1;
        let max_count = counts.iter().map(|(_k, v)| v).max().unwrap();
        let min_count = counts.iter().map(|(_k, v)| v).min().unwrap();
        max_count - min_count
    }

    fn read_input(input_path: &str) -> Result<(String, Vec<Rule>), Error>
    {
        let lines = io::read_lines_from_file(input_path)?;
        let mut iter = lines.iter();
        let polymer = iter.next().unwrap().to_string();
        let mut rules = Vec::new();
        if iter.next().unwrap() != "" {
            return Err(format_err!("No separator after polymer"));
        }
        for line in iter {
            let mut parts = line.split(" -> ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            rules.push(Rule {
                c1: left.chars().nth(0).unwrap(),
                c2: left.chars().nth(1).unwrap(),
                i: right.chars().nth(0).unwrap(),
            });
        }
        Ok((polymer, rules))
    }
}
