#![allow(dead_code)]

mod advent;
mod shared;

use argparse::{ArgumentParser, StoreOption, StoreTrue};

fn main() {
    let mut day: Option<usize> = None;
    let mut use_example: bool = false;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Advent of Code 2019");
        parser.refer(&mut day)
              .add_option(&["-d", "--day"], StoreOption,
                          "number of challenge to run");
        parser.refer(&mut use_example)
              .add_option(&["-e", "--example"], StoreTrue,
                          "Use the example input from the problem");
        parser.parse_args_or_exit();
    }
    match day {
        Some(ref day) => {
            match advent::solve(*day, use_example) {
                Ok(_) => {},
                Err(e) => println!("error: {}", e)
            }
        },
        None => println!("--day is required"),
    }
}
