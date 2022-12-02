#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock = 0,
    Scissors = 1,
    Paper = 2,
}

impl Shape {

    fn against(&self, other: &Self) -> Result {
        let x = *self as i32;
        let y = *other as i32;
        if x == y  {
            Result::Draw
        } else if (x+1) %3 == y {
            Result::Win
        } else if (y+1) %3 == x {
            Result::Lose
        } else {
            panic!("wtdf {} ({}) {} ({})", x, (x+1)%3, y, (y+1)%3);
        }
    }

    fn value(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn vs(&self, other:&Self) -> i32 {
        self.against(&other).value() + self.value()
    }

    fn get_move(&self, result: &Result) -> Shape {
        let x = *self as i32;
        match result {
            Result::Lose => Shape::fromu(x+1),
            Result::Draw => Shape::fromu(x),
            Result::Win => Shape::fromu(x-1),
        }
    }

    fn from(c: char) -> Shape {
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

    fn fromu(n: i32) -> Shape {
        let n = (n %3).abs();
        match n {
            0 => Shape::Rock,
            1 => Shape::Scissors,
            2 => Shape::Paper,
            _ => panic!("This cannot happen {}", n)
        } 
    }

}

#[derive(Debug, Clone, Copy)]
enum Result {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from(c : char) -> Result {
        match c {
            'X' => Result::Lose,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!("wrong result type")
        }
    }
}


use std::io;
use std::io::BufRead;

fn main() {
    let mut score = 0;
    let mut score2 = 0;

    for line in io::stdin().lock().lines().flatten() {
        if line.is_empty() {
            continue
        }
        let other = Shape::from(line.chars().nth(0).unwrap());
        let me = Shape::from(line.chars().nth(2).unwrap());
        // PROBLEM 1
        score += me.vs(&other);

        // PROBLEM 2
        let result = Result::from(line.chars().nth(2).unwrap());
        let my_move = other.get_move(&result);
        println!("{}, other:{:?} vs me:{:?} => {:?}", line, other, my_move, result);
        score2 += my_move.vs(&other);

    }

    println!("PROBLEM 1: {}", score);
    println!("PROBLEM 2: {}", score2);
}
