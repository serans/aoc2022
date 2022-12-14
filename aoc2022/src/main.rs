mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn main() {
    //day1::solve(read_lines("input/day1.txt"));
    //day2::solve(read_lines("input/day2.txt"));
    //day3::solve1(read_lines("input/day3.txt"));
    //day3::solve2(read_lines("input/day3.txt"));
    //day4::solve(read_lines("input/day4.txt"));
    //day5::solve(read_lines("input/day5.txt").flatten());
    //day6::solve(read_lines("input/day6.txt").flatten());
    //day7::solve(read_lines("input/day7.txt").flatten());
    //day8::solve(read_lines("input/day8.txt").flatten());
    //day9::solve(read_lines("input/day9.txt").flatten());
    //day10::solve(read_lines("input/day10.txt").flatten());
    //day11::solve();
    //day12::solve(read_lines("input/day12_test.txt").flatten());
    //day12::solve(read_lines("input/day12.txt").flatten());
    //day13::solve(read_lines("input/day13_test.txt").flatten());
    //day13::solve(read_lines("input/day13.txt").flatten());
    //day14::solve(read_lines("input/day14_test.txt").flatten());
    //day14::solve(read_lines("input/day14.txt").flatten());
    //day15::solve(read_lines("input/day15_test.txt").flatten());
    //day15::solve(read_lines("input/day15.txt").flatten());
    //day16::solve(read_lines("input/day16.txt").flatten());
    day17::solve(read_lines("input/day17_test.txt").flatten());
}
