use crate::advent::AdventSolver;
use crate::shared::io;
use anyhow::{format_err, Error};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Default)]
pub struct Solver;

// Non-mangled segment labels from problem description (we'll map to these when
// we decipher data).
//
//   𝗮𝗮𝗮𝗮    ....    𝗮𝗮𝗮𝗮    𝗮𝗮𝗮𝗮    ....
//  𝗯    𝗰  .    𝗰  .    𝗰  .    𝗰  𝗯    𝗰
//  𝗯    𝗰  .    𝗰  .    𝗰  .    𝗰  𝗯    𝗰
//   ....    ....    𝗱𝗱𝗱𝗱    𝗱𝗱𝗱𝗱    𝗱𝗱𝗱𝗱
//  𝗲    𝗳  .    𝗳  𝗲    .  .    𝗳  .    𝗳
//  𝗲    𝗳  .    𝗳  𝗲    .  .    𝗳  .    𝗳
//   𝗴𝗴𝗴𝗴    ....    𝗴𝗴𝗴𝗴    𝗴𝗴𝗴𝗴    ....
//
//   𝗮𝗮𝗮𝗮    𝗮𝗮𝗮𝗮    𝗮𝗮𝗮𝗮    𝗮𝗮𝗮𝗮    𝗮𝗮𝗮a
//  𝗯    .  𝗯    .  .    𝗰  𝗯    𝗰  𝗯    𝗰
//  𝗯    .  𝗯    .  .    𝗰  𝗯    𝗰  𝗯    𝗰
//   𝗱𝗱𝗱𝗱    𝗱𝗱𝗱𝗱    ....    𝗱𝗱𝗱𝗱    𝗱𝗱𝗱𝗱
//  .    𝗳  𝗲    𝗳  .    𝗳  𝗲    𝗳  .    𝗳
//  .    𝗳  𝗲    𝗳  .    𝗳  𝗲    𝗳  .    𝗳
//   𝗴𝗴𝗴𝗴    𝗴𝗴𝗴𝗴    ....    𝗴𝗴𝗴𝗴    𝗴𝗴𝗴𝗴
//
lazy_static! {
    static ref DIGIT_MAP: HashMap<&'static str, u64> = [
        ("abcefg",  0),
        ("cf",      1),
        ("acdeg",   2),
        ("acdfg",   3),
        ("bcdf",    4),
        ("abdfg",   5),
        ("abdefg",  6),
        ("acf",     7),
        ("abcdefg", 8),
        ("abcdfg",  9),
    ].into_iter().collect();
}

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error>
    {
        // For part 1
        let mut count1478 = 0;

        // For part 2
        let mut total = 0;

        for line in io::read_lines_from_file(input_path)?.iter() {
            let mut parts = line.split(" | ");
            let patterns = parts.next()
                                .ok_or(format_err!("Parse error: {}", line))?;
            let output = parts.next()
                              .ok_or(format_err!("Parse error: {}", line))?;

            let map = Self::find_wire_mapping(patterns)
                           .ok_or(format_err!("Failed to map: {}", patterns))?;
            let mut result = 0;
            for output in output.split_whitespace() {
                let digit = Self::unscramble_digit(output, &map).unwrap();
                if [1, 4, 7, 8].contains(&digit) {
                    count1478 += 1;
                }
                result *= 10;
                result += digit;
            }
            total += result;
        }

        println!("Count of 1, 4, 7, and 8 digits: {}", count1478);
        println!("Total sum: {}", total);

        Ok(())
    }
}

impl Solver {
    fn find_wire_mapping(patterns: &str) -> Option<HashMap<char, char>>
    {
        // Try every possible mapping of a-g -> a-g:
        let segments = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        println!("Decoding: {}\n\n\n\n\n\n\n", patterns);
        'outer: for permutation in segments.iter().permutations(7) {
            let map = segments.iter()
                              .zip_eq(permutation)
                              .map(|(a, b)| (*a, *b))
                              .collect::<HashMap<char, char>>();
            Self::render_patterns(patterns, &map);
            for pattern in patterns.split_whitespace() {
                match Self::unscramble_digit(pattern, &map) {
                    Some(_digit) => {},
                    None => continue 'outer,
                }
            }

            // Found mapping
            print!("\x1B[?25h"); // Show cursor
            println!("");
            return Some(map);
        }

        None
    }

    fn map_pattern(word: &str, map: &HashMap<char, char>) -> Vec<char>
    {
        let mut mapped = word.chars()
                             .map(|c| map[&c])
                             .collect::<Vec<char>>();
        mapped.sort();
        mapped
    }

    fn render_patterns(patterns: &str, map: &HashMap<char, char>)
    {
        let mapped = patterns.split_whitespace()
                             .map(|p| Self::map_pattern(p, map))
                             .collect::<Vec<Vec<char>>>();
        // Move cursor up 7 lines for each rendering
        print!("\x1B[7A\r\x1B[?25l");
        for i in 0..7 {
            for m in mapped.iter() {
                match i {
                    0|3|6 => {
                        if i == 0 && m.contains(&'a') ||
                           i == 3 && m.contains(&'d') ||
                           i == 6 && m.contains(&'g') {
                            print!(" ————  ");
                        } else {
                            print!("       ");
                        }
                    },
                    1|2|4|5 => {
                        if i < 3 && m.contains(&'b') ||
                           i > 3 && m.contains(&'e') {
                            print!("|    ");
                        } else {
                            print!("     ");
                        }
                        if i < 3 && m.contains(&'c') ||
                           i > 3 && m.contains(&'f') {
                            print!("| ");
                        } else {
                            print!("  ");
                        }
                    },
                    _ => {},
                }
            }
            println!("");
        }
    }

    fn unscramble_digit(word: &str, map: &HashMap<char, char>) -> Option<u64>
    {
        let mapped = Self::map_pattern(word, map).iter().join("");
        DIGIT_MAP.get(&*mapped).map(|n| *n)
    }
}
