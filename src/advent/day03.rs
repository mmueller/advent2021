use crate::advent::AdventSolver;
use crate::shared::io;
use anyhow::{Error, format_err};

#[derive(Default)]
pub struct Solver;

// My input seems to be all 12-bit values
const NUM_BITS: u64 = 12;

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let diagnostic_report = Self::read_diagnostic_report(input_path)?;

        // Part 1
        let mut gamma = 0;
        let mut epsilon = 0;
        for bit in 0..NUM_BITS {
            if Self::most_common_value(&diagnostic_report, bit) == 1 {
                gamma += 1 << bit;
            } else {
                epsilon += 1 << bit;
            }
        }
        let power_consumption = gamma * epsilon;
        println!("Power consumption: {}", power_consumption);

        // Part 2
        let mut oxygen_candidates = diagnostic_report.clone();
        let mut co2_candidates = diagnostic_report.clone();
        for bit in (0..NUM_BITS).rev() {
            let common = Self::most_common_value(&oxygen_candidates, bit);
            let bitmask = 1<<bit;
            if oxygen_candidates.len() > 1 {
                oxygen_candidates =
                    oxygen_candidates.iter()
                                     .filter(|v| *v & bitmask == common<<bit)
                                     .map(|v| *v)
                                     .collect();
            }
            let common = Self::least_common_value(&co2_candidates, bit);
            if co2_candidates.len() > 1 {
                co2_candidates =
                    co2_candidates.iter()
                                  .filter(|v| *v & bitmask == common<<bit)
                                  .map(|v| *v)
                                  .collect();
            }
        }
        if oxygen_candidates.len() != 1 {
            Err(format_err!("Found {} oxygen diagnostic candidates",
                            oxygen_candidates.len()))
        } else if co2_candidates.len() != 1 {
            Err(format_err!("Found {} co2 diagnostic candidates",
                            co2_candidates.len()))
        } else {
            println!("Life support rating: {}",
                     oxygen_candidates[0] * co2_candidates[0]);
            Ok(())
        }
    }
}

impl Solver {
    fn read_diagnostic_report(input_path: &str) -> Result<Vec<usize>, Error> {
        let lines = io::read_lines_from_file(input_path)?;
        Self::read_diagnostic_lines(&lines)
    }

    fn read_diagnostic_lines<T: AsRef<str>>(lines: &Vec<T>)
            -> Result<Vec<usize>, Error> {
        Ok(lines.iter()
                .map(|line| usize::from_str_radix(line.as_ref(), 2).unwrap())
                .collect())
    }

    fn most_common_value(vec: &Vec<usize>, bit: u64) -> usize {
        let bitmask = 1 << bit;
        let sum: usize = vec.iter()
                            .filter(|v| *v & bitmask != 0)
                            .count();
        (sum*2 >= vec.len()) as usize
    }

    fn least_common_value(vec: &Vec<usize>, bit: u64) -> usize {
        if Self::most_common_value(vec, bit) == 0 { 1 } else { 0 }
    }
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;
    use super::Solver;

    lazy_static! {
        static ref EXAMPLE: Vec<&'static str> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ];
    }

    #[test]
    fn test_most_common_value() {
        let diag = Solver::read_diagnostic_lines(&EXAMPLE).unwrap();
        assert_eq!(1, Solver::most_common_value(&diag, 4));
        assert_eq!(0, Solver::most_common_value(&diag, 3));
        assert_eq!(1, Solver::most_common_value(&diag, 2));
        assert_eq!(1, Solver::most_common_value(&diag, 1));
        assert_eq!(0, Solver::most_common_value(&diag, 0));
    }
}
