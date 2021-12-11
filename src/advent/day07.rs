use crate::advent::AdventSolver;
use anyhow::Error;
use std::fs;

#[derive(Default)]
pub struct Solver;

const EXAMPLE: [i64; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let crabs =
            fs::read_to_string(input_path)?
               .trim()
               .split(",")
               .map(|s| Ok(s.parse::<i64>()?))
               .collect::<Result<Vec<i64>, Error>>()?;

        // Part 1
        let (position, fuel) =
            Self::find_alignment(&crabs, |distance| distance);
        println!("Position {} requires {} total fuel.", position, fuel);

        // Part 2
        let (position, fuel) =
            Self::find_alignment(&crabs, |distance| distance*(distance+1)/2);
        println!("Position {} requires {} total fuel.", position, fuel);

        Ok(())
    }
}

impl Solver {
    fn find_alignment<F>(crabs: &Vec<i64>, fuel_used: F) -> (i64, i64)
    where
        F: Fn(i64) -> i64
    {
        let min = *crabs.iter().min().unwrap();
        let max = *crabs.iter().max().unwrap();
        (min..=max).map(|position| {
                        let total_fuel =
                            crabs.iter()
                                 .map(|c| fuel_used((c-position).abs()))
                                 .sum();
                        (position, total_fuel)
                   })
                   .min_by_key(|(_position, fuel)| *fuel)
                   .unwrap()
    }
}
