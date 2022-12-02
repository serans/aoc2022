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
        [((0 , 0)  ,4),
         ((0 , 1)  ,1),
         ((0 , 2)  ,7),
         ((1 , 0)  ,8),
         ((1 , 1)  ,5),
         ((1 , 2)  ,2),
         ((2 , 0)  ,3),
         ((2 , 1)  ,9),
         ((2 , 2)  ,6),].iter().cloned().collect();

    let game2_points: HashMap<(i32,i32),i32> =
        [((0 , 0)  ,3+0),
         ((0 , 1)  ,1+3),
         ((0 , 2)  ,2+6),
         ((1 , 0)  ,1+0),
         ((1 , 1)  ,2+3),
         ((1 , 2)  ,3+6),
         ((2 , 0)  ,2+0),
         ((2 , 1)  ,3+3),
         ((2 , 2)  ,1+6),].iter().cloned().collect();
    
    let mut score1 = 0;
    let mut score2 = 0;

    for line in io::stdin().lock().lines().flatten() {
        if line.is_empty() {
            break
        }

        let a = shape_value(line.chars().nth(0).unwrap());
        let b = shape_value(line.chars().nth(2).unwrap());

        score1 += game1_points.get(&(b, a)).unwrap();
        score2 += game2_points.get(&(b, a)).unwrap();

    }
    println!("part1: {}", score1);
    println!("part2: {}", score2);
}

