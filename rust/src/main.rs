mod dayone;
mod daytwo;
mod daythree;

use dayone::*;
use daytwo::*;
use daythree::*;
use std::env;
use std::fs::File;
use std::io::{Read, self};

fn input(day: usize) -> Result<String, io::Error> {
    let mut file = File::open(&format!("../input/day{day}.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

fn main() {
    let Some(day) = env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
    else {
        println!("Please pass the day you want to solve");
        return;
    };

    let input = match input(day) {
        Ok(string) => string,
        Err(e) => panic!("Error reading file: {e:?}"),
    };

    match day {
        1 => day_one(input),
        2 => day_two(input),
        3 => day_three(input),
        _ => println!("Not a valid day!"),
    }
}
