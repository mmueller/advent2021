use crate::advent::AdventSolver;
use anyhow::Error;
use std::fs;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let fish =
            fs::read_to_string(input_path)?
               .trim()
               .split(",")
               .map(|s| Ok(s.parse::<usize>()?))
               .collect::<Result<Vec<usize>, Error>>()?;

        // Initial count.
        let mut counts = vec![0; 9];
        for fish in fish.iter() {
            counts[*fish] += 1;
        }

        // Update lanternfish counts every day.
        for day in 1..=256 {
            counts = [
              /* New 0 */ counts[1],
              /* New 1 */ counts[2],
              /* New 2 */ counts[3],
              /* New 3 */ counts[4],
              /* New 4 */ counts[5],
              /* New 5 */ counts[6],
              /* New 6 */ counts[7] + counts[0],
              /* New 7 */ counts[8],
              /* New 8 */ counts[0],
            ].to_vec();

            if day == 80 || day == 256 {
                println!("After {} days, fish count is {}.",
                         day, counts.iter().sum::<usize>());
            }
        }

        Ok(())
    }
}
