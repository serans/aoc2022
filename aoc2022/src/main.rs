mod day1;
mod day2;

use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn main() {
    day1::solve(read_lines("input/day1.txt"));
    day2::solve(read_lines("input/day2.txt"));
}

