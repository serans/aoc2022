use std::io;
use std::io::BufRead;
use std::collections::HashMap;


fn shape_value(c: char) -> i32 {
    match c {
        'A' => 0, 
        'B' => 1,
        'C' => 2,
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => panic!("Unknown shape")
    }
}

fn main() {
    let game1_points: HashMap<(i32,i32),i32> =
    // (opponent, me), points_for_my_move + points_vs_opponent
        [((0, 0), 1 + 3),
         ((1, 0), 1 + 0),
         ((2, 0), 1 + 6),
         ((0, 1), 2 + 6),
         ((1, 1), 2 + 3),
         ((2, 1), 2 + 0),
         ((0, 2), 3 + 0),
         ((1, 2), 3 + 6),
         ((2, 2), 3 + 3),].iter().cloned().collect();

    let game2_points: HashMap<(i32,i32),i32> =
    // (opponent, desired_result), points_for_my_move + points_vs_opponent
        [((0, 0), 3 + 0),
         ((0, 1), 1 + 3),
         ((0, 2), 2 + 6),
         ((1, 0), 1 + 0),
         ((1, 1), 2 + 3),
         ((1, 2), 3 + 6),
         ((2, 0), 2 + 0),
         ((2, 1), 3 + 3),
         ((2, 2), 1 + 6),].iter().cloned().collect();
    
    let mut score1 = 0;
    let mut score2 = 0;

    for line in io::stdin().lock().lines().flatten() {
        if line.is_empty() {
            break
        }

        let a = shape_value(line.chars().nth(0).unwrap());
        let b = shape_value(line.chars().nth(2).unwrap());

        score1 += game1_points.get(&(a, b)).unwrap();
        score2 += game2_points.get(&(a, b)).unwrap();

    }
    println!("part1: {}", score1);
    println!("part2: {}", score2);
}

