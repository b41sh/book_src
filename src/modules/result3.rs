use std::io;
use std::num;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

// 定义类型别名
type Result<I32> = std::result::Result<I32, CliError>;

pub fn run(filename: &str) -> Result<i32> {

    File::open(filename).map_err(CliError::Io)
    .and_then(|mut f| {
        let mut contents = String::new();
        f.read_to_string(&mut contents)
        .map_err(CliError::Io)
        .map(|_|contents)
    })
    .and_then(|contents| {
        let mut sum = 0;
        for c in contents.lines() {
            match c.parse::<i32>() {
                Ok(n) => { sum += n },
                //Err(err) => { let _err: Box<dyn std::error::Error> = err.into(); },
                Err(err) => { return Err(CliError::Parse(err)); },
            }
        }
        Ok(sum)
    })
}
