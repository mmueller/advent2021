use crate::advent::AdventSolver;
use crate::shared::io;
use anyhow::{Error, format_err};
use std::collections::HashSet;

#[derive(Default)]
pub struct Solver;

#[derive(Copy,Clone,Debug)]
enum Fold {
    X(u64),
    Y(u64),
}

#[derive(Copy,Clone,Debug,Eq,Hash,PartialEq)]
struct Pos {
    x: u64,
    y: u64,
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let (mut points, folds) = Self::read_code_from_manual(input_path)?;
        println!("Read {} points and {} fold instructions.",
                 points.len(), folds.len());

        for fold in folds.iter() {
            Self::fold_points(&mut points, fold);
            println!("After fold {:?}, number of points: {}",
                     fold, points.len());
        }

        println!("");
        Self::render_points(&points);
        Ok(())
    }
}

impl Solver {
    fn read_code_from_manual(input_path: &str)
            -> Result<(HashSet<Pos>, Vec<Fold>), Error>
    {
        let lines = io::read_lines_from_file(input_path)?;
        let mut iter = lines.iter();
        let mut points = HashSet::new();
        let mut folds = Vec::new();

        // Read points
        for line in &mut iter {
            if line.len() == 0 {
                break;
            }
            let mut parts = line.split(",");
            let x = parts.next().unwrap().parse::<u64>()?;
            let y = parts.next().unwrap().parse::<u64>()?;
            points.insert(Pos { x: x, y: y });
        }

        // Read folds
        for line in &mut iter {
            let mut parts = line.split("=");
            let axis = parts.next().unwrap().chars().last().unwrap();
            let value = parts.next().unwrap().parse::<u64>()?;
            folds.push(match axis {
                'x' => Fold::X(value),
                'y' => Fold::Y(value),
                _ => return Err(format_err!("Bad axis: {}", axis)),
            });
        }

        Ok((points, folds))
    }

    fn fold_points(points: &mut HashSet<Pos>, fold: &Fold) {
        // Points that will be re-added (after folding) to the set
        let mut new_points = Vec::new();

        // Predicate keeps points unaffected by fold. Points affected by fold
        // are saved in new_points Vec for insertion after this loop.
        points.retain(|point| {
            match fold {
                Fold::X(value) => {
                    if point.x > *value {
                        new_points.push(Pos {x: 2*value-point.x, y: point.y});
                        false
                    } else {
                        true
                    }
                },
                Fold::Y(value) => {
                    if point.y > *value {
                        new_points.push(Pos {x: point.x, y: 2*value-point.y});
                        false
                    } else {
                        true
                    }
                },
            }
        });
        for point in new_points {
            points.insert(point);
        }
    }

    // Draw the state of the given points in the console.
    fn render_points(points: &HashSet<Pos>) {
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                if points.contains(&Pos { x: x, y: y}) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        println!("");
    }
}
