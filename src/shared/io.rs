use anyhow::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn read_numbers_from_file<T>(path: &str) -> Result<Vec<T>, Error>
                       where T: FromStr,
           <T as FromStr>::Err: 'static + Send + Sync + std::error::Error {
    BufReader::new(File::open(path)?)
              .lines()
              .map(|line| Ok(line?.parse::<T>()?))
              .collect::<Result<Vec<T>, Error>>()
}

pub fn read_lines_from_file(path: &str) -> Result<Vec<String>, Error> {
    BufReader::new(File::open(path)?)
              .lines()
              .map(|line| Ok(line?))
              .collect::<Result<Vec<String>, _>>()
}
