use std::collections::HashSet;

fn all_elements_unique(text: &Vec<char>) -> bool {
    let letters = HashSet::<char>::from_iter(text.iter().cloned());
    text.len() == letters.len()
}

fn find_unique_sequence(len: usize, text: &String) -> usize {
    let mut buff = std::vec::from_elem(' ', len);
    for (i,c) in text.chars().enumerate() {
        let c = c;
        buff[i % len] = c;
        if i > len && all_elements_unique(&buff) {
            return i+1
        }
    }
    panic!("no solution")
}

#[allow(dead_code)]
pub fn solve(mut lines: impl Iterator<Item=String>) {
    let text = lines.next().unwrap();
    println!("Problem 1: {}", find_unique_sequence( 4, &text));
    println!("Problem 2: {}", find_unique_sequence(14, &text));
}
