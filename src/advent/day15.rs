use crate::advent::AdventSolver;
use crate::shared::io;
use anyhow::{Error, format_err};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Solver;

#[derive(Copy,Clone,Debug,Eq,Hash,PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let grid = Self::read_grid(input_path)?;
        let target = Pos {
            row: grid.len()-1,
            col: grid[0].len()-1,
        };
        let distance = Self::shortest_path_length(
                           &grid, Pos { row: 0, col: 0 }, target);
        println!("Shortest distance (original graph): {}", distance);

        let grid = Self::enlarge_grid(&grid);
        let target = Pos {
            row: grid.len()-1,
            col: grid[0].len()-1,
        };
        let distance = Self::shortest_path_length(
                           &grid, Pos { row: 0, col: 0 }, target);
        println!("Shortest distance (enlarged graph): {}", distance);
        Ok(())
    }
}

impl Solver {
    // Dijkstra (thanks, Wikipedia)
    fn shortest_path_length(grid: &Vec<Vec<u32>>, start: Pos, end: Pos) -> u32
    {
        // 1. Mark all nodes unvisited
        let mut unvisited: HashSet<Pos> = HashSet::new();
        // 2. Create a tentative distance for each point
        let mut tentative_distance: HashMap<Pos, u32> = HashMap::new();
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                tentative_distance.insert(Pos { row, col }, u32::MAX);
                unvisited.insert(Pos { row, col });
            }
        }
        *tentative_distance.get_mut(&start).unwrap() = 0;

        // 3. Consider all unvisited neighbors of current point
        let mut current = start;
        while unvisited.contains(&end) {
            let neighbors = Self::neighbors(grid, current);
            for &neighbor in neighbors.iter() {
                if unvisited.contains(&neighbor) {
                    let distance = tentative_distance[&current] +
                                   grid[neighbor.row][neighbor.col];
                    if tentative_distance[&neighbor] > distance {
                        tentative_distance.insert(neighbor, distance);
                    }
                }
            }
            unvisited.remove(&current);
            match unvisited.iter()
                           .min_by_key(|p| tentative_distance[p]) {
                Some(node) => current = *node,
                None => break,
            }
        }

        tentative_distance[&end]
    }

    fn enlarge_grid(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>>
    {
        let mut result = Vec::new();
        for i in 0..5 {
            for row in 0..grid.len() {
                let mut new_row = Vec::new();
                for j in 0..5 {
                    for col in 0..grid[0].len() {
                        let mut v = grid[row][col] + i + j;
                        while v > 9 {
                            v -= 9;
                        }
                        new_row.push(v);
                    }
                }
                result.push(new_row);
            }
        }
        result
    }

    fn neighbors(grid: &Vec<Vec<u32>>, pos: Pos) -> Vec<Pos>
    {
        let mut neighbors = Vec::new();
        if pos.row > 0 {
            neighbors.push(Pos { row: pos.row-1, col: pos.col });
        }
        if pos.col > 0 {
            neighbors.push(Pos { row: pos.row, col: pos.col-1 });
        }
        if pos.row < grid.len() - 1 {
            neighbors.push(Pos { row: pos.row+1, col: pos.col });
        }
        if pos.col < grid[0].len() - 1 {
            neighbors.push(Pos { row: pos.row, col: pos.col+1 });
        }
        neighbors
    }

    fn path_weight(grid: &Vec<Vec<u32>>, path: &Vec<Pos>) -> u32
    {
        path.iter()
            .map(|p| grid[p.row][p.col])
            .sum()
    }

    fn read_grid(input_path: &str) -> Result<Vec<Vec<u32>>, Error>
    {
        let lines = io::read_lines_from_file(input_path)?;
        let mut result = Vec::new();
        for line in lines.iter() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10)
                          .ok_or(format_err!("Bad digit: {}", c))?);
            }
            result.push(row);
        }

        Ok(result)
    }
}
