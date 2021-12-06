use crate::advent::AdventSolver;
use crate::shared::io;
use anyhow::Error;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let input = io::read_numbers_from_file::<u64>(input_path)?;

        // Part 1
        let increased: usize =
            input.windows(2)
                 .map(|slice| if slice[1] > slice[0] { 1 } else { 0 })
                 .sum();
        println!("Number of increasing depths: {}", increased);

        // Part 2
        let increased: usize =
            input.windows(4)
                 .map(|slice| if slice[1]+slice[2]+slice[3] >
                                 slice[0]+slice[1]+slice[2] { 1 } else { 0 })
                 .sum();
        println!("Number of increasing depths: {}", increased);

        Ok(())
    }
}
