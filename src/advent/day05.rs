use crate::advent::AdventSolver;
use crate::pos;
use crate::shared::grid::{InfiniteGrid, Pos};
use crate::shared::io;
use anyhow::{Error, format_err};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Default)]
pub struct Solver;

pub struct Line {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)")
              .unwrap();
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error>
    {
        let lines = Self::read_lines(&input_path)?;
        let mut map: InfiniteGrid<u64> = InfiniteGrid::new(0);
        for line in lines.iter() {
            let mut x = line.x1;
            let mut y = line.y1;
            let xstep = line.x2.cmp(&line.x1) as isize;
            let ystep = line.y2.cmp(&line.y1) as isize;
            // For part 1, uncomment
            //if xstep != 0 && ystep != 0 {
                //continue;
            //}
            loop {
                map[pos!(x, y)] += 1;
                if x == line.x2 && y == line.y2 {
                    break;
                }
                x += xstep;
                y += ystep;
            }
        }
        println!("Overlapping points: {}",
                 map.crop()
                    .iter()
                    .map(|row| row.iter().filter(|&&v| v > 1).count())
                    .sum::<usize>());
        Ok(())
    }
}

impl Solver {
    fn read_lines(input_path: &str) -> Result<Vec<Line>, Error>
    {
        let mut result = Vec::new();
        let lines = io::read_lines_from_file(input_path)?;
        for line in lines {
            match LINE_REGEX.captures(&line) {
                Some(caps) => {
                    result.push(Line {
                        x1: caps["x1"].parse::<isize>()?,
                        y1: caps["y1"].parse::<isize>()?,
                        x2: caps["x2"].parse::<isize>()?,
                        y2: caps["y2"].parse::<isize>()?,
                    });
                },
                None => {
                    return Err(format_err!("Parse error: {}", line));
                },
            }
        }
        Ok(result)
    }
}
