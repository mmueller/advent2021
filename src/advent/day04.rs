use crate::advent::AdventSolver;
use crate::shared::io;
use anyhow::{Error, format_err};

#[derive(Default)]
pub struct Solver {
    number_sequence: Vec<u8>,
    boards: Vec<Board>,
}

#[derive(Clone, Default)]
pub struct Board {
    cells: Vec<(u8, bool)>
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        self.read_input(input_path)?;
        let mut active_boards = self.boards.clone();
        for (i, &number) in self.number_sequence.iter().enumerate() {
            // Announce the number
            println!("Round {}, drew {}!", i, number);
            for board in active_boards.iter_mut() {
                board.mark_if_present(number);
                if board.won() {
                    println!("Winning board: {}", board.score(number));
                }
            }
            // Remove winners from the game
            active_boards = active_boards.iter()
                                         .filter(|board| !board.won())
                                         .map(|board| board.clone())
                                         .collect();
            if active_boards.len() == 0 {
                println!("No active boards left, ending game.");
                break;
            }
        }
        Ok(())
    }
}

impl Solver {
    fn read_input(&mut self, input_path: &str) -> Result<(), Error> {
        let lines = io::read_lines_from_file(input_path)?;
        self.load_number_sequence(&lines[0])?;
        self.read_boards(&lines[1..])
    }

    fn load_number_sequence(&mut self, sequence_str: &str) -> Result<(), Error>
    {
        self.number_sequence =
            sequence_str.split(",")
                        .map(|s| Ok(s.parse::<u8>()?))
                        .collect::<Result<Vec<u8>, Error>>()?;
        Ok(())
    }

    fn read_boards(&mut self, lines: &[String]) -> Result<(), Error>
    {
        if lines.len() == 0 {
            Ok(())
        } else if lines[0] != "" {
            Err(format_err!("Expected blank line, got: {}", lines[0]))
        } else {
            self.boards.push(Board::from_text(&lines[1..=5])?);
            self.read_boards(&lines[6..])?;
            Ok(())
        }
    }
}

impl Board {
    fn from_text(lines: &[String]) -> Result<Self, Error>
    {
        let mut board = Board::default();
        for line in lines {
            for v in line.split_whitespace() {
                board.cells.push((v.parse::<u8>()?, false));
            }
        }
        Ok(board)
    }

    fn get(&self, row: usize, col: usize) -> (u8, bool)
    {
        self.cells[row*5+col]
    }

    fn mark_if_present(&mut self, number: u8)
    {
        for mut cell in self.cells.iter_mut() {
            if cell.0 == number {
                cell.1 = true;
            }
        }
    }

    fn score(&self, number: u8) -> u64
    {
        let number = number as u64;
        self.cells.iter()
                  .filter(|c| c.1 == false)
                  .map(|c| c.0 as u64)
                  .sum::<u64>() * number
    }

    fn won(&self) -> bool
    {
        // Check rows
        for row in 0..5 {
            let mut marked = true;
            for col in 0..5 {
                if !self.get(row, col).1 {
                    marked = false;
                    break;
                }
            }
            if marked {
                return true;
            }
        }

        // Check columns
        for col in 0..5 {
            let mut marked = true;
            for row in 0..5 {
                if !self.get(row, col).1 {
                    marked = false;
                    break;
                }
            }
            if marked {
                return true;
            }
        }

        false
    }
}
