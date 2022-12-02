use std::fs::File;
use std::io::prelude::*;

pub fn day_one() {
    let mut file = File::open("../input/day1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut elves = vec![0];

    for line in contents.lines() {
        if line.is_empty() {
            elves.push(0);
        } else {
            *elves.last_mut().unwrap() += line.parse::<usize>().unwrap();
        }
    }

    elves.sort_by(|x, y| y.cmp(x));
    let max = elves[0];
    let top_three: usize = elves.iter().take(3).sum();

    println!(
        "The elf with the most calories is carrying {} calories",
        max
    );
    println!("The top three elves are carrying {} calories", top_three);
}
