use std::fs::File;
use std::io::{BufReader, Lines};

fn priority(c: &u8) -> usize {
    let mut c = *c as usize;
    if c >= 97 {
        c = c - 96;
    } else if c >= 65 {
        c = c - 38
    } else {
        panic!("unknown item");
    }
    c
}

fn get_repeated_item_priority(line: &String) -> usize {
    let mut values:[bool; 52] = [false; 52];
    let half = line.len()/2; 
    for (i, item) in line.as_bytes().iter().enumerate() {
        let priority = priority(item);
        if i < half {
            values[priority-1] = true;
        } else if values[priority-1] {
            return priority
        }
    }
    panic!("No duplicate found")
}

fn get_group_badge(line1: &String, line2: &String, line3: &String) -> usize {
    let mut values:[u8; 52] = [0; 52];
    for item in line1.as_bytes() {
        let priority = priority(item);
        values[priority-1] = 1;
    }
    for item in line2.as_bytes() {
        let priority = priority(item);
        if values[priority-1] == 1 {
            values[priority-1] = 2;
        }
    }
    for item in line3.as_bytes() {
        let priority = priority(item);
        if values[priority-1] == 2 {
            return priority;
        }
    }
    panic!("no group badge found");
}

#[allow(dead_code)]
pub fn solve1(lines: Lines<BufReader<File>>) {
    let mut score = 0;

    for line in lines {
        score += get_repeated_item_priority(&line.unwrap());
    }
    println!("problem1: {}", score);

}

#[allow(dead_code)]
pub fn solve2(lines: Lines<BufReader<File>>) {
    let mut score = 0;

    let lines = lines.flatten().collect::<Vec<String>>();
    for c in lines.chunks_exact(3) {
        score += get_group_badge(&c[0], &c[1], &c[2]); 
    }
    println!("problem2: {}", score);
}
