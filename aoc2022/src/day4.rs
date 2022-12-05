use std::fs::File;
use std::io::{BufReader, Lines};

fn extract_range(range: &str) -> (u32, u32) {
    let range = range.split('-').collect::<Vec<&str>>();
    (range.get(0).unwrap().parse::<u32>().unwrap(),
     range.get(1).unwrap().parse::<u32>().unwrap())
}

fn contains(r1: (u32,u32), r2:(u32, u32)) -> bool {
    r1.0 <= r2.0 && r1.1 >= r2.1
}

fn overlaps(r1: (u32,u32), r2:(u32, u32)) -> bool {
    r1.1 >= r2.0 && r1.0 <= r2.1 
}

#[allow(dead_code)]
pub fn solve(lines: Lines<BufReader<File>>) {
    let mut count1 = 0;
    let mut count2 = 0;
    for line in lines.flatten() {
        let line = line.split(',').collect::<Vec<&str>>(); 
        let elve1 = extract_range(line.get(0).unwrap());
        let elve2 = extract_range(line.get(1).unwrap());
        if contains(elve1, elve2) || contains(elve2, elve1) {
            count1 += 1;
        }
        if overlaps(elve1, elve2) {
            count2 += 1;
        }
    }
    println!("# of sections fully contained in antoher: {}", count1);
    println!("# of overlapping sections: {}", count2);
}
