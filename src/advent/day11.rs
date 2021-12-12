use crate::advent::AdventSolver;
use anyhow::Error;

#[derive(Default)]
pub struct Solver {
    octopi: Vec<u8>,
}

const MY_INPUT: [u8; 100] = [
    1, 5, 5, 3, 4, 2, 1, 2, 8, 8,
    5, 2, 5, 5, 3, 8, 4, 8, 8, 2,
    1, 2, 2, 4, 3, 1, 5, 7, 3, 2,
    4, 2, 5, 8, 2, 4, 2, 2, 7, 4,
    1, 6, 5, 8, 5, 6, 4, 2, 1, 6,
    6, 8, 7, 2, 6, 5, 1, 1, 8, 2,
    5, 7, 7, 5, 5, 5, 2, 2, 3, 8,
    5, 6, 2, 2, 5, 4, 5, 1, 7, 2,
    8, 7, 6, 6, 6, 7, 2, 3, 1, 8,
    2, 1, 7, 8, 3, 7, 4, 8, 3, 5,
];

const EXAMPLE: [u8; 100] = [
    5, 4, 8, 3, 1, 4, 3, 2, 2, 3,
    2, 7, 4, 5, 8, 5, 4, 7, 1, 1,
    5, 2, 6, 4, 5, 5, 6, 1, 7, 3,
    6, 1, 4, 1, 3, 3, 6, 1, 4, 6,
    6, 3, 5, 7, 3, 8, 5, 4, 7, 8,
    4, 1, 6, 7, 5, 2, 4, 6, 4, 5,
    2, 1, 7, 6, 8, 4, 1, 7, 2, 1,
    6, 8, 8, 2, 8, 8, 1, 1, 3, 4,
    4, 8, 4, 6, 8, 4, 8, 5, 5, 4,
    5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
];

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

impl AdventSolver for Solver {
    fn solve(&mut self, _: &str) -> Result<(), Error>
    {
        self.octopi = MY_INPUT.to_vec();
        let mut flash_count = 0;
        for step in 1.. {
            flash_count += self.step();

            // Part 1: Number of flashes within 100 steps
            if step == 100 {
                println!("After 100 steps:");
                self.display();
                println!("Total flashes: {}\n", flash_count);
            }
            
            // Part 2: Detect when all flash
            if self.octopi.iter().all(|&o| o == 0) {
                println!("All octopi flashed in sync after {} steps!", step);
                break;
            }
        }

        Ok(())
    }
}

impl Solver {
    fn step(&mut self) -> usize
    {
        let mut flash_count = 0;
        self.increment_all();
        loop {
            let flashes = self.flash();
            for flash in flashes.iter() {
                self.increment_neighbors(flash.0, flash.1);
            }
            if flashes.len() == 0 {
                break;
            }
            flash_count += flashes.len();
        }
        flash_count
    }

    fn display(&self)
    {
        for (i, v) in self.octopi.iter().enumerate() {
            if i % WIDTH == 0 {
                println!("");
            }
            print!("{}", v);
        }
        println!("\n");
    }

    fn flash(&mut self) -> Vec<(usize, usize)>
    {
        let mut flashes = Vec::new();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.get(x, y) > 9 {
                    self.set(x, y, 0);
                    flashes.push((x, y));
                }
            }
        }
        flashes
    }

    fn increment_all(&mut self)
    {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set(x, y, self.get(x, y)+1);
            }
        }
    }

    fn increment_neighbors(&mut self, x: usize, y: usize)
    {
        for offset in OFFSETS {
            let nx = x as isize + offset.0;
            let ny = y as isize + offset.1;
            if nx >= 0 && nx < WIDTH as isize &&
               ny >= 0 && ny < HEIGHT as isize
            {
                let v = self.get(nx as usize, ny as usize);
                if v > 0 {
                    self.set(nx as usize, ny as usize, v+1);
                }
            }
        }
    }

    fn get(&self, x: usize, y: usize) -> u8
    {
        self.octopi[y*WIDTH + x]
    }

    fn set(&mut self, x: usize, y: usize, v: u8)
    {
        self.octopi[y*WIDTH + x] = v;
    }
}
