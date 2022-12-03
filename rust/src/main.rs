mod dayone;
mod daytwo;
mod daythree;

use dayone::*;
use daytwo::*;
use daythree::*;
use std::env;

fn main() {
    let Some(day) = env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
    else {
        println!("Please pass the day you want to solve");
        return;
    };

    match day {
        1 => day_one(),
        2 => day_two(),
        3 => day_three(),
        _ => println!("Not a valid day!"),
    }
}
