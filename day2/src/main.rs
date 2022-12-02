use std::io;
use std::io::BufRead;

fn main() {
    
    let mut score1 = 0;
    let mut score2 = 0;

    for line in io::stdin().lock().lines().flatten() {
        if line.is_empty() {
            break
        }

        score1 += match line.as_str() {
            // opponent, player => points_for_shape + points_against_opponent
            "A X" => 1 + 3,
            "B X" => 1 + 0,
            "C X" => 1 + 6,
            "A Y" => 2 + 6,
            "B Y" => 2 + 3,
            "C Y" => 2 + 0,
            "A Z" => 3 + 0,
            "B Z" => 3 + 6,
            "C Z" => 3 + 3,
            _ => panic!("wrong input")
        };

        score2 += match line.as_str() {
            // opponent, strategy => points_for_shape + points_against_opponent
            "A X" => 3 + 0,
            "B X" => 1 + 0,
            "C X" => 2 + 0,
            "A Y" => 1 + 3,
            "B Y" => 2 + 3,
            "C Y" => 3 + 3,
            "A Z" => 2 + 6,
            "B Z" => 3 + 6,
            "C Z" => 1 + 6,
            _ => panic!("wrong input")
        };
    }

    println!("part1: {}", score1);
    println!("part2: {}", score2);
}

