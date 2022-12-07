use std::fs::File;
use std::io::{BufReader, Lines};

#[allow(dead_code)]
pub fn solve(lines: Lines<BufReader<File>>) {
    let mut calories: Vec<u32> = Vec::new();
    let mut current_calories: u32 = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            calories.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<u32>().unwrap();
        }
    }

    calories.sort_by(|a, b| b.cmp(a)); // reverse sorting order (ie descending)
    println!("Top elve carries: {}", calories[0]);
    let top3: u32 = calories.iter().take(3).sum();
    println!("Top 3 elves carry: {}", top3);
}
