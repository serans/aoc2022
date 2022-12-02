use std::io;
use std::io::BufRead;
use std::collections::HashMap;


fn part1() {
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

    let mut score = 0;
    /*
    possible games:
     R vs R  1 + 3 = 4
     R vs P  1 + 0 = 1
     R vs S  1 + 6 = 7
     P vs R  2 + 6 = 8
     P vs P  2 + 3 = 5
     P vs S  2 + 0 = 2
     S vs R  3 + 0 = 3
     S vs P  3 + 6 = 9
     S vs S  3 + 3 = 6
    */
    let game_points: HashMap<(i32,i32),i32> =
        [((0 , 0)  ,4),
         ((0 , 1)  ,1),
         ((0 , 2)  ,7),
         ((1 , 0)  ,8),
         ((1 , 1)  ,5),
         ((1 , 2)  ,2),
         ((2 , 0)  ,3),
         ((2 , 1)  ,9),
         ((2 , 2)  ,6),].iter().cloned().collect();

    for line in io::stdin().lock().lines().flatten() {
        if line.is_empty() {
            break
        }

        // problem 1
        let other = shape_value(line.chars().nth(0).unwrap());
        let me = shape_value(line.chars().nth(2).unwrap());

        score += game_points.get(&(me, other)).unwrap();

    }

    println!("PROBLEM 1: {}", score);
}

fn part2() {
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

    let mut score = 0;
    let game_points: HashMap<(i32,i32),i32> =
        [((0 , 0)  ,3+0),
         ((0 , 1)  ,1+3),
         ((0 , 2)  ,2+6),
         ((1 , 0)  ,1+0),
         ((1 , 1)  ,2+3),
         ((1 , 2)  ,3+6),
         ((2 , 0)  ,2+0),
         ((2 , 1)  ,3+3),
         ((2 , 2)  ,1+6),].iter().cloned().collect();

    for line in io::stdin().lock().lines().flatten() {
        if line.is_empty() {
            break
        }

        // problem 1
        let other = shape_value(line.chars().nth(0).unwrap());
        let me = shape_value(line.chars().nth(2).unwrap());

        score += game_points.get(&(me, other)).unwrap();

    }

    println!("PROBLEM 2: {}", score);
}
fn main() {
    //part1();
    part2();
}
