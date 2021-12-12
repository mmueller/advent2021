use crate::advent::AdventSolver;
use crate::shared::io;
use anyhow::{Error, format_err};
use std::collections::{HashSet, VecDeque};

#[derive(Default)]
pub struct Solver;

const EXAMPLE: [i64; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let smoke_map: Vec<Vec<u32>> =
            io::read_lines_from_file(input_path)?
               .iter()
               .map(|line| line.chars()
                               .map(|c| c.to_digit(10)
                                         .ok_or(format_err!("bad: {}", c)))
                               .collect::<Result<Vec<u32>, Error>>())
               .collect::<Result<Vec<Vec<u32>>, Error>>()?;

        let mut total_risk = 0;
        let mut basins: Vec<usize> = Vec::new();
        let height = smoke_map.len();
        let width = smoke_map[0].len();
        for row in 0..height {
            for col in 0..width {
                let v = smoke_map[row][col];
                if (row == 0 || smoke_map[row-1][col] > v) &&
                   (row == height-1 || smoke_map[row+1][col] > v) &&
                   (col == 0 || smoke_map[row][col-1] > v) &&
                   (col == width-1 || smoke_map[row][col+1] > v)
                {
                    total_risk += v + 1;
                    basins.push(Self::basin_size(&smoke_map, row, col));
                }
            }
        }
        println!("Total risk: {}", total_risk);

        // Part 2
        basins.sort();
        basins.reverse();
        println!("Three largest basins: {} {} {} (product: {})",
                 basins[0], basins[1], basins[2],
                 basins[0] * basins[1] * basins[2]);

        Ok(())
    }
}

impl Solver {
    fn basin_size(smoke_map: &Vec<Vec<u32>>, row: usize, col: usize) -> usize
    {
        let height = smoke_map.len();
        let width = smoke_map[0].len();
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        // BFS
        queue.push_back((row, col));
        while queue.len() > 0 {
            let (row, col) = queue.pop_front().unwrap();
            if smoke_map[row][col] == 9 {
                continue;
            }
            basin.insert((row, col));
            if row > 0 && !basin.contains(&(row-1, col)) {
                queue.push_back((row-1, col));
            }
            if row < height-1 && !basin.contains(&(row+1, col)) {
                queue.push_back((row+1, col));
            }
            if col > 0 && !basin.contains(&(row, col-1)) {
                queue.push_back((row, col-1));
            }
            if col < width-1 && !basin.contains(&(row, col+1)) {
                queue.push_back((row, col+1));
            }
        }

        basin.len()
    }
}
