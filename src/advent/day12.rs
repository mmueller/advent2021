use crate::advent::AdventSolver;
use crate::shared::io;
use anyhow::{Error, format_err};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default)]
pub struct Solver {
    graph: HashMap<String, Vec<String>>,
    in_part1: bool
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        self.read_graph(input_path)?;
        self.in_part1 = true;
        let paths = self.traverse_graph("start", "end", &Vec::new())?;
        println!("Found {} paths through the graph.", paths.len());
        self.in_part1 = false;
        let paths = self.traverse_graph("start", "end", &Vec::new())?;
        println!("Found {} paths through the graph.", paths.len());
        Ok(())
    }
}

impl Solver {
    fn add_edge(&mut self, left: &str, right: &str) {
        self.graph.entry(left.to_string())
                  .or_insert_with(|| Vec::new())
                  .push(right.to_string());
        self.graph.entry(right.to_string())
                  .or_insert_with(|| Vec::new())
                  .push(left.to_string());
    }

    fn read_graph(&mut self, input_path: &str) -> Result<(), Error> {
        let lines = io::read_lines_from_file(input_path)?;
        for line in lines {
            let (left, right) =
                line.split("-").next_tuple()
                    .ok_or(format_err!("Parse error: {}", line))?;
            self.add_edge(left, right);
        }
        Ok(())
    }

    fn is_small_cave(cave: &str) -> bool {
        // Oh my god rust strings are clunky
        cave.chars().nth(0).unwrap().is_lowercase()
    }

    fn can_visit(&self, cave: &str, prefix: &Vec<String>) -> bool {
        let mut cave_counts: HashMap<&str, usize> = HashMap::new();
        cave_counts.insert(cave, 0); // To avoid clunky checks later
        for cave in prefix.iter() {
            *cave_counts.entry(cave).or_insert(0) += 1;
        }
        let small_count =
            cave_counts.iter()
                       .filter(|(k, _v)| Self::is_small_cave(k))
                       .map(|(_k, v)| *v)
                       .max().unwrap();

        if self.in_part1 {
            // Rule for part 1
            if cave.chars().nth(0).unwrap().is_lowercase() {
                cave_counts[cave] < 1
            } else {
                true
            }
        } else {
            // Rule for part 2
            if cave == "start" || cave == "end" {
                cave_counts[cave] < 1
            } else if Self::is_small_cave(cave) {
                small_count < 2 || cave_counts[cave] < 1
            } else {
                true
            }
        }
    }

    fn traverse_graph(&self, start: &str, end: &str, prefix: &Vec<String>)
            -> Result<Vec<Vec<String>>, Error>
    {

        // Create a new prefix that includes our local start
        let prefix = prefix.iter()
                           .chain(&[start.to_string()])
                           .cloned().collect();

        // Base case, we're already at end
        if !self.graph.contains_key(&start.to_string()) {
            return Err(format_err!("No such cave: {}", start));
        }
        if start == end {
            return Ok(vec![prefix]);
        }

        // Recursive case, traverse all neighbors
        let mut paths = Vec::new();
        for n in self.graph[&start.to_string()].iter() {
            if self.can_visit(&n, &prefix) {
                paths.append(&mut self.traverse_graph(n, end, &prefix)?);
            }
        }

        Ok(paths)
    }
}
