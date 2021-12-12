use crate::advent::AdventSolver;
use crate::shared::io;
use anyhow::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Default)]
pub struct Solver;

lazy_static! {
    static ref BRACKETS: HashMap<char, char> = [
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ].into_iter().collect();

    static ref COMPLETION_SCORES: HashMap<char, u64> = [
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ].into_iter().collect();

    static ref ERROR_SCORES: HashMap<char, u64> = [
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ].into_iter().collect();
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let program = io::read_lines_from_file(input_path)?;
        let mut error_score = 0;
        let mut completion_scores = Vec::new();
        'next_line: for line in program.iter() {
            let mut stack = Vec::new();
            for c in line.chars() {
                if BRACKETS.contains_key(&c) {
                    stack.push(c);
                } else {
                    let open = stack.pop();
                    if open.is_none() || BRACKETS[&open.unwrap()] != c {
                        error_score += ERROR_SCORES[&c];
                        continue 'next_line;
                    }
                }
            }
            // Handle incomplete lines
            let mut completion_score = 0;
            while let Some(open) = stack.pop() {
                completion_score *= 5;
                completion_score += COMPLETION_SCORES[&open];
            }
            completion_scores.push(completion_score);
        }
        completion_scores.sort();

        println!("Total error score: {}", error_score);
        println!("Middle completion score: {}",
                 completion_scores[completion_scores.len()/2]);
        Ok(())
    }
}
