use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        match &self {
            Shape::Rock => match &other {
                Shape::Rock     => Some(Ordering::Equal),
                Shape::Paper    => Some(Ordering::Less),
                Shape::Scissors => Some(Ordering::Greater),
            },
            Shape::Paper => match &other {
                Shape::Rock     => Some(Ordering::Greater),
                Shape::Paper    => Some(Ordering::Equal),
                Shape::Scissors => Some(Ordering::Less),
            },
            Shape::Scissors => match &other {
                Shape::Rock     => Some(Ordering::Less),
                Shape::Paper    => Some(Ordering::Greater),
                Shape::Scissors => Some(Ordering::Equal),
            }
        }
    }
}

impl Shape {
    fn value(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn score(&self, other:&Self) -> u32 {
        if other < self {
            6
        } else if other > self {
            0
        } else {
            3
        }
    }

    fn vs(&self, other:&Self) -> u32 {
        self.score(&other) + self.value()
    }
}

fn get_shape(c : char) -> Shape {
    match c {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        _ => panic!("Unknown shape")
    }
}

use std::io;
use std::io::BufRead;

fn main() {
    println!("Hello, world!");
    let mut score = 0;
    for line in io::stdin().lock().lines().flatten() {
        if line.is_empty() {
            continue
        }
        let other = get_shape(line.chars().nth(0).unwrap());
        let me = get_shape(line.chars().nth(2).unwrap());
        score += me.vs(&other);
    }
    println!("{}", score);
}
